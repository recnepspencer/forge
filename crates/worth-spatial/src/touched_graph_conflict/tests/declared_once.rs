use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictOverlapIdentityInput,
    ConflictPriorProofInput, ConflictRoutingPosture,
};

use crate::replay_undo_semantic_graph::{
    boolean_event_ledger_spatial_boundary_fixture, lower_spatial_replay_scope_identity,
};
use crate::workload_platform::evidence_ledger::{
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceStageKind,
};
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;

#[test]
fn one_evidence_declaration_serves_multiple_consumers_without_local_wiring() {
    let lookup_catalog = current_evidence_lookup_family_catalog().expect("lookup catalog closes");
    let family = lookup_catalog
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("evidence family exists");
    let authorities = [
        receipt_backed_touch_authority_for_admission_tests(
            BooleanEvidenceStageKind::SharedPlaneIdentity,
            "phase-3-shared-evidence-a",
        ),
        receipt_backed_touch_authority_for_admission_tests(
            BooleanEvidenceStageKind::SharedPlaneIdentity,
            "phase-3-shared-evidence-b",
        ),
    ];

    for authority in &authorities {
        let family_matches = family
            .matching_conflict_family_identities(authority)
            .expect("family declaration matches spatial conflict catalog");
        let authority_matches = authority
            .matching_conflict_family_identities(family)
            .expect("touch authority matches spatial conflict catalog");
        assert_eq!(
            family_matches,
            vec![crate::touched_graph_conflict::SpatialConflictFamilyIdentity::EvidenceSelection]
        );
        assert_eq!(family_matches, authority_matches);
    }

    let replay_fixture = boolean_event_ledger_spatial_boundary_fixture();
    let replay_catalog = current_evidence_lookup_family_catalog()
        .expect("lookup catalog closes");
    let replay_family = replay_catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("event-ledger family exists");
    let replay_scope = lower_spatial_replay_scope_identity(
        replay_fixture.authority(),
        replay_fixture.execution_receipt(),
        replay_fixture.stage_index_product(),
    )
    .expect("replay scope lowers");
    let replay_contract = admit_conflict_routing_contract(
        admit_conflict_overlap_identity(ConflictOverlapIdentityInput::replay_undo(
            replay_fixture
                .authority()
                .conflict_locality_identity()
                .expect("spatial locality admits"),
            vec![replay_scope.clone().into()],
        ))
        .expect("replay overlap admits"),
        ConflictPriorProofInput::from_identities(vec![replay_scope.into()]),
        ConflictRoutingPosture::RequiresFamilySelection,
    );

    assert!(replay_family
        .matching_conflict_family_identities_for_contract(
            replay_fixture.authority(),
            &replay_contract,
        )
        .is_empty());
    assert!(replay_fixture
        .authority()
        .matching_conflict_family_identities_for_contract(&replay_contract)
        .is_empty());
}
