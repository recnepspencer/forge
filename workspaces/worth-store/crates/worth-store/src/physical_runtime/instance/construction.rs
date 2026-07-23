use worth_store_physical_backend::QualifiedFilesystemMedia;

use crate::physical_runtime::{
    record_serving::{
        RecordAllocationFrontier, RecordFramePorts, RecordServingOwner, RecordServingState,
        ServingHealth,
    },
    runtime::PhysicalRuntimeCore,
    work::{
        PhysicalWorkAdmissionAuthority, PhysicalWorkSubmissionFoundation,
        PhysicalWorkSubmissionOwner,
    },
};

use super::{
    PhysicalSchedulerAdmissionOwner, PhysicalSignalConstructionFailure, PhysicalStoreInstanceParts,
    PhysicalWorkExecutor, PhysicalWorkSignalOwner,
};

pub(in crate::physical_runtime) struct PhysicalStoreInstanceConstructionFailure {
    termination: crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
    media: QualifiedFilesystemMedia,
    core: PhysicalRuntimeCore,
    frame_ports: RecordFramePorts,
    cause: PhysicalSignalConstructionFailure,
}

impl PhysicalStoreInstanceParts {
    pub(in crate::physical_runtime) fn from_record_admission(
        termination: crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
        media: QualifiedFilesystemMedia,
        core: PhysicalRuntimeCore,
        bootstrap: RecordServingState,
        allocation_frontier: RecordAllocationFrontier,
        frame_ports: RecordFramePorts,
        work_profile: crate::physical_runtime::PhysicalWorkProfileDeclaration,
    ) -> Result<Self, PhysicalStoreInstanceConstructionFailure> {
        let runtime_identity = core.runtime_identity();
        let lifecycle_generation = core.lifecycle_generation();
        let store_identity = media.store_identity();
        let record_owner = RecordServingOwner::new();
        let health = ServingHealth::new(!bootstrap.publication_residue.is_empty());
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
        let work_submission =
            PhysicalWorkSubmissionOwner::new(PhysicalWorkSubmissionFoundation {
                store: store_identity,
                runtime: runtime_identity,
                generation: lifecycle_generation,
                lifecycle: core.lifecycle_state(),
                signal_profile: signal_owner.profile(),
                bindings: signal_owner.bindings(),
                signal_admission: signal_owner.admission_status(),
            });
        let scheduler_admission = PhysicalSchedulerAdmissionOwner::new();
        let work_admission = PhysicalWorkAdmissionAuthority::from_qualified_instance(
            &media,
            runtime_identity,
            lifecycle_generation,
        );
        let executor = PhysicalWorkExecutor::new(media);

        Ok(Self {
            termination,
            work_admission,
            work_submission,
            signal_owner,
            scheduler_admission,
            record_owner,
            executor,
            core,
            format: bootstrap.format,
            access: bootstrap.access,
            current_root: bootstrap.current_root,
            free_space: bootstrap.free_space,
            allocation_frontier,
            publication_residue: bootstrap.publication_residue,
            health,
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
