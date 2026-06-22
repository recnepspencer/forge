use super::super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_prior_fact_from_declaration, branch_local_rebinding_inspection,
    certification_bundle_for_pair, historical_rebinding_inspection,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    rebinding_receipt_for_entry, replacement_neighborhood, scoped_branch_head_inspection_basis,
    PrimitiveRebindingKernelQueryExt,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_rebinding_declaration,
    AnchorCarrierOwnership, AuthorPrimitiveAnchorBindingIntent,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    NeighborhoodBindingFamily, RebindingOutcomeClass, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};
use worth_spatial::facade::inspection::PrimitiveRebindingReplaySource;

#[test]
fn binding_layer_certification_bundle_matches_independently_derived_admitted_proofs() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-details-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-details-left-weaker",
                    )
                    .expect("weaker"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-details-left-exact",
                    )
                    .expect("exact"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "closeout-details-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "closeout-details-right-exact",
                    )
                    .expect("exact"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "closeout-details-right-weaker",
                    )
                    .expect("weaker"),
                ],
            ),
        ),
    );
    let bundle_handle = admitted_rebinding_handle("binding-closeout-details");
    let handle = admitted_rebinding_handle("binding-closeout-details");
    let branch_basis = scoped_branch_head_inspection_basis("branch:binding-closeout-details");
    let bundle = certification_bundle_for_pair(
        bundle_handle,
        branch_basis.clone(),
        left_entry.clone(),
        right_entry.clone(),
        "branch-evidence:left",
        "branch-evidence:right",
    );
    let left_decision =
        rebinding_receipt_for_entry(&left_entry, "closeout-details-left").expect("left decision");
    let right_historical = historical_rebinding_inspection(&right_entry, &handle);
    let left_branch_local = branch_local_rebinding_inspection(
        &left_entry,
        &handle,
        &branch_basis,
        "branch-evidence:left",
    );
    let replay = left_entry
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(
                historical_rebinding_inspection(&left_entry, &handle).retained_fact_receipt(),
            ),
            PrimitiveRebindingReplaySource::BranchLocal(
                branch_local_rebinding_inspection(
                    &right_entry,
                    &handle,
                    &branch_basis,
                    "branch-evidence:right",
                )
                .retained_fact_receipt(),
            ),
        )
        .expect("replay parity");

    assert_eq!(
        bundle.deterministic_outcome_class(),
        left_decision.outcome_class()
    );
    assert_eq!(
        bundle.deterministic_continuity_class(),
        left_decision.continuity_class()
    );
    assert_eq!(
        bundle.selected_candidate_identity(),
        left_decision.selected_candidate_identity()
    );
    assert_eq!(
        bundle.historical_digest(),
        right_historical.historical_digest()
    );
    assert_eq!(
        bundle.historical_inspection_digest(),
        right_historical.inspection().inspection_digest()
    );
    assert_eq!(
        bundle.branch_local_digest(),
        left_branch_local.branch_local_digest()
    );
    assert_eq!(
        bundle.branch_local_inspection_digest(),
        left_branch_local.inspection().inspection_digest()
    );
    assert_eq!(bundle.replay_digest(), replay.replay_digest());
    assert_eq!(bundle.replay_ordinary_kind(), replay.ordinary_kind());
    assert_eq!(replay.outcome_class(), RebindingOutcomeClass::Ambiguous);
}

#[test]
fn binding_layer_certification_bundle_matches_independently_derived_denied_replay_truth() {
    let left_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_binding_declaration("vertex-old"),
                "closeout-details-denied-left-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &vertex_binding_declaration("vertex-new-a"),
                        "closeout-details-denied-left-a",
                    )
                    .expect("a"),
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &vertex_binding_declaration("vertex-new-b"),
                        "closeout-details-denied-left-b",
                    )
                    .expect("b"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &vertex_binding_declaration("vertex-old"),
                "closeout-details-denied-right-prior",
            ),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    rebinding_candidate_from_binding_declaration(
                        "b",
                        &vertex_binding_declaration("vertex-new-b"),
                        "closeout-details-denied-right-b",
                    )
                    .expect("b"),
                    rebinding_candidate_from_binding_declaration(
                        "a",
                        &vertex_binding_declaration("vertex-new-a"),
                        "closeout-details-denied-right-a",
                    )
                    .expect("a"),
                ],
            ),
        ),
    );
    let bundle_handle = admitted_rebinding_handle("binding-closeout-denied-details");
    let handle = admitted_rebinding_handle("binding-closeout-denied-details");
    let branch_basis =
        scoped_branch_head_inspection_basis("branch:binding-closeout-denied-details");
    let bundle = certification_bundle_for_pair(
        bundle_handle,
        branch_basis.clone(),
        left_entry.clone(),
        right_entry.clone(),
        "branch-evidence:left",
        "branch-evidence:right",
    );
    let replay = left_entry
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(
                historical_rebinding_inspection(&left_entry, &handle).retained_fact_receipt(),
            ),
            PrimitiveRebindingReplaySource::BranchLocal(
                branch_local_rebinding_inspection(
                    &right_entry,
                    &handle,
                    &branch_basis,
                    "branch-evidence:right",
                )
                .retained_fact_receipt(),
            ),
        )
        .expect("replay parity");

    assert_eq!(
        bundle.deterministic_outcome_class(),
        RebindingOutcomeClass::Unsupported
    );
    assert_eq!(bundle.replay_ordinary_kind(), "unsupported");
    assert_eq!(bundle.replay_digest(), replay.replay_digest());
    assert_eq!(replay.outcome_class(), RebindingOutcomeClass::Unsupported);
    assert_eq!(
        replay.next_step(),
        Some(forge_query::facade::ForgeQueryOrdinaryNextStep::CheckSupport)
    );
}

fn vertex_binding_declaration(
    vertex_id: &str,
) -> worth_spatial::facade::bindings::PrimitiveBindingDeclarationEntry {
    worth_spatial::facade::bindings::author_primitive_binding_declaration(
        worth_spatial::facade::bindings::AuthorPrimitiveBindingIntent::attach_vertex_geometry(
            VertexGeometryBindingSpec::new(
                VertexBindingSite::new(vertex_id),
                PrimitiveConstructionFamilyContractRegistry::contract_for(
                    &PrimitiveWitnessDescriptor::Orthotope,
                ),
                super::super::support::canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            ),
        ),
    )
}

fn anchored_surface_declaration(
    face_identity: &str,
    persistent_name: &str,
    parameter: [f64; 2],
    extent: f64,
) -> worth_spatial::facade::bindings::PrimitiveAnchorBindingDeclarationEntry {
    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_identity).with_persistent_name(persistent_name),
                crate::binding::tests::support::orthotope_contract(),
                crate::binding::tests::support::canonical_geometry([
                    [0.0, 0.0, 0.0],
                    [extent, 0.0, 0.0],
                ]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(
                    face_identity,
                    worth_geom::facade::ParameterDomain::plane(),
                )
                .expect("ownership"),
                worth_geom::facade::ParameterSpacePoint::try_new(parameter).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}
