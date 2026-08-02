mod admission;
mod availability;
#[cfg(feature = "certification-test-authority")]
mod certification_input;
mod diagnostics;
mod durability;
mod identity;
mod instance;
mod lifecycle;
#[cfg(feature = "certification-test-authority")]
mod media_evidence;
mod media_ownership;
mod observation;
mod record_serving;
mod resource_lifecycle;
mod root_admission;
mod runtime;
mod shutdown;
mod work;

pub use admission::{
    AdmissionError, CancelledPhysicalRuntimeAdmission, DeclaredStoreRootDenialKind,
    PhysicalRuntimeAdmission, PhysicalStore,
};
pub use availability::{CapabilityAvailability, InstalledCapabilityStatus, PhysicalCapability};
pub use diagnostics::{ProcessRuntimeCounterSnapshot, RuntimeCounterSnapshot};
pub use durability::{
    AdmittedPhysicalDurabilityGroup, AdmittedPhysicalDurabilityGroupMember,
    AdmittedPhysicalDurabilityPolicy, CanonicalRedoRecords, CertifiedPriorPageBasis,
    CertifiedPriorPageImage, CheckpointMemoryLimit, CleanedPhysicalDataDispatchRetry,
    CompletedPhysicalCheckpoint, CompletedPhysicalMutation, CompletedPhysicalRootPublication,
    CompletedUnobservedPhysicalMutation, ContiguousRetainedWalTail, DataDispatchedPhysicalMutation,
    DataSettledPhysicalMutation, DataSettledPhysicalMutationMembers, GroupCommitDelay,
    GroupCommitLimit, IdempotencyRetentionGenerations, IndeterminatePhysicalCheckpoint,
    IndeterminatePhysicalCurrentRootAdvance, IndeterminatePhysicalDataDispatch,
    IndeterminatePhysicalMutation, IndeterminatePhysicalMutationEvidence,
    IndeterminatePhysicalRootNamespaceDurability, IndeterminatePhysicalRootPublicationPreparation,
    IndeterminatePhysicalRootReplacement, IndeterminatePhysicalWalGroupAppend,
    IndeterminatePhysicalWalGroupBarrier, LiveIdempotencyBindingLimit, PageWalBasis,
    PendingUnresolvedMutationLimit, PhysicalBindingCompactionReopenFailure,
    PhysicalCheckpointCancellationOutcome, PhysicalCheckpointCaptureBasis,
    PhysicalCheckpointCaptureFailureKind, PhysicalCheckpointDeadline, PhysicalCheckpointDisposal,
    PhysicalCheckpointHandle, PhysicalCheckpointIdempotencyKey, PhysicalCheckpointOutcome,
    PhysicalCheckpointPolicy, PhysicalCheckpointPoll, PhysicalCheckpointProgress,
    PhysicalCheckpointProgressPhase, PhysicalCheckpointProvenNoEffectCause,
    PhysicalCheckpointRequest, PhysicalCheckpointShutdown, PhysicalCheckpointStartDeferred,
    PhysicalCheckpointStartDenial, PhysicalCheckpointStartFailure, PhysicalCheckpointStartOutcome,
    PhysicalCheckpointStartRebindRequired, PhysicalCheckpointStartStale,
    PhysicalCheckpointSubmission, PhysicalCurrentRootAdvanceFailureCause,
    PhysicalCurrentRootAdvanceOutcome, PhysicalDataDispatchFailureCause,
    PhysicalDataDispatchOutcome, PhysicalDataEffectSettlement, PhysicalDataEffectSource,
    PhysicalDataFrameIdentity, PhysicalDataFrameKind, PhysicalDataFrameSubject,
    PhysicalDataSettledGroupAdmissionOutcome, PhysicalDataSettledGroupDenial,
    PhysicalDataSettlementFailureCause, PhysicalDataSettlementOutcome,
    PhysicalDurabilityDeclaration, PhysicalDurabilityDeclarationBuilder,
    PhysicalDurabilityGroupAdmissionDenial, PhysicalDurabilityGroupAdmissionOutcome,
    PhysicalDurabilityGroupBasis, PhysicalDurabilityGroupIdentity,
    PhysicalDurabilityGroupMemberBinding, PhysicalDurabilityGroupSealingDenial,
    PhysicalDurabilityObservation, PhysicalDurabilityPolicyAdmissionOutcome,
    PhysicalDurabilityPolicyDeferred, PhysicalDurabilityPolicyDenial,
    PhysicalDurabilityPolicyFailure, PhysicalDurabilityPolicyIdentity,
    PhysicalDurabilityPolicyRebindRequired, PhysicalDurabilityPolicyStale,
    PhysicalDurabilityReopenObservation, PhysicalGroupAppendAmplificationObservation,
    PhysicalGroupBarrierAmplificationObservation, PhysicalGroupMemberOrdinal,
    PhysicalGroupQueueAdmissionTick, PhysicalGroupRootPublicationPlan, PhysicalIdempotencyPolicy,
    PhysicalIdempotencyReopenFailure, PhysicalMutationAcknowledgment,
    PhysicalMutationBindingCompaction, PhysicalMutationCancellationOutcome,
    PhysicalMutationCompletedBreadth, PhysicalMutationDeadline,
    PhysicalMutationExecutedBoundaryEvidence, PhysicalMutationHandle,
    PhysicalMutationIdempotencyIssuanceDenial, PhysicalMutationIdempotencyKey,
    PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdempotencyLease,
    PhysicalMutationIdempotencyMaterial, PhysicalMutationIdentity,
    PhysicalMutationIndeterminateStage, PhysicalMutationObservation, PhysicalMutationOutcome,
    PhysicalMutationPerformanceEvidence, PhysicalMutationPoll, PhysicalMutationProgress,
    PhysicalMutationProgressPhase, PhysicalMutationProvenNoEffectCause, PhysicalMutationRequest,
    PhysicalMutationRequestFingerprint, PhysicalMutationShutdown,
    PhysicalMutationTerminalObservation, PhysicalNamespaceDurableCheckpointGeneration,
    PhysicalRedoLsn, PhysicalRedoTargetClaim, PhysicalRootCandidateSynchronizationFailureCause,
    PhysicalRootCandidateWriteFailureCause, PhysicalRootCandidateWriteFailurePosture,
    PhysicalRootNamespaceDurabilityFailureCause, PhysicalRootNamespaceDurabilityNotStarted,
    PhysicalRootNamespaceDurabilityOutcome, PhysicalRootPublicationMemberIdentity,
    PhysicalRootPublicationPreparationFailureCause, PhysicalRootPublicationPreparationNotStarted,
    PhysicalRootPublicationPreparationOutcome, PhysicalRootPublicationTransitionDenial,
    PhysicalRootPublicationWorkFailureCause, PhysicalRootReplacementFailureCause,
    PhysicalRootReplacementNotStarted, PhysicalRootReplacementOutcome,
    PhysicalWalAppendDeclaration, PhysicalWalAppendFailureCause, PhysicalWalAppendSettlement,
    PhysicalWalBarrierSettlement, PhysicalWalFrameWriteDisposition,
    PhysicalWalGroupAppendContinuation, PhysicalWalGroupAppendFailureCause,
    PhysicalWalGroupAppendOutcome, PhysicalWalGroupBarrierDeclaration,
    PhysicalWalGroupBarrierDeclarationDenial, PhysicalWalGroupBarrierFailureCause,
    PhysicalWalGroupBarrierOutcome, PhysicalWalGroupBarrierSettlement, PhysicalWalMemberBasis,
    PhysicalWalMemberIdentity, PhysicalWalObservation, PhysicalWalOpenFailure, PhysicalWalPolicy,
    PhysicalWalReclamationObservation, PhysicalWalReclamationReport, PhysicalWalReservationDenial,
    ProvenNoEffectPhysicalCheckpoint, ProvenNoEffectPhysicalMutation,
    ProvenNoEffectPhysicalMutationEvidence, RedoRecord, RejectedDataSettledPhysicalMutationMembers,
    RejectedPhysicalDurabilityGroup, RetainedPhysicalRoot, RetainedWalSegment,
    RetainedWalTailLimit, RootNamespaceDurablePhysicalMutationMembers,
    RootPublicationPhysicalMutationMember, RootPublicationPreparedPhysicalMutationMembers,
    RootReplacedPhysicalMutationMembers, SealedPhysicalDurabilityGroupMembers,
    SharedPhysicalRootPublicationPlan, WalAppendedPhysicalMutation, WalBarrierMember,
    WalDurablePhysicalMutation, WalDurablePhysicalMutationMembers,
    WalRangeReservedPhysicalMutation, WalSegmentByteLimit, WalSegmentInventoryLimit,
};
pub use identity::{DeclaredStoreRoot, RuntimeIdentity};
pub use instance::{
    PhysicalDurabilityStateReopenFailure, PhysicalSignalClockObservation,
    PhysicalSignalClockObservationFailure, PhysicalSignalConstructionFailure,
    PhysicalSignalDeltaApplicationFailure, PhysicalSignalObservation,
    PhysicalSignalRuntimeIdentity, PhysicalSignalShutdownOutcome, PhysicalStoreAbortOutcome,
    PhysicalStoreCloseObservation, PhysicalStoreCloseOutcome, PhysicalStoreClosePhase,
    PhysicalStoreClosePlan, PhysicalWorkExecution,
};
pub use lifecycle::LifecycleGeneration;
pub use media_ownership::{
    FilesystemMediaAdmission, MediaAdmissionDeferred, MediaAdmissionDenial,
    MediaAdmissionInspectionCause, MediaAdmissionInspectionRequired, MediaAdmissionOutcome,
    MediaAdmissionRebindRequired, MediaAdmissionStale, MediaOwnedObservationPhase,
    MediaOwnedPhysicalRuntime, MediaShutdownOutcome, PhysicalMediaObservation,
    PhysicalMediaObserver, RecordServingObservationPhase,
};
pub use observation::{
    LifecycleObservation, ObservationError, ObservationHandle, RootAdmissionObservation,
    RuntimeObservation,
};
pub use record_serving::*;
pub use runtime::AdmittedPhysicalRuntime;
pub use shutdown::{AbortedRuntime, ClosedRuntime};
pub use work::{
    AdmittedPhysicalWork, AdmittedPhysicalWorkAuthority, BlockedPhysicalWork,
    CompletedPhysicalCheckpointAction, CompletedPhysicalPublicationEffect,
    CompletedPhysicalWalBarrier, CompletedPhysicalWalReclamationAction, DispatchedPhysicalWork,
    PhysicalCheckpointRecoveryAction, PhysicalEffectIdentity, PhysicalEffectObligation,
    PhysicalExecutorCommand, PhysicalExecutorCommandDenial, PhysicalMetadataReadWorkRequest,
    PhysicalMutationSubmission, PhysicalMutationWorkRequest, PhysicalOperationIdentity,
    PhysicalPublicationEffect, PhysicalReadSubmission, PhysicalReadWorkRequest,
    PhysicalRetryCommand, PhysicalSchedulerDemand, PhysicalSchedulerDenial,
    PhysicalSignalAspectBinding, PhysicalSignalAspectBindingDigest,
    PhysicalSignalAspectBindingObservation, PhysicalSignalAspectBindingSet,
    PhysicalSignalAspectDeclaration, PhysicalSignalAspectRole, PhysicalSignalAspectSubscription,
    PhysicalSignalBindingDenial, PhysicalSignalProfileIdentity, PhysicalSignalSettlementOutcome,
    PhysicalWalAppendScope, PhysicalWalBarrierScope, PhysicalWorkAdmission,
    PhysicalWorkAspectDelta, PhysicalWorkAspectDeltaDenial, PhysicalWorkBatchDenial,
    PhysicalWorkCancellationFailure, PhysicalWorkCancellationJoin, PhysicalWorkCapacity,
    PhysicalWorkCapacityDimension, PhysicalWorkCausalObservation, PhysicalWorkCausalRecord,
    PhysicalWorkConcurrencyRelation, PhysicalWorkConcurrencyScope, PhysicalWorkConsumerHandle,
    PhysicalWorkCounterSnapshot, PhysicalWorkCounterStage, PhysicalWorkDeclarationDenial,
    PhysicalWorkDrainObservation, PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass,
    PhysicalWorkEffectFate, PhysicalWorkExecutionBatchOutcome, PhysicalWorkExecutionOutcome,
    PhysicalWorkGeneration, PhysicalWorkHealthRevocation, PhysicalWorkIdentity, PhysicalWorkIntent,
    PhysicalWorkNoEffectEvidence, PhysicalWorkObservation, PhysicalWorkOperationFamily,
    PhysicalWorkPreEffectDenial, PhysicalWorkPressureClass, PhysicalWorkProfileDeclaration,
    PhysicalWorkProfileDenial, PhysicalWorkPublicationResiduePosture, PhysicalWorkReadiness,
    PhysicalWorkRecoveryDisposition, PhysicalWorkRecoveryLocator, PhysicalWorkRecoveryTarget,
    PhysicalWorkRetryAdmission, PhysicalWorkRetryFailure, PhysicalWorkRetrySchedule,
    PhysicalWorkRetryScheduleOutcome, PhysicalWorkScheduler, PhysicalWorkSchedulerPosture,
    PhysicalWorkScope, PhysicalWorkSemanticBasis, PhysicalWorkSemanticBasisDenial,
    PhysicalWorkSemanticPosture, PhysicalWorkSettlementEvidence, PhysicalWorkShutdownObservation,
    PhysicalWorkSignalDeclaration, PhysicalWorkSignalFamily, PhysicalWorkSignalFamilySet,
    PhysicalWorkSubmissionDeferred, PhysicalWorkSubmissionDenial, PhysicalWorkSubmissionFailure,
    PhysicalWorkSubmissionOutcome, PhysicalWorkSubmissionReceipt, PhysicalWorkSubmissionStale,
    PhysicalWorkSupersessionJoin, PhysicalWorkTerminalCause, PhysicalWorkTerminalDisposition,
    PhysicalWorkTerminalFailure, PhysicalWorkTerminalObservation, PhysicalWorkTerminalStage,
    PhysicalWorkTimeoutJoin, ReadyPhysicalWork, ResourceAdmittedPhysicalWork, SettledPhysicalWork,
};

