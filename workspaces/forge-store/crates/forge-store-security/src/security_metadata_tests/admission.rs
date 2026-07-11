use forge_proof::TransitionOutcome;

use crate::{
    admit_store_security_metadata, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityMetadata, StoreSecurityMetadataAdmissionDenial,
    StoreSecurityMetadataAdmissionInput,
};

use super::support::{admitted_scope, current_authority};

#[test]
fn platform_metadata_admission_denies_missing_unsupported_and_unavailable_metadata() {
    assert_metadata_denial(
        StoreSecurityMetadataAdmissionInput::MissingMetadata,
        StoreSecurityMetadataAdmissionDenial::MissingMetadata,
    );
    assert_metadata_denial(
        StoreSecurityMetadataAdmissionInput::UnsupportedMetadata,
        StoreSecurityMetadataAdmissionDenial::UnsupportedMetadata,
    );
    assert_metadata_denial(
        StoreSecurityMetadataAdmissionInput::UnavailableMetadata,
        StoreSecurityMetadataAdmissionDenial::UnavailableMetadata,
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
            StoreSecurityMetadataAdmissionInput::Candidate(candidate_metadata(
                StoreKeyVersionPosture::Current,
                legacy_posture,
            )),
            StoreSecurityMetadataAdmissionDenial::LegacyReadmissionRequired,
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
            StoreSecurityMetadataAdmissionInput::Candidate(candidate_metadata(
                unsupported_posture,
                StoreLegacySecurityPosture::NativeScoped,
            )),
            StoreSecurityMetadataAdmissionDenial::UnsupportedMetadata,
        );
    }
    for unavailable_posture in [
        StoreKeyVersionPosture::Unavailable,
        StoreKeyVersionPosture::Stale,
    ] {
        assert_metadata_denial(
            StoreSecurityMetadataAdmissionInput::Candidate(candidate_metadata(
                unavailable_posture,
                StoreLegacySecurityPosture::NativeScoped,
            )),
            StoreSecurityMetadataAdmissionDenial::UnavailableMetadata,
        );
    }
}

fn assert_metadata_denial(
    input: StoreSecurityMetadataAdmissionInput,
    expected_denial: StoreSecurityMetadataAdmissionDenial,
) {
    assert_eq!(
        admit_store_security_metadata(input),
        TransitionOutcome::Denied(expected_denial)
    );
}

fn candidate_metadata(
    key_version_posture: StoreKeyVersionPosture,
    legacy_posture: StoreLegacySecurityPosture,
) -> StoreSecurityMetadata {
    let authority = current_authority("s51.phase3.metadata_admission", "candidate");
    let witnesses = admitted_scope(&authority);
    StoreSecurityMetadata::from_current_security_scope(
        &witnesses,
        key_version_posture,
        legacy_posture,
    )
}
