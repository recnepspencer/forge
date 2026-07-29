use std::collections::BTreeSet;

use worth_relational::facade::identity::EntityId;

/// Exact entity identities reached while one pinned invariant projection ran.
///
/// This is carried proof, not a caller-authored touch assertion. Only the
/// installation-owned projection reader can add identities.
#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryRealizedProjectionScope {
    entity_ids: BTreeSet<EntityId>,
}

impl WorthQueryRealizedProjectionScope {
    pub(super) fn record(&mut self, entity_id: EntityId) {
        self.entity_ids.insert(entity_id);
    }

    pub(super) fn record_relation(&mut self, from: EntityId, to: EntityId) {
        self.record(from);
        self.record(to);
    }

    pub(in crate::domain_computation::primary_graph) fn contains(
        &self,
        entity_id: EntityId,
    ) -> bool {
        self.entity_ids.contains(&entity_id)
    }
}
