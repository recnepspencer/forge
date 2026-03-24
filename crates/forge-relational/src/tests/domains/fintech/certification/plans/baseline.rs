use forge_harness::facade::{InvariantCheck, WorkflowPlan, WorkflowState};

use super::super::steps::{certified_step, checkpoint_step, FintechCaseRef, FintechWorkflowStep};

pub(super) fn relational_fintech_analysis_baseline_plan() -> WorkflowPlan<FintechWorkflowStep> {
    WorkflowPlan::new(
        "analysis-branch-trade-correction",
        "smoke-book-baseline",
        "forge-relational",
        "fintech",
    )
    .step(checkpoint_step(
        "capture-main-snapshot",
        FintechWorkflowStep::CaptureMainSnapshot {
            alias: "baseline_snapshot",
        },
    ))
    .step(certified_step(
        "open-analysis-branch",
        FintechWorkflowStep::OpenAnalysisBranch { alias: "analysis" },
    ))
    .step(certified_step(
        "shock-analysis-market",
        FintechWorkflowStep::ShockMarket {
            branch_alias: "analysis",
        },
    ))
    .step(certified_step(
        "correct-analysis-trade",
        FintechWorkflowStep::CorrectCaseTrade {
            branch_alias: "analysis",
            case: FintechCaseRef::LateTradeCorrection,
        },
    ))
    .step(certified_step(
        "refresh-analysis-risk",
        FintechWorkflowStep::RefreshRisk {
            branch_alias: "analysis",
        },
    ))
    .step(certified_step(
        "read-main-baseline",
        FintechWorkflowStep::ReadSnapshot {
            snapshot_alias: "baseline_snapshot",
            read_alias: "main_baseline",
        },
    ))
    .step(certified_step(
        "probe-late-trade-correction-case",
        FintechWorkflowStep::ReadCaseProbe {
            case: FintechCaseRef::LateTradeCorrection,
            read_alias: "analysis_correction_case",
        },
    ))
    .step(certified_step(
        "promote-analysis-lineage",
        FintechWorkflowStep::PromoteCaseCorrespondence {
            branch_alias: "analysis",
            left_case: FintechCaseRef::BaselinePortfolio,
            right_case: FintechCaseRef::LateTradeCorrection,
            resolution_alias: "analysis_lineage",
        },
    ))
    .step(certified_step(
        "capture-analysis-replay",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: "analysis",
            alias: "analysis_lineage_replay",
        },
    ))
    .invariant(InvariantCheck::new(
        "fixture_shape_smoke",
        "smoke fintech world remains a legitimate baseline fixture",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "read_nonempty:main_baseline",
        "baseline snapshot should remain queryable",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "case_correction_truth_visible:analysis_correction_case",
        "analysis branch should expose correction truth and audit truth",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "read_matches_case:analysis_correction_case:LateTradeCorrection",
        "analysis correction probe should target the late trade correction case",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "lineage_promotion_succeeded:analysis_lineage",
        "analysis baseline workflow should publish lineage correspondence",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_has_lineage_authority_basis:analysis_lineage_replay",
        "analysis baseline replay should expose lineage authority basis",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_uses_exact_lineage_digest:analysis_lineage_replay",
        "analysis baseline replay should certify exact canonical lineage digests",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "branch_head_matches_latest:analysis",
        "analysis branch head should track the latest commit after the workflow",
        WorkflowState::Completed,
    ))
}
