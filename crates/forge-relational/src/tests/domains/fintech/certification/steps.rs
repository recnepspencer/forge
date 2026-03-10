use forge_harness::facade::{WorkflowState, WorkflowStep};

#[derive(Debug, Clone, Copy)]
pub(super) enum FintechCaseRef {
    LateTradeCorrection,
    IntradayRisk,
    FailedSettlementRepair,
}

#[derive(Debug, Clone)]
pub(super) enum FintechWorkflowStep {
    CaptureMainSnapshot {
        alias: &'static str,
    },
    OpenAnalysisBranch {
        alias: &'static str,
    },
    ShockMarket {
        branch_alias: &'static str,
    },
    CorrectCaseTrade {
        branch_alias: &'static str,
        case: FintechCaseRef,
    },
    StressCaseRisk {
        branch_alias: &'static str,
        case: FintechCaseRef,
    },
    RepairCaseSettlement {
        branch_alias: &'static str,
        case: FintechCaseRef,
    },
    RefreshRisk {
        branch_alias: &'static str,
    },
    ReadSnapshot {
        snapshot_alias: &'static str,
        read_alias: &'static str,
    },
    ReadCaseProbe {
        case: FintechCaseRef,
        read_alias: &'static str,
    },
    CaptureReplay {
        branch_alias: &'static str,
        alias: &'static str,
    },
}

pub(super) fn certified_step(
    name: impl Into<String>,
    operation: FintechWorkflowStep,
) -> WorkflowStep<FintechWorkflowStep> {
    WorkflowStep::new(name, operation).capture_at(WorkflowState::Inspected)
}

pub(super) fn checkpoint_step(
    name: impl Into<String>,
    operation: FintechWorkflowStep,
) -> WorkflowStep<FintechWorkflowStep> {
    certified_step(name, operation)
        .checkpoint_after()
        .capture_at(WorkflowState::Checkpointed)
}
