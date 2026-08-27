use crate::effect_lifecycle::{
    EffectDiagnosticsMaterialization, EffectExecutionReceipt, EffectReceiptTargetEvidence,
};
use crate::runtime::WorthQueryOrdinaryMergeFailureStage;
use crate::WorthQueryEvidenceIdentity;

use crate::ordinary::workflow::{
    WorthQueryAdmittedWorkflowEffect, WorthQueryLoweredWorkflowPlan, WorthQueryWorkflowCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBranchMergeStopSource {
    ForeignAuthority,
    StaleAuthority,
    MismatchedContext,
    InspectionUnavailable,
    Basis,
    Intent,
    Eligibility,
    Lowering,
    RelationalExecution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBranchMergeNextAction {
    ProvideCurrentBranchAuthority,
    RebindCurrentBranches,
    UseMatchingDeclaration,
    UseOperationalReceipt,
    InspectDenial,
    RetryDeferredPublication,
    RepairDeferredBranchMergeSettlement,
}

pub struct WorthQueryBranchMergeDeferred {
    message: String,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryBranchMergeDeferred {
    pub(crate) fn new(message: String, counters: WorthQueryWorkflowCounters) -> Self {
        Self { message, counters }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub fn next_action(&self) -> WorthQueryBranchMergeNextAction {
        WorthQueryBranchMergeNextAction::RetryDeferredPublication
    }
}

pub struct WorthQueryBranchMergeStop {
    source: WorthQueryBranchMergeStopSource,
    message: String,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryBranchMergeStop {
    pub fn source(&self) -> WorthQueryBranchMergeStopSource {
        self.source
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub fn next_action(&self) -> WorthQueryBranchMergeNextAction {
        match self.source {
            WorthQueryBranchMergeStopSource::ForeignAuthority => {
                WorthQueryBranchMergeNextAction::ProvideCurrentBranchAuthority
            }
            WorthQueryBranchMergeStopSource::StaleAuthority => {
                WorthQueryBranchMergeNextAction::RebindCurrentBranches
            }
            WorthQueryBranchMergeStopSource::MismatchedContext => {
                WorthQueryBranchMergeNextAction::UseMatchingDeclaration
            }
            WorthQueryBranchMergeStopSource::InspectionUnavailable => {
                WorthQueryBranchMergeNextAction::UseOperationalReceipt
            }
            WorthQueryBranchMergeStopSource::Basis
            | WorthQueryBranchMergeStopSource::Intent
            | WorthQueryBranchMergeStopSource::Eligibility
            | WorthQueryBranchMergeStopSource::Lowering
            | WorthQueryBranchMergeStopSource::RelationalExecution => {
                WorthQueryBranchMergeNextAction::InspectDenial
            }
        }
    }

    pub(crate) fn denied(
        source: WorthQueryBranchMergeStopSource,
        message: impl Into<String>,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source,
            message: message.into(),
            counters,
        }
    }

    pub(crate) fn from_execution_denial(
        stage: WorthQueryOrdinaryMergeFailureStage,
        message: String,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source: source_for_stage(stage),
            message,
            counters,
        }
    }
}

pub struct WorthQueryBranchMergeSettlementDeferred {
    message: String,
    counters: WorthQueryWorkflowCounters,
    settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
}

impl WorthQueryBranchMergeSettlementDeferred {
    pub(crate) fn new(
        message: String,
        settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            message,
            counters: counters.settlement_deferred(),
            settlement,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub(crate) fn settlement(
        &self,
    ) -> &worth_relational::facade::publication::DeferredPublicationSettlement {
        &self.settlement
    }

    pub fn next_action(&self) -> WorthQueryBranchMergeNextAction {
        WorthQueryBranchMergeNextAction::RepairDeferredBranchMergeSettlement
    }
}

fn source_for_stage(stage: WorthQueryOrdinaryMergeFailureStage) -> WorthQueryBranchMergeStopSource {
    match stage {
        WorthQueryOrdinaryMergeFailureStage::Basis => WorthQueryBranchMergeStopSource::Basis,
        WorthQueryOrdinaryMergeFailureStage::Intent => WorthQueryBranchMergeStopSource::Intent,
        WorthQueryOrdinaryMergeFailureStage::Eligibility => {
            WorthQueryBranchMergeStopSource::Eligibility
        }
        WorthQueryOrdinaryMergeFailureStage::Lowering => WorthQueryBranchMergeStopSource::Lowering,
        WorthQueryOrdinaryMergeFailureStage::RelationalExecution => {
            WorthQueryBranchMergeStopSource::RelationalExecution
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBranchMergeAftermath {
    commit_id: u64,
    version_id: u64,
    identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryBranchMergeAftermath {
    pub fn commit_id(&self) -> u64 {
        self.commit_id
    }

    pub fn version_id(&self) -> u64 {
        self.version_id
    }

    pub fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub fn identity_for_reporting(&self) -> &str {
        self.identity.as_str()
    }

    pub(crate) fn from_receipt(receipt: &EffectExecutionReceipt) -> Option<Self> {
        match receipt.target_evidence() {
            EffectReceiptTargetEvidence::MergeCommit {
                commit_id,
                version_id,
            } => Some(Self {
                commit_id,
                version_id,
                identity: receipt
                    .integrity_markers()
                    .authority_artifact_identity()
                    .clone(),
            }),
            _ => None,
        }
    }
}

pub struct WorthQueryBranchMergeCompletion {
    admitted_effect: WorthQueryAdmittedWorkflowEffect,
    lowered_plan: WorthQueryLoweredWorkflowPlan,
    receipt: EffectExecutionReceipt,
    aftermath: WorthQueryBranchMergeAftermath,
    diagnostics: Option<EffectDiagnosticsMaterialization>,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryBranchMergeCompletion {
    pub fn admitted_effect(&self) -> &WorthQueryAdmittedWorkflowEffect {
        &self.admitted_effect
    }

    pub fn lowered_plan(&self) -> &WorthQueryLoweredWorkflowPlan {
        &self.lowered_plan
    }

    pub fn receipt(&self) -> &EffectExecutionReceipt {
        &self.receipt
    }

    pub fn aftermath(&self) -> &WorthQueryBranchMergeAftermath {
        &self.aftermath
    }

    pub fn diagnostics(&self) -> Option<&EffectDiagnosticsMaterialization> {
        self.diagnostics.as_ref()
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub(crate) fn new(
        admitted_effect: WorthQueryAdmittedWorkflowEffect,
        lowered_plan: WorthQueryLoweredWorkflowPlan,
        receipt: EffectExecutionReceipt,
        aftermath: WorthQueryBranchMergeAftermath,
        diagnostics: Option<EffectDiagnosticsMaterialization>,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            admitted_effect,
            lowered_plan,
            receipt,
            aftermath,
            diagnostics,
            counters,
        }
    }
}

pub enum WorthQueryBranchMergeOutcome {
    Completed(WorthQueryBranchMergeCompletion),
    Stopped(WorthQueryBranchMergeStop),
    Deferred(WorthQueryBranchMergeDeferred),
    SettlementDeferred(WorthQueryBranchMergeSettlementDeferred),
}

impl WorthQueryBranchMergeOutcome {
    pub fn completed(&self) -> Option<&WorthQueryBranchMergeCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) | Self::Deferred(_) | Self::SettlementDeferred(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryBranchMergeStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
            Self::Deferred(_) | Self::SettlementDeferred(_) => None,
        }
    }

    pub fn deferred(&self) -> Option<&WorthQueryBranchMergeDeferred> {
        match self {
            Self::Deferred(deferred) => Some(deferred),
            Self::Completed(_) | Self::Stopped(_) | Self::SettlementDeferred(_) => None,
        }
    }

    pub fn settlement_deferred(&self) -> Option<&WorthQueryBranchMergeSettlementDeferred> {
        match self {
            Self::Completed(_) | Self::Stopped(_) | Self::Deferred(_) => None,
            Self::SettlementDeferred(deferred) => Some(deferred),
        }
    }
}
