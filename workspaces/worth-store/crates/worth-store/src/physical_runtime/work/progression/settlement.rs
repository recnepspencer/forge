use worth_signal::facade::{ResourceAttemptId, ResourceRequestHandle};
use worth_store_physical_backend::{
    ArtifactTreePublicationEffect, BackendQueueExecutionPlanBinding, CompletedArtifactAppend,
    CompletedArtifactMetadataRead, CompletedArtifactNewWrite, CompletedArtifactRangeRead,
    CompletedArtifactRangeWrite, IndeterminateArtifactAppend, IndeterminateArtifactNewWrite,
    IndeterminateArtifactRangeWrite,
};
use worth_store_physical_format::RecordFrameCoordinate;

use super::{AdmittedPhysicalWork, PhysicalSignalReadinessEvidence};
use crate::physical_runtime::work::{
    PhysicalPublicationEffect, PhysicalWorkEffectClass, PhysicalWorkEffectFate,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition, PhysicalWorkSettlementEvidence,
    PhysicalWorkTerminalStage,
};

pub struct DispatchedPhysicalWork {
    pub(super) admitted: AdmittedPhysicalWork,
    pub(super) signal: PhysicalSignalReadinessEvidence,
    pub(super) effect_activity:
        Option<crate::physical_runtime::work::submission::PhysicalEffectActivity>,
    pub(super) scheduler_capacity: Option<
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundCapacityLease,
    >,
    pub(super) scheduler_binding: BackendQueueExecutionPlanBinding,
    pub(super) payload_digest: Option<[u8; 32]>,
}

pub struct SettledPhysicalWork {
    dispatched: DispatchedPhysicalWork,
    evidence: PhysicalWorkSettlementEvidence,
    recovery: PhysicalWorkRecoveryDisposition,
}

impl DispatchedPhysicalWork {
    pub(in crate::physical_runtime::work) fn take_effect_activity(
        &mut self,
    ) -> super::super::submission::PhysicalEffectActivity {
        self.effect_activity
            .take()
            .expect("dispatched work owns one active-effect enrollment")
    }

    fn release_scheduler_capacity(&mut self) {
        drop(self.scheduler_capacity.take());
    }

    pub const fn intent(&self) -> &crate::physical_runtime::work::PhysicalWorkIntent {
        self.admitted.intent()
    }

    pub const fn signal_request(&self) -> ResourceRequestHandle {
        self.signal.signal_request
    }

    pub const fn request_attempt(&self) -> ResourceAttemptId {
        self.signal.attempt
    }

    pub(in crate::physical_runtime) const fn signal_evidence(
        &self,
    ) -> &PhysicalSignalReadinessEvidence {
        &self.signal
    }

    pub const fn scheduler_binding(&self) -> BackendQueueExecutionPlanBinding {
        self.scheduler_binding
    }

    pub fn coordinate(&self) -> Option<RecordFrameCoordinate> {
        let [coordinate] = self.intent().scope().coordinates() else {
            return None;
        };
        Some(*coordinate)
    }

    pub(in crate::physical_runtime) fn matches_metadata(
        &self,
        physical: CompletedArtifactMetadataRead,
    ) -> bool {
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && self.intent().scope().artifact_target() == Some(physical.artifact())
            && self.intent().operation() == PhysicalWorkOperationFamily::ArtifactMetadataRead
    }

    pub(in crate::physical_runtime) fn matches_read(
        &self,
        physical: CompletedArtifactRangeRead,
    ) -> bool {
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && Some(physical.coordinate()) == self.coordinate()
            && physical.completed_bytes() <= u64::from(physical.coordinate().length())
            && self.intent().operation() == PhysicalWorkOperationFamily::ArtifactRangeRead
    }

    pub(in crate::physical_runtime) fn matches_write(
        &self,
        physical: &CompletedArtifactRangeWrite,
    ) -> bool {
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && Some(physical.coordinate()) == self.coordinate()
            && physical.completed_bytes() == u64::from(physical.coordinate().length())
            && self.payload_digest == Some(physical.payload_digest())
            && crate::physical_runtime::work::execution::settlement::durability_satisfies(
                self.intent().durability(),
                physical.durability(),
            )
    }

