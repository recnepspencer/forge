use super::current_route_authority::{
    current_completed_split_route_authority, current_lookup_consumed_route_authority,
    current_replay_undo_boundary_route_authority,
};

#[test]
fn completed_split_route_authority_has_route_specific_payload() {
    let lookup = current_lookup_consumed_route_authority()
        .expect("lookup route authority should build from current boundaries");
    let completed = current_completed_split_route_authority()
        .expect("completed-split route authority should build from current split boundary");

    assert_ne!(
        completed.route_authority_digest(),
        lookup.route_authority_digest()
    );
    assert_eq!(
        completed.lookup_route_authority().route_authority_digest(),
        lookup.route_authority_digest()
    );
    assert_ne!(
        completed
            .split_boundary()
            .workload_handoff()
            .stage_receipt_identity(),
        lookup
            .left_boundary()
            .workload_handoff()
            .stage_receipt_identity()
    );
}

#[test]
fn replay_undo_route_authority_has_route_specific_packet_and_scope_payload() {
    let lookup = current_lookup_consumed_route_authority()
        .expect("lookup route authority should build from current boundaries");
    let replay_undo = current_replay_undo_boundary_route_authority()
        .expect("replay-undo route authority should build from current replay/undo surfaces");

    assert_eq!(
        replay_undo
            .lookup_route_authority()
            .route_authority_digest(),
        lookup.route_authority_digest()
    );
    assert_ne!(
        replay_undo.route_authority_digest(),
        lookup.route_authority_digest()
    );
    assert_eq!(
        replay_undo.source_identity().as_str(),
        "kernel.boolean_split_replay_undo_boundary_admission"
    );
    assert!(replay_undo
        .source_path()
        .ends_with("replay_undo_boundary/boolean_split_boundary_admission.rs"));
    assert!(replay_undo.inventory_row_count() > 0);
    assert!(replay_undo.forbidden_surface_denial_count() > 0);
    assert!(!replay_undo.boundary_proof_digest().is_empty());
    assert!(!replay_undo.transaction_packet_identity().is_empty());
    assert!(!replay_undo.replay_scope_identity().is_empty());
    assert!(!replay_undo.undo_scope_identity().is_empty());
}
