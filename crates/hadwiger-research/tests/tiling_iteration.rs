use hadwiger_research::facade::*;

#[path = "tiling_iteration/fixtures/mod.rs"]
mod fixtures;

use fixtures::{base_session, stale_session};

#[test]
fn lower_bound_iteration_packet_declares_query_and_replays() {
    let (handle, session) = base_session("lower-bound-packet");
    let request = TilingIterationPacketRequest::lower_bound_obstruction("lower-bound-pass-a")
        .from_cockpit_session(&session)
        .with_evidence_basis("edge-local retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_required_checker_lane("six_colorability_refutation")
        .with_reactivation_obligation("provide repaired exact coordinates")
        .with_expected_information_gain("extract reusable terminal-forcing motif")
        .unwrap();

    let packet = derive_tiling_iteration_packet_checked(&handle, request).unwrap();
    let replay = replay_tiling_iteration_packet_checked(&handle, &packet).unwrap();

    assert_eq!(packet.source_session_digest(), session.session_digest());
    assert_eq!(
        packet.packet_kind(),
        TilingIterationPacketKind::LowerBoundObstruction
    );
    assert_eq!(packet.packet_digest(), replay.packet_digest());
    assert!(packet.query_readiness_checks() > 0);
    assert_eq!(packet.evidence_basis(), &["edge-local retained rejection"]);
    assert_eq!(
        packet.reactivation_obligations(),
        &["provide repaired exact coordinates"]
    );
    assert_eq!(packet.counters(), replay.counters());
    assert!(!packet.admits_theorem_authority());
    assert!(!packet.executes_checker_work());
}

#[test]
fn upper_bound_periodic_packet_uses_same_query_native_rhythm() {
    let (handle, session) = base_session("upper-bound-packet");
    let request =
        TilingIterationPacketRequest::upper_bound_periodic_quotient("periodic-six-pass-a")
            .from_cockpit_session(&session)
            .with_evidence_basis("periodic quotient near miss")
            .with_required_checker_lane("boundary_ownership")
            .with_required_checker_lane("periodic_quotient_wraparound")
            .with_reactivation_obligation("supply exact wraparound conflict certificate")
            .with_expected_information_gain("find exact translated same-color conflict")
            .unwrap();

    let packet = derive_tiling_iteration_packet_checked(&handle, request).unwrap();

    assert_eq!(
        packet.packet_kind(),
        TilingIterationPacketKind::UpperBoundPeriodicQuotient
    );
    assert_eq!(packet.counters().query_declarations_checked(), 1);
    assert!(packet
        .query_declaration_reference()
        .declaration_family_key()
        .contains("upper_bound_periodic_quotient"));
    assert!(!packet.registers_query_invariant_authority());
}

#[test]
fn iteration_declaration_families_have_query_readiness_and_inventory() {
    let (handle, _session) = base_session("iteration-query-surfaces");

    assert!(
        !research_declaration_entry_readiness::<LowerBoundTilingIterationDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_inventory::<LowerBoundTilingIterationDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<UpperBoundTilingIterationDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_inventory::<UpperBoundTilingIterationDeclaration>(&handle)
            .rows()
            .is_empty()
    );
}

#[test]
fn equivalent_iteration_identity_fields_converge_despite_insertion_order() {
    let (handle, session) = base_session("equivalent-lanes");
    let left = TilingIterationPacketRequest::lower_bound_obstruction("same-lanes")
        .from_cockpit_session(&session)
        .with_evidence_basis("retained rejection")
        .with_evidence_basis("graph-resident failure")
        .with_required_checker_lane("six_colorability_refutation")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_reactivation_obligation("reactivate with new obstruction core")
        .with_expected_information_gain("same gain")
        .unwrap();
    let right = TilingIterationPacketRequest::lower_bound_obstruction("same-lanes")
        .from_cockpit_session(&session)
        .with_evidence_basis("graph-resident failure")
        .with_evidence_basis("retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_required_checker_lane("six_colorability_refutation")
        .with_reactivation_obligation("reactivate with new obstruction core")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("same gain")
        .unwrap();

    let left_packet = derive_tiling_iteration_packet_checked(&handle, left).unwrap();
    let right_packet = derive_tiling_iteration_packet_checked(&handle, right).unwrap();

    assert_eq!(
        left_packet.artifact_digest(),
        right_packet.artifact_digest()
    );
    assert_eq!(
        left_packet.required_checker_lanes(),
        right_packet.required_checker_lanes()
    );
}

#[test]
fn changed_information_gain_changes_packet_digest() {
    let (handle, session) = base_session("changed-gain");
    let first = TilingIterationPacketRequest::lower_bound_obstruction("gain-change")
        .from_cockpit_session(&session)
        .with_evidence_basis("retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("extract motif")
        .unwrap();
    let second = TilingIterationPacketRequest::lower_bound_obstruction("gain-change")
        .from_cockpit_session(&session)
        .with_evidence_basis("retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("rank motif frontier")
        .unwrap();

    let first_packet = derive_tiling_iteration_packet_checked(&handle, first).unwrap();
    let second_packet = derive_tiling_iteration_packet_checked(&handle, second).unwrap();

    assert_ne!(
        first_packet.artifact_digest(),
        second_packet.artifact_digest()
    );
}

#[test]
fn changed_evidence_basis_or_reactivation_obligation_changes_packet_digest() {
    let (handle, session) = base_session("changed-proof-fields");
    let first = TilingIterationPacketRequest::lower_bound_obstruction("proof-field-change")
        .from_cockpit_session(&session)
        .with_evidence_basis("retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("same gain")
        .unwrap();
    let second = TilingIterationPacketRequest::lower_bound_obstruction("proof-field-change")
        .from_cockpit_session(&session)
        .with_evidence_basis("different retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("same gain")
        .unwrap();
    let third = TilingIterationPacketRequest::lower_bound_obstruction("proof-field-change")
        .from_cockpit_session(&session)
        .with_evidence_basis("retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with new exact core")
        .with_expected_information_gain("same gain")
        .unwrap();

    let first_packet = derive_tiling_iteration_packet_checked(&handle, first).unwrap();
    let second_packet = derive_tiling_iteration_packet_checked(&handle, second).unwrap();
    let third_packet = derive_tiling_iteration_packet_checked(&handle, third).unwrap();

    assert_ne!(
        first_packet.artifact_digest(),
        second_packet.artifact_digest()
    );
    assert_ne!(
        first_packet.artifact_digest(),
        third_packet.artifact_digest()
    );
    assert_ne!(
        first_packet
            .query_declaration_reference()
            .declaration_digest(),
        second_packet
            .query_declaration_reference()
            .declaration_digest()
    );
    assert_ne!(
        first_packet
            .query_declaration_reference()
            .declaration_digest(),
        third_packet
            .query_declaration_reference()
            .declaration_digest()
    );
}

#[test]
fn query_declaration_identity_preserves_list_boundaries() {
    let (handle, session) = base_session("list-boundary-identity");
    let single_field = TilingIterationPacketRequest::lower_bound_obstruction("list-boundary")
        .from_cockpit_session(&session)
        .with_evidence_basis("basis-a|basis-b")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate")
        .with_expected_information_gain("same gain")
        .unwrap();
    let two_fields = TilingIterationPacketRequest::lower_bound_obstruction("list-boundary")
        .from_cockpit_session(&session)
        .with_evidence_basis("basis-a")
        .with_evidence_basis("basis-b")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate")
        .with_expected_information_gain("same gain")
        .unwrap();

    let single_packet = derive_tiling_iteration_packet_checked(&handle, single_field).unwrap();
    let two_packet = derive_tiling_iteration_packet_checked(&handle, two_fields).unwrap();

    assert_ne!(
        single_packet
            .query_declaration_reference()
            .declaration_digest(),
        two_packet
            .query_declaration_reference()
            .declaration_digest()
    );
}

#[test]
fn stale_frontier_blocks_checker_actions_without_hiding_advisory_preview() {
    let (handle, session) = stale_session("stale-packet");
    let request = TilingIterationPacketRequest::lower_bound_obstruction("stale-pass")
        .from_cockpit_session(&session)
        .with_evidence_basis("stale retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("recompute frontier before execution")
        .with_expected_information_gain("repair stale frontier")
        .unwrap();

    let packet = derive_tiling_iteration_packet_checked(&handle, request).unwrap();

    assert!(packet
        .actions()
        .iter()
        .any(|action| { action.blocker() == Some(TilingIterationBlocker::StaleDerivedFrontier) }));
    assert_eq!(
        packet.counters().stale_frontier_blocks(),
        packet.counters().blocked_actions()
    );
    assert!(packet.counters().advisory_only_rows() > 0);
}

#[test]
fn suppressed_frontier_work_is_blocked_and_counter_visible() {
    let (handle, session) = base_session("suppressed-packet");
    let request = TilingIterationPacketRequest::lower_bound_obstruction("suppressed-pass")
        .from_cockpit_session(&session)
        .with_evidence_basis("suppressed graph-resident failure")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("provide qualifying reactivation evidence")
        .with_expected_information_gain("avoid repeated dead end")
        .unwrap();

    let packet = derive_tiling_iteration_packet_checked(&handle, request).unwrap();

    assert!(packet.actions().iter().any(|action| {
        action.blocker() == Some(TilingIterationBlocker::SuppressedDeadEndEquivalence)
    }));
    assert!(packet.counters().suppression_blocks() > 0);
    assert!(packet.counters().equivalence_basis_rows() > 0);
}

#[test]
fn missing_checker_lane_rejects_request_construction() {
    let (_handle, session) = base_session("missing-lane");
    let error = TilingIterationPacketRequest::lower_bound_obstruction("missing-lane")
        .from_cockpit_session(&session)
        .with_evidence_basis("retained rejection")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("cannot build without checker lane")
        .expect_err("checker lanes are mandatory");

    assert_eq!(error, TilingIterationError::MissingRequiredCheckerLane);
}

#[test]
fn missing_evidence_basis_and_reactivation_obligation_are_typed_errors() {
    let (_handle, session) = base_session("missing-proof-fields");
    let missing_basis = TilingIterationPacketRequest::lower_bound_obstruction("missing-basis")
        .from_cockpit_session(&session)
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("reactivate with repaired embedding")
        .with_expected_information_gain("cannot build without evidence basis")
        .expect_err("evidence basis is mandatory");
    let missing_obligation =
        TilingIterationPacketRequest::lower_bound_obstruction("missing-reactivation")
            .from_cockpit_session(&session)
            .with_evidence_basis("retained rejection")
            .with_required_checker_lane("exact_tile_contact")
            .with_expected_information_gain("cannot build without reactivation obligation")
            .expect_err("reactivation obligation is mandatory");

    assert_eq!(missing_basis, TilingIterationError::MissingEvidenceBasis);
    assert_eq!(
        missing_obligation,
        TilingIterationError::MissingReactivationObligation
    );
}
mod installed_support;
