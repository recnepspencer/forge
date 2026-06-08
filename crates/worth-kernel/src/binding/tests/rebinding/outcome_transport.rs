use forge_query::facade::ForgeQueryOrdinaryOutcome;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face, attach_vertex_geometry,
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood,
    NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    ReplacementCandidateSet, SpatialAdmittedPrimitiveBinding, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::facade::authoring::binding::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};

use super::super::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract, shell_with_hole_contract,
};

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
fn rebinding_outcome_transport_through_kernel_and_query_does_not_collapse_denial_shape() {
    let handle = admitted_rebinding_handle("rebinding-outcome-transport");

    let supported_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            face_surface_binding(
                "face-old",
                "surface-alpha",
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![ReplacementCandidate::new(
                    "successor",
                    face_surface_binding(
                        "face-new",
                        "surface-beta",
                        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let supported = supported_entry.clone().ordinary_outcome_with_query(&handle);
    let supported_decision = supported_entry.admit().expect("supported decision");

    let ambiguous_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_curve_binding(
            SpatialAdmittedPrimitiveBinding::EdgeCurve(
                attach_curve_to_edge(EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new("edge-old"),
                    shell_with_hole_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ))
                .expect("prior"),
            ),
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
                            .expect("a"),
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
                            .expect("b"),
                        ),
                    )
                    .expect("b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous = ambiguous_entry.clone().ordinary_outcome_with_query(&handle);
    let ambiguous_decision = ambiguous_entry.admit().expect("ambiguous decision");

    let orphaned_entry = author_primitive_rebinding_declaration(
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
    );
    let orphaned = orphaned_entry.clone().ordinary_outcome_with_query(&handle);
    let orphaned_decision = orphaned_entry.admit().expect("orphaned decision");

    let unsupported_entry = author_primitive_rebinding_declaration(
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
                .expect("prior"),
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
                        .expect("successor"),
                    ),
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported = unsupported_entry
        .clone()
        .ordinary_outcome_with_query(&handle);
    let unsupported_decision = unsupported_entry.admit().expect("unsupported decision");

    assert_eq!(
        supported_decision.outcome_class(),
        RebindingOutcomeClass::ContinuityJustifiedReattachment
    );
    assert_eq!(
        ambiguous_decision.outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
    assert_eq!(
        orphaned_decision.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_eq!(
        unsupported_decision.outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert!(matches!(supported, ForgeQueryOrdinaryOutcome::Bound(_)));
    assert!(matches!(ambiguous, ForgeQueryOrdinaryOutcome::Ambiguous(_)));
    assert!(matches!(
        orphaned,
        ForgeQueryOrdinaryOutcome::RebindRequired(_)
    ));
    assert!(matches!(
        unsupported,
        ForgeQueryOrdinaryOutcome::Unsupported(_)
    ));
}
