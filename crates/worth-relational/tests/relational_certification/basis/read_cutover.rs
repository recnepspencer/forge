use std::collections::BTreeSet;
use std::sync::Arc;

use super::world::supply_chain::{
    certified_supply_chain_world, commit_branch_batch, fork_supply_chain_branch_from_main,
    lower_supply_chain_production_delta, observe_supply_chain_observation, DeltaId, EntityRecord,
    SupplyChainScale,
};
use worth_foundational::facade::{AspectKey, AspectValue, InternedString, ScalarAspectType};
use worth_relational::facade::branch::RelationalBranchBasisDenial;
use worth_relational::facade::bridge::RuntimeBridgeRelationalSource;
use worth_relational::facade::history::{BranchId, RelationalMergeBranchBasisDenial};
use worth_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, SnapshotReadContract, SnapshotReadPacket,
    SnapshotReadRequest, SnapshotReadSource,
};

#[test]
fn history_visibility_and_bridge_read_the_observation_selected_root() {
    let (world, _) = certified_supply_chain_world(SupplyChainScale::court());
    let branch_id = BranchId("storm".to_owned());
    fork_supply_chain_branch_from_main(&world.runtime, branch_id.clone());
    let identity = world.runtime.branch_identity(&branch_id).unwrap();
    let (_, basis) = world.runtime.observe_branch(&identity).unwrap();
    let observation = basis.observation();
    let before = observe_supply_chain_observation(
        &world.program,
        &world.handles,
        &world.runtime,
        &observation,
    )
    .unwrap();
    let EntityRecord::Voyage(voyage_before) =
        &before.entities[&world.handles.aurora_voyage().semantic]
    else {
        panic!("Aurora remains a voyage in the admitted root");
    };
    let expected_status =
        AspectValue::String(InternedString::Raw(format!("{:?}", voyage_before.status)));
    let original_head = world
        .runtime
        .history()
        .branch_head_for_observation(&observation)
        .unwrap()
        .expect("baseline has one canonical head");

    let batch = lower_supply_chain_production_delta(
        &world.runtime,
        &world.program,
        &world.handles,
        &branch_id,
        &BTreeSet::new(),
        DeltaId::StormRerouteAurora,
    )
    .unwrap();
    commit_branch_batch(&world.runtime, branch_id, batch);

    let (_, current_basis) = world.runtime.observe_branch(&identity).unwrap();
    let current_head = world
        .runtime
        .history()
        .branch_head_for_observation(&current_basis.observation())
        .unwrap()
        .unwrap();
    assert_ne!(current_head.commit_id, original_head.commit_id);
    assert_eq!(
        world
            .runtime
            .history()
            .branch_head_for_observation(&observation)
            .unwrap()
            .unwrap()
            .commit_id,
        original_head.commit_id
    );

    let voyage = world.handles.aurora_voyage().id;
    let source = RuntimeBridgeRelationalSource::for_graph_role(
        Arc::new(world.runtime),
        "phase6-certification",
    )
    .unwrap();
    let lease = source.retain_branch_basis_for_bridge(&basis).unwrap();
    let reader = source.open_snapshot(lease.snapshot_identity()).unwrap();
    let packet = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_relational_record(
        RelationalBridgeRecordIdentityParts::entity(
            voyage.partition_id.0,
            voyage.local_slot.0,
            voyage.generation.0,
        ),
        SnapshotReadContract::scalar(AspectKey::new("status").unwrap(), ScalarAspectType::String),
    )]);
    let result = reader.read_packet(&packet).unwrap();

    assert_eq!(
        result.records()[0].scalar_aspect_value(),
        Some(&expected_status)
    );
}

#[test]
fn merge_history_resolves_from_two_exact_observations() {
    let (world, _) = certified_supply_chain_world(SupplyChainScale::court());
    let main_identity = world.runtime.main_branch_identity();
    let (_, main_basis) = world.runtime.observe_branch(&main_identity).unwrap();
    let main_observation = main_basis.observation();
    let main_head = world
        .runtime
        .history()
        .branch_head_for_observation(&main_observation)
        .unwrap()
        .unwrap();

    let storm = BranchId("storm".to_owned());
    fork_supply_chain_branch_from_main(&world.runtime, storm.clone());
    let batch = lower_supply_chain_production_delta(
        &world.runtime,
        &world.program,
        &world.handles,
        &storm,
        &BTreeSet::new(),
        DeltaId::StormRerouteAurora,
    )
    .unwrap();
    commit_branch_batch(&world.runtime, storm.clone(), batch);
    let storm_identity = world.runtime.branch_identity(&storm).unwrap();
    let (_, storm_basis) = world.runtime.observe_branch(&storm_identity).unwrap();
    let storm_observation = storm_basis.observation();
    let storm_head = world
        .runtime
        .history()
        .branch_head_for_observation(&storm_observation)
        .unwrap()
        .unwrap();

    let merge_basis = world
        .runtime
        .history()
        .merge_branch_basis_for_observations(&storm_observation, &main_observation)
        .unwrap();

    assert_eq!(merge_basis.source_head().commit_id, storm_head.commit_id);
    assert_eq!(merge_basis.target_head().commit_id, main_head.commit_id);
    assert_eq!(
        merge_basis.merge_base().commit().commit_id,
        main_head.commit_id
    );
}

#[test]
fn merge_history_preserves_source_and_target_observation_denials() {
    let (owner, _) = certified_supply_chain_world(SupplyChainScale::court());
    let (foreign, _) = certified_supply_chain_world(SupplyChainScale::court());
    let owner_identity = owner.runtime.main_branch_identity();
    let foreign_identity = foreign.runtime.main_branch_identity();
    let (_, owner_basis) = owner.runtime.observe_branch(&owner_identity).unwrap();
    let (_, foreign_basis) = foreign.runtime.observe_branch(&foreign_identity).unwrap();
    let owner_observation = owner_basis.observation();
    let foreign_observation = foreign_basis.observation();

    let source_denial = owner
        .runtime
        .history()
        .merge_branch_basis_for_observations(&foreign_observation, &owner_observation)
        .expect_err("foreign source observation must be denied as the source");
    assert!(matches!(
        source_denial,
        RelationalMergeBranchBasisDenial::SourceObservationDenied(
            RelationalBranchBasisDenial::ForeignRuntime { .. }
        )
    ));

    let target_denial = owner
        .runtime
        .history()
        .merge_branch_basis_for_observations(&owner_observation, &foreign_observation)
        .expect_err("foreign target observation must be denied as the target");
    assert!(matches!(
        target_denial,
        RelationalMergeBranchBasisDenial::TargetObservationDenied(
            RelationalBranchBasisDenial::ForeignRuntime { .. }
        )
    ));
}
