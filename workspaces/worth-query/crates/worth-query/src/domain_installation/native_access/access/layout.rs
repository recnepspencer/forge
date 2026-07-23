use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryConsumerProjectionContract, WorthQueryDomainInstallationGeneration,
    WorthQueryInstalledDomainAuthority,
};
use crate::projection_consumption::{
    ConsumedFieldValueFact, WorthQueryConsumedProjectionAuthority,
};

use super::super::{
    WorthQueryNativeAccessDenial, WorthQueryNativeAccessDenialKind, WorthQueryNativeAccessKey,
    WorthQueryNativeAccessPlan, WorthQueryNativeFactLane,
};
use super::affected_key_index::WorthQueryAffectedNativeKeyIndex;
use super::{
    WorthQueryNativeAccessBindingCounters, WorthQueryNativeAccessCounters,
    WorthQueryNativeFieldAccess,
};

pub(crate) struct WorthQueryNativeAccessLayout {
    domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
    runtime_authority: u64,
    installation_generation: WorthQueryDomainInstallationGeneration,
    capability_identity: u64,
    source_family: crate::projection_consumption::ProjectionSourceFamily,
    source_identity: String,
    projection_authority: String,
    row_count: usize,
    display_keys: Vec<WorthQueryNativeAccessKey>,
    derived_keys: Vec<WorthQueryNativeAccessKey>,
    affected_key_index: WorthQueryAffectedNativeKeyIndex,
    binding_counters: WorthQueryNativeAccessBindingCounters,
}

impl WorthQueryNativeAccessLayout {
    pub(crate) fn shares_execution_projection_with(&self, candidate: &Self) -> bool {
        self.source_family == candidate.source_family
            && self.row_count == candidate.row_count
            && semantic_keys_match(&self.display_keys, &candidate.display_keys)
            && semantic_keys_match(&self.derived_keys, &candidate.derived_keys)
    }

    pub(crate) fn rebind<D, O, F, L: BasisOperationLane>(
        &self,
        consumer: &WorthQueryConsumerProjectionContract<D, O, F, L>,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Self, WorthQueryNativeAccessDenial> {
        let mut keys = self.display_keys.clone();
        keys.extend(self.derived_keys.iter().cloned());
        Self::admit(WorthQueryNativeAccessPlan { keys }, consumer, authority)
    }

    pub(crate) fn unbound_denial(
        authority: &WorthQueryConsumedProjectionAuthority,
        key: &WorthQueryNativeAccessKey,
    ) -> WorthQueryNativeAccessDenial {
        WorthQueryNativeAccessDenial::new(
            WorthQueryNativeAccessDenialKind::LayoutMismatch,
            key,
            authority.source_family(),
            authority.source_identity().as_str(),
            authority.contract().contract_digest(),
            WorthQueryNativeAccessCounters::default(),
        )
    }

    pub(crate) fn admit<D, O, F, L: BasisOperationLane>(
        plan: WorthQueryNativeAccessPlan,
        consumer: &WorthQueryConsumerProjectionContract<D, O, F, L>,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Self, WorthQueryNativeAccessDenial> {
        let partitioned = PartitionedNativeAccessPlan::from_plan(plan);
        let proof = authority.facts().native_layout();
        if !partitioned.matches_authority(authority) {
            return Err(WorthQueryNativeAccessDenial::new(
                WorthQueryNativeAccessDenialKind::LayoutMismatch,
                &partitioned.first,
                authority.source_family(),
                authority.source_identity().as_str(),
                authority.contract().contract_digest(),
                WorthQueryNativeAccessCounters::default(),
            ));
        }
        let affected_key_index = WorthQueryAffectedNativeKeyIndex::compile(
            &partitioned.display_keys,
            &partitioned.derived_keys,
        );
        Ok(Self {
            domain_authority: Arc::clone(consumer.domain_authority()),
            runtime_authority: consumer.runtime_authority(),
            installation_generation: consumer.installation_generation(),
            capability_identity: consumer.capability_identity(),
            source_family: authority.source_family(),
            source_identity: authority.source_identity().as_str().to_string(),
            projection_authority: authority.contract().contract_digest().to_string(),
            row_count: proof.row_count(),
            display_keys: partitioned.display_keys,
            derived_keys: partitioned.derived_keys,
            affected_key_index,
            binding_counters: WorthQueryNativeAccessBindingCounters {
                declared_key_routes: partitioned.declared_key_count,
                declared_key_layout_checks: partitioned.declared_key_count,
                lane_shape_checks: 2,
                fact_scans: 0,
                row_scans: 0,
                path_parses: 0,
                view_registry_inspections: 0,
                domain_registry_inspections: 0,
            },
        })
    }

    pub(crate) fn binding_counters(&self) -> WorthQueryNativeAccessBindingCounters {
        self.binding_counters
    }

    pub(crate) fn affected_keys(
        &self,
        touches: &[super::affected_key_index::WorthQueryNativeTouchCoordinate],
    ) -> (
        Vec<WorthQueryNativeAccessKey>,
        super::affected_key_index::WorthQueryNativeKeyNarrowingCounters,
    ) {
        self.affected_key_index.affected_keys(touches)
    }

    pub(crate) fn access<'a>(
        &self,
        authority: &'a WorthQueryConsumedProjectionAuthority,
        key: &WorthQueryNativeAccessKey,
        row: usize,
    ) -> Result<WorthQueryNativeFieldAccess<'a>, WorthQueryNativeAccessDenial> {
        let mut counters = WorthQueryNativeAccessCounters::default();
        self.ensure_affinity(key, &mut counters)?;
        let fact = self.indexed_lane_fact(authority, key, row, &mut counters)?;
        refine_selected_fact(fact, key, &mut counters)
            .map_err(|kind| self.denial(kind, key, counters))?;
        Ok(WorthQueryNativeFieldAccess { fact, counters })
    }

