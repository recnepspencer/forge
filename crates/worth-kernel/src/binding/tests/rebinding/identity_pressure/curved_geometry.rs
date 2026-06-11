use std::f64::consts::{PI, TAU};

use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_rebinding_declaration,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent, BindingContinuityClass,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveAnchorBindingDeclarationEntry, RebindingOutcomeClass, SpatialAnchorAuthorityError,
};

use crate::binding::tests::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, assert_workflow_artifact_parity,
    canonical_geometry, canonical_text_entries_for_rebinding, inspect_progressed_rebinding_entry,
    orthotope_contract, progress_rebinding_entry, rebind_surface_on_face,
    rebinding_ordinary_outcome_for_entry, rebinding_receipt_for_entry,
    rebinding_workflow_artifacts, replacement_neighborhood,
};

fn anchored_face_binding_declaration(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, domain).expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}

fn trimmed_face_binding_declaration(
    face_id: &str,
    outer_boundary: [[f64; 2]; 4],
    point: [f64; 2],
) -> PrimitiveAnchorBindingDeclarationEntry {
    let trimmed_region = PolygonalTrimmedParameterRegion::new(
        ParameterDomain::plane(),
        outer_boundary
            .into_iter()
            .map(|coords| ParameterSpacePoint::try_new(coords).expect("boundary point"))
            .collect(),
        vec![],
    )
    .expect("trimmed region");
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_trimmed_face_surface(face_id, trimmed_region)
                    .expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}

fn admitted_face_surface_point_anchor_identity(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
) -> String {
    crate::binding::tests::support::anchored_surface_prior_fact_from_declaration(
        declaration,
        "curved-pressure-anchor-identity",
    )
    .prior_binding_identity()
    .to_string()
}

