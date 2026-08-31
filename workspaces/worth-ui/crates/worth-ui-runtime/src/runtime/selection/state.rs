use std::collections::{BTreeMap, BTreeSet};

mod declared_activation;
mod inspection;
mod lifecycle;
mod record;

use record::empty_record;
pub(super) use record::validate_catalog;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiSelectionOwnerRecord {
    pub(super) incarnation: super::UiSelectionOwnerIncarnation,
    pub(super) policy: super::UiSelectionPolicy,
    pub(super) catalog: std::sync::Arc<[super::UiSelectionStableKey]>,
    pub(super) catalog_positions: std::sync::Arc<BTreeMap<super::UiSelectionStableKey, usize>>,
    pub(super) catalog_posture: super::UiSelectionCatalogPosture,
    pub(super) catalog_revision: u64,
    pub(super) catalog_available: bool,
    pub(super) selected: BTreeSet<super::UiSelectionStableKey>,
    pub(super) anchor: Option<super::UiSelectionStableKey>,
    pub(super) cursor: Option<super::UiSelectionStableKey>,
}

/// Sole owner of selection, range anchor, and selection cursor state. Query
/// contributes opaque stable row correlation only.
#[derive(Clone)]
pub(crate) struct UiSelectionRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
    policy: crate::declaration::UiSelectionPolicy,
    pub(super) owners: BTreeMap<super::UiSelectionOwnerIdentity, UiSelectionOwnerRecord>,
    pub(super) revision: u64,
    requests: u64,
    candidates_visited: u64,
    catalog_keys_reconciled: u64,
    pub(super) mounted_owners: BTreeMap<
        (
            worth_ui_host_contract::UiSemanticSurfaceIdentity,
            crate::graph::UiGraphNodeIdentity,
            super::UiSelectionOwnerIncarnation,
        ),
        BTreeSet<super::UiSelectionOwnerIdentity>,
    >,
    family_owners: BTreeMap<
        crate::runtime::UiApplicationItemKeyFamily,
        BTreeSet<super::UiSelectionOwnerIdentity>,
    >,
    last_drop: Option<super::UiSelectionDropInspectionRecord>,
}

impl UiSelectionRuntimeState {
    pub(crate) const fn new_session_restore_candidate() -> Self {
        Self::new_session_restore_candidate_with_policy(
            crate::declaration::UiSelectionPolicy::single(),
        )
    }

    pub(crate) const fn new_session_restore_candidate_with_policy(
        policy: crate::declaration::UiSelectionPolicy,
    ) -> Self {
        Self {
            persistence: crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate,
            policy,
            owners: BTreeMap::new(),
            revision: 0,
            requests: 0,
            candidates_visited: 0,
            catalog_keys_reconciled: 0,
            mounted_owners: BTreeMap::new(),
            family_owners: BTreeMap::new(),
            last_drop: None,
        }
    }

    pub(crate) fn apply_policy(&mut self, policy: crate::declaration::UiSelectionPolicy) {
        self.policy = policy;
    }

    pub(crate) const fn default_owner_policy(&self) -> super::UiSelectionPolicy {
        match self.policy.mode() {
            crate::declaration::UiSelectionMode::Single => super::UiSelectionPolicy::Single,
            crate::declaration::UiSelectionMode::Multiple => super::UiSelectionPolicy::Multiple,
            crate::declaration::UiSelectionMode::Range => {
                super::UiSelectionPolicy::MultipleWithRange
            }
        }
    }

