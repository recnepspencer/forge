use worth_store_physical_backend::{
    BackendCapabilityAdmissionDenial, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilityKind, BackendCapabilitySupportPosture,
    BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use crate::{
    admit_security_scope_for_scheduler, IoSchedulerBackendCapabilityDenial,
    IoSchedulerBackendCapabilityRequirement,
};

pub(super) fn externally_guaranteed_witness(
    kind: BackendCapabilityKind,
    posture: BackendCapabilitySupportPosture,
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    witness_from_basis_and_posture(
        kind,
        posture,
        BackendCapabilityEvidenceBasis::externally_guaranteed(1),
    )
}

pub(super) fn witness_from_basis_and_posture(
    kind: BackendCapabilityKind,
    posture: BackendCapabilitySupportPosture,
    basis: BackendCapabilityEvidenceBasis,
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported().with_posture(kind, posture);
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        basis,
        support,
        BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_sector_atomicity()
            .with_page_cache_policy()
            .with_mmap_coherence()
            .with_async_ordering()
            .with_secure_frame_io()
            .with_flush_ordering(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("baseline backend should admit")
}

pub(super) fn valid_security_scope() -> crate::IoSchedulerSecurityScopeAdmission {
    let security_scope = admitted_store_internal_security_scope_for_io_qos_test();
    admit_security_scope_for_scheduler(&security_scope)
        .expect("test security scope should admit for scheduler use")
}

pub(super) fn assert_evidence_denial(denial: IoSchedulerBackendCapabilityDenial) {
    assert!(matches!(
        denial,
        IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
            BackendCapabilityAdmissionDenial::EvidenceClassTooWeak { .. }
        )
    ));
}

pub(super) fn assert_scheduler_posture_denial(
    denial: IoSchedulerBackendCapabilityDenial,
    posture: BackendCapabilitySupportPosture,
) {
    match posture {
        BackendCapabilitySupportPosture::Unsupported => assert!(matches!(
            denial,
            IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                BackendCapabilityAdmissionDenial::UnsupportedCapability { .. }
            )
        )),
        BackendCapabilitySupportPosture::Unavailable => assert!(matches!(
            denial,
            IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                BackendCapabilityAdmissionDenial::UnavailableCapability { .. }
            )
        )),
        BackendCapabilitySupportPosture::Unknown => assert!(matches!(
            denial,
            IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                BackendCapabilityAdmissionDenial::UnknownCapability { .. }
            )
        )),
        BackendCapabilitySupportPosture::Stale => assert!(matches!(
            denial,
            IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                BackendCapabilityAdmissionDenial::StaleCapability { .. }
            )
        )),
        BackendCapabilitySupportPosture::RebindRequired => assert!(matches!(
            denial,
            IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied(
                BackendCapabilityAdmissionDenial::RebindRequired { .. }
            )
        )),
        BackendCapabilitySupportPosture::Supported => unreachable!(),
    }
}

pub(super) const fn platform_requirements() -> [IoSchedulerBackendCapabilityRequirement; 6] {
    [
        IoSchedulerBackendCapabilityRequirement::DirectIo,
        IoSchedulerBackendCapabilityRequirement::Mmap,
        IoSchedulerBackendCapabilityRequirement::AsyncIo,
        IoSchedulerBackendCapabilityRequirement::Fsync,
        IoSchedulerBackendCapabilityRequirement::DirectorySync,
        IoSchedulerBackendCapabilityRequirement::DurableRename,
    ]
}

pub(super) const fn weaker_than_external_evidence() -> [BackendCapabilityEvidenceBasis; 3] {
    [
        BackendCapabilityEvidenceBasis::declared_by_config(1),
        BackendCapabilityEvidenceBasis::observed_by_probe(1),
        BackendCapabilityEvidenceBasis::unverifiable_assumption(),
    ]
}

pub(super) const fn non_current_postures() -> [BackendCapabilitySupportPosture; 5] {
    [
        BackendCapabilitySupportPosture::Unsupported,
        BackendCapabilitySupportPosture::Unavailable,
        BackendCapabilitySupportPosture::Unknown,
        BackendCapabilitySupportPosture::Stale,
        BackendCapabilitySupportPosture::RebindRequired,
    ]
}
