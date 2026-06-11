use super::support::{
    canonical_geometry, orthotope_contract, rebind_surface_on_face_with_motion,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    rebinding_receipt_for_entry,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, BindingMotionSemanticsInput, CoedgeBindingSite,
    CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, MotionAwareBindingPosture,
    NeighborhoodBindingFamily, PrimitiveBindingDeclarationEntry, ReplacementCandidateSet,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};

fn all_phase_six_bindings() -> [(
    NeighborhoodBindingFamily,
    &'static str,
    PrimitiveBindingDeclarationEntry,
); 4] {
    [
        (
            NeighborhoodBindingFamily::FaceSurface,
            "face-1",
            author_primitive_binding_declaration(
                AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                    FaceBindingSite::new("face-1"),
                    orthotope_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                )),
            ),
        ),
        (
            NeighborhoodBindingFamily::EdgeCurve,
            "edge-1",
            author_primitive_binding_declaration(
                AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new("edge-1"),
                    orthotope_contract(),
                    canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                )),
            ),
        ),
        (
            NeighborhoodBindingFamily::CoedgePCurve,
            "coedge-1",
            author_primitive_binding_declaration(
                AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(
                    CoedgePCurveBindingSpec::new(
                        CoedgeBindingSite::new("coedge-1"),
                        orthotope_contract(),
                        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                    ),
                ),
            ),
        ),
        (
            NeighborhoodBindingFamily::VertexGeometry,
            "vertex-1",
            author_primitive_binding_declaration(
                AuthorPrimitiveBindingIntent::attach_vertex_geometry(
                    VertexGeometryBindingSpec::new(
                        VertexBindingSite::new("vertex-1"),
                        orthotope_contract(),
                        canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                        VertexGeometryProvenanceKind::CanonicalWitness,
                        VertexToleranceRegime::ExactBits,
                    ),
                ),
            ),
        ),
    ]
}

#[test]
fn motion_aware_binding_posture_distinguishes_preserved_transformed_invalidated_and_unresolved() {
    let bindings = all_phase_six_bindings();

    for (family, prior_site, declaration) in bindings {
        assert_eq!(
            motion_posture_from_declaration(
                family,
                prior_site,
                &declaration,
                "binding-motion-posture-moved",
                BindingMotionSemanticsInput::moved_with_carrier(),
            ),
            MotionAwareBindingPosture::TransformedWithCarrier
        );
        assert_eq!(
            motion_posture_from_declaration(
                family,
                prior_site,
                &declaration,
                "binding-motion-posture-rotated",
                BindingMotionSemanticsInput::rotated_with_carrier(0.0),
            ),
            MotionAwareBindingPosture::Preserved
        );
        assert_eq!(
            motion_posture_from_declaration(
                family,
                prior_site,
                &declaration,
                "binding-motion-posture-reoriented",
                BindingMotionSemanticsInput::reoriented_with_carrier(),
            ),
            MotionAwareBindingPosture::Unresolved
        );
        assert_eq!(
            motion_posture_from_declaration(
                family,
                prior_site,
                &declaration,
                "binding-motion-posture-invalidated",
                BindingMotionSemanticsInput::invalidated_by_local_topology_replacement(),
            ),
            MotionAwareBindingPosture::Invalidated
        );
    }
}

