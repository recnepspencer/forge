use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveGeometryIdentityBundle, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_binding_declaration,
    author_primitive_rebinding_declaration, AuthorPrimitiveAnchorBindingIntent,
    AuthorPrimitiveBindingIntent, AuthorPrimitiveRebindingIntent, BindingContinuityClass,
    EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveAnchorBindingDeclarationEntry, PrimitiveBindingDeclarationEntry,
    PrimitiveRebindingPriorBindingFact, ReplacementCandidateSet,
};

use super::super::support::{
    canonical_geometry, orthotope_contract, rebinding_candidate_from_anchor_declaration,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_anchor_declaration,
    rebinding_prior_fact_from_binding_declaration, rebinding_receipt_for_entry,
    shell_with_hole_contract,
};

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

fn face_point_binding_declaration(
    face_id: &str,
    persistent_name: &str,
    vertices: [[f64; 3]; 2],
    point: [f64; 2],
) -> PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
                orthotope_contract(),
                canonical_geometry(vertices),
            ),
            worth_spatial::facade::bindings::CarrierOwnedParameterPointAnchorSpec::new(
                worth_spatial::facade::bindings::AnchorCarrierOwnership::for_face_surface(
                    face_id,
                    worth_geom::facade::ParameterDomain::plane(),
                )
                .expect("ownership"),
                worth_geom::facade::ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
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

#[test]
fn continuity_classification_distinguishes_authoritative_successor_correspondence_and_insufficient_evidence(
) {
    let prior_surface = face_surface_binding_declaration(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let successor_surface = face_surface_binding_declaration(
        "face-successor",
        "surface-beta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let successor_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
            "successor",
            &successor_surface,
            "continuity-successor-candidate",
        )
        .expect("successor candidate")])
        .expect("candidate set"),
    )
    .expect("successor neighborhood");
    let successor_continuity = continuity_class_for_surface_rebinding(
        rebinding_prior_fact_from_binding_declaration(&prior_surface, "continuity-successor-prior"),
        successor_neighborhood.clone(),
    );
    let kernel_successor = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior_surface,
                "continuity-kernel-successor-prior",
            ),
            successor_neighborhood,
        ),
    );
    let kernel_successor =
        rebinding_receipt_for_entry(&kernel_successor, "continuity-kernel-successor")
            .expect("kernel successor");

    let prior_anchor = face_point_binding_declaration(
        "face-anchor-old",
        "surface-anchor-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        [0.25, 0.5],
    );
    let correspondence_anchor = face_point_binding_declaration(
        "face-anchor-new",
        "surface-anchor-beta",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        [0.5, 0.5],
    );
    let correspondence_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-anchor-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_anchor_declaration(
            "correspondence",
            &correspondence_anchor,
            "continuity-correspondence-candidate",
        )
        .expect("correspondence candidate")])
        .expect("candidate set"),
    )
    .expect("correspondence neighborhood");
    let correspondence_continuity = continuity_class_for_surface_rebinding(
        rebinding_prior_fact_from_anchor_declaration(
            &prior_anchor,
            "continuity-correspondence-prior",
        ),
        correspondence_neighborhood.clone(),
    );
    let kernel_correspondence = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_anchor_declaration(
                &prior_anchor,
                "continuity-kernel-correspondence-prior",
            ),
            correspondence_neighborhood,
        ),
    );
    let kernel_correspondence =
        rebinding_receipt_for_entry(&kernel_correspondence, "continuity-kernel-correspondence")
            .expect("kernel correspondence");

    let prior_edge = edge_curve_binding_declaration(
        "edge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let partial_edge = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-partial"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            PrimitiveGeometryIdentityBundle::new(
                vec![],
                vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
            ),
        )),
    );
    let admitted_partial_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
            "partial",
            &partial_edge,
            "continuity-partial-candidate",
        )
        .expect("partial candidate")])
        .expect("candidate set"),
    )
    .expect("partial neighborhood");
    let admitted_partial_continuity = continuity_class_for_curve_rebinding(
        rebinding_prior_fact_from_binding_declaration(&prior_edge, "continuity-partial-prior"),
        admitted_partial_neighborhood,
    );

    let denied_edge = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-denied"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
        )),
    );
    let denied_incomplete_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
            "denied",
            &denied_edge,
            "continuity-denied-candidate",
        )
        .expect("denied candidate")])
        .expect("candidate set"),
    )
    .expect("denied neighborhood");
    let denied_incomplete_continuity = continuity_class_for_curve_rebinding(
        rebinding_prior_fact_from_binding_declaration(&prior_edge, "continuity-denied-prior"),
        denied_incomplete_neighborhood,
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
    let ambiguous_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::EdgeCurve,
        "edge-old",
        ReplacementCandidateSet::new(vec![
            rebinding_candidate_from_binding_declaration(
                "a",
                &ambiguous_a,
                "continuity-ambiguous-a",
            )
            .expect("candidate a"),
            rebinding_candidate_from_binding_declaration(
                "b",
                &ambiguous_b,
                "continuity-ambiguous-b",
            )
            .expect("candidate b"),
        ])
        .expect("candidate set"),
    )
    .expect("ambiguous neighborhood");
    let ambiguous_continuity = continuity_class_for_curve_rebinding(
        rebinding_prior_fact_from_binding_declaration(&prior_edge, "continuity-ambiguous-prior"),
        ambiguous_neighborhood,
    );

    assert_eq!(
        successor_continuity,
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(
        kernel_successor.continuity_class(),
        BindingContinuityClass::AuthoritativeSuccessor
    );
    assert_eq!(
        correspondence_continuity,
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        kernel_correspondence.continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
    assert_eq!(
        admitted_partial_continuity,
        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
    );
    assert_eq!(
        denied_incomplete_continuity,
        BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
    );
    assert_eq!(ambiguous_continuity, BindingContinuityClass::Ambiguous);
}

fn continuity_class_for_surface_rebinding(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> BindingContinuityClass {
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(prior_binding, neighborhood),
    );
    rebinding_receipt_for_entry(&entry, "continuity-surface")
        .expect("surface rebinding receipt")
        .continuity_class()
}

fn continuity_class_for_curve_rebinding(
    prior_binding: PrimitiveRebindingPriorBindingFact,
    neighborhood: LocalTopologyReplacementNeighborhood,
) -> BindingContinuityClass {
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_curve_binding(prior_binding, neighborhood),
    );
    rebinding_receipt_for_entry(&entry, "continuity-curve")
        .expect("curve rebinding receipt")
        .continuity_class()
}
