use worth_harness::facade::{InvariantCheck, WorkflowPlan, WorkflowState};

use super::super::steps::{certified_step, FintechCaseRef, FintechWorkflowStep};

pub(super) fn relational_fintech_settlement_repair_plan() -> WorkflowPlan<FintechWorkflowStep> {
    WorkflowPlan::new(
        "failed-settlement-repair",
        "seeded-settlement-repair",
        "worth-relational",
        "fintech",
    )
    .step(certified_step(
        "open-analysis-branch",
        FintechWorkflowStep::OpenAnalysisBranch { alias: "analysis" },
    ))
    .step(certified_step(
        "repair-seeded-settlement",
        FintechWorkflowStep::RepairCaseSettlement {
            branch_alias: "analysis",
            case: FintechCaseRef::FailedSettlementRepair,
        },
    ))
    .step(certified_step(
        "probe-settlement-repair-case",
        FintechWorkflowStep::ReadCaseProbe {
            case: FintechCaseRef::FailedSettlementRepair,
            read_alias: "analysis_repair_case",
        },
    ))
    .step(certified_step(
        "promote-settlement-lineage",
        FintechWorkflowStep::PromoteCaseCorrespondence {
            branch_alias: "analysis",
            left_case: FintechCaseRef::BaselinePortfolio,
            right_case: FintechCaseRef::FailedSettlementRepair,
            resolution_alias: "analysis_repair_lineage",
        },
    ))
    .step(certified_step(
        "capture-analysis-replay",
        FintechWorkflowStep::CaptureReplay {
            branch_alias: "analysis",
            alias: "analysis_repair_replay",
        },
    ))
    .invariant(InvariantCheck::new(
        "fixture_shape_smoke",
        "smoke fintech world remains a legitimate baseline fixture",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "case_settlement_repair_visible:analysis_repair_case",
        "settlement repair workflow should expose repaired settlement truth and audit truth",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "read_matches_case:analysis_repair_case:FailedSettlementRepair",
        "settlement repair probe should target the failed settlement repair case",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "lineage_promotion_succeeded:analysis_repair_lineage",
        "settlement repair workflow should publish lineage correspondence",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "branch_head_matches_latest:analysis",
        "analysis branch head should track the latest commit after the workflow",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_has_no_failure:analysis_repair_replay",
        "settlement repair replay should complete without failure",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_has_lineage_authority_basis:analysis_repair_replay",
        "settlement repair replay should expose lineage authority basis",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_uses_exact_lineage_digest:analysis_repair_replay",
        "settlement repair replay should certify against exact canonical lineage digests",
        WorkflowState::Completed,
    ))
    .invariant(InvariantCheck::new(
        "replay_targets_branch:analysis_repair_replay:analysis",
        "settlement repair replay should remain local to the analysis branch",
        WorkflowState::Completed,
    ))
}
