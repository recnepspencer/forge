use crate::binding::tests::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    shell_with_hole_contract,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    primitive_rebinding_retained_fact_source, AuthorPrimitiveBindingIntent,
    AuthorPrimitiveRebindingIntent, CoedgeBindingSite, CoedgePCurveBindingSpec, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, ReplacementCandidateSet, VertexBindingSite,
    VertexGeometryBindingSpec, VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use worth_spatial::facade::recovery::{
    geometry_recovery_action_entry, primitive_rebinding_geometry_recovery_action,
    GeometryRecoveryAction, GeometryRecoveryTargetScope,
};

fn surface_binding_declaration(
    face_id: &str,
    name: &str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(name),
            orthotope_contract(),
            canonical_geometry(vertices),
        ),
    ))
}

fn pcurve_binding_declaration(
    coedge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: worth_primitives::PrimitiveConstructionBirthSynopsisContract,
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(
        CoedgePCurveBindingSpec::new(
            CoedgeBindingSite::new(coedge_id),
            contract,
            canonical_geometry(vertices),
        ),
    ))
}

#[test]
fn denied_rebinding_workflows_publish_typed_recovery_actions() {
    let prior = surface_binding_declaration(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let a =
        surface_binding_declaration("face-a", "surface-beta", [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
    let b = surface_binding_declaration(
        "face-b",
        "surface-gamma",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let ambiguous = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(&prior, "recovery-ambiguous-prior"),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![
                    rebinding_candidate_from_binding_declaration("a", &a, "recovery-ambiguous-a")
                        .expect("candidate a"),
                    rebinding_candidate_from_binding_declaration("b", &b, "recovery-ambiguous-b")
                        .expect("candidate b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let ambiguous_handle = admitted_rebinding_handle("kernel-recovery-ambiguous");
    let ambiguous_receipt = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(
            primitive_rebinding_retained_fact_source(&ambiguous, &ambiguous_handle)
                .expect("retained fact source"),
        ),
        &ambiguous_handle,
    )
    .expect("ambiguous recovery");
    assert_eq!(
        ambiguous_receipt.recovery_action_kind(),
        GeometryRecoveryAction::NarrowInput
    );
    assert_eq!(
        ambiguous_receipt.recovery_target_scope(),
        GeometryRecoveryTargetScope::InputNarrowing
    );

    let orphaned_prior = pcurve_binding_declaration(
        "coedge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        shell_with_hole_contract(),
    );
    let orphaned_successor = pcurve_binding_declaration(
        "coedge-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        orthotope_contract(),
    );
    let orphaned = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_pcurve_binding(
            rebinding_prior_fact_from_binding_declaration(
                &orphaned_prior,
                "recovery-orphaned-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "orphaned",
                    &orphaned_successor,
                    "recovery-orphaned-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let orphaned_handle = admitted_rebinding_handle("kernel-recovery-orphaned");
    let orphaned_receipt = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(
            primitive_rebinding_retained_fact_source(&orphaned, &orphaned_handle)
                .expect("retained fact source"),
        ),
        &orphaned_handle,
    )
    .expect("orphaned recovery");
    assert_eq!(
        orphaned_receipt.recovery_action_kind(),
        GeometryRecoveryAction::RebindContext
    );
    assert_eq!(
        orphaned_receipt.recovery_target_scope(),
        GeometryRecoveryTargetScope::TruthContinuationContext
    );

    let vertex_prior = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-old"),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let unsupported = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_prior,
                "recovery-unsupported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "vertex",
                    &vertex_prior,
                    "recovery-unsupported-candidate",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported_handle = admitted_rebinding_handle("kernel-recovery-unsupported");
    let unsupported_receipt = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(
            primitive_rebinding_retained_fact_source(&unsupported, &unsupported_handle)
                .expect("retained fact source"),
        ),
        &unsupported_handle,
    )
    .expect("unsupported recovery");
    assert_eq!(
        unsupported_receipt.recovery_action_kind(),
        GeometryRecoveryAction::CheckSupport
    );
    assert_eq!(
        unsupported_receipt.recovery_target_scope(),
        GeometryRecoveryTargetScope::SupportReadiness
    );
}
