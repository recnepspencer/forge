use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    evaluate_binding_motion_posture, rebind_surface_on_face, BindingMotionSemanticsInput,
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    MotionAwareBindingPosture, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use worth_spatial::facade::motion::{
    admit_spatial_move, admit_spatial_reorient, admit_spatial_rotate, SpatialMoveSpec,
    SpatialReorientSpec, SpatialRotateSpec,
};

use crate::spatial_intent::{MoveSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent};

use super::support::{canonical_geometry, orthotope_contract};

fn all_phase_six_bindings() -> [SpatialAdmittedPrimitiveBinding; 4] {
    [
        SpatialAdmittedPrimitiveBinding::FaceSurface(
            attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("face"),
        ),
        SpatialAdmittedPrimitiveBinding::EdgeCurve(
            attach_curve_to_edge(EdgeCurveBindingSpec::new(
                EdgeBindingSite::new("edge-1"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("edge"),
        ),
        SpatialAdmittedPrimitiveBinding::CoedgePCurve(
            attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                CoedgeBindingSite::new("coedge-1"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("coedge"),
        ),
        SpatialAdmittedPrimitiveBinding::VertexGeometry(
            attach_vertex_geometry(VertexGeometryBindingSpec::new(
                VertexBindingSite::new("vertex-1"),
                orthotope_contract(),
                canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            ))
            .expect("vertex"),
        ),
    ]
}

#[test]
fn motion_aware_binding_posture_distinguishes_preserved_transformed_invalidated_and_unresolved() {
    let direct_move =
        admit_spatial_move(SpatialMoveSpec::shape_origin().to([5.0, 0.0, 0.0])).expect("move");
    let kernel_move = MoveSpatialIntent::shape("shape-1")
        .to([5.0, 0.0, 0.0])
        .admit()
        .expect("kernel move");
    let direct_rotate =
        admit_spatial_rotate(SpatialRotateSpec::shape_origin().by_radians(0.0)).expect("rotate");
    let kernel_rotate = RotateSpatialIntent::shape("shape-1")
        .by_radians(0.0)
        .admit()
        .expect("kernel rotate");
    let direct_reorient =
        admit_spatial_reorient(SpatialReorientSpec::shape_origin().toward([0.0, 1.0, 0.0]))
            .expect("reorient");
    let kernel_reorient = ReorientSpatialIntent::shape("shape-1")
        .toward([0.0, 1.0, 0.0])
        .admit()
        .expect("kernel reorient");
    let bindings = all_phase_six_bindings();

    for binding in bindings {
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_move(&direct_move),
            )
            .expect("direct move posture"),
            MotionAwareBindingPosture::TransformedWithCarrier
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_move(&kernel_move),
            )
            .expect("kernel move posture"),
            MotionAwareBindingPosture::TransformedWithCarrier
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_rotate(&direct_rotate),
            )
            .expect("direct rotate posture"),
            MotionAwareBindingPosture::Preserved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_rotate(&kernel_rotate),
            )
            .expect("kernel rotate posture"),
            MotionAwareBindingPosture::Preserved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_reorient(&direct_reorient),
            )
            .expect("direct reorient posture"),
            MotionAwareBindingPosture::Unresolved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_reorient(&kernel_reorient),
            )
            .expect("kernel reorient posture"),
            MotionAwareBindingPosture::Unresolved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::invalidated_by_local_topology_replacement(),
            )
            .expect("invalidated"),
            MotionAwareBindingPosture::Invalidated
        );
    }
}

#[test]
fn motion_posture_is_not_rederived_from_rebinding_candidate_presence() {
    let prior = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let exact = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new-a"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("exact");
    let weaker = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new-b"),
        orthotope_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("weaker");
    let exact_only_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "exact",
            SpatialAdmittedPrimitiveBinding::FaceSurface(exact.clone()),
        )
        .expect("exact")])
        .expect("candidates"),
    )
    .expect("exact neighborhood");
    let rich_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new("exact", SpatialAdmittedPrimitiveBinding::FaceSurface(exact))
                .expect("exact"),
            ReplacementCandidate::new(
                "weaker",
                SpatialAdmittedPrimitiveBinding::FaceSurface(weaker),
            )
            .expect("weaker"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");
    let prior_binding = SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone());
    let direct_move =
        admit_spatial_move(SpatialMoveSpec::shape_origin().to([5.0, 0.0, 0.0])).expect("move");
    let motion_posture = evaluate_binding_motion_posture(
        &prior_binding,
        BindingMotionSemanticsInput::for_move(&direct_move),
    )
    .expect("motion posture");
    let exact_only_rebinding = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone()),
        exact_only_neighborhood,
    )
    .expect("exact rebinding");
    let rebinding = rebind_surface_on_face(prior_binding, rich_neighborhood).expect("rebinding");

    assert_eq!(
        motion_posture,
        MotionAwareBindingPosture::TransformedWithCarrier
    );
    assert_eq!(
        exact_only_rebinding.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        rebinding.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );
    assert!(rebinding
        .explanation()
        .selected_candidate_identity()
        .is_some());
    assert_eq!(
        exact_only_rebinding.explanation().motion_posture(),
        rebinding.explanation().motion_posture()
    );
    assert_ne!(rebinding.explanation().motion_posture(), &motion_posture);
}
