mod baseline;
mod intraday_risk;
mod regressions;
mod settlement_repair;

pub(super) fn relational_fintech_analysis_baseline_plan(
) -> worth_harness::facade::WorkflowPlan<super::steps::FintechWorkflowStep> {
    baseline::relational_fintech_analysis_baseline_plan()
}

pub(super) fn relational_fintech_replay_regression_plan(
) -> worth_harness::facade::WorkflowPlan<super::steps::FintechWorkflowStep> {
    regressions::relational_fintech_replay_regression_plan()
}

pub(super) fn relational_fintech_intraday_risk_plan(
) -> worth_harness::facade::WorkflowPlan<super::steps::FintechWorkflowStep> {
    intraday_risk::relational_fintech_intraday_risk_plan()
}

pub(super) fn relational_fintech_settlement_repair_plan(
) -> worth_harness::facade::WorkflowPlan<super::steps::FintechWorkflowStep> {
    settlement_repair::relational_fintech_settlement_repair_plan()
}
