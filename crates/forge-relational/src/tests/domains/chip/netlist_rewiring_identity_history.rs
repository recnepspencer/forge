use crate::facade::history::BranchId;
use crate::tests::support::*;

#[test]
fn netlist_rewiring_identity_history_smoke_tracks_branch_local_lineage() {
    let mut runtime = chip_runtime();
    let first = create_entity_outcome(&mut runtime, "net-a");
    let second = create_entity_outcome(&mut runtime, "net-b");
    let first_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;

    let candidate = record_lineage_candidate(
        &mut runtime,
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "rewire",
    );
    let resolution = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();

    assert!(resolution.promoted_event_id.is_some());
}
