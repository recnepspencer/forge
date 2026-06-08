#![cfg(test)]

use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
    PrimitiveWitnessDescriptor,
};

use crate::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    evaluate_binding_motion_posture, rebind_surface_on_face, BindingMotionSemanticsInput,
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    MotionAwareBindingPosture, NeighborhoodBindingFamily, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use crate::facade::motion::{
    admit_spatial_move, admit_spatial_reorient, admit_spatial_rotate, SpatialMoveSpec,
    SpatialReorientSpec, SpatialRotateSpec,
};

fn plane_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        vec![PrimitiveSupportPlaneIdentity::new(
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "0".to_string(),
        )],
        vertices
            .into_iter()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn all_phase_six_bindings(
    contract: PrimitiveConstructionBirthSynopsisContract,
) -> [SpatialAdmittedPrimitiveBinding; 4] {
    [
        SpatialAdmittedPrimitiveBinding::FaceSurface(
            attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("face"),
        ),
        SpatialAdmittedPrimitiveBinding::EdgeCurve(
            attach_curve_to_edge(EdgeCurveBindingSpec::new(
                EdgeBindingSite::new("edge-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("edge"),
        ),
        SpatialAdmittedPrimitiveBinding::CoedgePCurve(
            attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                CoedgeBindingSite::new("coedge-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ))
            .expect("coedge"),
        ),
        SpatialAdmittedPrimitiveBinding::VertexGeometry(
            attach_vertex_geometry(VertexGeometryBindingSpec::new(
                VertexBindingSite::new("vertex-1"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            ))
            .expect("vertex"),
        ),
    ]
}

#[test]
fn motion_aware_binding_posture_distinguishes_preserved_transformed_invalidated_and_unresolved() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let preserved_rotate =
        admit_spatial_rotate(SpatialRotateSpec::shape_origin().by_radians(0.0)).expect("rotate");
    let transformed_move =
        admit_spatial_move(SpatialMoveSpec::shape_origin().to([5.0, 0.0, 0.0])).expect("move");
    let unresolved_reorient =
        admit_spatial_reorient(SpatialReorientSpec::shape_origin().toward([0.0, 1.0, 0.0]))
            .expect("reorient");
    let bindings = all_phase_six_bindings(contract);

    for binding in bindings {
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_rotate(&preserved_rotate),
            )
            .expect("preserved"),
            MotionAwareBindingPosture::Preserved
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_move(&transformed_move),
            )
            .expect("transformed"),
            MotionAwareBindingPosture::TransformedWithCarrier
        );
        assert_eq!(
            evaluate_binding_motion_posture(
                &binding,
                BindingMotionSemanticsInput::for_reorient(&unresolved_reorient),
            )
            .expect("unresolved"),
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
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-old"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("prior");
    let successor = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-new"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("successor");
    let alternate = attach_surface_to_face(FaceSurfaceBindingSpec::new(
        FaceBindingSite::new("face-alt"),
        contract,
        plane_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
    ))
    .expect("alternate");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
            "successor",
            SpatialAdmittedPrimitiveBinding::FaceSurface(successor),
        )
        .expect("candidate")])
        .expect("candidate set"),
    )
    .expect("neighborhood");
    let richer_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            ReplacementCandidate::new(
                "successor",
                SpatialAdmittedPrimitiveBinding::FaceSurface(
                    attach_surface_to_face(FaceSurfaceBindingSpec::new(
                        FaceBindingSite::new("face-new-rich"),
                        contract,
                        plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                    ))
                    .expect("successor-rich"),
                ),
            )
            .expect("candidate"),
            ReplacementCandidate::new(
                "alternate",
                SpatialAdmittedPrimitiveBinding::FaceSurface(alternate),
            )
            .expect("alternate"),
        ])
        .expect("candidate set"),
    )
    .expect("richer neighborhood");
    let prior_binding = SpatialAdmittedPrimitiveBinding::FaceSurface(prior.clone());
    let invalidated = evaluate_binding_motion_posture(
        &prior_binding,
        BindingMotionSemanticsInput::invalidated_by_local_topology_replacement(),
    )
    .expect("invalidated");
    let rebinding = rebind_surface_on_face(prior_binding, neighborhood).expect("rebinding");
    let richer_rebinding = rebind_surface_on_face(
        SpatialAdmittedPrimitiveBinding::FaceSurface(prior),
        richer_neighborhood,
    )
    .expect("richer rebinding");

    assert_eq!(invalidated, MotionAwareBindingPosture::Invalidated);
    assert_eq!(
        rebinding.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );
    assert!(rebinding
        .explanation()
        .selected_candidate_identity()
        .is_some());
    assert_eq!(
        richer_rebinding.explanation().motion_posture(),
        &MotionAwareBindingPosture::Unresolved
    );
    assert_ne!(rebinding.explanation().motion_posture(), &invalidated);
    assert_eq!(
        rebinding.explanation().motion_posture(),
        richer_rebinding.explanation().motion_posture()
    );
}
