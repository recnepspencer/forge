use std::f64::consts::TAU;

use worth_geom::facade::ParameterDomain;
use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, primitive_rebinding_projection_facts,
    primitive_rebinding_retained_fact_source,
};

use super::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_declaration, anchored_surface_prior_fact_from_declaration,
    branch_local_rebinding_inspection, certification_bundle_for_pair,
    historical_rebinding_inspection, replacement_neighborhood, scoped_branch_head_inspection_basis,
};
use worth_spatial::facade::bindings::{NeighborhoodBindingFamily, RebindingOutcomeClass};
use worth_spatial::facade::inspection::{
    geometry_replay_parity_entry, PrimitiveRebindingReplaySource,
};
use worth_spatial::facade::projection::{
    geometry_projection_consumption_entry, primitive_rebinding_geometry_projection_consumption,
};

#[test]
fn geometry_runtime_story_stays_coherent_across_live_projection_historical_branch_and_replay() {
    let prior = anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "runtime-story-left-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "runtime-story-left-weaker",
                    )
                    .expect("weaker candidate"),
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "runtime-story-left-exact",
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "runtime-story-right-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    anchored_surface_candidate_from_declaration(
                        "exact",
                        &exact,
                        "runtime-story-right-exact",
                    )
                    .expect("exact candidate"),
                    anchored_surface_candidate_from_declaration(
                        "weaker",
                        &weaker,
                        "runtime-story-right-weaker",
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );

    let live_handle = admitted_rebinding_handle("phase-nine-runtime-story-live");
    let left_facts =
        primitive_rebinding_projection_facts(&left, &live_handle).expect("left projection facts");
    let right_facts =
        primitive_rebinding_projection_facts(&right, &live_handle).expect("right projection facts");
    let left_source = primitive_rebinding_retained_fact_source(&left, &live_handle)
        .expect("left retained source");
    let right_source = primitive_rebinding_retained_fact_source(&right, &live_handle)
        .expect("right retained source");
    let left_projection = primitive_rebinding_geometry_projection_consumption(
        &geometry_projection_consumption_entry(left_source.clone()),
        &live_handle,
    )
    .expect("left projection receipt");
    let right_projection = primitive_rebinding_geometry_projection_consumption(
        &geometry_projection_consumption_entry(right_source.clone()),
        &live_handle,
    )
    .expect("right projection receipt");

    let bundle = certification_bundle_for_pair(
        admitted_rebinding_handle("phase-nine-runtime-story-bundle"),
        scoped_branch_head_inspection_basis("branch:phase-nine-runtime-story"),
        left.clone(),
        right.clone(),
        "branch-evidence:left",
        "branch-evidence:right",
    );

    assert_eq!(left_facts.fact_digest(), right_facts.fact_digest());
    assert_eq!(
        left_projection.source_receipt_digest(),
        right_projection.source_receipt_digest()
    );
    assert_eq!(
        right_projection.source_receipt_digest(),
        left_projection.source_receipt_digest()
    );
    assert_eq!(
        left_projection.projection_digest(),
        right_projection.projection_digest()
    );
    assert_eq!(
        left_source.receipt().outcome_class(),
        left_facts.outcome_class()
    );
    assert_eq!(
        right_source.receipt().outcome_class(),
        right_facts.outcome_class()
    );
    assert_eq!(
        bundle.deterministic_outcome_class(),
        left_source.receipt().outcome_class()
    );
    assert_eq!(
        bundle.deterministic_continuity_class(),
        left_source.receipt().continuity_class()
    );
    assert_eq!(
        bundle.binding_identity(),
        left_source.receipt().prior_binding_identity()
    );
    assert_eq!(
        bundle.selected_candidate_identity(),
        left_source.receipt().selected_candidate_identity()
    );
    assert_eq!(bundle.replay_ordinary_kind(), "ambiguous");
    assert_eq!(
        bundle.deterministic_outcome_class(),
        RebindingOutcomeClass::Ambiguous
    );
}

