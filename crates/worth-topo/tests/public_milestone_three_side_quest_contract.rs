use forge_relational::facade::runtime::RelationalRuntime;
use topology::facade::{
    certify_milestone_three_closeout, MilestoneThreeHostileSuiteReport,
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
    MilestoneThreeSplitCollapseChurnWitness, TopologyCertificationError,
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
