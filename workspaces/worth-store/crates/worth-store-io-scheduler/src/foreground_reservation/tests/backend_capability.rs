use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};

use crate::{
    admit_backend_capability_for_scheduler_claim, IoSchedulerBackendCapabilityRequirement,
};

pub(super) fn backend_admission(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> crate::IoSchedulerBackendCapabilityAdmission {
    let witness = admitted_backend_witness_for(requirement);
    admit_backend_capability_for_scheduler_claim(&witness, requirement)
        .expect("backend capability should admit for scheduler claim")
}

pub(super) fn admitted_backend_witness(
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    admitted_backend_witness_for(IoSchedulerBackendCapabilityRequirement::DirectIo)
}

fn admitted_backend_witness_for(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        backend_evidence_basis_for(requirement),
        BackendCapabilitySupportSet::all_supported(),
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering()
            .with_fdatasync_durability(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend witness should admit")
}

const fn backend_evidence_basis_for(
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> BackendCapabilityEvidenceBasis {
    match requirement {
        IoSchedulerBackendCapabilityRequirement::BufferedFile => {
            BackendCapabilityEvidenceBasis::declared_by_config(2)
        }
        IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync => {
            BackendCapabilityEvidenceBasis::established_filesystem_admission_for_certification(1)
        }
        _ => BackendCapabilityEvidenceBasis::certified_backend_profile(),
    }
}
