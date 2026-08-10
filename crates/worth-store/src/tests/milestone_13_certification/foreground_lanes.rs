use crate::{
    ContinuationRetentionStatus, CursorContinuationRequest, PlacementBoundArtifactRef,
    PlacementExecutionOrigin, PlacementObservationScopeClass, PlacementRaceOutcome, WORTHStore,
    WORTHStoreBuilder,
};
use worth_relational::facade::history::BranchId;

use super::super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};
use super::super::live_query::helpers::stable_basis_request_for_store;
use super::world::{build_store, conservative_policy, demo_budget};

fn admit_branch_head_transfer(store: &mut WORTHStore, branch_id: &BranchId) {
    let plan = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::Branch,
            &branch_id.0,
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let _ = store.transfer_tier_replica(intent).unwrap();
}

pub(super) fn foreground_read_interleaving_lane(builder: WORTHStoreBuilder) -> WORTHStore {
    let (mut store, _) = build_store(builder);
    let export = store.export_authoritative_records().into_canonicalized();
    let envelope = export.commit_envelopes.first().unwrap().envelope.clone();
    let branch_id = envelope.branch_context.clone();
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            envelope.commit.commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    admit_branch_head_transfer(&mut store, &branch_id);
    let report = store.observe_stable_basis_interleaving(&basis).unwrap();
    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );
    store
}

pub(super) fn continuation_interleaving_lane(builder: WORTHStoreBuilder) -> WORTHStore {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let first_envelope = latest_envelope(&runtime);
    let branch_id = first_envelope.branch_context.clone();
    let first_commit_id = first_envelope.commit.commit_id;
    let mut store = builder.build().unwrap();
    store.append_canonical_commit(first_envelope).unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "alpha-2", Some(branch_id.clone()));
    let second_envelope = latest_envelope(&runtime);
    store.append_canonical_commit(second_envelope).unwrap();
    let basis = store
        .read_stable_basis(stable_basis_request_for_store(
            &store,
            branch_id.clone(),
            first_commit_id,
            "schema-support:v1",
            ContinuationRetentionStatus::Retained,
        ))
        .unwrap();
    store
        .acknowledge_cursor(crate::DurableCursorAcknowledgeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id.clone(),
            "demo-feed",
            "schema:v1",
            1,
            first_commit_id,
        ))
        .unwrap();
    admit_branch_head_transfer(&mut store, &branch_id);
    let plan = store
        .plan_cursor_continuation(CursorContinuationRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
            basis,
            demo_budget(),
        ))
        .unwrap();
    let result = store.execute_cursor_continuation(plan.clone()).unwrap();
    let report = store
        .observe_continuation_interleaving(&plan, Some(&result))
        .unwrap();
    assert_eq!(
        report.observation().race_outcome(),
        PlacementRaceOutcome::TransferObserved
    );
    store
}
