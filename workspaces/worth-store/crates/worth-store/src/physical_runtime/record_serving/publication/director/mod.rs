#[cfg(feature = "certification-test-authority")]
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Condvar, Mutex, Weak};

use worth_store_physical_format::{DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest};

use crate::physical_runtime::instance::PhysicalStoreWorkRuntime;

use super::super::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, CanonicalRecordMutationPort,
    RecordAllocationFrontier, RecordFramePorts, RecordPublicationResidueObservation,
};

mod execution;
mod lifecycle;
mod submission;

pub use submission::{PhysicalRecordSubmission, PreparedRecordAppend};

pub(in crate::physical_runtime) struct RecordPublicationDirector {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    mutation: CanonicalRecordMutationPort,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    frame_ports: RecordFramePorts,
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
    pub(in crate::physical_runtime) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime) current_root: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime) free_space: DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime) allocation_frontier: RecordAllocationFrontier,
    pub(in crate::physical_runtime) residue: RecordPublicationResidueObservation,
    pub(in crate::physical_runtime) frame_ports: RecordFramePorts,
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
        mutation: CanonicalRecordMutationPort,
        foundation: RecordPublicationFoundation,
    ) -> Arc<Self> {
        Arc::new(Self {
            runtime: Arc::downgrade(runtime),
            mutation,
            format: foundation.format,
            access: foundation.access,
            frame_ports: foundation.frame_ports,
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
