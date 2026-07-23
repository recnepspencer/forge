use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Included, Unbounded};

use crate::domain_installation::{
    WorthQueryBoundCollectionWindow, WorthQueryCollectionDeliveryCounters,
    WorthQueryCollectionPatchFact, WorthQueryCollectionRowHandle, WorthQueryNativeAccessKey,
};
use crate::memory_workspace::{WorthQueryEntity, WorthQueryEntityIdentity};
use crate::projection_consumption::ConsumedNativeValue;
use crate::runtime::{
    canonical_ordering_key, row_matches_predicates, WorthQueryCanonicalOrderingKey,
};

#[derive(Clone)]
struct MaintenanceRow {
    entity: WorthQueryEntity,
    source_row_identity: String,
    view_local_identity: String,
}

pub(crate) struct WorthQueryCollectionMaintenanceIndex {
    collection: String,
    request: crate::declarative_live::DeclarativeLiveQueryRequest,
    window_policy: crate::domain_installation::WorthQueryOperationWindowPolicy,
    continuation_posture: crate::domain_installation::WorthQueryOperationContinuationPosture,
    native_keys: Vec<WorthQueryNativeAccessKey>,
    delivery_supported: bool,
    rows: BTreeMap<WorthQueryCanonicalOrderingKey, MaintenanceRow>,
    identities: BTreeMap<WorthQueryEntityIdentity, WorthQueryCanonicalOrderingKey>,
}

pub(crate) struct WorthQueryCollectionMaintenanceInputs<'a> {
    pub request: crate::declarative_live::DeclarativeLiveQueryRequest,
    pub window_policy: crate::domain_installation::WorthQueryOperationWindowPolicy,
    pub continuation_posture: crate::domain_installation::WorthQueryOperationContinuationPosture,
    pub delivery_supported: bool,
    pub entities: Vec<WorthQueryEntity>,
    pub handles: &'a [WorthQueryCollectionRowHandle],
    pub native_keys: Vec<WorthQueryNativeAccessKey>,
}

pub(super) struct WorthQueryCollectionIndexDelta {
    removals: BTreeSet<WorthQueryCanonicalOrderingKey>,
    upserts: BTreeMap<WorthQueryCanonicalOrderingKey, MaintenanceRow>,
}

pub(super) struct WorthQueryCollectionIndexPreview {
    pub rows: Vec<WorthQueryCollectionRowHandle>,
    pub has_more: bool,
    pub facts: Vec<WorthQueryCollectionPatchFact>,
    pub delta: WorthQueryCollectionIndexDelta,
}

impl WorthQueryCollectionMaintenanceIndex {
    pub(crate) fn build(
        inputs: WorthQueryCollectionMaintenanceInputs<'_>,
        counters: &mut crate::domain_installation::WorthQueryCollectionCapabilityCounters,
    ) -> Self {
        let WorthQueryCollectionMaintenanceInputs {
            request,
            window_policy,
            continuation_posture,
            delivery_supported,
            entities,
            handles,
            native_keys,
        } = inputs;
        let mut rows = BTreeMap::new();
        let mut identities = BTreeMap::new();
        for (entity, handle) in entities.into_iter().zip(handles) {
            let key = canonical_ordering_key(&entity, request.ordering());
            identities.insert(entity.identity().clone(), key.clone());
            rows.insert(
                key,
                MaintenanceRow {
                    entity,
                    source_row_identity: handle.source_row_identity.clone(),
                    view_local_identity: handle.view_local_identity().to_string(),
                },
            );
            counters.maintenance_rows_indexed += 1;
        }
        Self {
            collection: request.target().to_string(),
            request,
            window_policy,
            continuation_posture,
            native_keys,
            delivery_supported,
            rows,
            identities,
        }
    }

    pub(super) const fn window_policy(
        &self,
    ) -> crate::domain_installation::WorthQueryOperationWindowPolicy {
        self.window_policy
    }

    pub(super) const fn continuation_posture(
        &self,
    ) -> crate::domain_installation::WorthQueryOperationContinuationPosture {
        self.continuation_posture
    }

