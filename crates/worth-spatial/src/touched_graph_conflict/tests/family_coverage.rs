use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictOverlapIdentityInput,
    ConflictPriorProofInput, ConflictRoutingPosture,
};

use crate::replay_undo_semantic_graph::{
    boolean_event_ledger_spatial_boundary_fixture, lower_spatial_replay_scope_identity,
};
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;

#[test]
fn spatial_replay_family_is_selected_through_real_contract_route() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let replay_scope = lower_spatial_replay_scope_identity(
        fixture.authority(),
        fixture.execution_receipt(),
        fixture.stage_index_product(),
    )
    .expect("replay scope lowers");
    let contract = admit_conflict_routing_contract(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::replay_undo(
            fixture
                .authority()
                .conflict_locality_identity()
                .expect("spatial locality admits"),
            vec![replay_scope.clone().into()],
        ))
        .expect("replay overlap admits"),
        ConflictPriorProofInput::from_identities(vec![replay_scope.into()]),
        ConflictRoutingPosture::RequiresFamilySelection,
    );

    let matches = fixture
        .authority()
        .matching_replay_conflict_family_identities_for_contract(&contract);

    assert_eq!(
        matches,
        vec![crate::touched_graph_conflict::SpatialConflictFamilyIdentity::ReplayBoundarySelection]
    );
    assert!(!matches.contains(
        &crate::touched_graph_conflict::SpatialConflictFamilyIdentity::EvidenceSelection
    ));
}

#[test]
fn spatial_evidence_family_does_not_claim_replay_route() {
    let fixture = boolean_event_ledger_spatial_boundary_fixture();
    let catalog = current_evidence_lookup_family_catalog().expect("lookup catalog closes");
    let family = catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("event-ledger family exists");
    let replay_scope = lower_spatial_replay_scope_identity(
        fixture.authority(),
        fixture.execution_receipt(),
        fixture.stage_index_product(),
    )
    .expect("replay scope lowers");
    let contract = admit_conflict_routing_contract(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::replay_undo(
            fixture
                .authority()
                .conflict_locality_identity()
                .expect("spatial locality admits"),
            vec![replay_scope.clone().into()],
        ))
        .expect("replay overlap admits"),
        ConflictPriorProofInput::from_identities(vec![replay_scope.into()]),
        ConflictRoutingPosture::RequiresFamilySelection,
    );

    assert!(family
        .matching_conflict_family_identities_for_contract(fixture.authority(), &contract)
        .is_empty());
}
