use std::collections::BTreeSet;

use worth_spatial::facade::bindings::{
    author_primitive_rebinding_declaration, MotionAwareBindingPosture, NeighborhoodBindingFamily,
    RebindingOutcomeClass,
};

use crate::binding::tests::support::{
    admitted_rebinding_handle, anchored_surface_candidate_from_declaration,
    anchored_surface_declaration, anchored_surface_prior_fact_from_declaration,
    canonical_text_entries_for_rebinding, inspect_progressed_rebinding_entry,
    progress_rebinding_entry, rebind_surface_on_face, rebinding_declaration_digest_string,
    rebinding_receipt_for_entry, replacement_neighborhood,
};

#[test]
fn local_topology_replacement_rebinds_or_denies_canonically_under_replay() {
    let prior_declaration =
        anchored_surface_declaration("face-old", "surface-alpha", [0.25, 0.5], 1.0);
    let exact_declaration =
        anchored_surface_declaration("face-new-a", "surface-beta", [0.25, 0.5], 1.0);
    let weaker_declaration =
        anchored_surface_declaration("face-new-b", "surface-gamma", [0.25, 0.5], 2.0);
    let left_neighborhood = replacement_neighborhood(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        vec![
            anchored_surface_candidate_from_declaration(
                "weaker",
                &weaker_declaration,
                "canonical-replay-left-weaker",
            )
            .expect("weaker"),
            anchored_surface_candidate_from_declaration(
                "exact",
                &exact_declaration,
                "canonical-replay-left-exact",
            )
            .expect("exact"),
        ],
    );
    let right_neighborhood = replacement_neighborhood(
        NeighborhoodBindingFamily::FaceSurfacePointAnchor,
        "face-old",
        vec![
            anchored_surface_candidate_from_declaration(
                "exact",
                &exact_declaration,
                "canonical-replay-right-exact",
            )
            .expect("exact"),
            anchored_surface_candidate_from_declaration(
                "weaker",
                &weaker_declaration,
                "canonical-replay-right-weaker",
            )
            .expect("weaker"),
        ],
    );

    let left_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &prior_declaration,
                "canonical-replay-left-prior",
            ),
            left_neighborhood.clone(),
        ),
    );
    let right_entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            anchored_surface_prior_fact_from_declaration(
                &prior_declaration,
                "canonical-replay-right-prior",
            ),
            right_neighborhood.clone(),
        ),
    );

    let handle = admitted_rebinding_handle("rebinding-replay");
    let left_progression = progress_rebinding_entry(&left_entry, &handle);
    let right_progression = progress_rebinding_entry(&right_entry, &handle);
    let left_inspection = inspect_progressed_rebinding_entry(&handle, left_progression.clone());

    assert_eq!(
        rebinding_declaration_digest_string(&left_progression),
        rebinding_declaration_digest_string(&right_progression)
    );
    let canonical = canonical_text_entries_for_rebinding(&left_entry);
    assert_eq!(
        canonical
            .get("rebinding.neighborhood.family")
            .map(String::as_str),
        Some("face_surface_point_anchor")
    );
    assert_eq!(
        canonical
            .get("rebinding.neighborhood.candidate_count")
            .map(String::as_str),
        Some("2")
    );
    assert_eq!(
        Some(left_progression.progression_digest()),
        left_inspection.progression_digest()
    );

    let kernel_receipt = rebinding_receipt_for_entry(&left_entry, "canonical-replay-kernel")
        .expect("kernel receipt");
    let direct_receipt = rebind_surface_on_face(
        anchored_surface_prior_fact_from_declaration(
            &prior_declaration,
            "canonical-replay-direct-prior",
        ),
        right_neighborhood,
    )
    .expect("direct receipt");

    assert_eq!(
        kernel_receipt.outcome_class(),
        direct_receipt.outcome_class()
    );
    assert_ne!(
        kernel_receipt.outcome_class(),
        RebindingOutcomeClass::Orphaned
    );
    assert_eq!(
        kernel_receipt.selected_candidate_identity(),
        direct_receipt.selected_candidate_identity()
    );
    assert_eq!(
        kernel_receipt.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        direct_receipt.motion_posture(),
        MotionAwareBindingPosture::Unresolved
    );
    assert_eq!(
        kernel_receipt.neighborhood_family(),
        NeighborhoodBindingFamily::FaceSurfacePointAnchor
    );
    assert_eq!(kernel_receipt.prior_site_identity(), "face-old");
    assert_eq!(
        kernel_receipt
            .candidate_labels()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["exact".to_string(), "weaker".to_string()])
    );
    assert_eq!(
        kernel_receipt
            .candidate_site_identities()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["face-new-a".to_string(), "face-new-b".to_string()])
    );
}
