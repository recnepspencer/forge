use worth_proof::prelude::ProofOutcomeKind;

use super::*;

#[test]
fn independent_admissions_with_same_basis_produce_equivalent_witnesses() {
    let left = admit(BackendCapabilityEvidenceBasis::certified_backend_profile());
    let right = admit(BackendCapabilityEvidenceBasis::certified_backend_profile());

    assert_equivalent_witness(&left, &right);
}

#[test]
fn admission_preserves_evidence_class_rebind_triggers_and_confidence_limits() {
    let witness = admit(BackendCapabilityEvidenceBasis::certified_backend_profile());

    assert_eq!(
        witness.evidence_class(),
        CapabilityEvidenceClass::CertifiedBackendProfile
    );
    assert!(witness
        .rebind_triggers()
        .contains(BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()));
    assert_eq!(
        witness.confidence_limits(),
        CapabilityConfidenceLimits::certified_backend_profile()
    );
}

#[test]
fn every_evidence_class_preserves_strength_and_denies_stronger_claims() {
    let cases = [
        (
            BackendCapabilityEvidenceBasis::declared_by_config(1),
            CapabilityEvidenceClass::DeclaredByConfig,
        ),
        (
            BackendCapabilityEvidenceBasis::observed_by_probe(1),
            CapabilityEvidenceClass::ObservedByProbe,
        ),
        (
            BackendCapabilityEvidenceBasis::externally_guaranteed(1),
            CapabilityEvidenceClass::ExternallyGuaranteed,
        ),
        (
            BackendCapabilityEvidenceBasis::unverifiable_assumption(),
            CapabilityEvidenceClass::UnverifiableAssumption,
        ),
    ];

    for (basis, expected_class) in cases {
        let witness = admit(basis);
        assert_eq!(witness.evidence_class(), expected_class);

        let denial = witness
            .require(
                BackendCapabilityKind::DirectIo,
                CapabilityEvidenceClass::CertifiedBackendProfile,
            )
            .expect_err("weaker evidence must not satisfy certified profile");
        assert_eq!(
            denial,
            BackendCapabilityAdmissionDenial::EvidenceClassTooWeak {
                required: CapabilityEvidenceClass::CertifiedBackendProfile,
                actual: expected_class,
            }
        );
    }
}

#[test]
fn unverifiable_assumption_cannot_back_even_matching_runtime_claim() {
    let witness = admit(BackendCapabilityEvidenceBasis::unverifiable_assumption());

    assert_eq!(
        witness.require(
            BackendCapabilityKind::DirectIo,
            CapabilityEvidenceClass::UnverifiableAssumption
        ),
        Err(BackendCapabilityAdmissionDenial::ConfidenceLimitTooWeak)
    );
}

#[test]
fn certification_only_evidence_remains_a_typed_non_authority_rejection() {
    assert_eq!(
        reject_certification_only_evidence(),
        Err(BackendCapabilityAdmissionDenial::CertificationOnlyEvidence)
    );
}

#[test]
fn raw_surfaces_deny_with_distinct_variants_before_witness() {
    let denials = [
        reject_raw_backend_label(),
        reject_raw_config_string(),
        reject_raw_os_name(),
        reject_raw_probe_observation(),
        reject_same_process_metric_projection(),
        reject_environment_variable(),
        reject_terminal_projection(),
        reject_copied_qualification_row(),
    ];
    let expected = [
        BackendCapabilityAdmissionDenial::RawBackendLabel,
        BackendCapabilityAdmissionDenial::RawConfigString,
        BackendCapabilityAdmissionDenial::RawOsName,
        BackendCapabilityAdmissionDenial::RawProbeObservation,
        BackendCapabilityAdmissionDenial::SameProcessMetricProjection,
        BackendCapabilityAdmissionDenial::EnvironmentVariable,
        BackendCapabilityAdmissionDenial::TerminalProjection,
        BackendCapabilityAdmissionDenial::CopiedQualificationRow,
    ];

    for (actual, expected) in denials.into_iter().zip(expected) {
        assert_eq!(actual, Err(expected));
    }
}