    pub(in crate::physical_runtime) fn matches_indeterminate(
        &self,
        physical: IndeterminateArtifactRangeWrite,
    ) -> bool {
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && Some(physical.coordinate()) == self.coordinate()
            && physical.completed_bytes() <= u64::from(physical.coordinate().length())
            && self.payload_digest == Some(physical.payload_digest())
            && self.intent().operation() != PhysicalWorkOperationFamily::ArtifactRangeRead
    }

    pub(in crate::physical_runtime) fn matches_new_artifact(
        &self,
        physical: &CompletedArtifactNewWrite,
    ) -> bool {
        self.matches_write(physical.write())
            && self.intent().operation() == PhysicalWorkOperationFamily::ArtifactPublication
            && physical.create_operation() != physical.write().operation()
    }

    pub(in crate::physical_runtime) fn matches_wal_append(
        &self,
        physical: &CompletedArtifactAppend,
    ) -> bool {
        let Some(scope) = self.intent().scope().wal_append_target() else {
            return false;
        };
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && physical.range().offset() == scope.offset()
            && physical.range().byte_count() == scope.byte_count()
            && self.payload_digest == Some(physical.payload_digest())
            && self.intent().operation() == PhysicalWorkOperationFamily::WalAppend
    }

    pub(in crate::physical_runtime) fn matches_wal_append_indeterminate(
        &self,
        physical: &IndeterminateArtifactAppend,
    ) -> bool {
        let Some(scope) = self.intent().scope().wal_append_target() else {
            return false;
        };
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && physical.range().offset() == scope.offset()
            && physical.range().byte_count() == scope.byte_count()
            && physical.completed_bytes() <= scope.byte_count()
            && self.payload_digest == Some(physical.payload_digest())
            && self.intent().operation() == PhysicalWorkOperationFamily::WalAppend
    }

    pub(in crate::physical_runtime) fn matches_wal_barrier(
        &self,
        physical: &crate::physical_runtime::work::CompletedPhysicalWalBarrier,
    ) -> bool {
        self.matches_wal_barrier_binding(
            physical.physical().store(),
            physical.physical().owner(),
            physical.artifact(),
            physical.physical().effect(),
        )
    }

    pub(in crate::physical_runtime) fn matches_wal_barrier_indeterminate(
        &self,
        physical: &crate::physical_runtime::work::IndeterminatePhysicalWalBarrier,
    ) -> bool {
        self.matches_wal_barrier_binding(
            physical.physical().store(),
            physical.physical().owner(),
            physical.artifact(),
            physical.physical().effect(),
        )
    }

    fn matches_wal_barrier_binding(
        &self,
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        owner: worth_store_physical_backend::MediaOwnerIdentity,
        artifact: &worth_store_physical_backend::ArtifactTreeFile,
        effect: &ArtifactTreePublicationEffect,
    ) -> bool {
        store == self.intent().identity().store()
            && owner == self.admitted.authority().media_owner_observation().owner()
            && self.intent().scope().wal_barrier_target().is_some()
            && self.intent().operation() == PhysicalWorkOperationFamily::DurabilityBarrier
            && matches!(
                effect,
                ArtifactTreePublicationEffect::FileSynchronization(observed)
                    if observed == artifact
            )
    }

    pub(in crate::physical_runtime) fn matches_new_artifact_indeterminate(
        &self,
        physical: IndeterminateArtifactNewWrite,
    ) -> bool {
        physical.store() == self.intent().identity().store()
            && physical.owner() == self.admitted.authority().media_owner_observation().owner()
            && Some(physical.coordinate()) == self.coordinate()
            && physical.completed_bytes() <= u64::from(physical.coordinate().length())
            && self.payload_digest == Some(physical.payload_digest())
            && self.intent().operation() == PhysicalWorkOperationFamily::ArtifactPublication
    }

