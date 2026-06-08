use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_parameter_space_point_to_face, attach_pcurve_to_coedge,
    attach_surface_to_face, attach_vertex_geometry, AnchorCarrierOwnership,
    CarrierOwnedParameterPointAnchorSpec, CoedgeBindingSite, CoedgePCurveBindingSpec,
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, RebindingOutcomeClass,
    ReplacementCandidate, ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding,
    UnsupportedRebindingReason, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{canonical_geometry, orthotope_contract, shell_with_hole_contract};

fn face_point_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
    point: [f64; 2],
) -> worth_spatial::facade::bindings::AdmittedFaceSurfacePointAnchorBinding {
    attach_parameter_space_point_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ),
        CarrierOwnedParameterPointAnchorSpec::new(
            AnchorCarrierOwnership::for_face_surface(face_id, ParameterDomain::plane())
                .expect("ownership"),
            ParameterSpacePoint::try_new(point).expect("parameter"),
        )
        .expect("anchor spec"),
    )
    .expect("face point binding")
}

fn face_surface_binding(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::FaceSurface(
        attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ))
        .expect("face surface binding"),
    )
}

#[test]
fn typed_rebinding_outcomes_remain_distinct_under_equivalent_candidate_pressure() {
    let prior = face_surface_binding(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let exact = face_surface_binding(
        "face-exact",
        "surface-beta",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let successor = face_surface_binding(
        "face-successor",
        "surface-gamma",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let prior_point = face_point_binding(
        "face-anchor-old",
        "surface-anchor-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        [0.25, 0.5],
    );
    let correspondence = face_point_binding(
        "face-correspondence",
        "surface-delta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [0.5, 0.5],
    );
    let preserved = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "preserved",
                    prior.clone(),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("preserved");
    let exact = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![
                    ReplacementCandidate::new("exact", exact).expect("candidate")
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("exact");
    let successor = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            prior.clone(),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    successor,
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("successor");
    let correspondence = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior_point),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-anchor-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "correspondence",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(correspondence),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("correspondence");
    let ambiguous_prior = attach_curve_to_edge(EdgeCurveBindingSpec::new(
        EdgeBindingSite::new("edge-old"),
        shell_with_hole_contract(),
        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
    ))
    .expect("ambiguous prior");
    let ambiguous = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_curve_binding(
            SpatialAdmittedPrimitiveBinding::EdgeCurve(ambiguous_prior),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::EdgeCurve,
                "edge-old",
                ReplacementCandidateSet::new(vec![
                    ReplacementCandidate::new(
                        "a",
                        SpatialAdmittedPrimitiveBinding::EdgeCurve(
                            attach_curve_to_edge(EdgeCurveBindingSpec::new(
                                EdgeBindingSite::new("edge-a"),
                                shell_with_hole_contract(),
                                canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                            ))
                            .expect("edge a"),
                        ),
                    )
                    .expect("a"),
                    ReplacementCandidate::new(
                        "b",
                        SpatialAdmittedPrimitiveBinding::EdgeCurve(
                            attach_curve_to_edge(EdgeCurveBindingSpec::new(
                                EdgeBindingSite::new("edge-b"),
                                shell_with_hole_contract(),
                                canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                            ))
                            .expect("edge b"),
                        ),
                    )
                    .expect("b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("ambiguous");
    let orphaned = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_pcurve_binding(
            SpatialAdmittedPrimitiveBinding::CoedgePCurve(
                attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                    CoedgeBindingSite::new("coedge-old"),
                    shell_with_hole_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ))
                .expect("prior"),
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "weak",
                    SpatialAdmittedPrimitiveBinding::CoedgePCurve(
                        attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                            CoedgeBindingSite::new("coedge-new"),
                            orthotope_contract(),
                            canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
                        ))
                        .expect("weak"),
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("orphaned");
    let unsupported = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::VertexGeometry(
                attach_vertex_geometry(VertexGeometryBindingSpec::new(
                    VertexBindingSite::new("vertex-old"),
                    PrimitiveConstructionFamilyContractRegistry::contract_for(
                        &PrimitiveWitnessDescriptor::Orthotope,
                    ),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                    VertexGeometryProvenanceKind::CanonicalWitness,
                    VertexToleranceRegime::ExactBits,
                ))
                .expect("vertex prior"),
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "vertex-successor",
                    SpatialAdmittedPrimitiveBinding::VertexGeometry(
                        attach_vertex_geometry(VertexGeometryBindingSpec::new(
                            VertexBindingSite::new("vertex-new"),
                            PrimitiveConstructionFamilyContractRegistry::contract_for(
                                &PrimitiveWitnessDescriptor::Orthotope,
                            ),
                            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                            VertexGeometryProvenanceKind::CanonicalWitness,
                            VertexToleranceRegime::ExactBits,
                        ))
                        .expect("vertex successor"),
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    )
    .admit()
    .expect("unsupported");

    assert_eq!(preserved.outcome_class(), RebindingOutcomeClass::Preserved);
    assert_eq!(
        exact.outcome_class(),
        RebindingOutcomeClass::ExactReattachment
    );
    assert_eq!(
        successor.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert_eq!(
        correspondence.outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_eq!(ambiguous.outcome_class(), RebindingOutcomeClass::Ambiguous);
    assert_eq!(orphaned.outcome_class(), RebindingOutcomeClass::Orphaned);
    assert_eq!(
        unsupported.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert_eq!(
        unsupported.explanation().unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
}
