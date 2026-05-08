use forge_relational::facade::runtime::RelationalRuntime;
use topology::facade::{
    certify_milestone_three_closeout, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedRegionCoverageRow, MilestoneThreeEditBreadthCounterRow,
    MilestoneThreeFailureLocalityRow, MilestoneThreeHostileSuiteReport,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeRejectedEditScopeReportRow,
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
    MilestoneThreeSplitCollapseChurnWitness, MilestoneThreeTopologyEditDigestRow,
    TopologyCertificationError,
};

fn _milestone_three_closeout_function_contract() {
    let _: fn(
        fn() -> RelationalRuntime,
        &str,
    ) -> Result<MilestoneThreeHostileSuiteReport, TopologyCertificationError> =
        certify_milestone_three_closeout::<fn() -> RelationalRuntime>;
}

fn _milestone_three_side_quest_report_contract(report: &MilestoneThreeHostileSuiteReport) {
    let _: &MilestoneThreeSideQuestCloseoutReport = &report.side_quest_closeout_report;
    let _: &[MilestoneThreeSideQuestContractRow] =
        report.side_quest_closeout_report.contract_rows.as_slice();
    let _: &[MilestoneThreeSideQuestBlockerRow] =
        report.side_quest_closeout_report.blocker_rows.as_slice();
    let _: usize = report.side_quest_closeout_report.domain_read_request_count;
    let _: usize = report.side_quest_closeout_report.domain_read_parity_count;
    let _: bool = report.side_quest_closeout_report.phase_three_ready;
    let _: bool = report.side_quest_gate_ready;
    let _: &[MilestoneThreeReturnGateBlockerRow] =
        report.milestone_three_return_gate_blocker_rows.as_slice();
    let _: bool = report.milestone_three_return_gate_ready;
    let _: &[MilestoneThreeTopologyEditDigestRow] = report.topology_edit_digest_rows.as_slice();
    let _: &[MilestoneThreeNamingContinuityMatrixRow] =
        report.naming_edit_continuity_matrix_rows.as_slice();
    let _: &[MilestoneThreeRejectedEditScopeReportRow] =
        report.rejected_edit_scope_report_rows.as_slice();
    let _: &[topology::facade::MilestoneThreeEditReplayParityRow] =
        report.edit_replay_parity_rows.as_slice();
    let _: &[MilestoneThreeChangedScopeCoverageRow] = report.changed_scope_coverage_rows.as_slice();
    let _: &[MilestoneThreeDerivedRegionCoverageRow] =
        report.derived_region_coverage_rows.as_slice();
    let _: &[MilestoneThreeEditBreadthCounterRow] = report.edit_breadth_counter_rows.as_slice();
    let _: &[MilestoneThreeFailureLocalityRow] = report.failure_locality_rows.as_slice();
    let _: Option<&MilestoneThreeSplitCollapseChurnWitness> = report
        .scenario_reports
        .iter()
        .find_map(|scenario| scenario.split_collapse_churn_witness.as_ref());
}

fn _milestone_three_side_quest_row_contracts(
    contract_row: &MilestoneThreeSideQuestContractRow,
    blocker_row: &MilestoneThreeSideQuestBlockerRow,
) {
    let _: &str = contract_row.contract_name.as_str();
    let _: &str = contract_row.status.as_str();
    let _: &str = contract_row.reason.as_str();
    let _: &str = contract_row.row_digest.as_str();
    let _: &str = blocker_row.blocker_name.as_str();
    let _: &str = blocker_row.status.as_str();
    let _: &str = blocker_row.reason.as_str();
    let _: &str = blocker_row.row_digest.as_str();
}

fn _milestone_three_return_gate_blocker_row_contract(
    blocker_row: &MilestoneThreeReturnGateBlockerRow,
) {
    let _: &str = blocker_row.blocker_name.as_str();
    let _: &str = blocker_row.reason.as_str();
    let _: &str = blocker_row.row_digest.as_str();
}

fn _milestone_three_direct_acceptance_row_contracts(
    digest_row: &MilestoneThreeTopologyEditDigestRow,
    naming_row: &MilestoneThreeNamingContinuityMatrixRow,
    rejection_row: &MilestoneThreeRejectedEditScopeReportRow,
    replay_row: &topology::facade::MilestoneThreeEditReplayParityRow,
) {
    let _: topology::facade::MilestoneThreeHostileScenario = digest_row.scenario();
    let _: &topology::facade::TopologyEditDigest = digest_row.topology_edit_digest();
    let _: &str = digest_row.row_digest();
    let _: &topology::facade::NamingEditContinuityMatrix =
        naming_row.naming_edit_continuity_matrix();
    let _: topology::facade::TopologyEditNamingOutcome = naming_row.continuity_outcome_class();
    let _: Option<topology::facade::TopologyEditRejectionClass> =
        naming_row.continuity_rejection_class();
    let _: &topology::facade::RejectedEditScopeReport = rejection_row.rejected_edit_scope_report();
    let _: topology::facade::TopologyEditRejectionClass = rejection_row.rejection_class();
    let _: bool = replay_row.replay_checked();
    let _: topology::facade::ReplayParityStatus = replay_row.parity_status();
    let _: usize = replay_row.mismatch_count();
    let _: &str = replay_row.row_digest();
}

fn _milestone_three_aggregate_acceptance_row_contracts(
    scope_row: &MilestoneThreeChangedScopeCoverageRow,
    region_row: &MilestoneThreeDerivedRegionCoverageRow,
    breadth_row: &MilestoneThreeEditBreadthCounterRow,
    locality_row: &MilestoneThreeFailureLocalityRow,
) {
    let _: topology::facade::TopologyEditChangedScope = scope_row.changed_scope();
    let _: usize = scope_row.scenario_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] = scope_row.scenarios();
    let _: &str = scope_row.row_digest();
    let _: topology::facade::TopologyDerivedRegion = region_row.derived_region();
    let _: usize = region_row.scenario_count();
    let _: &[topology::facade::MilestoneThreeHostileScenario] = region_row.scenarios();
    let _: topology::facade::MilestoneThreeHostileScenario = breadth_row.scenario();
    let _: usize = breadth_row.contract_count();
    let _: usize = breadth_row.changed_scope_count();
    let _: bool = breadth_row.replay_checked();
    let _: topology::facade::TopologyEditRejectionClass = locality_row.rejection_class();
    let _: &[topology::facade::TopologyEditFamily] = locality_row.families();
    let _: &[topology::facade::TopologyEditChangedScope] = locality_row.changed_scopes();
    let _: &[topology::facade::TopologyDerivedRegion] = locality_row.derived_regions();
}

fn _milestone_three_split_collapse_witness_contract(
    witness: &MilestoneThreeSplitCollapseChurnWitness,
) {
    let _: &str = witness.original_wire_identity.as_str();
    let _: &str = witness.split_wire_identity.as_str();
    let _: &str = witness.collapse_wire_identity.as_str();
    let _: &[String] = witness.moved_half_edge_identities.as_slice();
    let _: &[String] = witness.retained_half_edge_identities.as_slice();
    let _: usize = witness.split_step_wire_count;
    let _: usize = witness.final_wire_count;
}

#[test]
fn milestone_three_side_quest_closeout_is_public_report_surface() {}