#[test]
fn curved_carrier_pressure_breaks_planar_anchor_and_rebinding_shortcuts() {
    let handle = admitted_rebinding_handle("curved-rebinding-pressure");
    let prior = anchored_face_binding_declaration(
        "face-curved",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let preserved =
        anchored_face_binding_declaration("face-curved", ParameterDomain::cylinder(), [0.25, 0.5]);
    let planarized =
        anchored_face_binding_declaration("face-curved", ParameterDomain::plane(), [0.25, 0.5]);

    let curved_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "curved-pressure-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                vec![anchored_surface_candidate_from_declaration(
                    "curved",
                    &preserved,
                    "curved-pressure-curved",
                )
                .expect("candidate")],
            ),
        ),
    );
    let curved_progression = progress_rebinding_entry(&curved_entry, &handle);
    let curved_inspection = inspect_progressed_rebinding_entry(&handle, curved_progression.clone());
    let curved_workflow = rebinding_workflow_artifacts(&curved_entry, &handle);
    let curved_decision = rebinding_receipt_for_entry(&curved_entry, "curved-pressure-curved")
        .expect("curved decision");
    let curved_outcome = rebinding_ordinary_outcome_for_entry(&curved_entry, &handle);

    let planarized_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "curved-planarized-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                vec![anchored_surface_candidate_from_declaration(
                    "planarized",
                    &planarized,
                    "curved-planarized-candidate",
                )
                .expect("candidate")],
            ),
        ),
    );
    let planarized_decision =
        rebinding_receipt_for_entry(&planarized_entry, "curved-pressure-planarized")
            .expect("planarized decision");
    let direct_planarized = rebind_surface_on_face(
        anchored_surface_prior_fact_from_declaration(&prior, "curved-pressure-direct-prior"),
        replacement_neighborhood(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-curved",
            vec![anchored_surface_candidate_from_declaration(
                "planarized",
                &planarized,
                "curved-direct-planarized",
            )
            .expect("candidate")],
        ),
    )
    .expect("direct receipt");

    assert_eq!(
        admitted_face_surface_point_anchor_identity(&prior),
        admitted_face_surface_point_anchor_identity(&preserved)
    );
    assert_eq!(
        curved_decision.outcome_class(),
        RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        planarized_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(
        direct_planarized.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_ne!(
        curved_decision.selected_candidate_identity(),
        planarized_decision.selected_candidate_identity()
    );
    assert_eq!(
        planarized_decision.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_workflow_artifact_parity(
        &curved_workflow,
        &handle,
        curved_progression.clone(),
        curved_entry.clone(),
    );
    assert_eq!(
        canonical_text_entries_for_rebinding(&curved_entry)
            .get("rebinding.neighborhood.family")
            .map(String::as_str),
        Some("face_surface_point_anchor")
    );
    assert_eq!(
        Some(curved_progression.progression_digest()),
        curved_inspection.progression_digest()
    );
    assert!(matches!(
        curved_outcome,
        ForgeQueryOrdinaryOutcome::Bound(_)
    ));
}

#[test]
fn curved_binding_and_rebinding_do_not_fall_back_to_planarized_identity_or_domain_assumptions() {
    let prior = anchored_face_binding_declaration(
        "face-periodic",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let canonical = anchored_face_binding_declaration(
        "face-periodic",
        ParameterDomain::cylinder(),
        [0.25, 0.5],
    );
    let planarized =
        anchored_face_binding_declaration("face-periodic", ParameterDomain::plane(), [0.25, 0.5]);
    let trimmed_a = trimmed_face_binding_declaration(
        "face-trimmed",
        [[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]],
        [1.0, 1.0],
    );
    let trimmed_b = trimmed_face_binding_declaration(
        "face-trimmed",
        [[0.5, 0.0], [3.0, 0.0], [3.0, 3.0], [0.5, 3.0]],
        [1.0, 1.0],
    );

    let curved_neighborhood = replacement_neighborhood(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-periodic",
        vec![anchored_surface_candidate_from_declaration(
            "canonical",
            &canonical,
            "curved-canonical-candidate",
        )
        .expect("candidate")],
    );
    let planarized_neighborhood = replacement_neighborhood(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-periodic",
        vec![anchored_surface_candidate_from_declaration(
            "planarized",
            &planarized,
            "curved-planarized-candidate-two",
        )
        .expect("candidate")],
    );

    let curved_continuity = continuity_class_for_surface_rebinding(&prior, curved_neighborhood);
    let planarized_continuity =
        continuity_class_for_surface_rebinding(&prior, planarized_neighborhood);
    let trimmed_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&trimmed_a, "curved-trimmed-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-trimmed",
                vec![anchored_surface_candidate_from_declaration(
                    "trimmed-b",
                    &trimmed_b,
                    "curved-trimmed-candidate",
                )
                .expect("candidate")],
            ),
        ),
    );
    let trimmed_handle = admitted_rebinding_handle("trimmed-curved-pressure");
    let trimmed_progression = progress_rebinding_entry(&trimmed_entry, &trimmed_handle);
    let trimmed_inspection =
        inspect_progressed_rebinding_entry(&trimmed_handle, trimmed_progression.clone());
    let trimmed_outcome = rebinding_ordinary_outcome_for_entry(&trimmed_entry, &trimmed_handle);
    let trimmed_decision = rebinding_receipt_for_entry(&trimmed_entry, "curved-pressure-trimmed")
        .expect("trimmed decision");
    let denied = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-sphere", ParameterDomain::sphere())
            .expect("ownership"),
        ParameterSpacePoint::try_new([0.25, PI]).expect("parameter"),
    )
    .expect_err("sphere latitude outside admitted domain should deny");

    assert_eq!(
        admitted_face_surface_point_anchor_identity(&prior),
        admitted_face_surface_point_anchor_identity(&canonical)
    );
    assert_ne!(
        admitted_face_surface_point_anchor_identity(&prior),
        admitted_face_surface_point_anchor_identity(&planarized)
    );
    assert_ne!(
        admitted_face_surface_point_anchor_identity(&trimmed_a),
        admitted_face_surface_point_anchor_identity(&trimmed_b)
    );
    assert_eq!(curved_continuity, BindingContinuityClass::Exact);
    assert_eq!(
        planarized_continuity,
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        trimmed_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(
        trimmed_decision.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        Some(trimmed_progression.progression_digest()),
        trimmed_inspection.progression_digest()
    );
    assert!(matches!(
        trimmed_outcome,
        ForgeQueryOrdinaryOutcome::Bound(_)
    ));
    assert!(matches!(
        denied,
        SpatialAnchorAuthorityError::ParameterDomainViolation(_)
    ));
}

fn continuity_class_for_surface_rebinding(
    prior_binding: &PrimitiveAnchorBindingDeclarationEntry,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> BindingContinuityClass {
    let entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(prior_binding, "curved-surface-prior"),
            neighborhood,
        ),
    );
    rebinding_receipt_for_entry(&entry, "curved-pressure-surface")
        .expect("surface rebinding receipt")
        .continuity_class()
}
