use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};
use worth_spatial::facade::bindings::{
    attach_vertex_geometry, NeighborhoodBindingFamily, RebindingOutcomeClass, ReplacementCandidate,
    SpatialAdmittedPrimitiveBinding, VertexBindingSite, VertexGeometryBindingSpec,
    VertexGeometryProvenanceKind, VertexToleranceRegime,
};

use crate::{
    binding::rebinding::PrimitiveRebindingReplaySource,
    facade::authoring::binding::{
        author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    },
};

use super::super::support::{
    admitted_rebinding_handle, anchored_surface, branch_local_rebinding_inspection,
    certification_bundle_for_pair, historical_rebinding_inspection, replacement_neighborhood,
    scoped_branch_head_inspection_basis,
};

#[test]
fn binding_layer_certification_bundle_matches_independently_derived_admitted_proofs() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
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
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact),
                    )
                    .expect("exact"),
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker),
                    )
                    .expect("weaker"),
                ],
            ),
        ),
    );
    let bundle_handle = admitted_rebinding_handle("phase-sixteen-closeout-details");
    let handle = admitted_rebinding_handle("phase-sixteen-closeout-details");
    let branch_basis = scoped_branch_head_inspection_basis("branch:phase-sixteen-closeout-details");
    let bundle = certification_bundle_for_pair(
        bundle_handle,
        branch_basis.clone(),
        left_entry.clone(),
        right_entry.clone(),
        "branch-evidence:left",
        "branch-evidence:right",
    );
    let left_decision = left_entry.clone().admit().expect("left decision");
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
            PrimitiveRebindingReplaySource::Historical(historical_rebinding_inspection(
                &left_entry,
                &handle,
            )),
            &right_entry,
            PrimitiveRebindingReplaySource::BranchLocal(branch_local_rebinding_inspection(
                &right_entry,
                &handle,
                &branch_basis,
                "branch-evidence:right",
            )),
        )
        .expect("replay parity");

    assert_eq!(
        bundle.deterministic_outcome_class(),
        left_decision.outcome_class()
    );
    assert_eq!(
        bundle.deterministic_continuity_class(),
        left_decision.explanation().continuity_class()
    );
    assert_eq!(
        bundle.selected_candidate_identity(),
        left_decision.explanation().selected_candidate_identity()
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
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            vertex_binding("vertex-old"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    ReplacementCandidate::new("a", vertex_binding("vertex-new-a")).expect("a"),
                    ReplacementCandidate::new("b", vertex_binding("vertex-new-b")).expect("b"),
                ],
            ),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            vertex_binding("vertex-old"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                vec![
                    ReplacementCandidate::new("b", vertex_binding("vertex-new-b")).expect("b"),
                    ReplacementCandidate::new("a", vertex_binding("vertex-new-a")).expect("a"),
                ],
            ),
        ),
    );
    let bundle_handle = admitted_rebinding_handle("phase-sixteen-closeout-denied-details");
    let handle = admitted_rebinding_handle("phase-sixteen-closeout-denied-details");
    let branch_basis =
        scoped_branch_head_inspection_basis("branch:phase-sixteen-closeout-denied-details");
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
            PrimitiveRebindingReplaySource::Historical(historical_rebinding_inspection(
                &left_entry,
                &handle,
            )),
            &right_entry,
            PrimitiveRebindingReplaySource::BranchLocal(branch_local_rebinding_inspection(
                &right_entry,
                &handle,
                &branch_basis,
                "branch-evidence:right",
            )),
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

fn vertex_binding(vertex_id: &str) -> SpatialAdmittedPrimitiveBinding {
    SpatialAdmittedPrimitiveBinding::VertexGeometry(
        attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new(vertex_id),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            super::super::support::canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ))
        .expect("vertex geometry binding"),
    )
}
