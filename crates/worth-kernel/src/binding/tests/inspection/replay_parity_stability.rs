use forge_query::facade::ForgeQueryDeclarationEntryInspectionInput;
use worth_spatial::facade::bindings::{
    BindingContinuityClass, NeighborhoodBindingFamily, ReplacementCandidate,
    SpatialAdmittedPrimitiveBinding,
};

use crate::{
    binding::rebinding::PrimitiveRebindingReplaySource,
    facade::authoring::binding::{
        author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
    },
};

use super::super::support::{
    admitted_rebinding_handle, anchored_surface, progress_rebinding_entry,
    replacement_neighborhood, retained_digest_for_decision,
};

#[test]
fn replay_parity_does_not_depend_on_live_runtime_memory_or_host_order_accident() {
    let prior = anchored_surface("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact = anchored_surface("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker = anchored_surface("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left = author_primitive_rebinding_declaration(
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
                    .expect("weaker candidate"),
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                    )
                    .expect("exact candidate"),
                ],
            ),
        ),
    );
    let right = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior.clone()),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![
                    ReplacementCandidate::new(
                        "exact",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(exact.clone()),
                    )
                    .expect("exact candidate"),
                    ReplacementCandidate::new(
                        "weaker",
                        SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker.clone()),
                    )
                    .expect("weaker candidate"),
                ],
            ),
        ),
    );
    let mutated_live = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(prior),
            replacement_neighborhood(
                NeighborhoodBindingFamily::FaceSurfacePointAnchor,
                "face-old",
                vec![ReplacementCandidate::new(
                    "weaker",
                    SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(weaker),
                )
                .expect("weaker candidate")],
            ),
        ),
    )
    .admit()
    .expect("mutated live decision");
    let handle = admitted_rebinding_handle("phase-fifteen-live-independence");

    let parity = left
        .replay_parity_with_query(
            &handle,
            PrimitiveRebindingReplaySource::Historical(
                left.historical_inspection_with_query(
                    &handle,
                    ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                        handle.orchestrate_envelope_from_progressed_checked(
                            progress_rebinding_entry(&left, &handle),
                        ),
                    ),
                )
                .expect("left historical inspection"),
            ),
            &right,
            PrimitiveRebindingReplaySource::Historical(
                right
                    .historical_inspection_with_query(
                        &handle,
                        ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
                            handle.orchestrate_envelope_from_progressed_checked(
                                progress_rebinding_entry(&right, &handle),
                            ),
                        ),
                    )
                    .expect("right historical inspection"),
            ),
        )
        .expect("replay parity");
    let left_decision = left.clone().admit().expect("left decision");
    let right_decision = right.clone().admit().expect("right decision");

    assert_eq!(
        parity.binding_identity(),
        left_decision.explanation().prior_identity()
    );
    assert_eq!(
        parity.anchor_identity(),
        left_decision.explanation().prior_site_identity()
    );
    assert_eq!(parity.outcome_class(), left_decision.outcome_class());
    assert_eq!(
        parity.continuity_class(),
        left_decision.explanation().continuity_class()
    );
    assert_eq!(
        parity.selected_candidate_identity(),
        left_decision.explanation().selected_candidate_identity()
    );
    assert_eq!(
        parity.selected_candidate_label(),
        left_decision.explanation().selected_candidate_label()
    );
    assert_eq!(
        parity.binding_identity(),
        right_decision.explanation().prior_identity()
    );
    assert_eq!(parity.ordinary_kind(), "ambiguous");
    assert_eq!(
        parity.next_step(),
        Some(forge_query::facade::ForgeQueryOrdinaryNextStep::NarrowInput)
    );
    assert_ne!(
        parity.replay_digest(),
        retained_digest_for_decision(&mutated_live)
    );
    assert_eq!(
        mutated_live.explanation().continuity_class(),
        BindingContinuityClass::CorrespondenceOnly
    );
}
