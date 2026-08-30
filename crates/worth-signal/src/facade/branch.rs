//! Signal branch contracts.
//!
//! Owner-issued managed branch and owner-service contracts are composition-facing. Descriptor-only
//! readmission and portable snapshot reconstruction remain owner-root compatibility surfaces; their
//! exports here do not make them composition-port contracts.

pub use crate::branch::{
    signal_branch_identity, signal_branch_observation, AdmittedSignalBranchBasis,
    AdmittedSignalBranchSnapshot, ManagedSignalBranchReference,
    ManagedSignalBranchReferenceAdmissionDenial, PlannedSignalBranchRetirement,
    PlannedSignalBranchRetirementBatch, SignalBranchAdvanceDenial, SignalBranchAdvanceEngineDenial,
    SignalBranchAdvanceOutcome, SignalBranchBasisAuthority, SignalBranchBasisAuthorityMarker,
    SignalBranchBasisLifecyclePosture, SignalBranchBasisObservationDenial,
    SignalBranchBasisOwnerProof, SignalBranchBasisProof, SignalBranchBasisReadmissionDenial,
    SignalBranchComparisonBasis, SignalBranchForkBasis, SignalBranchForkOperationDenial,
    SignalBranchForkOutcome, SignalBranchIdentity, SignalBranchIdentityConstructionDenial,
    SignalBranchMergeDenial, SignalBranchMergeOutcome, SignalBranchObservation,
    SignalBranchObservationConstructionDenial, SignalBranchRestoreDenial,
    SignalBranchRetainedReadmissionDenial, SignalBranchRetentionAcquisitionDenial,
    SignalBranchRetentionLease, SignalBranchRetentionOwnerPosture,
    SignalBranchRetentionReleaseDenial, SignalBranchRetentionReleaseOutcome,
    SignalBranchRetentionReleaseReceipt, SignalBranchRetentionTerminalCounts,
    SignalBranchRetentionTerminalOutcome, SignalBranchRetirementBatchDenial,
    SignalBranchRetirementBatchReceipt, SignalBranchRetirementDenial, SignalBranchRetirementReason,
    SignalBranchRetirementReceipt, SignalBranchSnapshotCaptureDenial,
    SignalBranchSnapshotCaptureOutcome, SignalBranchTarget, SignalBranchTargetConstructionDenial,
    SignalOwnerLifecycleObservation, SignalOwnerServiceCostSnapshot, SignalOwnerUnavailable,
};

/// Owner-root compatibility exports, not composition-port inputs or operations.
pub use crate::branch::{
    SignalBranchBasisCompatibilityDenial, SignalBranchBasisDescriptor,
    SignalBranchSnapshotReconstructionDenial, SignalBranchSnapshotReconstructionOutcome,
    SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION,
};

#[cfg(feature = "test-operation-control")]
pub use crate::branch::SignalOwnerOperationBoundary;