    pub(super) const fn delivery_supported(&self) -> bool {
        self.delivery_supported
    }

    pub(super) fn preview(
        &self,
        request: WorthQueryCollectionPreviewRequest<'_>,
        counters: &mut WorthQueryCollectionDeliveryCounters,
    ) -> Result<WorthQueryCollectionIndexPreview, ()> {
        let WorthQueryCollectionPreviewRequest {
            window,
            affected,
            keys,
            workspace,
        } = request;
        let delta = self.delta(affected, workspace, counters)?;
        let selected = self.window_rows(window, &delta, counters);
        let has_more = selected.len() > window.admitted_width();
        let selected = selected
            .into_iter()
            .take(window.admitted_width())
            .collect::<Vec<_>>();
        let facts = patch_facts(
            PatchFactRequest {
                selected: &selected,
                prior: window,
                affected,
                affected_keys: keys,
                selected_keys: &self.native_keys,
            },
            counters,
        );
        let rows = selected
            .into_iter()
            .enumerate()
            .map(|(ordinal, row)| {
                WorthQueryCollectionRowHandle::new(
                    crate::domain_installation::WorthQueryCollectionRowParts {
                        entity_identity: row.entity.identity().clone(),
                        view_local_identity: row.view_local_identity,
                        source_row_identity: row.source_row_identity,
                        row_ordinal: ordinal,
                        capability_identity: window.capability_identity,
                        capability_generation: window.capability_generation,
                    },
                )
            })
            .collect();
        Ok(WorthQueryCollectionIndexPreview {
            rows,
            has_more,
            facts,
            delta,
        })
    }

    pub(super) fn apply(&mut self, delta: WorthQueryCollectionIndexDelta) {
        for key in delta.removals {
            if let Some(row) = self.rows.remove(&key) {
                self.identities.remove(row.entity.identity());
            }
        }
        for (key, row) in delta.upserts {
            self.identities
                .insert(row.entity.identity().clone(), key.clone());
            self.rows.insert(key, row);
        }
    }

    fn delta(
        &self,
        affected: &BTreeSet<WorthQueryEntityIdentity>,
        workspace: &crate::runtime::WorthQueryWorkspace,
        counters: &mut WorthQueryCollectionDeliveryCounters,
    ) -> Result<WorthQueryCollectionIndexDelta, ()> {
        if !self.request.traversal().is_empty() {
            return Err(());
        }
        let mut removals = BTreeSet::new();
        let mut upserts = BTreeMap::new();
        for identity in affected {
            counters.entity_point_lookups += 1;
            if let Some(old) = self.identities.get(identity) {
                removals.insert(old.clone());
            }
            let entity = match workspace.collection_entity(&self.collection, identity) {
                crate::runtime::WorthQueryBackendEntityLookup::Found(entity) => entity,
                crate::runtime::WorthQueryBackendEntityLookup::Absent => continue,
                crate::runtime::WorthQueryBackendEntityLookup::Unsupported => return Err(()),
            };
            if !row_matches_predicates(&entity, self.request.predicate_filters()) {
                continue;
            }
            let old = self
                .identities
                .get(identity)
                .and_then(|key| self.rows.get(key));
            let source_row_identity = old
                .map(|row| row.source_row_identity.clone())
                .unwrap_or_else(|| identity.terminal_projection_for_reporting());
            let view_local_identity = old
                .map(|row| row.view_local_identity.clone())
                .unwrap_or_else(|| source_row_identity.clone());
            let key = canonical_ordering_key(&entity, self.request.ordering());
            upserts.insert(
                key,
                MaintenanceRow {
                    entity,
                    source_row_identity,
                    view_local_identity,
                },
            );
            counters.ordering_index_updates += 1;
        }
        Ok(WorthQueryCollectionIndexDelta { removals, upserts })
    }