    fn ensure_affinity(
        &self,
        key: &WorthQueryNativeAccessKey,
        counters: &mut WorthQueryNativeAccessCounters,
    ) -> Result<(), WorthQueryNativeAccessDenial> {
        counters.authority_checks += 1;
        if !self.domain_authority.is_current_installation_generation() {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::StaleInstallationGeneration,
                key,
                *counters,
            ));
        }
        counters.authority_checks += 1;
        if key.runtime_authority() != self.runtime_authority {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::RuntimeMismatch,
                key,
                *counters,
            ));
        }
        counters.authority_checks += 1;
        if key.installation_generation() != self.installation_generation {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::AccessKeyInstallationGenerationMismatch,
                key,
                *counters,
            ));
        }
        counters.authority_checks += 1;
        if key.capability_identity() != self.capability_identity {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::CapabilityMismatch,
                key,
                *counters,
            ));
        }
        Ok(())
    }

    fn indexed_lane_fact<'a>(
        &self,
        authority: &'a WorthQueryConsumedProjectionAuthority,
        key: &WorthQueryNativeAccessKey,
        row: usize,
        counters: &mut WorthQueryNativeAccessCounters,
    ) -> Result<&'a ConsumedFieldValueFact, WorthQueryNativeAccessDenial> {
        let (expected_key, facts) = match key.lane() {
            WorthQueryNativeFactLane::Display => (
                self.display_keys.get(key.lane_slot()),
                authority.facts().display_fields(),
            ),
            WorthQueryNativeFactLane::Derived => (
                self.derived_keys.get(key.lane_slot()),
                authority.facts().derived_fields(),
            ),
        };
        counters.authority_checks += 1;
        let Some(expected_key) = expected_key else {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::LayoutMismatch,
                key,
                *counters,
            ));
        };
        if expected_key.selection_identity() != key.selection_identity()
            || expected_key.lane_slot() != key.lane_slot()
            || expected_key.lane_width() != key.lane_width()
        {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::LayoutMismatch,
                key,
                *counters,
            ));
        }
        if row >= self.row_count {
            return Err(self.denial(
                WorthQueryNativeAccessDenialKind::RowOutOfBounds,
                key,
                *counters,
            ));
        }
        Ok(indexed_fact(
            facts,
            row * key.lane_width() + key.lane_slot(),
            counters,
        ))
    }

    fn denial(
        &self,
        kind: WorthQueryNativeAccessDenialKind,
        key: &WorthQueryNativeAccessKey,
        counters: WorthQueryNativeAccessCounters,
    ) -> WorthQueryNativeAccessDenial {
        WorthQueryNativeAccessDenial::new(
            kind,
            key,
            self.source_family,
            &self.source_identity,
            &self.projection_authority,
            counters,
        )
    }
}