#[test]
fn geometry_runtime_story_fails_loudly_when_retained_histories_are_semantically_different() {
    let prior = anchored_surface_with_domain(
        "face-curved",
        "surface-alpha",
        ParameterDomain::cylinder(),
        [TAU + 0.25, 0.5],
        1.0,
    );
    let exact = anchored_surface_with_domain(
        "face-curved",
        "surface-alpha",
        ParameterDomain::cylinder(),
        [0.25, 0.5],
        1.0,
    );
    let planarized = anchored_surface_with_domain(
        "face-curved",
        "surface-alpha",
        ParameterDomain::plane(),
        [0.25, 0.5],
        1.0,
    );
    let exact_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "runtime-story-exact-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                vec![anchored_surface_candidate_from_declaration(
                    "exact",
                    &exact,
                    "runtime-story-exact-candidate",
                )
                .expect("exact candidate")],
            ),
        ),
    );
    let planarized_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(&prior, "runtime-story-planarized-prior"),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-curved",
                vec![anchored_surface_candidate_from_declaration(
                    "planarized",
                    &planarized,
                    "runtime-story-planarized-candidate",
                )
                .expect("planarized candidate")],
            ),
        ),
    );

    let handle = admitted_rebinding_handle("phase-nine-runtime-story-mismatch");
    let exact_source = primitive_rebinding_retained_fact_source(&exact_entry, &handle)
        .expect("exact retained source");
    let planarized_source = primitive_rebinding_retained_fact_source(&planarized_entry, &handle)
        .expect("planarized retained source");
    let exact_historical = historical_rebinding_inspection(&exact_entry, &handle);
    let branch_basis =
        scoped_branch_head_inspection_basis("branch:phase-nine-runtime-story-mismatch");
    let planarized_branch = branch_local_rebinding_inspection(
        &planarized_entry,
        &handle,
        &branch_basis,
        "branch-evidence:planarized",
    );
    let exact_projection = primitive_rebinding_geometry_projection_consumption(
        &geometry_projection_consumption_entry(exact_source.clone()),
        &handle,
    )
    .expect("exact projection receipt");
    let planarized_projection = primitive_rebinding_geometry_projection_consumption(
        &geometry_projection_consumption_entry(planarized_source.clone()),
        &handle,
    )
    .expect("planarized projection receipt");

    let replay = geometry_replay_parity_entry(
        PrimitiveRebindingReplaySource::Historical(exact_historical.retained_fact_receipt()),
        PrimitiveRebindingReplaySource::BranchLocal(planarized_branch.retained_fact_receipt()),
    )
    .compare(&handle);

    assert_eq!(
        exact_source.receipt().outcome_class(),
        RebindingOutcomeClass::Preserved
    );
    assert_eq!(
        planarized_source.receipt().outcome_class(),
        RebindingOutcomeClass::CorrespondenceOnly
    );
    assert_ne!(
        exact_projection.source_receipt_digest(),
        planarized_projection.source_receipt_digest()
    );
    assert_ne!(
        exact_projection.projection_digest(),
        planarized_projection.projection_digest()
    );
    assert!(replay.is_err());
}

fn anchored_surface_with_domain(
    face_id: &str,
    persistent_name: &str,
    domain: ParameterDomain,
    point: [f64; 2],
    extent: f64,
) -> worth_spatial::facade::bindings::PrimitiveAnchorBindingDeclarationEntry {
    use worth_geom::facade::ParameterSpacePoint;
    use worth_spatial::facade::bindings::{
        author_primitive_anchor_binding_declaration, AnchorCarrierOwnership,
        AuthorPrimitiveAnchorBindingIntent, CarrierOwnedParameterPointAnchorSpec, FaceBindingSite,
        FaceSurfaceBindingSpec,
    };

    author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(face_id).with_persistent_name(persistent_name),
                crate::binding::tests::support::orthotope_contract(),
                crate::binding::tests::support::canonical_geometry([
                    [0.0, 0.0, 0.0],
                    [extent, 0.0, 0.0],
                ]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(face_id, domain).expect("ownership"),
                ParameterSpacePoint::try_new(point).expect("parameter"),
            )
            .expect("anchor spec"),
        ),
    )
}
