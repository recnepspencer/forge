use crate::ordinary::live::WorthQueryManagedLiveDelivery;
use crate::runtime::WorthQueryStagedOwnerDeliveryAdmission;

/// Exact work performed by one lifecycle refresh.
///
/// Promotion counters deliberately do not represent this lane: refresh owns
/// an already-open resource and reports its maintenance, delivery, read,
/// projection, and native-rebind work separately.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryLiveProjectionRefreshWork {
    authority_checks: usize,
    drain_calls: usize,
    delivery_batches: usize,
    maintenance_batches: usize,
    mutation_deltas: usize,
    affected_requirement_rows: usize,
    touched_edges: usize,
    touched_frontiers: usize,
    index_updates: usize,
    live_view_updates: usize,
    skipped_unaffected_requirements: usize,
    strategy_recomputes: usize,
    background_index_builds: usize,
    read_calls: usize,
    projection_calls: usize,
    native_rebind_calls: usize,
    impact_classifications: usize,
    impact_classification_reuses: usize,
    conditional_decision_reuses: usize,
    conditional_reentry_runtime_key_checks: usize,
    conditional_reentry_lowering_identity_checks: usize,
    conditional_reentry_installed_lowering_lookups: usize,
    conditional_reentry_signal_graph_checks: usize,
    conditional_reentry_signal_contract_checks: usize,
    conditional_reentry_snapshot_identity_checks: usize,
    conditional_reentry_query_rebindings: usize,
    conditional_reentry_unrelated_lowering_scans: usize,
    causal_staged_changes_inspected: usize,
    causal_owner_changes_inspected: usize,
    causal_keys_materialized: usize,
    causal_key_lookups: usize,
    conditional_dependency_checks: usize,
    conditional_semantic_reads: usize,
    conditional_condition_checks: usize,
    conditional_condition_deferrals: usize,
    conditional_temporal_deferrals: usize,
    conditional_on_demand_deferrals: usize,
    conditional_comparator_checks: usize,
    conditional_compute_contacts: usize,
    conditional_reverted_clean_outcomes: usize,
    conditional_semantic_changes: usize,
    conditional_reuse_checks: usize,
    conditional_decisions_delivered: usize,
}

impl WorthQueryLiveProjectionRefreshWork {
    pub(in crate::domain_installation::operation_execution) fn authority_checked() -> Self {
        Self {
            authority_checks: 1,
            ..Self::default()
        }
    }

    pub(super) fn retain_delivery(&mut self, delivery: &WorthQueryManagedLiveDelivery) {
        self.delivery_batches = delivery.batches().len();
        for batch in delivery.batches() {
            let Some(work) = batch.maintenance_work() else {
                continue;
            };
            self.maintenance_batches += 1;
            self.mutation_deltas += work.mutation_delta_count();
            self.affected_requirement_rows += work.affected_requirement_row_count();
            self.touched_edges += work.touched_edge_count();
            self.touched_frontiers += work.touched_frontier_count();
            self.index_updates += work.index_update_count();
            self.live_view_updates += work.live_view_update_count();
            self.skipped_unaffected_requirements += work.skipped_unaffected_requirement_count();
            self.strategy_recomputes += work.strategy_recompute_count();
            self.background_index_builds += work.background_index_build_count();
        }
    }

    pub(super) fn begin_drain(&mut self) {
        self.drain_calls = 1;
    }

    pub(super) fn begin_read(&mut self) {
        self.read_calls = 1;
    }

    pub(super) fn retain_impact(
        &mut self,
        _impact: &crate::domain_installation::WorthQueryImpactDecision,
    ) {
        self.impact_classifications = 1;
    }

    pub(super) fn retain_impact_reuse(&mut self) {
        self.impact_classification_reuses = 1;
    }

    pub(super) fn begin_conditional_decision_reentry(&mut self) {
        self.conditional_decision_reuses = 1;
    }

    pub(super) fn retain_conditional_decision_reentry(
        &mut self,
        counters: worth_runtime_bridge::facade::BridgeConditionalReentryCounters,
    ) {
        self.conditional_reentry_runtime_key_checks = counters.runtime_key_checks;
        self.conditional_reentry_lowering_identity_checks = counters.lowering_identity_checks;
        self.conditional_reentry_installed_lowering_lookups = counters.installed_lowering_lookups;
        self.conditional_reentry_signal_graph_checks = counters.signal_graph_checks;
        self.conditional_reentry_signal_contract_checks = counters.signal_contract_checks;
        self.conditional_reentry_snapshot_identity_checks = counters.snapshot_identity_checks;
        self.conditional_reentry_query_rebindings = counters.query_continuation_rebindings;
        self.conditional_reentry_unrelated_lowering_scans = counters.unrelated_lowering_scans;
    }

    pub(super) fn retain_causal_admission(
        &mut self,
        causal: WorthQueryStagedOwnerDeliveryAdmission,
    ) {
        self.causal_staged_changes_inspected = causal.staged_changes_inspected();
        self.causal_owner_changes_inspected = causal.owner_changes_inspected();
        self.causal_keys_materialized = causal.causal_keys_materialized();
        self.causal_key_lookups = causal.causal_key_lookups();
    }

