use worth_harness::facade::{
    InvariantCheck, RegressionTarget, RegressionTargetKind, WorkflowPlan, WorkflowState,
};

use super::super::steps::{certified_step, FintechCaseRef, FintechWorkflowStep};

pub(super) fn relational_fintech_replay_regression_plan() -> WorkflowPlan<FintechWorkflowStep> {
    WorkflowPlan::new(
        "analysis-branch-replay-regression",
        "smoke-book-replay-drift",
        "worth-relational",
        "fintech",
    )
    .with_regression_target(RegressionTarget {
        kind: RegressionTargetKind::KnownFailing,
        issue_key: "relational-fintech-replay-drift".to_string(),
        summary: "Known replay drift under partitioned fintech branch trade correction workflow"
            .to_string(),
        reproduction_hint: Some(
            "Run the harness workflow and inspect the replay-recovery artifact plus failure bundle"
                .to_string(),
        ),
    })
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
        "capture-analysis-replay",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: "analysis",
            alias: "analysis_replay",
        },
    ))
    .step(certified_step(
        "refresh-analysis-risk",
        FintechWorkflowStep::RefreshRisk {
            branch_alias: "analysis",
        },
    ))
    .invariant(InvariantCheck::new(
        "replay_has_no_failure:analysis_replay",
        "analysis replay should remain stable after trade correction",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_targets_branch:analysis_replay:analysis",
        "analysis replay should remain local to the analysis branch",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_has_lineage_authority_basis:analysis_replay",
        "analysis replay should expose lineage authority basis even in regression mode",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_uses_exact_lineage_digest:analysis_replay",
        "analysis replay should certify exact canonical lineage digests even in regression mode",
        WorkflowState::Completed,
    ))
}
