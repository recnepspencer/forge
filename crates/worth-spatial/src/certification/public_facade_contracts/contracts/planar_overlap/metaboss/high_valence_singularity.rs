use super::high_valence_subject::{
    certify_platform_high_valence_singularity,
    certify_platform_high_valence_singularity_with_explicit_valence,
    high_valence_integrity_mismatch_outcome, high_valence_missing_neighborhood_outcome,
    high_valence_policy_required_outcome, high_valence_predicate_uncertain_outcome,
    high_valence_rebuild_motion_break_outcome, high_valence_unsupported_explicit_valence_outcome,
    high_valence_unsupported_valence_outcome,
};
use worth_spatial::facade::user_response::{WorthUserOutcome, WorthUserOutcomeKind};

#[test]
fn mb_m6_2_high_valence_planar_singularity_contract() {
    let subject = certify_platform_high_valence_singularity("contract");
    let counters = subject.receipt.counters();

    assert_eq!(counters.neighborhood_valence(), 5);
    assert!(counters.topology_entity_count() > counters.neighborhood_valence());
    assert_eq!(
        counters.topology_face_count(),
        counters.neighborhood_valence()
    );
    assert!(counters.topology_relation_count() >= counters.neighborhood_valence());
    assert!(counters.binding_target_count() >= counters.neighborhood_valence() * 2);
    assert!(counters.surface_support_count() > 0);
    assert!(counters.projected_entity_count() >= counters.neighborhood_valence() * 2);
    assert!(counters.local_basis_part_count() > 0);
    assert!(counters.transform_step_count() > 0);
    assert_eq!(counters.local_rebuild_evidence_row_count(), 1);
    assert!(counters.retained_artifact_count() > 0);
    assert!(counters.replay_checkpoint_count() > 0);
    assert_eq!(counters.diagnostic_count(), 1);
    assert_eq!(counters.user_outcome_count(), 1);
    assert!(!subject.receipt.singularity_digest().is_empty());
    assert!(!subject.receipt.workload_identity().is_empty());
    assert!(!subject.receipt.center_vertex_identity().is_empty());
    assert!(!subject.receipt.local_rebuild_evidence_digest().is_empty());
    assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);
    assert_human_readable(subject.user_outcome.human_response().summary());
}

#[test]
fn mb_m6_2_valence_support_boundary_is_exact_and_receipt_backed() {
    for valence in [3, 16] {
        let subject =
            certify_platform_high_valence_singularity_with_explicit_valence("boundary", valence);
        let counters = subject.receipt.counters();

        assert_eq!(counters.neighborhood_valence(), valence);
        assert_eq!(counters.topology_face_count(), valence);
        assert!(counters.binding_target_count() >= valence * 2);
        assert!(counters.projected_entity_count() >= valence * 2);
        assert_eq!(counters.local_rebuild_evidence_row_count(), 1);
        assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);
    }

    for valence in [2, 17] {
        let outcome = high_valence_unsupported_explicit_valence_outcome("boundary-denial", valence);

        assert_eq!(outcome.kind(), WorthUserOutcomeKind::Unsupported);
        assert_eq!(
            outcome.human_response().summary(),
            format!(
                "high-valence singularity supports valence 3 through 16 today; valence {valence} needs an explicit widening phase"
            )
        );
    }
}

#[test]
fn mb_m6_2_singularity_no_options_matrix_names_exact_blocker() {
    let outcomes = vec![
        high_valence_policy_required_outcome("matrix-policy"),
        high_valence_predicate_uncertain_outcome("matrix-predicate"),
        high_valence_missing_neighborhood_outcome("matrix-topology"),
        high_valence_unsupported_valence_outcome("matrix-unsupported-valence"),
        high_valence_rebuild_motion_break_outcome("matrix-motion"),
        high_valence_integrity_mismatch_outcome("matrix-integrity"),
    ];

    assert_one_kind(&outcomes, WorthUserOutcomeKind::PolicyRequired);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::PredicateUncertain);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::Unsupported);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::Denied);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::IntegrityMismatch);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::NoOptions);

    assert_message_contains(&outcomes, "user policy decision");
    assert_message_contains(&outcomes, "predicate authority could not certify");
    assert_message_contains(&outcomes, "topology neighborhood receipt");
    assert_message_contains(&outcomes, "valence 32 needs an explicit widening phase");
    assert_message_contains(&outcomes, "before correspondence");
    assert_message_contains(&outcomes, "projection evidence");

    for outcome in outcomes {
        assert_human_readable(outcome.human_response().summary());
        assert!(!outcome.evidence().digest().is_empty());
        assert!(!outcome.evidence().source_identity().is_empty());
    }
}

#[test]
fn mb_m6_2_rebuild_movement_break_denies_before_correspondence() {
    let outcome = high_valence_rebuild_motion_break_outcome("motion-break");

    assert_eq!(outcome.kind(), WorthUserOutcomeKind::Denied);
    assert_eq!(
        outcome.human_response().summary(),
        "high-valence rebuild motion must match retained neighborhood posture before correspondence"
    );
    assert!(outcome.choices().is_empty());
}

fn assert_one_kind(outcomes: &[WorthUserOutcome], kind: WorthUserOutcomeKind) {
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.kind() == kind)
            .count(),
        1
    );
}

fn assert_message_contains(outcomes: &[WorthUserOutcome], expected: &str) {
    assert!(
        outcomes
            .iter()
            .any(|outcome| outcome.human_response().summary().contains(expected)),
        "missing high-valence outcome message containing {expected:?}"
    );
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "user-facing high-valence message must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "user-facing high-valence message must explain causes in prose: {message}"
    );
}
