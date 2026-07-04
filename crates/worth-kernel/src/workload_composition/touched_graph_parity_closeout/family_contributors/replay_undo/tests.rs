use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};

use super::contributor_catalog::{
    current_replay_undo_family_contributor_catalog, ReplayUndoFamilyContributorCatalog,
};
use super::parity::{
    current_replay_undo_family_parity_claim, replay_undo_family_parity_claim_from_catalog,
    ReplayUndoFamilyParityErrorKind,
};
use super::row::ReplayUndoContributorRowKind;

#[test]
fn replay_undo_family_parity_holds_through_carried_scope_and_boundary_proof() {
    let catalog =
        current_replay_undo_family_contributor_catalog().expect("replay/undo contributor catalog");
    let claim = current_replay_undo_family_parity_claim().expect("replay/undo parity claim");

    assert_eq!(
        claim.kind(),
        TouchedGraphParityClaimKind::SelectedRouteParity
    );
    assert_eq!(catalog.rows().len(), 2);
    assert_eq!(claim.rows().len(), 2);

    let replay = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == ReplayUndoContributorRowKind::Replay)
        .expect("replay row");
    assert_eq!(
        replay.family_kind(),
        TouchedGraphParityFamilyKind::ReplayUndo
    );
    assert_eq!(
        replay.current_packet_or_identity_source(),
        "current_replay_undo_boundary_route_authority"
    );
    assert!(replay
        .carried_scope_identity_source()
        .contains("replay_scope_identity"));
    assert!(replay
        .carried_witness_or_boundary_source()
        .contains("boundary_proof_digest"));

    let undo = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == ReplayUndoContributorRowKind::Undo)
        .expect("undo row");
    assert_eq!(undo.family_kind(), TouchedGraphParityFamilyKind::ReplayUndo);
    assert!(undo
        .carried_scope_identity_source()
        .contains("undo_scope_identity"));
    assert!(claim.rows().iter().any(|row| {
        row.kind() == ReplayUndoContributorRowKind::Replay
            && row.family_kind() == TouchedGraphParityFamilyKind::ReplayUndo
    }));
    assert!(claim.rows().iter().any(|row| {
        row.kind() == ReplayUndoContributorRowKind::Undo
            && row.family_kind() == TouchedGraphParityFamilyKind::ReplayUndo
    }));
}

#[test]
fn replay_undo_family_parity_rejects_replay_scope_identity_divergence_even_if_outputs_match() {
    let mut hostile_rows = current_replay_undo_family_contributor_catalog()
        .expect("replay/undo contributor catalog")
        .rows()
        .to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_identity_override(
        hostile_rows[0].route_packet_identity(),
        hostile_rows[0].transaction_packet_identity(),
        "foreign-replay-scope",
        hostile_rows[0].carried_boundary_proof_digest(),
    );

    assert_eq!(
        replay_undo_family_parity_claim_from_catalog(
            &ReplayUndoFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("replay scope drift must be rejected")
        .kind(),
        ReplayUndoFamilyParityErrorKind::MismatchedReplayIdentity
    );
}

#[test]
fn replay_undo_family_parity_rejects_undo_scope_identity_divergence_even_if_outputs_match() {
    let mut hostile_rows = current_replay_undo_family_contributor_catalog()
        .expect("replay/undo contributor catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_identity_override(
        hostile_rows[1].route_packet_identity(),
        hostile_rows[1].transaction_packet_identity(),
        "foreign-undo-scope",
        hostile_rows[1].carried_boundary_proof_digest(),
    );

    assert_eq!(
        replay_undo_family_parity_claim_from_catalog(
            &ReplayUndoFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("undo scope drift must be rejected")
        .kind(),
        ReplayUndoFamilyParityErrorKind::MismatchedUndoIdentity
    );
}
