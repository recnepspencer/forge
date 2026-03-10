use forge_harness::facade::{InvariantCheck, WorkflowPlan, WorkflowState};

use super::super::steps::{certified_step, FintechCaseRef, FintechWorkflowStep};

pub(super) fn relational_fintech_intraday_risk_plan() -> WorkflowPlan<FintechWorkflowStep> {
    WorkflowPlan::new(
        "intraday-risk-breach",
        "seeded-intraday-risk",
        "forge-relational",
        "fintech",
    )
    .step(certified_step(
        "open-analysis-branch",
        FintechWorkflowStep::OpenAnalysisBranch { alias: "analysis" },
    ))
    .step(certified_step(
        "stress-seeded-intraday-risk",
        FintechWorkflowStep::StressCaseRisk {
            branch_alias: "analysis",
            case: FintechCaseRef::IntradayRisk,
        },
    ))
    .step(certified_step(
        "probe-intraday-risk-case",
        FintechWorkflowStep::ReadCaseProbe {
            case: FintechCaseRef::IntradayRisk,
            read_alias: "analysis_intraday_case",
        },
    ))
    .step(certified_step(
        "capture-analysis-replay",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: "analysis",
            alias: "analysis_intraday_replay",
        },
    ))
    .invariant(InvariantCheck::new(
        "fixture_shape_smoke",
        "smoke fintech world remains a legitimate baseline fixture",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "read_has_open_breach:analysis_intraday_case",
        "intraday workflow should expose an open risk breach",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "read_matches_case:analysis_intraday_case:IntradayRisk",
        "intraday probe should target the intraday risk case",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "branch_head_matches_latest:analysis",
        "analysis branch head should track the latest commit after the workflow",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_has_no_failure:analysis_intraday_replay",
        "intraday replay should complete without failure",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_targets_branch:analysis_intraday_replay:analysis",
        "intraday replay should remain local to the analysis branch",
        WorkflowState::Completed,
    ))
}
