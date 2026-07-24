#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionChangeCounters {
    patch_operations_visited: usize,
    patch_facts_reported: usize,
    row_references_minted: usize,
    graph_effects_minted: usize,
    measurement_effects_minted: usize,
    allocation_effects_minted: usize,
    diagnostic_effects_observed: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCollectionQueryWorkInspection {
    invalidation_authority_checks: usize,
    lease_checks: usize,
    generation_checks: usize,
    cursor_checks: usize,
    semantic_contract_checks: usize,
    pending_patch_checks: usize,
    prior_window_rows_visited: usize,
    fresh_window_rows_visited: usize,
    affected_identity_lookups: usize,
    entity_point_lookups: usize,
    ordering_index_updates: usize,
    operations_materialized: usize,
    native_facts_materialized: usize,
    full_collection_scans: usize,
    unrelated_consumer_scans: usize,
}

impl WorthUiCollectionChangeCounters {
    pub(crate) fn visit_operation(&mut self) {
        self.patch_operations_visited += 1;
    }

    pub(crate) fn record_reported_facts(&mut self, count: usize) {
        self.patch_facts_reported += count;
    }

    pub(crate) fn mint_row_reference(&mut self) {
        self.row_references_minted += 1;
    }

    pub(crate) fn mint_graph_effect(&mut self) {
        self.graph_effects_minted += 1;
    }

    pub(crate) fn mint_measurement_effect(&mut self) {
        self.measurement_effects_minted += 1;
    }

    pub(crate) fn mint_allocation_effect(&mut self) {
        self.allocation_effects_minted += 1;
    }

    pub(crate) fn observe_diagnostic_effect(&mut self) {
        self.diagnostic_effects_observed += 1;
    }

    pub fn patch_operations_visited(self) -> usize {
        self.patch_operations_visited
    }

    pub fn patch_facts_reported(self) -> usize {
        self.patch_facts_reported
    }

    pub fn row_references_minted(self) -> usize {
        self.row_references_minted
    }

    pub fn graph_effects_minted(self) -> usize {
        self.graph_effects_minted
    }

    pub fn measurement_effects_minted(self) -> usize {
        self.measurement_effects_minted
    }

    pub fn allocation_effects_minted(self) -> usize {
        self.allocation_effects_minted
    }

    pub fn diagnostic_effects_observed(self) -> usize {
        self.diagnostic_effects_observed
    }
}

impl WorthUiCollectionQueryWorkInspection {
    pub(crate) fn from_query(
        value: worth_query::facade::installed::collection::WorthQueryCollectionDeliveryCounters,
    ) -> Self {
        Self {
            invalidation_authority_checks: value.invalidation_authority_checks,
            lease_checks: value.lease_checks,
            generation_checks: value.generation_checks,
            cursor_checks: value.cursor_checks,
            semantic_contract_checks: value.semantic_contract_checks,
            pending_patch_checks: value.pending_patch_checks,
            prior_window_rows_visited: value.prior_window_rows_visited,
            fresh_window_rows_visited: value.fresh_window_rows_visited,
            affected_identity_lookups: value.affected_identity_lookups,
            entity_point_lookups: value.entity_point_lookups,
            ordering_index_updates: value.ordering_index_updates,
            operations_materialized: value.operations_materialized,
            native_facts_materialized: value.native_facts_materialized,
            full_collection_scans: value.full_collection_scans,
            unrelated_consumer_scans: value.unrelated_consumer_scans,
        }
    }

    pub fn operations_materialized(self) -> usize {
        self.operations_materialized
    }

    pub fn native_facts_materialized(self) -> usize {
        self.native_facts_materialized
    }

    pub fn full_collection_scans(self) -> usize {
        self.full_collection_scans
    }

    pub fn unrelated_consumer_scans(self) -> usize {
        self.unrelated_consumer_scans
    }

    pub fn prior_window_rows_visited(self) -> usize {
        self.prior_window_rows_visited
    }

    pub fn fresh_window_rows_visited(self) -> usize {
        self.fresh_window_rows_visited
    }

    pub fn invalidation_authority_checks(self) -> usize {
        self.invalidation_authority_checks
    }

    pub fn lease_checks(self) -> usize {
        self.lease_checks
    }

    pub fn generation_checks(self) -> usize {
        self.generation_checks
    }

    pub fn cursor_checks(self) -> usize {
        self.cursor_checks
    }

    pub fn semantic_contract_checks(self) -> usize {
        self.semantic_contract_checks
    }

    pub fn pending_patch_checks(self) -> usize {
        self.pending_patch_checks
    }

    pub fn affected_identity_lookups(self) -> usize {
        self.affected_identity_lookups
    }

    pub fn entity_point_lookups(self) -> usize {
        self.entity_point_lookups
    }

    pub fn ordering_index_updates(self) -> usize {
        self.ordering_index_updates
    }
}
