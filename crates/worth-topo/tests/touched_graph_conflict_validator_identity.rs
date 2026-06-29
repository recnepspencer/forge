use schema::facade::platform::authority::touched_graph_conflict::ConflictParticipantAuthority;
use topology::facade::current_worth_topology_legality_catalog_closeout;
use topology::facade::WorthTopologyLegalityFamilyRecord;

#[test]
fn legality_catalog_validator_identity_lowers_into_shared_validator_participant() {
    let closeout = current_worth_topology_legality_catalog_closeout()
        .expect("legality catalog closeout should build");
    let record = closeout
        .catalog()
        .records()
        .iter()
        .find(|record| matches!(record, WorthTopologyLegalityFamilyRecord::Validator(_)))
        .expect("catalog should expose at least one validator family");
    let identity = record.identity();

    let participant = identity
        .conflict_participant_identity()
        .expect("validator family identity should admit shared validator participant");

    assert_eq!(
        participant.authority(),
        ConflictParticipantAuthority::Validator
    );
    assert_eq!(participant.digest(), identity.identity_digest());
}

#[test]
fn legality_catalog_invariant_identity_lowers_into_shared_validator_participant() {
    let closeout = current_worth_topology_legality_catalog_closeout()
        .expect("legality catalog closeout should build");
    let record = closeout
        .catalog()
        .records()
        .iter()
        .find(|record| matches!(record, WorthTopologyLegalityFamilyRecord::Invariant(_)))
        .expect("catalog should expose at least one invariant family");
    let identity = record.identity();

    let participant = identity
        .conflict_participant_identity()
        .expect("invariant family identity should admit shared validator participant");

    assert_eq!(
        participant.authority(),
        ConflictParticipantAuthority::Validator
    );
    assert_eq!(participant.digest(), identity.identity_digest());
}
