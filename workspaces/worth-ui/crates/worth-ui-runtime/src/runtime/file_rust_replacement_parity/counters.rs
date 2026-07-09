#[cfg(test)]
use crate::runtime::WorthUiCandidateAuthoringLane;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiFileRustReplacementParityCounters {
    file_candidate_count: usize,
    rust_candidate_count: usize,
    candidate_admission_count: usize,
    artifact_comparison_count: usize,
    impact_classification_count: usize,
    impact_narrowing_count: usize,
    identity_matching_count: usize,
    node_replacement_count: usize,
    durable_state_reconciliation_count: usize,
    query_binding_comparison_count: usize,
    query_live_rebind_count: usize,
    plan_lowering_count: usize,
    handle_allocation_count: usize,
    lane_admission_count: usize,
    topology_assembly_count: usize,
    activation_stage_count: usize,
    ready_activation_count: usize,
    plan_swap_count: usize,
    parity_comparison_count: usize,
    rust_active_plan_injection_count: usize,
    rust_direct_handle_injection_count: usize,
    canonical_constructor_bypass_count: usize,
    source_reparse_on_swap_count: usize,
    registry_rebuild_on_swap_count: usize,
    denial_count: usize,
}

impl WorthUiFileRustReplacementParityCounters {
    #[cfg(test)]
    pub(crate) fn record_candidate(&mut self, lane: WorthUiCandidateAuthoringLane) {
        match lane {
            WorthUiCandidateAuthoringLane::FileAuthored => self.file_candidate_count += 1,
            WorthUiCandidateAuthoringLane::RustAuthored => self.rust_candidate_count += 1,
        }
    }

