use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanDownstreamSplitConsumptionDenial;
use worth_spatial::facade::planar_boolean_events::PlanarBooleanEventLedgerLookupExecutionDenial;
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
};
use worth_spatial::facade::workload_vocabulary::{
    SpatialEvidenceLookupDenial, SpatialGeometryEvidenceTouchDenial,
    SpatialGeometryEvidenceTouchDenialKind, WorkloadEvidenceStage,
};

use super::lookup_consumed_workload::LookupConsumedWorkloadReuseResolutionDenied;
use crate::workload_composition::{
    conflict_input::ConflictInputAdmissionError, WorkloadStageRequirement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LookupConsumedWorkloadDenial {
    StageIndexIdentityMismatch,
    BroadEvidenceFallbackScan,
    CallerOwnedLookupScan,
    ReuseResolutionDenied(LookupConsumedWorkloadReuseResolutionDenied),
    ReuseResolutionSelectedPlanMismatch,
    ReuseResolutionSelectedFamilyMismatch,
    ReuseResolutionSelectedReuseBasisMismatch,
    SplitLookupLineageMismatch,
    MissingWorkloadAttachedBatchAdmissionExecutionReceipt,
    SuppliedBatchAdmissionExecutionReceiptMismatch,
    CutoverProof(String),
}

impl LookupConsumedWorkloadDenial {
    pub fn human_reason(&self) -> String {
        match self {
            Self::StageIndexIdentityMismatch => {
                "lookup-consumed workload handoff must match the workload stage-index identity"
                    .to_string()
            }
            Self::BroadEvidenceFallbackScan => {
                "lookup-consumed workload composition rejects raw evidence and broad receipt fallback"
                    .to_string()
            }
            Self::CallerOwnedLookupScan => {
                "lookup-consumed workload composition rejects caller-owned lookup scans".to_string()
            }
            Self::ReuseResolutionDenied(denial) => denial.human_reason(),
            Self::ReuseResolutionSelectedPlanMismatch => {
                "lookup-consumed workload composition requires reuse resolution whose selected plan matches the admitted lookup handoff"
                    .to_string()
            }
            Self::ReuseResolutionSelectedFamilyMismatch => {
                "lookup-consumed workload composition requires reuse resolution whose selected equivalence family matches the admitted lookup handoff"
                    .to_string()
            }
            Self::ReuseResolutionSelectedReuseBasisMismatch => {
                "lookup-consumed workload composition requires reuse resolution whose selected reuse basis matches the admitted lookup handoff"
                    .to_string()
            }
            Self::SplitLookupLineageMismatch => {
                "split ledger lookup lineage must match the admitted event-ledger lookup packet"
                    .to_string()
            }
            Self::MissingWorkloadAttachedBatchAdmissionExecutionReceipt => {
                "lookup-consumed grouped consumers require a workload-attached batch-admission execution receipt"
                    .to_string()
            }
            Self::SuppliedBatchAdmissionExecutionReceiptMismatch => {
                "lookup-consumed grouped consumers require the supplied batch-admission execution receipt to match the workload"
                    .to_string()
            }
            Self::CutoverProof(reason) => reason.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoBoundaryDenial {
    MissingMigratedTransactionBoundaryPacket,
    SplitLookupReceiptIdentityMismatch,
    PacketStageIndexMismatchCompletedSplit,
    PacketLookupReceiptMismatchCompletedSplit,
    PacketStageIndexMismatchSpatialTouchAuthority,
}

impl ReplayUndoBoundaryDenial {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingMigratedTransactionBoundaryPacket => {
                "loop reconstruction handoff does not carry a migrated replay/undo transaction boundary packet"
                    .to_string()
            }
            Self::SplitLookupReceiptIdentityMismatch => {
                "boolean split replay/undo boundary requires one matching split-ledger receipt and lookup stage receipt identity"
                    .to_string()
            }
            Self::PacketStageIndexMismatchCompletedSplit => {
                "boolean split replay/undo boundary packet must match the completed split workload stage-index identity"
                    .to_string()
            }
            Self::PacketLookupReceiptMismatchCompletedSplit => {
                "boolean split replay/undo boundary packet must match the completed split lookup execution receipt identity"
                    .to_string()
            }
            Self::PacketStageIndexMismatchSpatialTouchAuthority => {
                "boolean split replay/undo boundary packet must match the split spatial touch authority stage-index identity"
                    .to_string()
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkloadCompositionError {
    UnsupportedStage(WorkloadStageRequirement),
    MissingEvidenceStage(WorkloadEvidenceStage),
    ManualEvidenceStage(WorkloadEvidenceStage),
    CounterlessEvidenceStage(WorkloadEvidenceStage),
    MismatchedEvidenceStage(WorkloadEvidenceStage),
    LoopReconstructionCloseout(String),
    LoopRuntimeRegistration(String),
    OverlapRegionCloseout(String),
    OverlapRegionSummumBonumCloseout(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial),
    OverlapRuntimeRegistration(String),
    BooleanChainHandoff(String),
    SpatialTouchAuthority(SpatialGeometryEvidenceTouchDenial),
    SpatialEvidenceLookup(SpatialEvidenceLookupDenial),
    EventLedgerLookupExecution(PlanarBooleanEventLedgerLookupExecutionDenial),
    EventLedgerLookupExecutionPacket(PlanarBooleanEventLedgerLookupExecutionDenial),
    DownstreamSplitConsumption(PlanarBooleanDownstreamSplitConsumptionDenial),
    ReplayUndoBoundary(ReplayUndoBoundaryDenial),
    ReplayUndoTransactionBoundary(
        crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryError,
    ),
    ConflictInput(ConflictInputAdmissionError),
    LookupConsumedWorkload(LookupConsumedWorkloadDenial),
}

impl WorkloadCompositionError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::UnsupportedStage(stage) => format!(
                "{} is not admitted for operator composition",
                stage.human_name()
            ),
            Self::MissingEvidenceStage(stage) => {
                format!("workload evidence ledger is missing {}", stage.human_name())
            }
            Self::ManualEvidenceStage(stage) => format!(
                "workload evidence ledger has hand-filled {} instead of a source receipt",
                stage.human_name()
            ),
            Self::CounterlessEvidenceStage(stage) => format!(
                "workload evidence ledger cannot count {} without receipt-backed counters",
                stage.human_name()
            ),
            Self::MismatchedEvidenceStage(stage) => format!(
                "workload evidence ledger does not match the {} receipt",
                stage.human_name()
            ),
            Self::LoopReconstructionCloseout(reason) => reason.clone(),
            Self::LoopRuntimeRegistration(reason) => reason.clone(),
            Self::OverlapRegionCloseout(reason) => reason.clone(),
            Self::OverlapRegionSummumBonumCloseout(denial) => denial.detail().to_string(),
            Self::OverlapRuntimeRegistration(reason) => reason.clone(),
            Self::BooleanChainHandoff(reason) => reason.clone(),
            Self::SpatialTouchAuthority(denial) => denial.human_reason(),
            Self::SpatialEvidenceLookup(denial) => denial.detail().to_string(),
            Self::EventLedgerLookupExecution(denial) => denial.detail().to_string(),
            Self::EventLedgerLookupExecutionPacket(denial) => denial.detail().to_string(),
            Self::DownstreamSplitConsumption(denial) => denial.human_reason().to_string(),
            Self::ReplayUndoBoundary(denial) => denial.human_reason(),
            Self::ReplayUndoTransactionBoundary(error) => format!("{error:?}"),
            Self::ConflictInput(error) => error.detail().to_string(),
            Self::LookupConsumedWorkload(denial) => denial.human_reason(),
        }
    }

    pub fn spatial_touch_denial(&self) -> Option<&SpatialGeometryEvidenceTouchDenial> {
        match self {
            Self::SpatialTouchAuthority(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn spatial_touch_denial_kind(&self) -> Option<SpatialGeometryEvidenceTouchDenialKind> {
        self.spatial_touch_denial().map(|denial| denial.kind())
    }

    pub fn lookup_consumed_workload_denial(&self) -> Option<&LookupConsumedWorkloadDenial> {
        match self {
            Self::LookupConsumedWorkload(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn replay_undo_boundary_denial(&self) -> Option<&ReplayUndoBoundaryDenial> {
        match self {
            Self::ReplayUndoBoundary(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn overlap_region_summum_bonum_closeout_denial(
        &self,
    ) -> Option<&PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
        match self {
            Self::OverlapRegionSummumBonumCloseout(denial) => Some(denial),
            _ => None,
        }
    }
}
