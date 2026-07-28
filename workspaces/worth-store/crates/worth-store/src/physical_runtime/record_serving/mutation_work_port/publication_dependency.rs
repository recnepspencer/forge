use std::sync::{Arc, Weak};

use worth_proof::TransitionOutcome;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::RecordArtifactFile;

use crate::physical_runtime::{
    instance::PhysicalStoreWorkRuntime, PhysicalExecutorCommand, PhysicalMutationWorkRequest,
    PhysicalPublicationEffect, PhysicalSchedulerDemand, PhysicalWorkAdmission,
    PhysicalWorkExecution, PhysicalWorkIdentity, PhysicalWorkScope,
};

use super::{
    CanonicalRecordMutationFailure, CanonicalRecordMutationKind, CanonicalRecordMutationPort,
    PreparedCanonicalRecordMutation,
};

pub(in crate::physical_runtime::record_serving) struct PreparedCatalogReplacement {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    execution: PhysicalWorkExecution,
    blocked: crate::physical_runtime::BlockedPhysicalWork,
    scheduler: crate::physical_runtime::instance::PhysicalSchedulerAdmissionOwner,
    record: Arc<super::super::RecordWorkAdmission>,
    identity: PhysicalWorkIdentity,
    candidate: RecordArtifactFile,
}

impl CanonicalRecordMutationPort {
    pub(in crate::physical_runtime::record_serving) fn prepare_catalog_replacement_dependency(
        &self,
        candidate: RecordArtifactFile,
    ) -> Result<PreparedCatalogReplacement, CanonicalRecordMutationFailure> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or_else(CanonicalRecordMutationFailure::runtime_released)?;
        let request = PhysicalMutationWorkRequest::publication(
            PhysicalWorkScope::artifact(candidate),
            self.record
                .mutation_basis(super::super::RecordPublicationStage::CatalogReplacement),
            self.record.security(),
            ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization,
        )
        .map_err(|_| CanonicalRecordMutationFailure::submission_rejected())?;
        let receipt = match self.submission.submit(request).into_raw() {
            TransitionOutcome::Success(receipt) => receipt,
            _ => return Err(CanonicalRecordMutationFailure::submission_rejected()),
        };
        let identity = receipt.identity();
        let admitted = PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.physical,
            &runtime.health,
        )
        .map_err(|failure| CanonicalRecordMutationFailure::pre_effect(identity, failure))?;
        let blocked = runtime
            .signal
            .begin_publication_dependency(admitted)
            .map_err(|failure| CanonicalRecordMutationFailure::pre_effect(identity, failure))?;
        Ok(PreparedCatalogReplacement {
            runtime: self.runtime.clone(),
            execution: self.execution.clone(),
            blocked,
            scheduler: self.scheduler.clone(),
            record: self.record.clone(),
            identity,
            candidate,
        })
    }
}

impl PreparedCatalogReplacement {
    pub(in crate::physical_runtime::record_serving) fn execute(
        self,
        eligibility: super::super::publication::CatalogReplacementEligibility,
    ) -> Result<super::CanonicalRecordMutationSettlement, CanonicalRecordMutationFailure> {
        let Self {
            runtime,
            execution,
            blocked,
            scheduler,
            record,
            identity,
            candidate,
        } = self;
        if !eligibility.matches(candidate) {
            return Err(
                CanonicalRecordMutationFailure::catalog_replacement_eligibility_mismatch(identity),
            );
        }
        let runtime = runtime
            .upgrade()
            .ok_or_else(CanonicalRecordMutationFailure::runtime_released)?;
        let ready = runtime
            .signal
            .advance_publication_dependency(blocked)
            .map_err(|failure| CanonicalRecordMutationFailure::pre_effect(identity, failure))?;
        let (reservation, backend) = scheduler
            .record_publication_effect(record.scheduler_security())
            .map_err(|failure| {
                CanonicalRecordMutationFailure::scheduler_reservation(identity, failure)
            })?;
        let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
            .map_err(|failure| CanonicalRecordMutationFailure::scheduler(identity, failure))?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            demand.intent(),
            &runtime.health,
        )
        .map_err(|failure| CanonicalRecordMutationFailure::pre_effect(identity, failure))?;
        let policy =
            super::super::record_queue_policy::admit_record_queue_policy(demand.queue_work());
        let work = crate::physical_runtime::PhysicalWorkScheduler::admit(demand, &backend, policy)
            .map_err(|failure| CanonicalRecordMutationFailure::scheduler(identity, failure))?;
        let command = PhysicalExecutorCommand::publication_effect(
            work,
            PhysicalPublicationEffect::ReplaceCatalog,
        )
        .map_err(|failure| CanonicalRecordMutationFailure::command(identity, failure))?;
        let completion = PreparedCanonicalRecordMutation {
            execution,
            command,
            identity,
            expected: CanonicalRecordMutationKind::PublicationEffect,
            target: crate::physical_runtime::PhysicalWorkRecoveryTarget::CatalogReplacement(
                candidate,
            ),
        }
        .execute()?;
        let settlement = completion.settlement();
        match completion {
            super::CanonicalRecordMutationCompletion::PublicationEffect(settlement) => {
                Ok(settlement)
            }
            super::CanonicalRecordMutationCompletion::CandidateFrame(_) => Err(
                CanonicalRecordMutationFailure::settlement_mismatch(settlement),
            ),
        }
    }
}
