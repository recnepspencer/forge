use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_binding_declaration,
    author_primitive_rebinding_declaration, AnchorCarrierOwnership,
    AuthorPrimitiveAnchorBindingIntent, AuthorPrimitiveBindingIntent,
    CarrierOwnedParameterPointAnchorSpec, CoedgeBindingSite, CoedgePCurveBindingSpec,
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, RebindingOutcomeClass, ReplacementCandidateSet,
    UnsupportedRebindingReason, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::binding::tests::support::{
    canonical_geometry, orthotope_contract, rebinding_candidate_from_anchor_declaration,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_anchor_declaration,
    rebinding_prior_fact_from_binding_declaration, rebinding_receipt_for_entry,
    shell_with_hole_contract,
};

fn face_point_binding_declaration(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
    point: [f64; 2],
) -> worth_spatial::facade::bindings::PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
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
        ),
    )
}

fn face_surface_binding_declaration(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ),
    ))
}

fn edge_curve_binding_declaration(
    edge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: PrimitiveConstructionBirthSynopsisContract,
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_curve_to_edge(
        EdgeCurveBindingSpec::new(
            EdgeBindingSite::new(edge_id),
            contract,
            canonical_geometry(vertices),
        ),
    ))
}

fn coedge_pcurve_binding_declaration(
    coedge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: PrimitiveConstructionBirthSynopsisContract,
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(
        CoedgePCurveBindingSpec::new(
            CoedgeBindingSite::new(coedge_id),
            contract,
            canonical_geometry(vertices),
        ),
    ))
}

fn vertex_geometry_binding_declaration(
    vertex_id: &str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_vertex_geometry(
        VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            canonical_geometry(vertices),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ),
    ))
}

#[test]
fn typed_rebinding_outcomes_remain_distinct_under_equivalent_candidate_pressure() {
    let prior = face_surface_binding_declaration(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let exact = face_surface_binding_declaration(
        "face-exact",
        "surface-beta",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let successor = face_surface_binding_declaration(
        "face-successor",
        "surface-gamma",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let prior_point = face_point_binding_declaration(
        "face-anchor-old",
        "surface-anchor-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        [0.25, 0.5],
    );
    let correspondence = face_point_binding_declaration(
        "face-correspondence",
        "surface-delta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [0.5, 0.5],
    );
    let preserved = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "outcomes-preserved-prior"),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "preserved",
                    &prior,
                    "outcomes-preserved-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let preserved =
        rebinding_receipt_for_entry(&preserved, "outcomes-preserved").expect("preserved");
    let exact = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "outcomes-exact-prior"),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "exact",
                    &exact,
                    "outcomes-exact-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let exact = rebinding_receipt_for_entry(&exact, "outcomes-exact").expect("exact");
    let successor = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "outcomes-successor-prior"),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "successor",
                    &successor,
                    "outcomes-successor-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let successor =
        rebinding_receipt_for_entry(&successor, "outcomes-successor").expect("successor");
    let correspondence = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_anchor_declaration(
                &prior_point,
                "outcomes-correspondence-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-anchor-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_anchor_declaration(
                    "correspondence",
                    &correspondence,
                    "outcomes-correspondence-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let correspondence = rebinding_receipt_for_entry(&correspondence, "outcomes-correspondence")
        .expect("correspondence");
    let ambiguous_prior = edge_curve_binding_declaration(
        "edge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let ambiguous_a = edge_curve_binding_declaration(
        "edge-a",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let ambiguous_b = edge_curve_binding_declaration(
        "edge-b",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let ambiguous = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_curve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &ambiguous_prior,
                "outcomes-ambiguous-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::EdgeCurve,
                "edge-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &ambiguous_a,
                        "outcomes-ambiguous-a",
                    )
                    .expect("a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &ambiguous_b,
                        "outcomes-ambiguous-b",
                    )
                    .expect("b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous =
        rebinding_receipt_for_entry(&ambiguous, "outcomes-ambiguous").expect("ambiguous");
    let orphaned_prior = coedge_pcurve_binding_declaration(
        "coedge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let orphaned_candidate = coedge_pcurve_binding_declaration(
        "coedge-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        orthotope_contract(),
    );
    let orphaned = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_pcurve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &orphaned_prior,
                "outcomes-orphaned-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "weak",
                    &orphaned_candidate,
                    "outcomes-orphaned-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let orphaned = rebinding_receipt_for_entry(&orphaned, "outcomes-orphaned").expect("orphaned");
    let unsupported_prior =
        vertex_geometry_binding_declaration("vertex-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let unsupported_candidate =
        vertex_geometry_binding_declaration("vertex-new", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let unsupported = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &unsupported_prior,
                "outcomes-unsupported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "vertex-successor",
                    &unsupported_candidate,
                    "outcomes-unsupported-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported =
        rebinding_receipt_for_entry(&unsupported, "outcomes-unsupported").expect("unsupported");

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
        unsupported.unsupported_reason(),
        Some(
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: NeighborhoodBindingFamily::FaceSurface,
                actual: NeighborhoodBindingFamily::VertexGeometry,
            },
        )
    );
}
