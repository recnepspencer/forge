use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority, StoreDurabilityAdmission,
    StoreDurabilityBoundaryReached, StoreDurabilityDenial, StoreDurabilityFileSyncKind,
    StoreDurabilityRequirement, StoreDurabilityRuntime, StoreDurabilityWriteAccepted,
};

pub(super) fn admitted(requirement: StoreDurabilityRequirement) -> StoreDurabilityAdmission {
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();
    StoreDurabilityAdmission::admit(requirement, &witness).unwrap()
}

pub(super) fn reach_boundary<S>(
    accepted: StoreDurabilityWriteAccepted<S>,
    sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
) -> Result<StoreDurabilityBoundaryReached<S>, StoreDurabilityDenial>
where
    S: Clone + Eq + core::fmt::Debug,
{
    assert_eq!(accepted.requirement().required_file_sync(), sync);
    assert_eq!(
        accepted.requirement().requires_directory_sync(),
        directory_sync_completed
    );
    assert_eq!(
        accepted.requirement().requires_rename_durable(),
        rename_completed
    );
    assert_eq!(
        accepted.requirement().requires_ordering_barrier(),
        ordering_barrier_completed
    );
    let proof = StoreDurabilityRuntime::new()
        .persist_and_execute(&std::env::temp_dir(), b"recovery-durable-write", &accepted)
        .unwrap();
    accepted.reach_durability_boundary(proof)
}
