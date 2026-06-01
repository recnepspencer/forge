use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::ForgeQueryDeclarationFoundationalEvidenceInput;

use super::domain::{
    admitted_handle, digest_text, AdmittedFamily, Declaration, DeniedFamily,
    DescriptiveDeferredSignalFamily,
};

#[test]
fn equivalent_admitted_progression_paths_share_foundational_bundle_digest() {
    let handle = admitted_handle("collaborative");
    let explicit = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                handle
                    .progress_declaration(
                        handle
                            .declare_and_review(Declaration::<AdmittedFamily>::new("edge:42"))
                            .unwrap_or_else(|_| panic!("legality should pass")),
                    )
                    .unwrap_or_else(|_| panic!("progression should admit")),
            ),
        )
        .expect("foundational description should admit");
    let convenience = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                handle
                    .declare_review_and_progress(Declaration::<AdmittedFamily>::new("edge:42"))
                    .unwrap_or_else(|_| panic!("convenience progression should admit")),
            ),
        )
        .expect("foundational description should admit");

    assert_eq!(
        digest_text(explicit.attachment_bundle_digest()),
        digest_text(convenience.attachment_bundle_digest())
    );
}

#[test]
fn evidence_digest_changes_with_world_and_progression_outcome() {
    let collaborative = admitted_handle("collaborative");
    let mirror = admitted_handle("mirror");
    let admitted_left = collaborative
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                collaborative
                    .declare_review_and_progress(Declaration::<AdmittedFamily>::new("edge:42"))
                    .unwrap_or_else(|_| panic!("progression should admit")),
            ),
        )
        .expect("foundational description should admit");
    let admitted_right = mirror
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                mirror
                    .declare_review_and_progress(Declaration::<AdmittedFamily>::new("edge:42"))
                    .unwrap_or_else(|_| panic!("progression should admit")),
            ),
        )
        .expect("foundational description should admit");
    let denied = collaborative
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::progression_checked(
                collaborative.progress_declaration_checked(
                    collaborative
                        .declare_and_review(Declaration::<DeniedFamily>::new("edge:42"))
                        .unwrap_or_else(|_| panic!("legality should pass")),
                ),
            ),
        )
        .expect("denied progression should still describe");

    assert_ne!(
        digest_text(admitted_left.attachment_bundle_digest()),
        digest_text(admitted_right.attachment_bundle_digest())
    );
    assert_ne!(
        digest_text(admitted_left.attachment_bundle_digest()),
        digest_text(denied.attachment_bundle_digest())
    );
}

#[test]
fn legality_only_evidence_preserves_world_identity_and_world_sensitive_digest() {
    let collaborative = admitted_handle("collaborative");
    let mirror = admitted_handle("mirror");
    let collaborative_evidence = collaborative
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::legality_evidence(
                collaborative
                    .declare_and_review(Declaration::<AdmittedFamily>::new("edge:42"))
                    .unwrap_or_else(|_| panic!("legality should pass")),
            ),
        )
        .expect("legality evidence should describe");
    let mirror_evidence = mirror
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::legality_evidence(
                mirror
                    .declare_and_review(Declaration::<AdmittedFamily>::new("edge:42"))
                    .unwrap_or_else(|_| panic!("legality should pass")),
            ),
        )
        .expect("legality evidence should describe");

    assert_eq!(
        collaborative_evidence.operating_context_identity_digest(),
        "geometry.collaborative"
    );
    assert_eq!(
        mirror_evidence.operating_context_identity_digest(),
        "geometry.mirror"
    );
    assert_ne!(
        digest_text(collaborative_evidence.attachment_bundle_digest()),
        digest_text(mirror_evidence.attachment_bundle_digest())
    );
}

#[test]
fn descriptive_only_families_still_describe_honestly() {
    let handle = admitted_handle("collaborative");
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                handle
                    .declare_review_and_progress(
                        Declaration::<DescriptiveDeferredSignalFamily>::new("edge:42"),
                    )
                    .unwrap_or_else(|_| panic!("descriptive progression should admit")),
            ),
        )
        .expect("descriptive evidence should describe");

    assert_eq!(evidence.declaration_family_key(), "split-edge");
}

#[test]
fn profile_elision_is_preserved_on_wrapped_evidence() {
    let handle = admitted_handle("collaborative");
    let evidence = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                handle
                    .declare_review_and_progress(Declaration::<AdmittedFamily>::new("edge:42"))
                    .unwrap_or_else(|_| panic!("progression should admit")),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
        )
        .expect("lean foundational evidence should describe");

    assert_eq!(
        evidence.materialization_profile(),
        FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics
    );
    assert!(evidence.attachment_bundle().support().is_none());
    assert!(evidence.support_attachment().is_some());
}