    #[cfg(test)]
    pub(crate) fn record_candidate_admission(&mut self) {
        self.candidate_admission_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_artifact_comparison(&mut self) {
        self.artifact_comparison_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_impact_classification(&mut self) {
        self.impact_classification_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_impact_narrowing(&mut self) {
        self.impact_narrowing_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_identity_matching(&mut self) {
        self.identity_matching_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_node_replacement(&mut self) {
        self.node_replacement_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_durable_state_reconciliation(&mut self) {
        self.durable_state_reconciliation_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_query_binding_comparison(&mut self) {
        self.query_binding_comparison_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_query_live_rebind(&mut self) {
        self.query_live_rebind_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_plan_lowering(&mut self) {
        self.plan_lowering_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_handle_allocation(&mut self) {
        self.handle_allocation_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_lane_admission(&mut self) {
        self.lane_admission_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_topology_assembly(&mut self) {
        self.topology_assembly_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_activation_stage(&mut self) {
        self.activation_stage_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_ready_activation(&mut self) {
        self.ready_activation_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_plan_swap(&mut self) {
        self.plan_swap_count += 1;
    }

    #[cfg(test)]
    pub(crate) fn record_swap_forbidden_work(
        &mut self,
        source_reparse_count: usize,
        registry_rebuild_count: usize,
    ) {
        self.source_reparse_on_swap_count += source_reparse_count;
        self.registry_rebuild_on_swap_count += registry_rebuild_count;
    }

    pub(crate) fn record_parity_comparison(&mut self) {
        self.parity_comparison_count += 1;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub(crate) fn merge(self, other: Self) -> Self {
        Self {
            file_candidate_count: self.file_candidate_count + other.file_candidate_count,
            rust_candidate_count: self.rust_candidate_count + other.rust_candidate_count,
            candidate_admission_count: self.candidate_admission_count
                + other.candidate_admission_count,
            artifact_comparison_count: self.artifact_comparison_count
                + other.artifact_comparison_count,
            impact_classification_count: self.impact_classification_count
                + other.impact_classification_count,
            impact_narrowing_count: self.impact_narrowing_count + other.impact_narrowing_count,
            identity_matching_count: self.identity_matching_count + other.identity_matching_count,
            node_replacement_count: self.node_replacement_count + other.node_replacement_count,
            durable_state_reconciliation_count: self.durable_state_reconciliation_count
                + other.durable_state_reconciliation_count,
            query_binding_comparison_count: self.query_binding_comparison_count
                + other.query_binding_comparison_count,
            query_live_rebind_count: self.query_live_rebind_count + other.query_live_rebind_count,
            plan_lowering_count: self.plan_lowering_count + other.plan_lowering_count,
            handle_allocation_count: self.handle_allocation_count + other.handle_allocation_count,
            lane_admission_count: self.lane_admission_count + other.lane_admission_count,
            topology_assembly_count: self.topology_assembly_count + other.topology_assembly_count,
            activation_stage_count: self.activation_stage_count + other.activation_stage_count,
            ready_activation_count: self.ready_activation_count + other.ready_activation_count,
            plan_swap_count: self.plan_swap_count + other.plan_swap_count,
            parity_comparison_count: self.parity_comparison_count + other.parity_comparison_count,
            rust_active_plan_injection_count: self.rust_active_plan_injection_count
                + other.rust_active_plan_injection_count,
            rust_direct_handle_injection_count: self.rust_direct_handle_injection_count
                + other.rust_direct_handle_injection_count,
            canonical_constructor_bypass_count: self.canonical_constructor_bypass_count
                + other.canonical_constructor_bypass_count,
            source_reparse_on_swap_count: self.source_reparse_on_swap_count
                + other.source_reparse_on_swap_count,
            registry_rebuild_on_swap_count: self.registry_rebuild_on_swap_count
                + other.registry_rebuild_on_swap_count,
            denial_count: self.denial_count + other.denial_count,
        }
    }

    pub fn file_candidate_count(self) -> usize {
        self.file_candidate_count
    }
    pub fn rust_candidate_count(self) -> usize {
        self.rust_candidate_count
    }
    pub fn candidate_admission_count(self) -> usize {
        self.candidate_admission_count
    }
    pub fn artifact_comparison_count(self) -> usize {
        self.artifact_comparison_count
    }
    pub fn impact_classification_count(self) -> usize {
        self.impact_classification_count
    }
    pub fn impact_narrowing_count(self) -> usize {
        self.impact_narrowing_count
    }
    pub fn identity_matching_count(self) -> usize {
        self.identity_matching_count
    }
    pub fn node_replacement_count(self) -> usize {
        self.node_replacement_count
    }
    pub fn durable_state_reconciliation_count(self) -> usize {
        self.durable_state_reconciliation_count
    }
    pub fn query_binding_comparison_count(self) -> usize {
        self.query_binding_comparison_count
    }
    pub fn query_live_rebind_count(self) -> usize {
        self.query_live_rebind_count
    }
    pub fn plan_lowering_count(self) -> usize {
        self.plan_lowering_count
    }
    pub fn handle_allocation_count(self) -> usize {
        self.handle_allocation_count
    }
    pub fn lane_admission_count(self) -> usize {
        self.lane_admission_count
    }
    pub fn topology_assembly_count(self) -> usize {
        self.topology_assembly_count
    }
    pub fn activation_stage_count(self) -> usize {
        self.activation_stage_count
    }
    pub fn ready_activation_count(self) -> usize {
        self.ready_activation_count
    }
    pub fn plan_swap_count(self) -> usize {
        self.plan_swap_count
    }
    pub fn parity_comparison_count(self) -> usize {
        self.parity_comparison_count
    }
    pub fn rust_active_plan_injection_count(self) -> usize {
        self.rust_active_plan_injection_count
    }
    pub fn rust_direct_handle_injection_count(self) -> usize {
        self.rust_direct_handle_injection_count
    }
    pub fn canonical_constructor_bypass_count(self) -> usize {
        self.canonical_constructor_bypass_count
    }
    pub fn source_reparse_on_swap_count(self) -> usize {
        self.source_reparse_on_swap_count
    }
    pub fn registry_rebuild_on_swap_count(self) -> usize {
        self.registry_rebuild_on_swap_count
    }
    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
