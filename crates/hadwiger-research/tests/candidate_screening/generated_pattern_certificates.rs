use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, path_graph, transcript};

#[test]
fn final_certificate_required_lanes_have_query_readiness() {
    let handle = handle();

    assert_ready::<BoundaryOwnershipScreeningDeclaration>(&handle);
    assert_ready::<MonodromyColorHolonomyScreeningDeclaration>(&handle);
    assert_ready::<TranslationRotationClosureScreeningDeclaration>(&handle);
    assert_ready::<SubstitutionConsistencyScreeningDeclaration>(&handle);
    assert_ready::<FinitePatchBoundaryExtensionScreeningDeclaration>(&handle);
}

#[test]
fn boundary_ownership_replays_ownership_and_boundary_conflicts() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();

    let clean = evaluate_boundary_ownership_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        BoundaryOwnershipCertificate::new(
            "clean-boundary",
            vec![
                owned_region("left", 0, "red", true),
                owned_region("right", 3, "red", true),
            ],
            transcript("clean-boundary"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(clean.verdict(), CandidateScreeningVerdict::Passed);

    let uncovered = evaluate_boundary_ownership_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        BoundaryOwnershipCertificate::new(
            "uncovered-boundary",
            vec![owned_region("left", 0, "red", false)],
            transcript("uncovered-boundary"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(uncovered.rejects_candidate());

    let ambiguous = evaluate_boundary_ownership_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        BoundaryOwnershipCertificate::new(
            "ambiguous-boundary",
            vec![
                owned_region("same", 0, "red", true),
                owned_region("same", 0, "blue", true),
            ],
            transcript("ambiguous-boundary"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(ambiguous.rejects_candidate());

    let conflict = evaluate_boundary_ownership_screening_checked(
        &handle,
        &catalog,
        subject,
        BoundaryOwnershipCertificate::new(
            "unit-boundary-conflict",
            vec![
                owned_region("left", 0, "red", true),
                owned_region("right", 1, "red", true),
            ],
            transcript("unit-boundary-conflict"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(conflict.rejects_candidate());
    assert!(conflict.evidence().contains("query_declaration_digest="));
}

#[test]
fn generated_pattern_lanes_replay_pass_and_reject_cases() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();

    let monodromy_pass = evaluate_monodromy_color_holonomy_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        MonodromyColorHolonomyCertificate::new(
            "identity-loop",
            "red",
            vec![ColorPermutation::new(vec![("red".to_string(), "red".to_string())]).unwrap()],
            transcript("identity-loop"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(monodromy_pass.verdict(), CandidateScreeningVerdict::Passed);

    let monodromy_fail = evaluate_monodromy_color_holonomy_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        MonodromyColorHolonomyCertificate::new(
            "bad-loop",
            "red",
            vec![ColorPermutation::new(vec![("red".to_string(), "blue".to_string())]).unwrap()],
            transcript("bad-loop"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(monodromy_fail.rejects_candidate());

    let substitution = evaluate_substitution_consistency_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        SubstitutionConsistencyCertificate::new(
            "bad-substitution",
            2,
            vec![SubstitutionConsistencyFailureKind::Boundary],
            transcript("bad-substitution"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(substitution.rejects_candidate());

    let extension = evaluate_finite_patch_boundary_extension_screening_checked(
        &handle,
        &catalog,
        subject,
        FinitePatchBoundaryExtensionCertificate::new(
            "no-extension",
            vec!["red-blue".to_string(), "blue-red".to_string()],
            Vec::new(),
            transcript("no-extension"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(extension.rejects_candidate());
}

#[test]
fn translation_rotation_closure_replays_mapping_and_conflicts() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(2);

    let conflict = evaluate_translation_rotation_closure_screening_checked(
        &handle,
        &catalog,
        &graph,
        TranslationRotationClosureCertificate::new(
            "generated-conflict",
            vec![
                ("v0".to_string(), "v0".to_string()),
                ("v1".to_string(), "v1".to_string()),
            ],
            vec![("v0".to_string(), "v1".to_string())],
            transcript("generated-conflict"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(conflict.rejects_candidate());

    let err = evaluate_translation_rotation_closure_screening_checked(
        &handle,
        &catalog,
        &graph,
        TranslationRotationClosureCertificate::new(
            "bad-mapping",
            vec![
                ("v0".to_string(), "v0".to_string()),
                ("v1".to_string(), "v0".to_string()),
            ],
            vec![("v0".to_string(), "v0".to_string())],
            transcript("bad-mapping"),
        )
        .unwrap(),
    )
    .expect_err("closure mapping must be injective");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::TranslationRotationClosure,
            reason: "closure_mapping_not_injective"
        }
    );
}

#[test]
fn maximum_degree_is_not_an_invariant_catalog_row() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();

    assert_eq!(catalog.nodes().len(), 35);
    assert!(!catalog.has_family(CandidateScreeningInvariantFamily::MaximumDegreeSanityCheck));
}

fn assert_ready<I: HadwigerResearchDeclarationInput>(handle: &HadwigerResearchHandle) {
    assert!(!research_declaration_entry_readiness::<I>(handle)
        .rows()
        .is_empty());
}

fn owned_region(
    region_id: &str,
    x_offset: i128,
    color_id: &str,
    boundary_owner: bool,
) -> BoundaryOwnedRegion {
    BoundaryOwnedRegion::new(
        ScreeningRectangularRegion::new(
            region_id,
            ScreeningRational::integer(x_offset),
            ScreeningRational::integer(x_offset + 1),
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
        )
        .unwrap(),
        color_id,
        boundary_owner,
    )
    .unwrap()
}