    pub(in crate::physical_runtime) fn matches_publication_effect(
        &self,
        physical: &crate::physical_runtime::work::CompletedPhysicalPublicationEffect,
    ) -> bool {
        physical.physical().store() == self.intent().identity().store()
            && physical.physical().owner()
                == self.admitted.authority().media_owner_observation().owner()
            && self.intent().scope().artifact_target() == Some(physical.artifact())
            && self.intent().operation() == PhysicalWorkOperationFamily::ArtifactPublication
            && publication_effect_matches(physical.effect(), physical.physical().effect())
    }

    pub(in crate::physical_runtime) fn matches_publication_effect_indeterminate(
        &self,
        physical: &crate::physical_runtime::work::IndeterminatePhysicalPublicationEffect,
    ) -> bool {
        physical.physical().store() == self.intent().identity().store()
            && physical.physical().owner()
                == self.admitted.authority().media_owner_observation().owner()
            && self.intent().scope().artifact_target() == Some(physical.artifact())
            && self.intent().operation() == PhysicalWorkOperationFamily::ArtifactPublication
            && publication_effect_matches(physical.effect(), physical.physical().effect())
    }
}

fn publication_effect_matches(
    declared: PhysicalPublicationEffect,
    observed: &ArtifactTreePublicationEffect,
) -> bool {
    matches!(
        (declared, observed),
        (
            PhysicalPublicationEffect::SynchronizeArtifact,
            ArtifactTreePublicationEffect::FileSynchronization(_)
        ) | (
            PhysicalPublicationEffect::SynchronizeArtifactParent
                | PhysicalPublicationEffect::SynchronizeRecordFamily,
            ArtifactTreePublicationEffect::DirectorySynchronization(_)
        ) | (
            PhysicalPublicationEffect::ReplaceCatalog,
            ArtifactTreePublicationEffect::Replacement { .. }
        )
    )
}

impl SettledPhysicalWork {
    pub const fn intent(&self) -> &crate::physical_runtime::work::PhysicalWorkIntent {
        self.dispatched.intent()
    }

    pub const fn evidence(&self) -> &PhysicalWorkSettlementEvidence {
        &self.evidence
    }

    pub(in crate::physical_runtime) fn into_evidence(self) -> PhysicalWorkSettlementEvidence {
        self.evidence
    }

    pub fn effect_identity(&self) -> Option<crate::physical_runtime::PhysicalEffectIdentity> {
        let backend = match &self.evidence {
            PhysicalWorkSettlementEvidence::Metadata { physical, .. } => physical.operation(),
            PhysicalWorkSettlementEvidence::Read { physical, .. } => physical.operation(),
            PhysicalWorkSettlementEvidence::Write { physical, .. }
            | PhysicalWorkSettlementEvidence::Publication { physical, .. } => physical.operation(),
            PhysicalWorkSettlementEvidence::NewArtifact { physical, .. } => {
                physical.write().operation()
            }
            PhysicalWorkSettlementEvidence::PublicationEffect { physical, .. } => {
                physical.physical().operation()
            }
            PhysicalWorkSettlementEvidence::WalAppend { physical, .. } => physical.operation(),
            PhysicalWorkSettlementEvidence::WalBarrier { physical, .. } => {
                physical.physical().operation()
            }
            PhysicalWorkSettlementEvidence::TerminalFailure(failure) => failure.backend_operation(),
            PhysicalWorkSettlementEvidence::NoEffect(_)
            | PhysicalWorkSettlementEvidence::StaleOrForeign => return None,
        };
        Some(crate::physical_runtime::PhysicalEffectIdentity::new(
            self.intent().identity(),
            backend,
        ))
    }

    pub const fn signal_request(&self) -> ResourceRequestHandle {
        self.dispatched.signal_request()
    }

    pub const fn request_attempt(&self) -> ResourceAttemptId {
        self.dispatched.request_attempt()
    }

