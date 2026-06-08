use std::f64::consts::{PI, TAU};

use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};
use worth_spatial::facade::bindings::{
    attach_parameter_space_point_to_face, evaluate_continuity, rebind_surface_on_face,
    AnchorCarrierOwnership, BindingContinuityClass, CarrierOwnedParameterPointAnchorSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, SpatialAnchorAuthorityError,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{
    admitted_rebinding_handle, assert_workflow_artifact_parity, canonical_geometry,
    canonical_text_entries_for_rebinding, inspect_progressed_rebinding_entry, orthotope_contract,
    progress_rebinding_entry, rebinding_workflow_artifacts,
};

fn anchored_face_binding(
    face_id: &str,
    domain: ParameterDomain,
    point: [f64; 2],
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
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
        )
        .expect("point anchor binding"),
    )
}

fn trimmed_face_binding(
    face_id: &str,
    outer_boundary: [[f64; 2]; 4],
    point: [f64; 2],
) -> SpatialAdmittedPrimitiveBinding {
    let trimmed_region = PolygonalTrimmedParameterRegion::new(
        ParameterDomain::plane(),
        outer_boundary
            .into_iter()
            .map(|coords| ParameterSpacePoint::try_new(coords).expect("boundary point"))
            .collect(),
        vec![],
    )
    .expect("trimmed region");
    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(
        attach_parameter_space_point_to_face(
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
        )
        .expect("point anchor binding"),
    )
}

#[test]
fn curved_carrier_pressure_breaks_planar_anchor_and_rebinding_shortcuts() {
    let handle = admitted_rebinding_handle("curved-rebinding-pressure");
    let prior = anchored_face_binding(
        "face-curved",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let preserved = anchored_face_binding("face-curved", ParameterDomain::cylinder(), [0.25, 0.5]);
    let planarized = anchored_face_binding("face-curved", ParameterDomain::plane(), [0.25, 0.5]);

    let curved_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "curved",
                    preserved.clone(),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("curved neighborhood"),
        ),
    );
    let curved_progression = progress_rebinding_entry(&curved_entry, &handle);
    let curved_inspection = inspect_progressed_rebinding_entry(&handle, curved_progression.clone());
    let curved_workflow = rebinding_workflow_artifacts(&curved_entry, &handle);
    let curved_decision = curved_entry.clone().admit().expect("curved decision");
    let curved_outcome = curved_entry.ordinary_outcome_with_query(&handle);

    let planarized_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "planarized",
                    planarized.clone(),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("planarized neighborhood"),
        ),
    );
    let planarized_decision = planarized_entry
        .clone()
        .admit()
        .expect("planarized decision");
    let direct_planarized = rebind_surface_on_face(
        prior.clone(),
        LocalTopologyReplacementNeighborhood::new(
            NeighborhoodBindingFamily::FaceSurfacePointAnchor,
            "face-curved",
            ReplacementCandidateSet::new(vec![
                ReplacementCandidate::new("planarized", planarized).expect("candidate")
            ])
            .expect("candidate set"),
        )
        .expect("planarized neighborhood"),
    )
    .expect("direct decision");

    assert_eq!(prior.identity(), preserved.identity());
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
        curved_decision.selected_binding().unwrap().identity(),
        planarized_decision.selected_binding().unwrap().identity()
    );
    assert_eq!(
        planarized_decision.explanation().continuity_class(),
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
            .get("neighborhood_family")
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
    let prior = anchored_face_binding(
        "face-periodic",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
    );
    let canonical =
        anchored_face_binding("face-periodic", ParameterDomain::cylinder(), [0.25, 0.5]);
    let planarized = anchored_face_binding("face-periodic", ParameterDomain::plane(), [0.25, 0.5]);
    let trimmed_a = trimmed_face_binding(
        "face-trimmed",
        [[0.0, 0.0], [3.0, 0.0], [3.0, 3.0], [0.0, 3.0]],
        [1.0, 1.0],
    );
    let trimmed_b = trimmed_face_binding(
        "face-trimmed",
        [[0.5, 0.0], [3.0, 0.0], [3.0, 3.0], [0.5, 3.0]],
        [1.0, 1.0],
    );

    let curved_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-periodic",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "canonical",
            canonical.clone(),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("curved neighborhood");
    let planarized_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-periodic",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "planarized",
            planarized.clone(),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("planarized neighborhood");

    let curved_continuity =
        evaluate_continuity(&prior, &curved_neighborhood).expect("curved continuity");
    let planarized_continuity =
        evaluate_continuity(&prior, &planarized_neighborhood).expect("planarized continuity");
    let trimmed_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            trimmed_a.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-trimmed",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "trimmed-b",
                    trimmed_b.clone(),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("trimmed neighborhood"),
        ),
    );
    let trimmed_handle = admitted_rebinding_handle("trimmed-curved-pressure");
    let trimmed_progression = progress_rebinding_entry(&trimmed_entry, &trimmed_handle);
    let trimmed_inspection =
        inspect_progressed_rebinding_entry(&trimmed_handle, trimmed_progression.clone());
    let trimmed_outcome = trimmed_entry.ordinary_outcome_with_query(&trimmed_handle);
    let trimmed_decision = trimmed_entry.clone().admit().expect("trimmed decision");
    let denied = CarrierOwnedParameterPointAnchorSpec::new(
        AnchorCarrierOwnership::for_face_surface("face-sphere", ParameterDomain::sphere())
            .expect("ownership"),
        ParameterSpacePoint::try_new([0.25, PI]).expect("parameter"),
    )
    .expect_err("sphere latitude outside admitted domain should deny");

    assert_eq!(prior.identity(), canonical.identity());
    assert_ne!(prior.identity(), planarized.identity());
    assert_ne!(trimmed_a.identity(), trimmed_b.identity());
    assert_eq!(
        curved_continuity.continuity_class(),
        BindingContinuityClass::Exact
    );
    assert_eq!(
        planarized_continuity.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        trimmed_decision.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(
        trimmed_decision.explanation().continuity_class(),
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