    fn window_rows(
        &self,
        window: &WorthQueryBoundCollectionWindow,
        delta: &WorthQueryCollectionIndexDelta,
        counters: &mut WorthQueryCollectionDeliveryCounters,
    ) -> Vec<MaintenanceRow> {
        let start = (!window.cursor().is_beginning())
            .then(|| {
                window.rows().first().and_then(|row| {
                    self.identities
                        .get(row.entity_identity())
                        .map(|key| Included(key.clone()))
                })
            })
            .flatten();
        let lower = start.unwrap_or(Unbounded);
        let mut retained = self
            .rows
            .range((lower.clone(), Unbounded))
            .filter(|(key, _)| !delta.removals.contains(*key))
            .peekable();
        let mut inserted = delta.upserts.range((lower, Unbounded)).peekable();
        let mut selected = Vec::new();
        while selected.len() <= window.admitted_width() {
            let next = match (retained.peek(), inserted.peek()) {
                (Some((left, _)), Some((right, _))) if left <= right => {
                    retained.next().map(|(_, row)| row.clone())
                }
                (Some(_), Some(_)) => inserted.next().map(|(_, row)| row.clone()),
                (Some(_), None) => retained.next().map(|(_, row)| row.clone()),
                (None, Some(_)) => inserted.next().map(|(_, row)| row.clone()),
                (None, None) => None,
            };
            let Some(row) = next else { break };
            counters.fresh_window_rows_visited += 1;
            selected.push(row);
        }
        selected
    }
}

pub(super) struct WorthQueryCollectionPreviewRequest<'a> {
    pub window: &'a WorthQueryBoundCollectionWindow,
    pub affected: &'a BTreeSet<WorthQueryEntityIdentity>,
    pub keys: &'a [WorthQueryNativeAccessKey],
    pub workspace: &'a crate::runtime::WorthQueryWorkspace,
}

struct PatchFactRequest<'a> {
    selected: &'a [MaintenanceRow],
    prior: &'a WorthQueryBoundCollectionWindow,
    affected: &'a BTreeSet<WorthQueryEntityIdentity>,
    affected_keys: &'a [WorthQueryNativeAccessKey],
    selected_keys: &'a [WorthQueryNativeAccessKey],
}

fn patch_facts(
    request: PatchFactRequest<'_>,
    counters: &mut WorthQueryCollectionDeliveryCounters,
) -> Vec<WorthQueryCollectionPatchFact> {
    let PatchFactRequest {
        selected,
        prior,
        affected,
        affected_keys,
        selected_keys,
    } = request;
    let mut facts = Vec::new();
    let prior_identities = prior
        .rows()
        .iter()
        .map(|row| row.entity_identity())
        .collect::<BTreeSet<_>>();
    for row in selected {
        let was_mounted = prior_identities.contains(row.entity.identity());
        let keys = if !was_mounted {
            selected_keys
        } else if affected.contains(row.entity.identity()) {
            affected_keys
        } else {
            continue;
        };
        for key in keys {
            let value = native_value(&row.entity, key);
            facts.push(WorthQueryCollectionPatchFact::new(
                row.entity.identity().clone(),
                key.clone(),
                value,
            ));
            counters.native_facts_materialized += 1;
        }
    }
    facts
}

fn native_value(entity: &WorthQueryEntity, key: &WorthQueryNativeAccessKey) -> ConsumedNativeValue {
    if let Some(field) = key.field_path().native_field_key() {
        return entity
            .struct_aspect_value(key.contract_key())
            .and_then(|value| value.get(field))
            .cloned()
            .map(ConsumedNativeValue::scalar)
            .unwrap_or_else(|| ConsumedNativeValue::absent(key.absence_posture()));
    }
    if let Some(path) = key.field_path().canonical_field_path() {
        return entity
            .scalar_value_at(path)
            .cloned()
            .map(ConsumedNativeValue::scalar)
            .unwrap_or_else(|| ConsumedNativeValue::absent(key.absence_posture()));
    }
    if let Some(value) = entity.struct_aspect_value(key.contract_key()) {
        return ConsumedNativeValue::struct_value(value.clone());
    }
    entity
        .aspect_value(key.contract_key())
        .cloned()
        .map(ConsumedNativeValue::scalar)
        .unwrap_or_else(|| ConsumedNativeValue::absent(key.absence_posture()))
}