    pub(crate) fn synchronize(
        &mut self,
        registration: super::UiSelectionRegistration,
    ) -> Result<super::UiSelectionReconciliationReceipt, super::UiSelectionRequestDenial> {
        let owner = registration.owner();
        if registration.catalog_revision() != 0
            && self.catalog_is_current(
                owner,
                registration.incarnation(),
                registration.catalog_revision(),
            )
        {
            let selected = self
                .owners
                .get(&owner)
                .map_or(0, |record| record.selected.len());
            return Ok(super::UiSelectionReconciliationReceipt::new(
                super::UiSelectionDelta::new(Vec::new(), Vec::new(), selected, 0, self.revision),
                false,
                0,
            ));
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(super::UiSelectionRequestDenial::RevisionExhausted)?;
        let catalog_keys_reconciled = self
            .catalog_keys_reconciled
            .checked_add(u64::try_from(registration.catalog().len()).unwrap_or(u64::MAX))
            .ok_or(super::UiSelectionRequestDenial::CounterOverflow)?;
        let prior_incarnation = self.owners.get(&owner).map(|record| record.incarnation);
        let (order_changed, removed, missing_count, selected_count) = {
            let record = self
                .owners
                .entry(owner)
                .or_insert_with(|| empty_record(&registration));
            if record.incarnation != registration.incarnation() {
                *record = empty_record(&registration);
            }
            let order_changed = record.catalog.as_ref() != registration.catalog();
            let available = registration.catalog_positions();
            let missing = record
                .selected
                .iter()
                .filter(|key| !available.contains_key(key))
                .copied()
                .collect::<Vec<_>>();
            let complete =
                registration.catalog_posture() == super::UiSelectionCatalogPosture::Complete;
            let remove_missing = complete || !self.policy.preserves_stable_keys();
            let removed = if remove_missing {
                for key in &missing {
                    record.selected.remove(key);
                }
                if record
                    .anchor
                    .is_some_and(|key| !available.contains_key(&key))
                {
                    record.anchor = None;
                }
                if record
                    .cursor
                    .is_some_and(|key| !available.contains_key(&key))
                {
                    record.cursor = None;
                }
                missing.clone()
            } else {
                Vec::new()
            };
            record.policy = registration.policy();
            record.catalog = registration.catalog().to_vec().into();
            record.catalog_positions = std::sync::Arc::clone(registration.catalog_positions());
            record.catalog_posture = registration.catalog_posture();
            record.catalog_revision = registration.catalog_revision();
            record.catalog_available = true;
            (
                order_changed,
                removed,
                if remove_missing { 0 } else { missing.len() },
                record.selected.len(),
            )
        };
        if prior_incarnation != Some(registration.incarnation()) {
            if let Some(prior) = prior_incarnation {
                self.unindex_owner(owner, prior);
            }
            self.index_owner(owner, registration.incarnation());
        }
        self.revision = revision;
        self.catalog_keys_reconciled = catalog_keys_reconciled;
        let receipt = super::UiSelectionReconciliationReceipt::new(
            super::UiSelectionDelta::new(
                Vec::new(),
                removed,
                selected_count,
                u32::try_from(registration.catalog().len()).unwrap_or(u32::MAX),
                revision,
            ),
            order_changed,
            missing_count,
        );
        self.record_drop(
            owner,
            super::UiSelectionDropInspectionReason::CatalogReconciliation,
            receipt.delta(),
        );
        Ok(receipt)
    }

    pub(crate) fn synchronize_and_apply(
        &mut self,
        registration: super::UiSelectionRegistration,
        request: super::UiSelectionRequest,
    ) -> Result<
        (
            super::UiSelectionReconciliationReceipt,
            super::UiSelectionDelta,
        ),
        super::UiSelectionRequestDenial,
    > {
        let owner = registration.owner();
        let incarnation = registration.incarnation();
        let mut staged = Self {
            persistence: self.persistence,
            policy: self.policy,
            owners: self
                .owners
                .get(&owner)
                .cloned()
                .map(|record| BTreeMap::from([(owner, record)]))
                .unwrap_or_default(),
            revision: self.revision,
            requests: self.requests,
            candidates_visited: self.candidates_visited,
            catalog_keys_reconciled: self.catalog_keys_reconciled,
            mounted_owners: BTreeMap::new(),
            family_owners: BTreeMap::new(),
            last_drop: self.last_drop,
        };
        let reconciliation = staged.synchronize(registration)?;
        let delta = staged.apply(owner, incarnation, request)?;
        let record = staged
            .owners
            .remove(&owner)
            .expect("successful staged selection retains its exact owner");
        let prior_incarnation = self.owners.get(&owner).map(|record| record.incarnation);
        self.owners.insert(owner, record);
        if prior_incarnation != Some(incarnation) {
            if let Some(prior) = prior_incarnation {
                self.unindex_owner(owner, prior);
            }
            self.index_owner(owner, incarnation);
        }
        self.revision = staged.revision;
        self.requests = staged.requests;
        self.candidates_visited = staged.candidates_visited;
        self.catalog_keys_reconciled = staged.catalog_keys_reconciled;
        self.last_drop = staged.last_drop;
        Ok((reconciliation, delta))
    }

    pub(crate) fn apply(
        &mut self,
        owner: super::UiSelectionOwnerIdentity,
        incarnation: super::UiSelectionOwnerIncarnation,
        request: super::UiSelectionRequest,
    ) -> Result<super::UiSelectionDelta, super::UiSelectionRequestDenial> {
        let record = self
            .owners
            .get_mut(&owner)
            .ok_or(super::UiSelectionRequestDenial::UnknownOwner)?;
        if record.incarnation != incarnation {
            return Err(super::UiSelectionRequestDenial::StaleOwnerIncarnation);
        }
        if !record.catalog_available {
            return Err(super::UiSelectionRequestDenial::CatalogUnavailable);
        }
        let visited = super::reducer::validate_request(record, request)?;
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(super::UiSelectionRequestDenial::RevisionExhausted)?;
        let requests = self
            .requests
            .checked_add(1)
            .ok_or(super::UiSelectionRequestDenial::CounterOverflow)?;
        let candidates_visited = self
            .candidates_visited
            .checked_add(u64::from(visited))
            .ok_or(super::UiSelectionRequestDenial::CounterOverflow)?;
        let mutation = super::reducer::apply_request(record, request)?;
        self.revision = revision;
        self.requests = requests;
        self.candidates_visited = candidates_visited;
        let delta = super::UiSelectionDelta::new(
            mutation.added,
            mutation.removed,
            record.selected.len(),
            visited,
            revision,
        );
        self.record_drop(
            owner,
            super::UiSelectionDropInspectionReason::Interaction,
            &delta,
        );
        Ok(delta)
    }

    #[cfg(test)]
    pub(crate) fn selected(
        &self,
        owner: super::UiSelectionOwnerIdentity,
    ) -> Option<&BTreeSet<super::UiSelectionStableKey>> {
        self.owners.get(&owner).map(|record| &record.selected)
    }

    pub(crate) const fn selection_keys_visited(&self) -> u64 {
        self.candidates_visited
    }

    pub(crate) fn compact_posture_for(
        &self,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        graph_node: crate::graph::UiGraphNodeIdentity,
        incarnation: super::UiSelectionOwnerIncarnation,
    ) -> Option<(usize, u64)> {
        let owners = self
            .mounted_owners
            .get(&(surface, graph_node, incarnation))?;
        if owners.len() != 1 {
            return None;
        }
        let record = self.owners.get(owners.first()?)?;
        Some((record.selected.len(), self.revision))
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_for_certification(
        &self,
    ) -> (
        usize,
        usize,
        usize,
        u64,
        u64,
        u64,
        u64,
        Box<[core::num::NonZeroU64]>,
    ) {
        (
            self.owners.len(),
            self.owners
                .values()
                .filter(|owner| owner.catalog_available)
                .count(),
            self.owners.values().map(|owner| owner.selected.len()).sum(),
            self.revision,
            self.requests,
            self.candidates_visited,
            self.catalog_keys_reconciled,
            self.owners
                .values()
                .flat_map(|owner| owner.selected.iter().copied())
                .map(super::UiSelectionStableKey::application_value)
                .collect(),
        )
    }

    fn index_owner(
        &mut self,
        owner: super::UiSelectionOwnerIdentity,
        incarnation: super::UiSelectionOwnerIncarnation,
    ) {
        self.mounted_owners
            .entry((owner.semantic_surface(), owner.graph_node(), incarnation))
            .or_default()
            .insert(owner);
        self.family_owners
            .entry(owner.key_family())
            .or_default()
            .insert(owner);
    }

    fn unindex_owner(
        &mut self,
        owner: super::UiSelectionOwnerIdentity,
        incarnation: super::UiSelectionOwnerIncarnation,
    ) {
        let mounted_key = (owner.semantic_surface(), owner.graph_node(), incarnation);
        let remove_mounted = if let Some(owners) = self.mounted_owners.get_mut(&mounted_key) {
            owners.remove(&owner);
            owners.is_empty()
        } else {
            false
        };
        if remove_mounted {
            self.mounted_owners.remove(&mounted_key);
        }
        let family = owner.key_family();
        let remove_family = if let Some(owners) = self.family_owners.get_mut(&family) {
            owners.remove(&owner);
            owners.is_empty()
        } else {
            false
        };
        if remove_family {
            self.family_owners.remove(&family);
        }
    }
}