#[test]
fn unsupported_posture_denies_claim_consumption() {
    let witness = admit_with(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        BackendCapabilitySupportSet::buffered_durable_only(),
        direct_io_media_assumptions(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    );

    assert!(matches!(
        witness.require(
            BackendCapabilityKind::DirectIo,
            CapabilityEvidenceClass::CertifiedBackendProfile
        ),
        Err(BackendCapabilityAdmissionDenial::UnsupportedCapability { .. })
    ));
}

#[test]
fn every_platform_claim_denies_all_non_current_postures() {
    let claim_kinds = [
        BackendCapabilityKind::DirectIo,
        BackendCapabilityKind::Mmap,
        BackendCapabilityKind::AsyncIo,
        BackendCapabilityKind::Fsync,
        BackendCapabilityKind::DirectorySync,
        BackendCapabilityKind::DurableRename,
        BackendCapabilityKind::SecureFrameIo,
    ];
    let denied_postures = [
        BackendCapabilitySupportPosture::Unsupported,
        BackendCapabilitySupportPosture::Unavailable,
        BackendCapabilitySupportPosture::Unknown,
        BackendCapabilitySupportPosture::Stale,
        BackendCapabilitySupportPosture::RebindRequired,
    ];

    for kind in claim_kinds {
        for posture in denied_postures {
            let witness = admitted_with_claim_posture(kind, posture);
            let outcome =
                witness.require_checked(kind, CapabilityEvidenceClass::CertifiedBackendProfile);

            match posture {
                BackendCapabilitySupportPosture::Unsupported
                | BackendCapabilitySupportPosture::Unavailable
                | BackendCapabilitySupportPosture::Unknown => {
                    assert_eq!(outcome.kind(), ProofOutcomeKind::Denied);
                }
                BackendCapabilitySupportPosture::Stale => {
                    assert_eq!(outcome.kind(), ProofOutcomeKind::Stale);
                }
                BackendCapabilitySupportPosture::RebindRequired => {
                    assert_eq!(outcome.kind(), ProofOutcomeKind::RebindRequired);
                }
                BackendCapabilitySupportPosture::Supported => unreachable!(),
            }
        }
    }
}

#[test]
fn every_platform_claim_denies_missing_media_assumptions_before_consumption() {
    for kind in platform_claim_kinds() {
        let witness = admit_with(
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            missing_media_assumptions_for(kind),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        );

        assert_eq!(
            witness.require(kind, CapabilityEvidenceClass::CertifiedBackendProfile),
            Err(BackendCapabilityAdmissionDenial::MissingMediaAssumption(
                kind
            ))
        );
    }
}

#[test]
fn stale_and_rebind_required_postures_deny_claim_consumption() {
    let stale = admitted_with_direct_io_posture(BackendCapabilitySupportPosture::Stale);
    let rebind = admitted_with_direct_io_posture(BackendCapabilitySupportPosture::RebindRequired);

    assert!(matches!(
        stale.require(
            BackendCapabilityKind::DirectIo,
            CapabilityEvidenceClass::CertifiedBackendProfile
        ),
        Err(BackendCapabilityAdmissionDenial::StaleCapability { .. })
    ));
    assert!(matches!(
        rebind.require(
            BackendCapabilityKind::DirectIo,
            CapabilityEvidenceClass::CertifiedBackendProfile
        ),
        Err(BackendCapabilityAdmissionDenial::RebindRequired { .. })
    ));
}

#[test]
fn environment_drift_triggers_rebind_required_progression() {
    let support = BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::Mmap,
        BackendCapabilitySupportPosture::RebindRequired,
    );
    let triggers = BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
        .with_cloud_volume()
        .with_sector_alignment()
        .with_security_posture();
    let witness = admit_with(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        support,
        all_media_assumptions(),
        triggers,
    );

    let outcome = witness.require_checked(
        BackendCapabilityKind::Mmap,
        CapabilityEvidenceClass::CertifiedBackendProfile,
    );

    assert_eq!(outcome.kind(), ProofOutcomeKind::RebindRequired);
    assert!(witness.rebind_triggers().contains(triggers));
}

