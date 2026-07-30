use std::sync::{Arc, Weak};

use crate::physical_runtime::{
    instance::{
        PhysicalSchedulerAdmissionOwner, PhysicalStoreWorkRuntime, RecordSchedulerReservationDenial,
    },
    work::PhysicalWorkAdmissionAuthority,
    PhysicalExecutorCommand, PhysicalExecutorCommandDenial, PhysicalMutationSubmission,
    PhysicalSchedulerDenial, PhysicalWorkExecution, PhysicalWorkIdentity,
    PhysicalWorkPreEffectDenial, PhysicalWorkTerminalFailure,
};

use super::{
    residency::{candidate_frame_residency::CandidateFramePhysicalWrite, FrameWritebackPort},
    RecordFramePorts, RecordWorkAdmission,
};

mod admission;
mod publication_dependency;
mod settlement;
mod settlement_fact;

pub(in crate::physical_runtime::record_serving) use publication_dependency::PreparedCatalogReplacement;
pub(in crate::physical_runtime) use settlement::CanonicalRecordMutationCompletion;
pub(in crate::physical_runtime) use settlement_fact::CanonicalRecordMutationSettlement;

#[derive(Clone)]
pub(in crate::physical_runtime) struct CanonicalRecordMutationPort {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    execution: PhysicalWorkExecution,
    submission: PhysicalMutationSubmission,
    physical: PhysicalWorkAdmissionAuthority,
    scheduler: PhysicalSchedulerAdmissionOwner,
    record: Arc<RecordWorkAdmission>,
}

