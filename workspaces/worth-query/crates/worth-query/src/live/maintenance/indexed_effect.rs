use std::sync::Arc;

use crate::domain_installation::{
    WorthQueryCollectionDeliveryCounters, WorthQueryCollectionPatchFact,
    WorthQueryCollectionPatchOperation, WorthQueryCollectionRowHandle,
    WorthQueryLiveProjectionRefresh, WorthQueryPerformedCollectionStateMutation,
};
use crate::memory_workspace::WorthQueryEntityIdentity;
use crate::projection_consumption::ConsumedFieldValueFact;

use super::{
    WorthQueryCoalescedMaintenancePlan, WorthQueryMaintenanceStrategy,
    WorthQueryPerformedMaintenanceEffect,
};

/// Query-owned evidence that a scoped refresh produced executable index/view
/// patch commands for the affected entities.
pub struct WorthQueryPerformedIndexedLivePatch {
    identity: String,
    fact_set_digest: String,
    affected_entities: Vec<WorthQueryEntityIdentity>,
    fields: Vec<ConsumedFieldValueFact>,
    strategies: Vec<WorthQueryMaintenanceStrategy>,
    operations: Vec<WorthQueryCollectionPatchOperation>,
    collection_facts: Vec<WorthQueryCollectionPatchFact>,
    rows: Vec<WorthQueryCollectionRowHandle>,
    collection_work: WorthQueryCollectionDeliveryCounters,
    work: WorthQueryPerformedLiveMaintenanceWork,
}

impl WorthQueryPerformedIndexedLivePatch {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn fact_set_digest(&self) -> &str {
        &self.fact_set_digest
    }

    pub fn affected_entities(&self) -> &[WorthQueryEntityIdentity] {
        &self.affected_entities
    }

    pub fn fields(&self) -> &[ConsumedFieldValueFact] {
        &self.fields
    }

    pub fn strategies(&self) -> &[WorthQueryMaintenanceStrategy] {
        &self.strategies
    }

    pub fn operations(&self) -> &[WorthQueryCollectionPatchOperation] {
        &self.operations
    }

    pub fn collection_facts(&self) -> &[WorthQueryCollectionPatchFact] {
        &self.collection_facts
    }

    pub fn rows(&self) -> &[WorthQueryCollectionRowHandle] {
        &self.rows
    }

    pub const fn collection_work(&self) -> WorthQueryCollectionDeliveryCounters {
        self.collection_work
    }

    pub const fn work(&self) -> WorthQueryPerformedLiveMaintenanceWork {
        self.work
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPerformedLiveMaintenanceWork {
    source_reads: usize,
    projection_calls: usize,
    mutation_deltas: usize,
    affected_requirement_rows: usize,
    index_updates: usize,
    live_view_updates: usize,
    prior_field_comparisons: usize,
}

impl WorthQueryPerformedLiveMaintenanceWork {
    pub const fn source_reads(self) -> usize {
        self.source_reads
    }

    pub const fn projection_calls(self) -> usize {
        self.projection_calls
    }

    pub const fn mutation_deltas(self) -> usize {
        self.mutation_deltas
    }

    pub const fn affected_requirement_rows(self) -> usize {
        self.affected_requirement_rows
    }

    pub const fn index_updates(self) -> usize {
        self.index_updates
    }

    pub const fn live_view_updates(self) -> usize {
        self.live_view_updates
    }

    pub const fn prior_field_comparisons(self) -> usize {
        self.prior_field_comparisons
    }
}

pub(super) fn derive(
    plan: &WorthQueryCoalescedMaintenancePlan,
    refresh: &WorthQueryLiveProjectionRefresh,
    affected_entities: Vec<WorthQueryEntityIdentity>,
    fields: Vec<ConsumedFieldValueFact>,
    prior_field_comparisons: usize,
    collection: WorthQueryPerformedCollectionStateMutation,
) -> Option<Arc<WorthQueryPerformedMaintenanceEffect>> {
    let refresh_work = refresh.work();
    let mut work = WorthQueryPerformedLiveMaintenanceWork {
        source_reads: refresh_work.read_calls(),
        projection_calls: refresh_work.projection_calls(),
        mutation_deltas: 0,
        affected_requirement_rows: 0,
        index_updates: 0,
        live_view_updates: 0,
        prior_field_comparisons,
    };
    let mut delta_identities = Vec::new();
    for batch in refresh.delivery().batches() {
        let Some(performed) = batch.maintenance_work() else {
            continue;
        };
        work.mutation_deltas += performed.mutation_delta_count();
        work.affected_requirement_rows += performed.affected_requirement_row_count();
        work.index_updates += performed.index_update_count();
        work.live_view_updates += performed.live_view_update_count();
        delta_identities.push(performed.maintenance_delta_identity().as_str().to_owned());
    }
    if work.source_reads != 1 || work.projection_calls != 1 {
        return None;
    }
    let operations = collection.operations;
    let collection_facts = collection.facts;
    let rows = collection.rows;
    let collection_work = collection.counters;
    work.index_updates += collection_work.ordering_index_updates;
    work.live_view_updates += operations.len();
    delta_identities.sort();
    delta_identities.dedup();
    let mut identity_parts = vec![
        "worth_query_performed_indexed_live_patch_v1".to_owned(),
        format!("facts:{}", refresh.authority().facts().fact_set_digest()),
        format!("source-reads:{}", work.source_reads),
        format!("projection-calls:{}", work.projection_calls),
        format!("mutation-deltas:{}", work.mutation_deltas),
        format!("requirements:{}", work.affected_requirement_rows),
        format!("index-updates:{}", work.index_updates),
        format!("view-updates:{}", work.live_view_updates),
        format!("prior-field-comparisons:{}", work.prior_field_comparisons),
    ];
    identity_parts.extend(
        plan.strategies()
            .iter()
            .map(|strategy| format!("strategy:{strategy:?}")),
    );
    identity_parts.extend(
        delta_identities
            .into_iter()
            .map(|delta| format!("delta:{delta}")),
    );
    identity_parts.extend(
        operations
            .iter()
            .map(|operation| format!("operation:{operation:?}")),
    );
    identity_parts.extend(
        affected_entities
            .iter()
            .map(|entity| format!("entity:{}", entity.evidence_identity().as_str())),
    );
    Some(Arc::new(
        WorthQueryPerformedMaintenanceEffect::IndexedLivePatch(
            WorthQueryPerformedIndexedLivePatch {
                identity: crate::identity::hash_parts(&identity_parts),
                fact_set_digest: refresh.authority().facts().fact_set_digest().to_owned(),
                affected_entities,
                fields,
                strategies: plan.strategies().to_vec(),
                operations,
                collection_facts,
                rows,
                collection_work,
                work,
            },
        ),
    ))
}
