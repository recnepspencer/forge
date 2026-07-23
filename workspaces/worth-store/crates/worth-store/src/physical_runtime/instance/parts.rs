use worth_store_physical_format::{DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest};

use crate::physical_runtime::{
    lifecycle::LifecycleTerminationGuard,
    record_serving::{
        AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, RecordAllocationFrontier,
        RecordFramePorts, RecordPublicationResidueObservation, RecordServingOwner, ServingHealth,
    },
    runtime::PhysicalRuntimeCore,
    work::{PhysicalWorkAdmissionAuthority, PhysicalWorkSubmissionOwner},
};

use super::{PhysicalSchedulerAdmissionOwner, PhysicalWorkExecutor, PhysicalWorkSignalOwner};

/// Exhaustive construction packet for the owners installed in record-serving.
///
/// This remains private to the physical composition root. Adding a managed
/// owner therefore makes construction and terminal destructuring fail until
/// every lifecycle boundary handles it.
pub(in crate::physical_runtime) struct PhysicalStoreInstanceParts {
    pub(in crate::physical_runtime) termination: LifecycleTerminationGuard,
    pub(in crate::physical_runtime) work_admission: PhysicalWorkAdmissionAuthority,
    pub(in crate::physical_runtime) work_submission: PhysicalWorkSubmissionOwner,
    pub(in crate::physical_runtime) signal_owner: PhysicalWorkSignalOwner,
    pub(in crate::physical_runtime) scheduler_admission: PhysicalSchedulerAdmissionOwner,
    pub(in crate::physical_runtime) record_owner: RecordServingOwner,
    pub(in crate::physical_runtime) executor: PhysicalWorkExecutor,
    pub(in crate::physical_runtime) core: PhysicalRuntimeCore,
    pub(in crate::physical_runtime) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime) current_root: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime) free_space: DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime) allocation_frontier: RecordAllocationFrontier,
    pub(in crate::physical_runtime) publication_residue: RecordPublicationResidueObservation,
    pub(in crate::physical_runtime) health: ServingHealth,
    pub(in crate::physical_runtime) frame_ports: RecordFramePorts,
}