#[test]
fn motion_posture_is_not_rederived_from_rebinding_candidate_presence() {
    let prior_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-old"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let exact_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new-a"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let weaker_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-new-b"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]),
        )),
    );
    let exact_only_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
            "exact",
            &exact_declaration,
            "binding-motion-posture-exact-only",
        )
        .expect("exact")])
        .expect("candidates"),
    )
    .expect("exact neighborhood");
    let rich_neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        ReplacementCandidateSet::new(vec![
            rebinding_candidate_from_binding_declaration(
                "exact",
                &exact_declaration,
                "binding-motion-posture-rich-exact",
            )
            .expect("exact"),
            rebinding_candidate_from_binding_declaration(
                "weaker",
                &weaker_declaration,
                "binding-motion-posture-rich-weaker",
            )
            .expect("weaker"),
        ])
        .expect("candidates"),
    )
    .expect("neighborhood");
    let motion_posture = motion_posture_from_declaration(
        NeighborhoodBindingFamily::FaceSurface,
        "face-old",
        &prior_declaration,
        "binding-motion-posture-prior",
        BindingMotionSemanticsInput::moved_with_carrier(),
    );
    let exact_only_rebinding = rebind_surface_on_face_with_motion(
        rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "binding-motion-posture-exact-direct-prior",
        ),
        exact_only_neighborhood,
        BindingMotionSemanticsInput::moved_with_carrier(),
    )
    .expect("exact rebinding");
    let rebinding = rebind_surface_on_face_with_motion(
        rebinding_prior_fact_from_binding_declaration(
            &prior_declaration,
            "binding-motion-posture-direct-prior",
        ),
        rich_neighborhood,
        BindingMotionSemanticsInput::moved_with_carrier(),
    )
    .expect("rebinding");

    assert_eq!(
        motion_posture,
        MotionAwareBindingPosture::TransformedWithCarrier
    );
    assert_eq!(
        exact_only_rebinding.motion_posture(),
        MotionAwareBindingPosture::TransformedWithCarrier
    );
    assert_eq!(
        rebinding.motion_posture(),
        MotionAwareBindingPosture::TransformedWithCarrier
    );
    assert!(rebinding.selected_candidate_identity().is_some());
    assert_eq!(
        exact_only_rebinding.motion_posture(),
        rebinding.motion_posture()
    );
    assert_eq!(rebinding.motion_posture(), motion_posture);
}

fn motion_posture_from_declaration(
    family: NeighborhoodBindingFamily,
    prior_site: &str,
    prior_declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
    motion: BindingMotionSemanticsInput,
) -> MotionAwareBindingPosture {
    let neighborhood = self_candidate_neighborhood(family, prior_site, prior_declaration, world);
    let prior_fact = rebinding_prior_fact_from_binding_declaration(prior_declaration, world);
    let entry = match family {
        NeighborhoodBindingFamily::FaceSurface
        | NeighborhoodBindingFamily::FaceSurfacePointAnchor
        | NeighborhoodBindingFamily::FaceSurfaceDirectionAnchor => {
            author_primitive_rebinding_declaration(
                crate::binding::tests::support::replace_surface_binding_with_motion(
                    prior_fact,
                    neighborhood,
                    motion,
                ),
            )
        }
        NeighborhoodBindingFamily::EdgeCurve
        | NeighborhoodBindingFamily::EdgeCurvePointAnchor
        | NeighborhoodBindingFamily::EdgeCurveDirectionAnchor => {
            author_primitive_rebinding_declaration(
                crate::binding::tests::support::replace_curve_binding_with_motion(
                    prior_fact,
                    neighborhood,
                    motion,
                ),
            )
        }
        NeighborhoodBindingFamily::CoedgePCurve
        | NeighborhoodBindingFamily::CoedgePCurvePointAnchor
        | NeighborhoodBindingFamily::CoedgePCurveDirectionAnchor => {
            author_primitive_rebinding_declaration(
                crate::binding::tests::support::replace_pcurve_binding_with_motion(
                    prior_fact,
                    neighborhood,
                    motion,
                ),
            )
        }
        NeighborhoodBindingFamily::VertexGeometry => author_primitive_rebinding_declaration(
            crate::binding::tests::support::replace_geometry_binding_with_motion(
                prior_fact,
                neighborhood,
                motion,
            ),
        ),
    };
    rebinding_receipt_for_entry(&entry, "binding-motion-posture")
        .expect("decision receipt")
        .motion_posture()
}

fn self_candidate_neighborhood(
    family: NeighborhoodBindingFamily,
    prior_site: &str,
    declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
) -> LocalTopologyReplacementNeighborhood {
    LocalTopologyReplacementNeighborhood::new(
        family,
        prior_site,
        ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
            "self",
            declaration,
            world,
        )
        .expect("self candidate")])
        .expect("candidate set"),
    )
    .expect("self neighborhood")
}
