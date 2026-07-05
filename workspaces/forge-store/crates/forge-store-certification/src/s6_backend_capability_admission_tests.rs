use crate::{certify_s6_backend_capability_admission, publish_s6_backend_capability_readiness};
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};

#[test]
fn s6_backend_capability_certification_preserves_exact_witness_and_readiness_fields() {
    let witness =
        admitted_backend_witness(BackendCapabilityEvidenceBasis::certified_backend_profile());
    let readiness = publish_s6_backend_capability_readiness(&witness);

    let evidence = certify_s6_backend_capability_admission(&witness, &readiness)
        .expect("matching readiness should certify admitted backend capability");

    assert_eq!(evidence.profile(), witness.profile());
    assert_eq!(evidence.evidence_class(), witness.evidence_class());
    assert_eq!(evidence.support(), witness.support());
    assert_eq!(evidence.media_assumptions(), witness.media_assumptions());
    assert_eq!(evidence.rebind_triggers(), witness.rebind_triggers());
    assert_eq!(evidence.confidence_limits(), witness.confidence_limits());
}

#[test]
fn s6_backend_capability_certification_denies_mismatched_readiness() {
    let witness =
        admitted_backend_witness(BackendCapabilityEvidenceBasis::certified_backend_profile());
    let different_witness =
        admitted_backend_witness(BackendCapabilityEvidenceBasis::declared_by_config(11));
    let mismatched_readiness = publish_s6_backend_capability_readiness(&different_witness);

    assert_eq!(
        certify_s6_backend_capability_admission(&witness, &mismatched_readiness),
        None
    );
}

fn admitted_backend_witness(
    basis: BackendCapabilityEvidenceBasis,
) -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        basis,
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
