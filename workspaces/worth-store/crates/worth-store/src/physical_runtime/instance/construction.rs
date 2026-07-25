use worth_store_physical_backend::QualifiedFilesystemMedia;

use crate::physical_runtime::{
    record_serving::{
        CanonicalRecordMutationPort, RecordAllocationFrontier, RecordFramePorts,
        RecordPublicationDirector, RecordPublicationFoundation, RecordServingOwner,
        RecordServingState, RecordWorkAdmission, ServingHealth,
    },
    runtime::PhysicalRuntimeCore,
    work::{
        PhysicalWorkAdmissionAuthority, PhysicalWorkSubmissionFoundation,
        PhysicalWorkSubmissionOwner,
    },
};

use super::{
    PhysicalSchedulerAdmissionOwner, PhysicalSignalConstructionFailure, PhysicalStoreInstanceParts,
    PhysicalStoreWorkRuntime, PhysicalWorkExecutor, PhysicalWorkSignalOwner,
};

pub(in crate::physical_runtime) struct PhysicalStoreInstanceFoundation {
    pub(in crate::physical_runtime) termination:
        crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
    pub(in crate::physical_runtime) media: QualifiedFilesystemMedia,
    pub(in crate::physical_runtime) core: PhysicalRuntimeCore,
    pub(in crate::physical_runtime) bootstrap: RecordServingState,
    pub(in crate::physical_runtime) allocation_frontier: RecordAllocationFrontier,
    pub(in crate::physical_runtime) frame_ports: RecordFramePorts,
    pub(in crate::physical_runtime) work_profile:
        crate::physical_runtime::PhysicalWorkProfileDeclaration,
}

pub(in crate::physical_runtime) struct PhysicalStoreInstanceConstructionFailure {
    termination: crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
    media: QualifiedFilesystemMedia,
    core: PhysicalRuntimeCore,
    frame_ports: RecordFramePorts,
    cause: PhysicalSignalConstructionFailure,
}

impl PhysicalStoreInstanceParts {
    pub(in crate::physical_runtime) fn from_record_admission(
        foundation: PhysicalStoreInstanceFoundation,
    ) -> Result<Self, PhysicalStoreInstanceConstructionFailure> {
        let PhysicalStoreInstanceFoundation {
            termination,
            media,
            core,
            bootstrap,
            allocation_frontier,
            frame_ports,
            work_profile,
        } = foundation;
        let runtime_identity = core.runtime_identity();
        let lifecycle_generation = core.lifecycle_generation();
        let store_identity = media.store_identity();
        let record_owner = RecordServingOwner::new();
        let (record_work, work_profile) = match RecordWorkAdmission::install(work_profile) {
            Ok(installed) => installed,
            Err(denial) => {
                return Err(PhysicalStoreInstanceConstructionFailure {
                    termination,
                    media,
                    core,
                    frame_ports,
                    cause: PhysicalSignalConstructionFailure::ProfileRejected(denial),
                })
            }
        };
        let record_work = std::sync::Arc::new(record_work);
        let work_capacity = work_profile.capacity();
        let recovery = PhysicalWorkExecutor::inspect_recovery(&media, work_capacity.commands());
        let health = ServingHealth::new(
            !bootstrap.publication_residue.is_empty() || recovery.requires_inspection(),
        );
        let signal_owner =
            match PhysicalWorkSignalOwner::build_foundation(lifecycle_generation, work_profile) {
                Ok(owner) => owner,
                Err(cause) => {
                    return Err(PhysicalStoreInstanceConstructionFailure {
                        termination,
                        media,
                        core,
                        frame_ports,
                        cause,
                    })
                }
            };
        let work_submission = PhysicalWorkSubmissionOwner::new(PhysicalWorkSubmissionFoundation {
            store: store_identity,
            runtime: runtime_identity,
            generation: lifecycle_generation,
            lifecycle: core.lifecycle_state(),
            signal_profile: signal_owner.profile(),
            bindings: signal_owner.bindings(),
            signal_admission: signal_owner.admission_status(),
            abandonment: signal_owner.abandonment_publisher(),
        });
        let scheduler_admission = match PhysicalSchedulerAdmissionOwner::new(&media, work_capacity)
        {
            Ok(owner) => owner,
            Err(denial) => {
                return Err(PhysicalStoreInstanceConstructionFailure {
                    termination,
                    media,
                    core,
                    frame_ports,
                    cause: PhysicalSignalConstructionFailure::SchedulerCapabilityRejected(denial),
                })
            }
        };
        let work_admission = PhysicalWorkAdmissionAuthority::from_qualified_instance(
            &media,
            runtime_identity,
            lifecycle_generation,
        );
        let executor = PhysicalWorkExecutor::new(media);
        let work_runtime = PhysicalStoreWorkRuntime::new(
            work_submission,
            signal_owner,
            executor,
            health,
            recovery,
        );
        let planning_read = crate::physical_runtime::record_serving::CanonicalRecordReadPort::new(
            &work_runtime,
            lifecycle_generation,
            work_admission,
            scheduler_admission.clone(),
            std::sync::Arc::clone(&record_work),
        );
        let mutation = CanonicalRecordMutationPort::new(
            &work_runtime,
            lifecycle_generation,
            work_admission,
            scheduler_admission.clone(),
            std::sync::Arc::clone(&record_work),
        );
        let publication = RecordPublicationDirector::new(
            &work_runtime,
            planning_read,
            mutation,
            RecordPublicationFoundation {
                format: bootstrap.format,
                access: bootstrap.access,
                current_root: bootstrap.current_root,
                free_space: bootstrap.free_space,
                allocation_frontier,
                residue: bootstrap.publication_residue,
                frame_ports: frame_ports.clone(),
            },
        );

        Ok(Self {
            termination,
            work_admission,
            work_runtime,
            scheduler_admission,
            record_owner,
            record_work,
            core,
            format: bootstrap.format,
            access: bootstrap.access,
            publication,
            frame_ports,
        })
    }
}

impl PhysicalStoreInstanceConstructionFailure {
    pub(in crate::physical_runtime) fn abort(
        self,
    ) -> (
        crate::physical_runtime::RuntimeIdentity,
        crate::physical_runtime::MediaShutdownOutcome<crate::physical_runtime::AbortedRuntime>,
        PhysicalSignalConstructionFailure,
    ) {
        let identity = self.core.runtime_identity();
        let _residency = self.frame_ports.close();
        drop(self.termination);
        let release = self.media.close();
        let terminal =
            crate::physical_runtime::MediaShutdownOutcome::new(self.core.abort(), release);
        (identity, terminal, self.cause)
    }
}
