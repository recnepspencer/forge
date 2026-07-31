#[cfg(feature = "certification-test-authority")]
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Condvar, Mutex, Weak};

use worth_store_physical_format::{DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest};

use super::append::{RecordAppendDenial, RecordAppendError};
use crate::physical_runtime::instance::PhysicalStoreWorkRuntime;

use super::super::{
    residency::{frame_loading::CanonicalFrameReadSource, PhysicalResidencyWorkPort},
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, CanonicalRecordMutationPort,
    CanonicalRecordReadPort, RecordAllocationFrontier, RecordFramePorts,
    RecordPublicationResidueObservation,
};

mod durable_data;
mod durable_preparation;
mod execution;
mod lifecycle;
mod submission;

pub use submission::{PhysicalRecordSubmission, PreparedRecordAppend};

pub(in crate::physical_runtime) struct RecordPublicationDirector {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    mutation_identity: crate::physical_runtime::PhysicalMutationSubmission,
    idempotency: crate::physical_runtime::durability::PhysicalMutationIdempotencyRuntimeAuthority,
    durability: crate::physical_runtime::PhysicalDurabilityObservation,
    signal_profile: crate::physical_runtime::PhysicalSignalProfileIdentity,
    security_basis: [u8; 32],
    durability_policy_basis: crate::physical_runtime::PhysicalWorkSemanticBasis,
    wal: crate::physical_runtime::durability::PhysicalWalAppendPort,
    wal_barrier: crate::physical_runtime::durability::PhysicalWalBarrierPort,
    residency: PhysicalResidencyWorkPort,
    mutation: CanonicalRecordMutationPort,
    generation: crate::physical_runtime::LifecycleGeneration,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    state: Mutex<RecordPublicationState>,
    preparation: Mutex<RecordPreparationState>,
    gate: Mutex<RecordPublicationGate>,
    changed: Condvar,
    #[cfg(feature = "certification-test-authority")]
    reject_catalog_eligibility_join: AtomicBool,
}

pub(in crate::physical_runtime) struct RecordPublicationTerminalState {
    pub(in crate::physical_runtime) residue: RecordPublicationResidueObservation,
}

pub(in crate::physical_runtime) struct RecordPublicationFoundation {
    pub(in crate::physical_runtime) idempotency:
        crate::physical_runtime::durability::PhysicalMutationIdempotencyRuntimeAuthority,
    pub(in crate::physical_runtime) durability:
        crate::physical_runtime::PhysicalDurabilityObservation,
    pub(in crate::physical_runtime) signal_profile:
        crate::physical_runtime::PhysicalSignalProfileIdentity,
    pub(in crate::physical_runtime) security_basis: [u8; 32],
    pub(in crate::physical_runtime) durability_policy_basis:
        crate::physical_runtime::PhysicalWorkSemanticBasis,
    pub(in crate::physical_runtime) wal: crate::physical_runtime::durability::PhysicalWalAppendPort,
    pub(in crate::physical_runtime) wal_barrier:
        crate::physical_runtime::durability::PhysicalWalBarrierPort,
    pub(in crate::physical_runtime) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime) current_root: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime) free_space: DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime) allocation_frontier: RecordAllocationFrontier,
    pub(in crate::physical_runtime) residue: RecordPublicationResidueObservation,
    pub(in crate::physical_runtime) frame_ports: RecordFramePorts,
    pub(in crate::physical_runtime) generation: crate::physical_runtime::LifecycleGeneration,
}

struct RecordPublicationState {
    current_root: DurablePhysicalRootManifest,
    free_space: DurableFreeSpaceManifestHeader,
    residue: RecordPublicationResidueObservation,
}

struct RecordPreparationState {
    allocation_frontier: RecordAllocationFrontier,
    published_tail_reserved: bool,
}

struct RecordPublicationGate {
    accepting: bool,
    active: usize,
}

struct RecordPublicationCall {
    director: Arc<RecordPublicationDirector>,
}

impl RecordPublicationDirector {
    pub(in crate::physical_runtime) fn new(
        runtime: &Arc<PhysicalStoreWorkRuntime>,
        planning_read: CanonicalRecordReadPort,
        mutation: CanonicalRecordMutationPort,
        foundation: RecordPublicationFoundation,
    ) -> Arc<Self> {
        let writeback = mutation.frame_writeback_port(foundation.frame_ports.clone());
        Arc::new(Self {
            runtime: Arc::downgrade(runtime),
            mutation_identity: runtime.submission.mutation_submission(),
            idempotency: foundation.idempotency,
            durability: foundation.durability,
            signal_profile: foundation.signal_profile,
            security_basis: foundation.security_basis,
            durability_policy_basis: foundation.durability_policy_basis,
            wal: foundation.wal,
            wal_barrier: foundation.wal_barrier,
            residency: PhysicalResidencyWorkPort::new(
                foundation.frame_ports,
                CanonicalFrameReadSource::new(planning_read),
                writeback,
            ),
            mutation,
            generation: foundation.generation,
            format: foundation.format,
            access: foundation.access,
            state: Mutex::new(RecordPublicationState {
                current_root: foundation.current_root,
                free_space: foundation.free_space,
                residue: foundation.residue,
            }),
            preparation: Mutex::new(RecordPreparationState {
                allocation_frontier: foundation.allocation_frontier,
                published_tail_reserved: false,
            }),
            gate: Mutex::new(RecordPublicationGate {
                accepting: true,
                active: 0,
            }),
            changed: Condvar::new(),
            #[cfg(feature = "certification-test-authority")]
            reject_catalog_eligibility_join: AtomicBool::new(false),
        })
    }

    pub(in crate::physical_runtime) fn submission(
        director: &Arc<Self>,
    ) -> PhysicalRecordSubmission {
        PhysicalRecordSubmission::new(Arc::downgrade(director))
    }

    fn project_pressure(&self, error: RecordAppendError) -> RecordAppendError {
        let RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(denial)) = error
        else {
            return error;
        };
        match super::super::PhysicalRecordPressureEvidence::from_store_failure(
            denial,
            self.generation,
        ) {
            Some(evidence) => RecordAppendError::PhysicalPressure { evidence },
            None => RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(denial)),
        }
    }

    pub(in crate::physical_runtime) fn current_root(&self) -> DurablePhysicalRootManifest {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .current_root
            .clone()
    }

    pub(in crate::physical_runtime) fn residue(&self) -> RecordPublicationResidueObservation {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .residue
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime) fn planning_snapshot(
        &self,
    ) -> (DurablePhysicalRootManifest, DurableFreeSpaceManifestHeader) {
        let state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        (state.current_root.clone(), state.free_space.clone())
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime) fn reject_next_catalog_eligibility_join(&self) {
        self.reject_catalog_eligibility_join
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(feature = "certification-test-authority")]
    fn take_catalog_eligibility_join_rejection(&self) -> bool {
        self.reject_catalog_eligibility_join
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }
}
