use super::super::{
    FilesystemMediaAdmissionAuthority, FilesystemMediaOwner, MediaCapabilityQualificationOutcome,
};
use crate::io_capability::{
    BackendCapabilityAdmissionDenial, BackendCapabilityKind,
    BackendCapabilityQualificationDeferred, BackendCapabilityQualificationFailure,
    BackendCapabilityRebindRequired, BackendCapabilityStale, BackendRebindTriggers,
};
use worth_proof::{ProofOutcomeKind, TransitionOutcome};

#[test]
fn qualification_preserves_every_non_success_progression_category() {
    let kind = BackendCapabilityKind::DirectIo;
    let denied: MediaCapabilityQualificationOutcome =
        TransitionOutcome::denied(BackendCapabilityAdmissionDenial::UnsupportedCapability {
            kind,
            posture: crate::BackendCapabilitySupportPosture::Unsupported,
        })
        .into();
    let deferred: MediaCapabilityQualificationOutcome =
        TransitionOutcome::deferred(BackendCapabilityQualificationDeferred::for_test(kind)).into();
    let stale: MediaCapabilityQualificationOutcome =
        TransitionOutcome::stale(BackendCapabilityStale::for_test(kind)).into();
    let rebind: MediaCapabilityQualificationOutcome =
        TransitionOutcome::rebind_required(BackendCapabilityRebindRequired::for_test(
            kind,
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .into();
    let failed: MediaCapabilityQualificationOutcome =
        TransitionOutcome::failed(BackendCapabilityQualificationFailure::for_test(kind)).into();

    assert_eq!(denied.kind(), ProofOutcomeKind::Denied);
    assert_eq!(deferred.kind(), ProofOutcomeKind::Deferred);
    assert_eq!(stale.kind(), ProofOutcomeKind::Stale);
    assert_eq!(rebind.kind(), ProofOutcomeKind::RebindRequired);
    assert_eq!(failed.kind(), ProofOutcomeKind::Failed);
}

#[test]
fn admitted_identity_and_live_owner_must_share_one_capability_basis() {
    let parent = tempfile::tempdir().unwrap();
    let owner_a = FilesystemMediaOwner::admit(
        &parent.path().join("a"),
        FilesystemMediaAdmissionAuthority::for_test(),
    )
    .unwrap();
    let identity = super::super::namespace_identity_admission::admit_store_identity(&owner_a)
        .expect("establish owner A identity");
    let owner_b = FilesystemMediaOwner::admit(
        &parent.path().join("b"),
        FilesystemMediaAdmissionAuthority::for_test(),
    )
    .unwrap();
    let profile = super::super::profile_observation::observe_profile(
        owner_b.root_directory_handle(),
        owner_b.boundary(),
    )
    .unwrap();

    assert_eq!(
        super::super::capability_qualification::qualify_backend_claims(
            &owner_b, &identity, &profile,
        ),
        Err(crate::BackendCapabilityAdmissionDenial::FilesystemAdmissionEvidenceUnavailable)
    );
}