fn admit(basis: BackendCapabilityEvidenceBasis) -> AdmittedBackendCapabilityWitness {
    admit_with(
        basis,
        BackendCapabilitySupportSet::all_supported(),
        all_media_assumptions(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    )
}

fn admit_with(
    basis: BackendCapabilityEvidenceBasis,
    support: BackendCapabilitySupportSet,
    media_assumptions: BackendMediaAssumptionSet,
    rebind_triggers: BackendRebindTriggers,
) -> AdmittedBackendCapabilityWitness {
    let request = BackendCapabilityAdmissionRequest::new(
        BackendTargetProfile::PosixFileFsyncDirSync,
        basis,
        support,
        media_assumptions,
        rebind_triggers,
    );
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(request)
        .expect("backend capability should admit")
}

fn assert_equivalent_witness(
    left: &AdmittedBackendCapabilityWitness,
    right: &AdmittedBackendCapabilityWitness,
) {
    assert_eq!(left.profile(), right.profile());
    assert_eq!(left.evidence_class(), right.evidence_class());
    assert_eq!(left.support(), right.support());
    assert_eq!(left.media_assumptions(), right.media_assumptions());
    assert_eq!(left.rebind_triggers(), right.rebind_triggers());
    assert_eq!(left.confidence_limits(), right.confidence_limits());
}

fn admitted_with_direct_io_posture(
    posture: BackendCapabilitySupportPosture,
) -> AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported()
        .with_posture(BackendCapabilityKind::DirectIo, posture);
    admit_with(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        support,
        direct_io_media_assumptions(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    )
}

fn admitted_with_claim_posture(
    kind: BackendCapabilityKind,
    posture: BackendCapabilitySupportPosture,
) -> AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported().with_posture(kind, posture);
    admit_with(
        BackendCapabilityEvidenceBasis::certified_backend_profile(),
        support,
        all_media_assumptions(),
        BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
    )
}

fn direct_io_media_assumptions() -> BackendMediaAssumptionSet {
    BackendMediaAssumptionSet::platform_file_defaults()
        .with_direct_io_alignment()
        .with_sector_atomicity()
        .with_page_cache_policy()
}

fn all_media_assumptions() -> BackendMediaAssumptionSet {
    direct_io_media_assumptions()
        .with_mmap_coherence()
        .with_async_ordering()
        .with_secure_frame_io()
        .with_flush_ordering()
        .with_fdatasync_durability()
}

const fn platform_claim_kinds() -> [BackendCapabilityKind; 7] {
    [
        BackendCapabilityKind::DirectIo,
        BackendCapabilityKind::Mmap,
        BackendCapabilityKind::AsyncIo,
        BackendCapabilityKind::Fsync,
        BackendCapabilityKind::DirectorySync,
        BackendCapabilityKind::DurableRename,
        BackendCapabilityKind::SecureFrameIo,
    ]
}

fn missing_media_assumptions_for(kind: BackendCapabilityKind) -> BackendMediaAssumptionSet {
    match kind {
        BackendCapabilityKind::DirectIo => BackendMediaAssumptionSet::platform_file_defaults()
            .with_direct_io_alignment()
            .with_page_cache_policy(),
        BackendCapabilityKind::Mmap => {
            BackendMediaAssumptionSet::platform_file_defaults().with_page_cache_policy()
        }
        BackendCapabilityKind::AsyncIo => BackendMediaAssumptionSet::platform_file_defaults(),
        BackendCapabilityKind::Fsync => BackendMediaAssumptionSet::empty().with_flush_ordering(),
        BackendCapabilityKind::DirectorySync => {
            BackendMediaAssumptionSet::empty().with_flush_ordering()
        }
        BackendCapabilityKind::DurableRename => {
            BackendMediaAssumptionSet::empty().with_flush_ordering()
        }
        BackendCapabilityKind::SecureFrameIo => BackendMediaAssumptionSet::empty()
            .with_page_cache_policy()
            .with_flush_ordering(),
        BackendCapabilityKind::BufferedFile => unreachable!("not a platform-grade claim"),
    }
}
