use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationFoundationalEvidenceInput,
};

use super::domain::{admitted_handle, AspectRichFamily, ConflictingAspectFamily, Declaration};

#[test]
fn foundational_profiles_publish_semantic_slices_honestly() {
    let handle = admitted_handle("collaborative");
    let progression = handle
        .declare_review_and_progress(Declaration::<AspectRichFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let lean = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
        )
        .expect("lean foundational evidence should describe");
    let support_ready = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics,
        )
        .expect("support-ready foundational evidence should describe");
    let full = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progression),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
        .expect("full foundational evidence should describe");

    assert_eq!(
        lean.aspect_publication().present(),
        &["selection.active_edge".to_string()]
    );
    assert!(lean.aspect_publication().widened().is_empty());
    assert_eq!(
        lean.aspect_publication().elided(),
        &[
            "selection.local_topology".to_string(),
            "selection.material_edit".to_string()
        ]
    );

    assert_eq!(
        support_ready.aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string()
        ]
    );
    assert_eq!(
        support_ready.aspect_publication().widened(),
        &["selection.local_topology".to_string()]
    );

    assert_eq!(
        full.aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string(),
            "selection.material_edit".to_string()
        ]
    );
    assert_eq!(
        full.aspect_publication().masked(),
        &["selection.private_authority".to_string()]
    );
    assert_eq!(lean.aspect_contract(), support_ready.aspect_contract());
    assert_eq!(support_ready.aspect_coverage(), full.aspect_coverage());
    assert_eq!(
        full.aspect_coverage_basis(),
        ForgeQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage
    );
}

#[test]
fn foundational_publication_changes_without_changing_retained_truth_identity() {
    let handle = admitted_handle("collaborative");
    let progression = handle
        .declare_review_and_progress(Declaration::<AspectRichFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let lean = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
        )
        .expect("lean foundational evidence should describe");
    let full = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progression),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
        .expect("full foundational evidence should describe");

    assert_eq!(lean.declaration_digest(), full.declaration_digest());
    assert_eq!(lean.progression_digest(), full.progression_digest());
    assert_ne!(lean.aspect_publication(), full.aspect_publication());
}

#[test]
fn legality_denial_foundational_evidence_marks_support_reported_coverage_basis() {
    let handle = admitted_handle("collaborative");
    let declaration = handle
        .declare(Declaration::<super::domain::IllegalRoleFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("declaration should admit"));
    let denial = handle
        .review_legality(declaration)
        .err()
        .unwrap_or_else(|| panic!("legality should deny"));

    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::legality_denial(denial),
        )
        .expect("denied foundational evidence should still describe");

    assert_eq!(
        evidence.aspect_coverage_basis(),
        ForgeQueryDeclarationAspectCoverageBasis::SupportReportedCoverage
    );
}

#[test]
fn conflicting_aspects_stay_masked_even_on_full_foundational_publication() {
    let handle = admitted_handle("collaborative");
    let progression = handle
        .declare_review_and_progress(Declaration::<ConflictingAspectFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let evidence = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progression),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
        .expect("full foundational evidence should describe");

    assert_eq!(
        evidence.aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string()
        ]
    );
    assert_eq!(
        evidence.aspect_publication().masked(),
        &[
            "selection.material_edit".to_string(),
            "selection.private_authority".to_string()
        ]
    );
}
