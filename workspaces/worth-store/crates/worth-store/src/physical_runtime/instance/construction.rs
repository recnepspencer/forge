use worth_store_physical_backend::QualifiedFilesystemMedia;

use crate::physical_runtime::{
    durability::{PhysicalWalAppendPort, PhysicalWalBarrierPort, PhysicalWalRuntimeOwner},
    record_serving::{
        CanonicalRecordMutationPort, RecordAllocationFrontier, RecordPublicationDirector,
        RecordPublicationFoundation, RecordServingOwner, RecordServingState, RecordWorkAdmission,
        ServingHealth,
    },
    runtime::PhysicalRuntimeCore,
    work::{
        PhysicalWorkAdmissionAuthority, PhysicalWorkSubmissionFoundation,
        PhysicalWorkSubmissionOwner,
    },
};

use super::{
    PhysicalResidencyOwner, PhysicalSchedulerAdmissionOwner, PhysicalSignalConstructionFailure,
    PhysicalStoreInstanceParts, PhysicalStoreWorkRuntime, PhysicalWorkExecutor,
    PhysicalWorkSignalOwner,
};

pub(in crate::physical_runtime) struct PhysicalStoreInstanceFoundation {
    pub(in crate::physical_runtime) termination:
        crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
    pub(in crate::physical_runtime) media: QualifiedFilesystemMedia,
    pub(in crate::physical_runtime) core: PhysicalRuntimeCore,
    pub(in crate::physical_runtime) bootstrap: RecordServingState,
    pub(in crate::physical_runtime) allocation_frontier: RecordAllocationFrontier,
    pub(in crate::physical_runtime) residency: PhysicalResidencyOwner,
    pub(in crate::physical_runtime) work_profile:
        crate::physical_runtime::PhysicalWorkProfileDeclaration,
    pub(in crate::physical_runtime) durability:
        crate::physical_runtime::durability::PhysicalDurabilityRuntimeOwner,
}

pub(in crate::physical_runtime) struct PhysicalStoreInstanceConstructionFailure {
    termination: crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
    media: QualifiedFilesystemMedia,
    core: PhysicalRuntimeCore,
    residency: PhysicalResidencyOwner,
    durability: crate::physical_runtime::durability::PhysicalDurabilityRuntimeOwner,
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
            residency,
            work_profile,
            durability,
        } = foundation;
        let frame_ports = residency.ports().clone();
        let runtime_identity = core.runtime_identity();
        let lifecycle_generation = core.lifecycle_generation();
        let store_identity = media.store_identity();
        let record_owner = RecordServingOwner::new();
        let durability_observation = durability.observation();
        let (record_work, work_profile) =
            match RecordWorkAdmission::install(work_profile, durability_observation) {
                Ok(installed) => installed,
                Err(denial) => {
                    return Err(PhysicalStoreInstanceConstructionFailure {
                        termination,
                        media,
                        core,
                        residency,
                        durability,
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
                        residency,
                        durability,
                        cause,
                    })
                }
            };
        let signal_profile = signal_owner.profile();
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
                    residency,
                    durability,
                    cause: PhysicalSignalConstructionFailure::SchedulerCapabilityRejected(denial),
                })
            }
        };
        let work_admission = PhysicalWorkAdmissionAuthority::from_qualified_instance(
            &media,
            runtime_identity,
            lifecycle_generation,
        );
        let wal_owner =
            match PhysicalWalRuntimeOwner::initialize(&media, runtime_identity, signal_profile) {
                Ok(owner) => owner,
                Err(failure) => {
                    return Err(PhysicalStoreInstanceConstructionFailure {
                        termination,
                        media,
                        core,
                        residency,
                        durability,
                        cause: PhysicalSignalConstructionFailure::WalArtifactInitializationRejected(
                            failure,
                        ),
                    })
                }
            };
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
        let wal = PhysicalWalAppendPort::new(
            &work_runtime,
            lifecycle_generation,
            work_admission,
            scheduler_admission.clone(),
            std::sync::Arc::clone(&record_work),
            wal_owner,
        );
        let wal_barrier = PhysicalWalBarrierPort::new(
            &work_runtime,
            lifecycle_generation,
            work_admission,
            scheduler_admission.clone(),
            std::sync::Arc::clone(&record_work),
            durability_observation,
        );
        let publication = RecordPublicationDirector::new(
            &work_runtime,
            planning_read,
            mutation,
            RecordPublicationFoundation {
                idempotency: durability.idempotency_authority(),
                durability: durability.observation(),
                signal_profile,
                security_basis: record_work
                    .security()
                    .receipt()
                    .identity()
                    .stable_fingerprint(),
                durability_policy_basis: record_work.durability_policy_basis(),
                wal,
                wal_barrier,
                format: bootstrap.format,
                access: bootstrap.access,
                current_root: bootstrap.current_root,
                free_space: bootstrap.free_space,
                allocation_frontier,
                residue: bootstrap.publication_residue,
                frame_ports: frame_ports.clone(),
                generation: lifecycle_generation,
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
            residency,
            durability,
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
        let _residency = self.residency.close();
        drop(self.termination);
        drop(self.durability);
        let release = self.media.close();
        let terminal =
            crate::physical_runtime::MediaShutdownOutcome::new(self.core.abort(), release);
        (identity, terminal, self.cause)
    }
}