pub(in crate::physical_runtime) use work::{
    PhysicalEffectRecoveryObligation, PhysicalExecutorDispatch, PhysicalExecutorOutcome,
    PhysicalPublicationExecutorCommand, PhysicalReadExecutorCommand,
    PhysicalResidencyWritebackCompletion, PhysicalResidencyWritebackExecutorCommand,
    PhysicalRetryPayload, PhysicalRootPublicationWorkAction, PhysicalRootPublicationWorkScope,
    PhysicalWalBarrierExecutorCommand, PhysicalWalFrameCompletionBinding, PhysicalWorkSettlement,
    PhysicalWriteExecutorCommand,
};

pub(in crate::physical_runtime) use durability::{
    CompletedPhysicalMutationFact, PhysicalMutationAttempt, PhysicalMutationCancellationClass,
    PhysicalMutationObservationCounters, PhysicalMutationRuntimeOwner, PhysicalMutationStartPort,
    PhysicalMutationTerminalClass, PhysicalMutationTerminalFact,
};

#[cfg(feature = "certification-test-authority")]
pub mod certification {
    pub use super::certification_input::CertificationDurableMutationInput;
    pub use super::durability::{
        CertificationPhysicalMutationCheckpoint, CertificationPhysicalMutationPauseGate,
    };
    pub use super::instance::{
        CertificationPhysicalClosePauseGate, CertificationPhysicalExecutionCheckpoint,
        CertificationPhysicalExecutionPauseGate, CertificationPhysicalSignalPauseGate,
    };
    pub use super::media_evidence::{
        lower_media_operation_summary, MediaEvidenceLoweringDenial, MediaOperationSummary,
        StoreMediaPerformanceReceipt,
    };
    pub use super::record_serving::CertificationPhysicalRecordSubmission;
    pub use super::work::CertificationPhysicalSubmissionPauseGate;
    pub use worth_store_physical_backend::{
        CertificationMediaFaultAuthority, MediaFaultDirective, MediaFaultRule, MediaFaultSchedule,
        MediaFaultScheduleDenial, MediaPauseGate,
    };
}
