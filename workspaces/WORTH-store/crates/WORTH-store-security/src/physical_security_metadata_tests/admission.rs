use worth_proof::TransitionOutcome;
use worth_store_physical_format::PhysicalSecurityMetadataDenial;

use crate::{
    admit_store_physical_security_metadata, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StorePhysicalSecurityMetadataAdmissionInput, StorePhysicalSecurityMetadataCarrier,
};

use super::support::{admitted_scope, current_authority};

#[test]
fn platform_metadata_admission_denies_missing_unsupported_and_unavailable_metadata() {
    assert_metadata_denial(
        StorePhysicalSecurityMetadataAdmissionInput::MissingPlatformMetadata,
        PhysicalSecurityMetadataDenial::MissingPlatformSecurityMetadata,
    );
    assert_metadata_denial(
        StorePhysicalSecurityMetadataAdmissionInput::UnsupportedPlatformMetadata,
        PhysicalSecurityMetadataDenial::UnsupportedPlatformSecurityMetadata,
    );
    assert_metadata_denial(
        StorePhysicalSecurityMetadataAdmissionInput::UnavailablePlatformMetadata,
        PhysicalSecurityMetadataDenial::UnavailablePlatformSecurityMetadata,
    );
}

#[test]
fn platform_metadata_admission_denies_readmission_required_legacy_candidates() {
    for legacy_posture in [
        StoreLegacySecurityPosture::LegacyUnscoped,
        StoreLegacySecurityPosture::ReadmissionRequired,
        StoreLegacySecurityPosture::SecurityMetadataUnavailable,
        StoreLegacySecurityPosture::UnsupportedLegacyArtifact,
    ] {
        assert_metadata_denial(
            StorePhysicalSecurityMetadataAdmissionInput::Candidate(candidate_metadata(
                StoreKeyVersionPosture::Current,
                legacy_posture,
            )),
            PhysicalSecurityMetadataDenial::LegacyReadmissionRequired,
        );
    }
}

#[test]
fn platform_metadata_admission_denies_bad_key_version_candidates_distinctly() {
    for unsupported_posture in [
        StoreKeyVersionPosture::Unsupported,
        StoreKeyVersionPosture::Denied,
    ] {
        assert_metadata_denial(
            StorePhysicalSecurityMetadataAdmissionInput::Candidate(candidate_metadata(
                unsupported_posture,
                StoreLegacySecurityPosture::NativeScoped,
            )),
            PhysicalSecurityMetadataDenial::UnsupportedPlatformSecurityMetadata,
        );
    }
    for unavailable_posture in [
        StoreKeyVersionPosture::Unavailable,
        StoreKeyVersionPosture::Stale,
    ] {
        assert_metadata_denial(
            StorePhysicalSecurityMetadataAdmissionInput::Candidate(candidate_metadata(
                unavailable_posture,
                StoreLegacySecurityPosture::NativeScoped,
            )),
            PhysicalSecurityMetadataDenial::UnavailablePlatformSecurityMetadata,
        );
    }
}

fn assert_metadata_denial(
    input: StorePhysicalSecurityMetadataAdmissionInput,
    expected_denial: PhysicalSecurityMetadataDenial,
) {
    assert_eq!(
        admit_store_physical_security_metadata(input),
        TransitionOutcome::Denied(expected_denial)
    );
}

fn candidate_metadata(
    key_version_posture: StoreKeyVersionPosture,
    legacy_posture: StoreLegacySecurityPosture,
) -> StorePhysicalSecurityMetadataCarrier {
    let authority = current_authority("s51.phase3.metadata_admission", "candidate");
    let witnesses = admitted_scope(&authority);
    StorePhysicalSecurityMetadataCarrier::for_page_header(
        &witnesses,
        key_version_posture,
        legacy_posture,
    )
}
