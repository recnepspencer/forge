use std::collections::BTreeSet;

use worth_store_formal_models::assumptions::{
    admit_protocol_backend_capabilities, admit_protocol_backend_profile,
    current_protocol_backend_assumption_matrix, ProtocolBackendCapabilityDenial,
    PublicationAtomicityAssumption, TornWriteAssumption, UnsupportedProtocolBackendProfile,
};
use worth_store_formal_models::ProtocolFamily;
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, AdversarialReorderedFlushProfile,
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendDurabilityProfile, BackendMediaAssumptionSet, BackendRebindTriggers,
    BackendTargetProfile, MmapFlushNotDurabilityCertifiedProfile,
    PhysicalBackendCapabilityAdmissionAuthority, PosixFileFsyncDirFsyncProfile,
};

#[test]
fn every_checked_family_has_one_explicit_media_assumption_row() {
    let matrix = current_protocol_backend_assumption_matrix();
    let protocols = matrix
        .iter()
        .map(|row| row.protocol())
        .collect::<BTreeSet<_>>();

    assert_eq!(protocols, BTreeSet::from(ProtocolFamily::all()));
    assert!(matrix.iter().all(|row| {
        row.torn_write() == TornWriteAssumption::TornPagePossible
            && row.publication_atomicity()
                != PublicationAtomicityAssumption::AtomicReplacementAfterDirectoryFence
    }));
}

#[test]
fn certified_and_adversarial_reordering_profiles_are_runtime_derived_claims() {
    for protocol in ProtocolFamily::all() {
        let posix_runtime = admitted_backend(PosixFileFsyncDirFsyncProfile::TARGET);
        let posix = admit_protocol_backend_profile::<PosixFileFsyncDirFsyncProfile>(protocol)
            .expect("fsync plus directory fsync profile is certified");
        assert_eq!(posix.row().protocol(), protocol);
        assert_eq!(
            posix.durability().runtime_profile(),
            PosixFileFsyncDirFsyncProfile::ID
        );
        let admitted = admit_protocol_backend_capabilities::<PosixFileFsyncDirFsyncProfile>(
            protocol,
            &posix_runtime,
        )
        .expect("profile claim must be backed by concrete capability claims");
        assert!(!admitted.capabilities().is_empty());

        let reordered_runtime = admitted_backend(AdversarialReorderedFlushProfile::TARGET);
        let reordered =
            admit_protocol_backend_profile::<AdversarialReorderedFlushProfile>(protocol)
                .expect("ordered persistence fence admits the reordered-flush profile");
        assert_eq!(
            reordered.durability().runtime_profile(),
            AdversarialReorderedFlushProfile::ID
        );
        admit_protocol_backend_capabilities::<AdversarialReorderedFlushProfile>(
            protocol,
            &reordered_runtime,
        )
        .expect("ordered persistence capability must back the reordered profile");
    }
}

#[test]
fn copied_profile_label_cannot_substitute_for_runtime_capability_identity() {
    let windows_runtime = admitted_backend(BackendTargetProfile::WindowsFlushFileBuffers);
    let denial = admit_protocol_backend_capabilities::<PosixFileFsyncDirFsyncProfile>(
        ProtocolFamily::DurabilityRecovery,
        &windows_runtime,
    )
    .expect_err("runtime capability identity must match the modeled profile");

    assert!(matches!(
        denial,
        ProtocolBackendCapabilityDenial::RuntimeProfileMismatch {
            expected: BackendTargetProfile::PosixFileFsyncDirSync,
            actual: BackendTargetProfile::WindowsFlushFileBuffers,
        }
    ));
}

fn admitted_backend(profile: BackendTargetProfile) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            profile,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("certified backend fixture is admissible")
}

#[test]
fn unsupported_mmap_durability_is_a_typed_non_claim_for_every_family() {
    for protocol in ProtocolFamily::all() {
        let denial: UnsupportedProtocolBackendProfile =
            admit_protocol_backend_profile::<MmapFlushNotDurabilityCertifiedProfile>(protocol)
                .expect_err("mmap flush alone cannot back a durability claim");
        assert_eq!(denial.protocol(), protocol);
        assert_eq!(denial.profile(), MmapFlushNotDurabilityCertifiedProfile::ID);
    }
}