    pub const fn scheduler_binding(&self) -> BackendQueueExecutionPlanBinding {
        self.dispatched.scheduler_binding()
    }

    pub const fn recovery_disposition(&self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }

    pub(in crate::physical_runtime) const fn signal_binding(
        &self,
    ) -> crate::physical_runtime::work::PhysicalSignalAspectBindingDigest {
        self.dispatched.admitted.authority().binding()
    }

    pub(in crate::physical_runtime) const fn signal_family(
        &self,
    ) -> crate::physical_runtime::work::PhysicalWorkSignalFamily {
        self.dispatched.admitted.authority().signal_family()
    }

    pub(in crate::physical_runtime) fn retry_is_physically_safe(&self) -> bool {
        if self.evidence.fate() != PhysicalWorkEffectFate::ProvenNoEffect
            || self.recovery != self.intent().recovery()
        {
            return false;
        }
        matches!(
            (self.intent().effect(), self.intent().recovery()),
            (
                PhysicalWorkEffectClass::ReadOnly,
                PhysicalWorkRecoveryDisposition::NoEffect,
            ) | (
                PhysicalWorkEffectClass::IdempotentExactWrite,
                PhysicalWorkRecoveryDisposition::RetryExact,
            )
        )
    }

    pub(in crate::physical_runtime) const fn signal_evidence(
        &self,
    ) -> &PhysicalSignalReadinessEvidence {
        self.dispatched.signal_evidence()
    }

    pub(in crate::physical_runtime) fn from_settlement(
        mut dispatched: DispatchedPhysicalWork,
        evidence: PhysicalWorkSettlementEvidence,
        recovery_obligation: crate::physical_runtime::PhysicalEffectRecoveryObligation,
    ) -> Self {
        dispatched.release_scheduler_capacity();
        dispatched
            .admitted
            .mark_stage(PhysicalWorkTerminalStage::Settling);
        let recovery = if recovery_obligation.is_retained() {
            PhysicalWorkRecoveryDisposition::InspectionRequired
        } else {
            evidence.recovery_disposition(dispatched.intent().recovery())
        };
        if !recovery_obligation.is_retained()
            && retry_is_physically_safe(dispatched.intent(), &evidence)
        {
            // A proven-no-effect attempt remains the same logical Store command.
            // Keep its capacity and command identity alive until retry admission
            // either re-enters Ready or the packet is dropped safely.
            dispatched.admitted.mark_retry_pending();
        } else {
            dispatched
                .admitted
                .release_settled(evidence.fate(), recovery);
        }
        Self {
            dispatched,
            evidence,
            recovery,
        }
    }

    pub(in crate::physical_runtime) fn into_retry_parts(
        self,
        admitted: worth_signal::facade::AdmittedResourceRetry,
    ) -> Option<(
        super::ReadyPhysicalWork,
        crate::physical_runtime::work::PhysicalRetryCommand,
    )> {
        if !self.retry_is_physically_safe() {
            return None;
        }
        let identity = self.intent().identity();
        let signal = self.dispatched.signal.for_retry(admitted);
        let retry = match self.evidence {
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => evidence.retry,
            _ => return None,
        };
        Some((
            super::ReadyPhysicalWork::new(self.dispatched.admitted, signal),
            crate::physical_runtime::work::PhysicalRetryCommand::new(identity, retry),
        ))
    }
}

fn retry_is_physically_safe(
    intent: &crate::physical_runtime::work::PhysicalWorkIntent,
    evidence: &PhysicalWorkSettlementEvidence,
) -> bool {
    if evidence.fate() != PhysicalWorkEffectFate::ProvenNoEffect {
        return false;
    }
    matches!(
        (intent.effect(), intent.recovery()),
        (
            PhysicalWorkEffectClass::ReadOnly,
            PhysicalWorkRecoveryDisposition::NoEffect,
        ) | (
            PhysicalWorkEffectClass::IdempotentExactWrite,
            PhysicalWorkRecoveryDisposition::RetryExact,
        )
    )
}