    pub(super) fn retain_conditional(
        &mut self,
        counters: crate::domain_installation::WorthQueryOperationExecutionCounters,
    ) {
        self.conditional_dependency_checks = counters.conditional_dependency_checks;
        self.conditional_semantic_reads = counters.conditional_semantic_reads;
        self.conditional_condition_checks = counters.conditional_condition_checks;
        self.conditional_condition_deferrals = counters.conditional_condition_deferrals;
        self.conditional_temporal_deferrals = counters.conditional_temporal_deferrals;
        self.conditional_on_demand_deferrals = counters.conditional_on_demand_deferrals;
        self.conditional_comparator_checks = counters.conditional_comparator_checks;
        self.conditional_compute_contacts = counters.conditional_compute_contacts;
        self.conditional_reverted_clean_outcomes = counters.conditional_reverted_clean_outcomes;
        self.conditional_semantic_changes = counters.conditional_semantic_changes;
        self.conditional_reuse_checks = counters.conditional_reuse_checks;
        self.conditional_decisions_delivered = counters.conditional_decisions_delivered;
    }

    pub(super) fn retain_projection(&mut self) {
        self.projection_calls = 1;
    }

    pub(super) fn begin_native_rebind(&mut self) {
        self.native_rebind_calls = 1;
    }

    pub fn authority_checks(self) -> usize {
        self.authority_checks
    }

    pub fn drain_calls(self) -> usize {
        self.drain_calls
    }

    pub fn delivery_batches(self) -> usize {
        self.delivery_batches
    }

    pub fn maintenance_batches(self) -> usize {
        self.maintenance_batches
    }

    pub fn mutation_deltas(self) -> usize {
        self.mutation_deltas
    }

    pub fn affected_requirement_rows(self) -> usize {
        self.affected_requirement_rows
    }

    pub fn touched_edges(self) -> usize {
        self.touched_edges
    }

    pub fn touched_frontiers(self) -> usize {
        self.touched_frontiers
    }

    pub fn index_updates(self) -> usize {
        self.index_updates
    }

    pub fn live_view_updates(self) -> usize {
        self.live_view_updates
    }

    pub fn skipped_unaffected_requirements(self) -> usize {
        self.skipped_unaffected_requirements
    }

    pub fn strategy_recomputes(self) -> usize {
        self.strategy_recomputes
    }

    pub fn background_index_builds(self) -> usize {
        self.background_index_builds
    }

    pub fn read_calls(self) -> usize {
        self.read_calls
    }

    pub fn projection_calls(self) -> usize {
        self.projection_calls
    }

    pub fn native_rebind_calls(self) -> usize {
        self.native_rebind_calls
    }

    pub fn impact_classifications(self) -> usize {
        self.impact_classifications
    }

    pub fn impact_classification_reuses(self) -> usize {
        self.impact_classification_reuses
    }

    pub fn conditional_decision_reuses(self) -> usize {
        self.conditional_decision_reuses
    }

    pub fn conditional_reentry_runtime_key_checks(self) -> usize {
        self.conditional_reentry_runtime_key_checks
    }

    pub fn conditional_reentry_lowering_identity_checks(self) -> usize {
        self.conditional_reentry_lowering_identity_checks
    }

    pub fn conditional_reentry_installed_lowering_lookups(self) -> usize {
        self.conditional_reentry_installed_lowering_lookups
    }

    pub fn conditional_reentry_signal_graph_checks(self) -> usize {
        self.conditional_reentry_signal_graph_checks
    }

    pub fn conditional_reentry_signal_contract_checks(self) -> usize {
        self.conditional_reentry_signal_contract_checks
    }

    pub fn conditional_reentry_snapshot_identity_checks(self) -> usize {
        self.conditional_reentry_snapshot_identity_checks
    }

    pub fn conditional_reentry_query_rebindings(self) -> usize {
        self.conditional_reentry_query_rebindings
    }

    pub fn conditional_reentry_unrelated_lowering_scans(self) -> usize {
        self.conditional_reentry_unrelated_lowering_scans
    }

    pub fn causal_staged_changes_inspected(self) -> usize {
        self.causal_staged_changes_inspected
    }

    pub fn causal_owner_changes_inspected(self) -> usize {
        self.causal_owner_changes_inspected
    }

    pub fn causal_keys_materialized(self) -> usize {
        self.causal_keys_materialized
    }

    pub fn causal_key_lookups(self) -> usize {
        self.causal_key_lookups
    }

    pub fn conditional_dependency_checks(self) -> usize {
        self.conditional_dependency_checks
    }

    pub fn conditional_semantic_reads(self) -> usize {
        self.conditional_semantic_reads
    }

    pub fn conditional_condition_checks(self) -> usize {
        self.conditional_condition_checks
    }

    pub fn conditional_condition_deferrals(self) -> usize {
        self.conditional_condition_deferrals
    }

    pub fn conditional_temporal_deferrals(self) -> usize {
        self.conditional_temporal_deferrals
    }

    pub fn conditional_on_demand_deferrals(self) -> usize {
        self.conditional_on_demand_deferrals
    }

    pub fn conditional_comparator_checks(self) -> usize {
        self.conditional_comparator_checks
    }

    pub fn conditional_compute_contacts(self) -> usize {
        self.conditional_compute_contacts
    }

    pub fn conditional_reverted_clean_outcomes(self) -> usize {
        self.conditional_reverted_clean_outcomes
    }

    pub fn conditional_semantic_changes(self) -> usize {
        self.conditional_semantic_changes
    }

    pub fn conditional_reuse_checks(self) -> usize {
        self.conditional_reuse_checks
    }

    pub fn conditional_decisions_delivered(self) -> usize {
        self.conditional_decisions_delivered
    }
}
