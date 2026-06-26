use super::super::closeout::current_worth_graph_read_access_posture_matrix_closeout;
use super::{
    carried_gap_for_tests, phase_two_closeout_with_attempts_for_tests,
    production_phase_three_closeout, production_phase_two_closeout,
    required_or_denied_attempt_for_tests,
};

#[test]
fn every_phase_two_requirement_or_gap_has_exactly_one_resolved_posture() {
    let phase_two = production_phase_two_closeout();
    let phase_three = production_phase_three_closeout();
    let expected_count = phase_two.posture_report().posture_rows().len()
        + phase_two.adoption_ledger().carried_capability_gap_count();

    assert_eq!(
        expected_count,
        phase_three.posture_map().resolved_postures().len()
    );
    assert_eq!(
        expected_count,
        phase_three.posture_map().requirement_identity_count()
    );

    for row in phase_two.posture_report().posture_rows() {
        let resolved = phase_three
            .posture_map()
            .posture_for_requirement(row.requirement_row_digest())
            .unwrap_or_else(|| {
                panic!(
                    "missing resolved posture for requirement row {}",
                    row.requirement_row_digest()
                )
            });
        assert_eq!(
            Some(row.source_attempt_digest()),
            resolved.source_attempt_digest()
        );
        assert_eq!(
            Some(row.source_pairing_digest()),
            resolved.source_pairing_digest()
        );
        assert_eq!(
            row.source_requirement_record_digest(),
            resolved.source_requirement_record_digest()
        );
        assert_eq!(
            Some(row.read_family_identity_digest()),
            resolved.read_family_identity_digest()
        );
        assert_eq!(
            Some(row.requirement_row_digest()),
            resolved.requirement_row_digest()
        );
        assert_eq!(Some(row.query_family_name()), resolved.query_family_name());
        assert_eq!(
            row.query_family_digest_seed(),
            resolved.query_family_digest_seed()
        );
        assert_eq!(row.query_posture(), resolved.query_posture());
        assert_eq!(row.denial_kind(), resolved.denial_kind());
        assert_eq!(row.blocker(), resolved.blocker());
        assert_eq!(row.removal_trigger(), resolved.removal_trigger());
    }
}

#[test]
fn carried_gap_rows_preserve_cap_and_removal_metadata() {
    let carried_gap = carried_gap_for_tests("gap-a");
    let phase_two = phase_two_closeout_with_attempts_for_tests(
        vec![required_or_denied_attempt_for_tests(
            "requirement-a",
            "persistent_index_required",
            "required_persistent_index",
        )],
        vec![carried_gap.clone()],
    );
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should resolve carried gaps");
    let resolved = phase_three
        .posture_map()
        .posture_for_requirement(&format!("carried_gap:{}", carried_gap.source_gap_digest()))
        .expect("carried gap should become a resolved posture");

    assert_eq!("carried_capability_gap", resolved.posture_family());
    assert_eq!(
        Some(carried_gap.source_gap_digest()),
        resolved.source_carried_gap_digest()
    );
    assert_eq!(
        carried_gap.source_requirement_record_digest(),
        resolved.source_requirement_record_digest()
    );
    assert_eq!(
        carried_gap.query_family_anchor_digest(),
        resolved.query_family_digest_seed()
    );
    assert_eq!(Some(carried_gap.owner()), resolved.owner());
    assert_eq!(
        Some(carried_gap.expected_denial()),
        resolved.expected_denial()
    );
    assert_eq!(
        Some(carried_gap.suggested_posture()),
        resolved.suggested_posture()
    );
    assert_eq!(Some(carried_gap.blocker()), resolved.blocker());
    assert_eq!(
        Some(carried_gap.removal_trigger()),
        resolved.removal_trigger()
    );
}
