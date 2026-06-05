use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_parameter_space_point_to_face, attach_pcurve_to_coedge,
    rebind_curve_on_edge, rebind_pcurve_on_coedge, rebind_surface_on_face, AnchorCarrierOwnership,
    CarrierOwnedParameterPointAnchorSpec, CoedgeBindingSite, CoedgePCurveBindingSpec,
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass, ReplacementCandidate, ReplacementCandidateSet,
    SpatialAdmittedPrimitiveBinding,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::support::{
    admitted_rebinding_handle, canonical_geometry, canonical_text_entries_for_rebinding,
    inspect_progressed_rebinding_entry, orthotope_contract, progress_rebinding_entry,
    rebinding_declaration_digest_string, shell_with_hole_contract,
};

#[test]
fn local_topology_replacement_rebinds_or_denies_canonically_under_replay() {
    let face_spec = FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old").with_persistent_name("surface-alpha"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    );
    let prior = attach_parameter_space_point_to_face(
        face_spec.clone(),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-old", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("prior");
    let exact = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new-a").with_persistent_name("surface-beta"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-new-a", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("exact");
    let weaker = attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new-b").with_persistent_name("surface-gamma"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface("face-new-b", ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new([0.25, 0.5]).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("weaker");

    let left_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "weaker",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
            )
            .expect("weaker"),
            ReplacementCandidate::new(
                "exact",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
            )
            .expect("exact"),
        ])
        .expect("candidate set"),
    )
    .expect("left neighborhood");
    let right_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "exact",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
            )
            .expect("exact"),
            ReplacementCandidate::new(
                "weaker",
                SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
            )
            .expect("weaker"),
        ])
        .expect("candidate set"),
    )
    .expect("right neighborhood");

    let left_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            left_neighborhood.clone(),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            right_neighborhood.clone(),
        ),
    );

    let handle = admitted_rebinding_handle("rebinding-replay");
    let left_progression = progress_rebinding_entry(&left_entry, &handle);
    let right_progression = progress_rebinding_entry(&right_entry, &handle);
    let left_inspection = inspect_progressed_rebinding_entry(&handle, left_progression.clone());

    assert_eq!(
        rebinding_declaration_digest_string(&left_progression),
        rebinding_declaration_digest_string(&right_progression)
    );
    let canonical = canonical_text_entries_for_rebinding(&left_entry);
    assert_eq!(
        canonical.get("neighborhood_family").map(String::as_str),
        Some("face_surface_point_anchor")
    );
    assert_eq!(
        canonical.get("candidate_count").map(String::as_str),
        Some("2")
    );
    assert_eq!(
        Some(left_progression.progression_digest()),
        left_inspection.progression_digest()
    );

    let kernel_decision = left_entry.admit().expect("kernel decision");
    let direct_decision = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
        right_neighborhood,
    )
    .expect("direct decision");

    assert_eq!(
        kernel_decision.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        kernel_decision.outcome_class(),
        direct_decision.outcome_class()
    );
    assert_eq!(
        kernel_decision.explanation().selected_candidate_identity(),
        direct_decision.explanation().selected_candidate_identity()
    );
    assert_eq!(
        kernel_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert_eq!(
        direct_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert_eq!(
        kernel_decision.explanation().neighborhood_family(),
        NeighborhoodBindingFamily::FaceSurfacePointAnchor
    );
    assert_eq!(
        kernel_decision.explanation().prior_site_identity(),
        "face-old"
    );
    assert_eq!(
        kernel_decision.explanation().candidate_labels(),
        ["weaker", "exact"]
    );
    assert_eq!(
        kernel_decision.explanation().candidate_site_identities(),
        ["face-new-b", "face-new-a"]
    );
}

#[test]
fn host_order_variation_does_not_change_rebinding_outcome_or_diagnostics() {
    let prior_edge = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-old"),
        shell_with_hole_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior edge");
    let ambiguous_a = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-a"),
        shell_with_hole_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("edge a");
    let ambiguous_b = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-b"),
        shell_with_hole_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("edge b");
    let first = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "a",
                SpatialAdmittedPrimitiveBinding::EdgeCurve(ambiguous_a.clone()),
            )
            .expect("a"),
            ReplacementCandidate::new(
                "b",
                SpatialAdmittedPrimitiveBinding::EdgeCurve(ambiguous_b.clone()),
            )
            .expect("b"),
        ])
        .expect("candidate set"),
    )
    .expect("first");
    let second = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new("b", SpatialAdmittedPrimitiveBinding::EdgeCurve(ambiguous_b))
                .expect("b"),
            ReplacementCandidate::new("a", SpatialAdmittedPrimitiveBinding::EdgeCurve(ambiguous_a))
                .expect("a"),
        ])
        .expect("candidate set"),
    )
    .expect("second");

    let first_decision = rebind_curve_on_edge(
        SpatialAdmittedPrimitiveBinding::EdgeCurve(prior_edge.clone()),
        first,
    )
    .expect("first decision");
    let second_decision = rebind_curve_on_edge(
        SpatialAdmittedPrimitiveBinding::EdgeCurve(prior_edge),
        second,
    )
    .expect("second decision");

    assert_eq!(
        first_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        first_decision.outcome_class(),
        second_decision.outcome_class()
    );
    assert_eq!(
        first_decision.explanation().continuity_class(),
        second_decision.explanation().continuity_class()
    );
    assert_eq!(
        first_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert_eq!(
        second_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert!(first_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
    assert!(second_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());

    let prior_coedge = attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
        CoedgeBindingSite::new("coedge-old"),
        shell_with_hole_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior coedge");
    let weak_candidate = attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
        CoedgeBindingSite::new("coedge-new"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("weak candidate");
    let weak_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::CoedgePCurve,
        "coedge-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "weak",
            SpatialAdmittedPrimitiveBinding::CoedgePCurve(weak_candidate),
        )
        .expect("weak")])
        .expect("candidate set"),
    )
    .expect("weak neighborhood");

    let weak_decision = rebind_pcurve_on_coedge(
        SpatialAdmittedPrimitiveBinding::CoedgePCurve(prior_coedge),
        weak_neighborhood,
    )
    .expect("weak decision");

    assert_eq!(
        weak_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_ne!(
        weak_decision.explanation().continuity_class(),
        first_decision.explanation().continuity_class()
    );
    assert_eq!(
        weak_decision.explanation().motion_posture(),
        &MotionAwareBindingPosture::RequiresRebinding
    );
    assert!(weak_decision
        .explanation()
        .selected_candidate_identity()
        .is_none());
}