struct PartitionedNativeAccessPlan {
    first: WorthQueryNativeAccessKey,
    declared_key_count: usize,
    display_keys: Vec<WorthQueryNativeAccessKey>,
    derived_keys: Vec<WorthQueryNativeAccessKey>,
}

impl PartitionedNativeAccessPlan {
    fn from_plan(plan: WorthQueryNativeAccessPlan) -> Self {
        let first = plan.keys[0].clone();
        let declared_key_count = plan.keys.len();
        let mut display_keys = Vec::new();
        let mut derived_keys = Vec::new();
        for key in plan.keys {
            match key.lane() {
                WorthQueryNativeFactLane::Display => display_keys.push(key),
                WorthQueryNativeFactLane::Derived => derived_keys.push(key),
            }
        }
        Self {
            first,
            declared_key_count,
            display_keys,
            derived_keys,
        }
    }

    fn matches_authority(&self, authority: &WorthQueryConsumedProjectionAuthority) -> bool {
        let proof = authority.facts().native_layout();
        lane_layout_matches(
            &self.display_keys,
            proof.display_selections(),
            authority.facts().display_fields().len(),
            proof.row_count(),
        ) && lane_layout_matches(
            &self.derived_keys,
            proof.derived_selections(),
            authority.facts().derived_fields().len(),
            proof.row_count(),
        )
    }
}

fn semantic_keys_match(
    subject: &[WorthQueryNativeAccessKey],
    candidate: &[WorthQueryNativeAccessKey],
) -> bool {
    subject.len() == candidate.len()
        && subject.iter().zip(candidate).all(|(subject, candidate)| {
            subject.contract_key() == candidate.contract_key()
                && subject.contract_identity() == candidate.contract_identity()
                && subject.contract_revision() == candidate.contract_revision()
                && subject.field_path() == candidate.field_path()
                && subject.expected_shape() == candidate.expected_shape()
                && subject.absence_posture() == candidate.absence_posture()
                && subject.lane() == candidate.lane()
                && subject.lane_slot() == candidate.lane_slot()
                && subject.lane_width() == candidate.lane_width()
        })
}

fn indexed_fact<'a>(
    facts: &'a [ConsumedFieldValueFact],
    index: usize,
    counters: &mut WorthQueryNativeAccessCounters,
) -> &'a ConsumedFieldValueFact {
    counters.indexed_accesses += 1;
    &facts[index]
}

fn refine_selected_fact(
    fact: &ConsumedFieldValueFact,
    key: &WorthQueryNativeAccessKey,
    counters: &mut WorthQueryNativeAccessCounters,
) -> Result<(), WorthQueryNativeAccessDenialKind> {
    counters.refinement_checks += 1;
    if fact.native_selection_identity() == Some(key.selection_identity()) {
        Ok(())
    } else {
        Err(WorthQueryNativeAccessDenialKind::LayoutMismatch)
    }
}

fn lane_layout_matches(
    keys: &[WorthQueryNativeAccessKey],
    selections: &[u64],
    fact_count: usize,
    row_count: usize,
) -> bool {
    keys.len() == selections.len()
        && fact_count == row_count * keys.len()
        && keys
            .iter()
            .zip(selections)
            .enumerate()
            .all(|(slot, (key, selection))| {
                key.lane_slot() == slot
                    && key.lane_width() == keys.len()
                    && key.selection_identity() == *selection
            })
}