pub(in crate::physical_runtime) struct PreparedCanonicalRecordMutation {
    execution: PhysicalWorkExecution,
    command: PhysicalExecutorCommand,
    identity: PhysicalWorkIdentity,
    expected: CanonicalRecordMutationKind,
    target: crate::physical_runtime::PhysicalWorkRecoveryTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CanonicalRecordMutationKind {
    NewArtifact,
    PublicationEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum CanonicalRecordPublicationEffect {
    Artifact,
    ArtifactParent,
    RecordFamily,
}

pub(in crate::physical_runtime) struct CanonicalCandidateFrameWrite {
    physical: CandidateFramePhysicalWrite,
}

pub(in crate::physical_runtime) struct CanonicalRecordMutationFailure {
    evidence: PhysicalRecordMutationFailureEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecordMutationFailureCause {
    RuntimeReleased,
    InvalidCoordinate,
    SubmissionRejected,
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked,
    CatalogReplacementEligibilityMismatch,
    SchedulerReservationDenied(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
    Scheduler(PhysicalSchedulerDenial),
    Command(PhysicalExecutorCommandDenial),
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Terminal(crate::physical_runtime::PhysicalWorkTerminalCause),
    SettlementMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordMutationFailureEvidence {
    identity: Option<PhysicalWorkIdentity>,
    cause: PhysicalRecordMutationFailureCause,
    effect_fate: crate::physical_runtime::PhysicalWorkEffectFate,
    recovery_target: Option<crate::physical_runtime::PhysicalWorkRecoveryTarget>,
    recovery: Option<crate::physical_runtime::PhysicalWorkRecoveryDisposition>,
    backend_operation: Option<worth_store_physical_backend::MediaOperationIdentity>,
}

impl CanonicalRecordMutationPort {
    pub(in crate::physical_runtime) fn new(
        runtime: &Arc<PhysicalStoreWorkRuntime>,
        generation: crate::physical_runtime::LifecycleGeneration,
        physical: PhysicalWorkAdmissionAuthority,
        scheduler: PhysicalSchedulerAdmissionOwner,
        record: Arc<RecordWorkAdmission>,
    ) -> Self {
        Self {
            runtime: Arc::downgrade(runtime),
            execution: PhysicalStoreWorkRuntime::execution(runtime, generation),
            submission: runtime.submission.mutation_submission(),
            physical,
            scheduler,
            record,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn frame_writeback_port(
        &self,
        frame_ports: RecordFramePorts,
    ) -> FrameWritebackPort {
        FrameWritebackPort::new(
            self.runtime.clone(),
            self.execution.clone(),
            self.submission.clone(),
            self.physical,
            self.scheduler.clone(),
            Arc::clone(&self.record),
            frame_ports,
        )
    }
}

impl CanonicalCandidateFrameWrite {
    pub(in crate::physical_runtime::record_serving) fn into_physical(
        self,
    ) -> CandidateFramePhysicalWrite {
        self.physical
    }
}

impl CanonicalRecordMutationFailure {
    fn unidentified(cause: PhysicalRecordMutationFailureCause) -> Self {
        Self {
            evidence: PhysicalRecordMutationFailureEvidence {
                identity: None,
                cause,
                effect_fate: crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect,
                recovery_target: None,
                recovery: None,
                backend_operation: None,
            },
        }
    }

    fn identified(
        identity: PhysicalWorkIdentity,
        cause: PhysicalRecordMutationFailureCause,
    ) -> Self {
        Self {
            evidence: PhysicalRecordMutationFailureEvidence {
                identity: Some(identity),
                cause,
                effect_fate: crate::physical_runtime::PhysicalWorkEffectFate::ProvenNoEffect,
                recovery_target: None,
                recovery: None,
                backend_operation: None,
            },
        }
    }

    pub(in crate::physical_runtime) fn runtime_released() -> Self {
        Self::unidentified(PhysicalRecordMutationFailureCause::RuntimeReleased)
    }

    pub(in crate::physical_runtime) fn submission_rejected() -> Self {
        Self::unidentified(PhysicalRecordMutationFailureCause::SubmissionRejected)
    }

    pub(in crate::physical_runtime) fn pre_effect(
        identity: PhysicalWorkIdentity,
        failure: PhysicalWorkPreEffectDenial,
    ) -> Self {
        Self::identified(
            identity,
            PhysicalRecordMutationFailureCause::PreEffect(failure),
        )
    }

    pub(in crate::physical_runtime) fn dependency_blocked(identity: PhysicalWorkIdentity) -> Self {
        Self::identified(
            identity,
            PhysicalRecordMutationFailureCause::DependencyBlocked,
        )
    }

    pub(in crate::physical_runtime) fn catalog_replacement_eligibility_mismatch(
        identity: PhysicalWorkIdentity,
    ) -> Self {
        Self::identified(
            identity,
            PhysicalRecordMutationFailureCause::CatalogReplacementEligibilityMismatch,
        )
    }

    pub(in crate::physical_runtime) fn scheduler_reservation(
        identity: PhysicalWorkIdentity,
        failure: RecordSchedulerReservationDenial,
    ) -> Self {
        let RecordSchedulerReservationDenial::Admission(denial) = failure;
        Self::identified(
            identity,
            PhysicalRecordMutationFailureCause::SchedulerReservationDenied(denial),
        )
    }

    pub(in crate::physical_runtime) fn scheduler(
        identity: PhysicalWorkIdentity,
        failure: PhysicalSchedulerDenial,
    ) -> Self {
        Self::identified(
            identity,
            PhysicalRecordMutationFailureCause::Scheduler(failure),
        )
    }

    pub(in crate::physical_runtime) fn command(
        identity: PhysicalWorkIdentity,
        failure: PhysicalExecutorCommandDenial,
    ) -> Self {
        Self::identified(
            identity,
            PhysicalRecordMutationFailureCause::Command(failure),
        )
    }

    pub(in crate::physical_runtime) fn backend(
        settlement: CanonicalRecordMutationSettlement,
        target: crate::physical_runtime::PhysicalWorkRecoveryTarget,
        failure: worth_store_physical_backend::ArtifactTreeFailure,
    ) -> Self {
        Self {
            evidence: PhysicalRecordMutationFailureEvidence {
                identity: Some(settlement.identity()),
                cause: PhysicalRecordMutationFailureCause::Backend(failure),
                effect_fate: settlement.effect_fate(),
                recovery_target: Some(target),
                recovery: Some(settlement.recovery()),
                backend_operation: settlement
                    .effect()
                    .map(crate::physical_runtime::PhysicalEffectIdentity::backend_operation),
            },
        }
    }

    pub(in crate::physical_runtime) fn terminal(
        settlement: CanonicalRecordMutationSettlement,
        failure: PhysicalWorkTerminalFailure,
    ) -> Self {
        debug_assert_eq!(settlement.identity(), failure.identity());
        Self {
            evidence: PhysicalRecordMutationFailureEvidence {
                identity: Some(settlement.identity()),
                cause: PhysicalRecordMutationFailureCause::Terminal(failure.cause()),
                effect_fate: settlement.effect_fate(),
                recovery_target: Some(failure.target()),
                recovery: Some(settlement.recovery()),
                backend_operation: settlement
                    .effect()
                    .map(crate::physical_runtime::PhysicalEffectIdentity::backend_operation),
            },
        }
    }

    pub(in crate::physical_runtime) fn settlement_mismatch(
        settlement: CanonicalRecordMutationSettlement,
    ) -> Self {
        Self {
            evidence: PhysicalRecordMutationFailureEvidence {
                identity: Some(settlement.identity()),
                cause: PhysicalRecordMutationFailureCause::SettlementMismatch,
                effect_fate: crate::physical_runtime::PhysicalWorkEffectFate::StaleOrForeignOutcome,
                recovery_target: None,
                recovery: Some(
                    crate::physical_runtime::PhysicalWorkRecoveryDisposition::InspectionRequired,
                ),
                backend_operation: settlement
                    .effect()
                    .map(crate::physical_runtime::PhysicalEffectIdentity::backend_operation),
            },
        }
    }

    pub(in crate::physical_runtime::record_serving) fn evidence(
        &self,
    ) -> PhysicalRecordMutationFailureEvidence {
        self.evidence
    }

    pub(in crate::physical_runtime::record_serving) const fn effect_fate(
        &self,
    ) -> crate::physical_runtime::PhysicalWorkEffectFate {
        self.evidence.effect_fate
    }
}

impl PhysicalRecordMutationFailureEvidence {
    pub const fn identity(self) -> Option<PhysicalWorkIdentity> {
        self.identity
    }

    pub const fn cause(self) -> PhysicalRecordMutationFailureCause {
        self.cause
    }

    pub const fn effect_fate(self) -> crate::physical_runtime::PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn recovery_target(
        self,
    ) -> Option<crate::physical_runtime::PhysicalWorkRecoveryTarget> {
        self.recovery_target
    }

    pub const fn recovery(
        self,
    ) -> Option<crate::physical_runtime::PhysicalWorkRecoveryDisposition> {
        self.recovery
    }

    pub const fn backend_operation(
        self,
    ) -> Option<worth_store_physical_backend::MediaOperationIdentity> {
        self.backend_operation
    }
}
