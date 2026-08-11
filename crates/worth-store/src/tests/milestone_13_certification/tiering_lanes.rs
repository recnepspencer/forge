use crate::{
    ColdDerivedFamilyPolicy, PlacementBoundArtifactRef, PlacementExecutionOrigin,
    PlacementObservationScopeClass, WORTHStore, WORTHStoreBuilder,
};

use super::world::{build_store, conservative_policy};

fn execute_authoritative_tier_movement(store: &mut WORTHStore, snapshot_id: u64) {
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");
    let authoritative = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let authoritative_intent = store
        .prepare_authoritative_tier_move(authoritative)
        .unwrap();
    let authoritative_transferred = store.transfer_tier_replica(authoritative_intent).unwrap();
    let authoritative_verified = store
        .verify_tier_replica(authoritative_transferred)
        .unwrap();
    let authoritative_cutover = store.cutover_tier_replica(authoritative_verified).unwrap();
    store.retire_tier_replica(authoritative_cutover).unwrap();
}

fn execute_derived_tier_movement(store: &mut WORTHStore, snapshot_id: u64) {
    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let derived_intent = store.prepare_derived_tier_move(derived).unwrap();
    let derived_transferred = store.transfer_tier_replica(derived_intent).unwrap();
    let derived_verified = store.verify_tier_replica(derived_transferred).unwrap();
    let derived_cutover = store.cutover_tier_replica(derived_verified).unwrap();
    store.retire_tier_replica(derived_cutover).unwrap();
}

fn execute_coalesced_cold_recall(store: &mut WORTHStore, snapshot_id: u64) {
    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    store
        .admit_inflight_cold_recall(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let second = store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    assert_eq!(
        second.disposition(),
        crate::RecallExecutionDisposition::CoalescedJoin
    );
    assert!(second.completion_witness().is_none());
}

pub(super) fn execute_tiering_batch(store: &mut WORTHStore, snapshot_id: u64) {
    execute_authoritative_tier_movement(store, snapshot_id);
    execute_derived_tier_movement(store, snapshot_id);
    execute_coalesced_cold_recall(store, snapshot_id);
}

pub(super) fn interleaved_tiering_lane(builder: WORTHStoreBuilder) -> (WORTHStore, u64) {
    let (mut store, snapshot_id) = build_store(builder);
    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_derived_tier_move(derived).unwrap();
    let _ = store.transfer_tier_replica(intent).unwrap();

    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let joined = store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    assert_eq!(joined.artifact_key(), format!("snapshot:{snapshot_id}"));
    (store, snapshot_id)
}

pub(super) fn recalled_tiering_lane(builder: WORTHStoreBuilder) -> (WORTHStore, u64) {
    let (mut store, snapshot_id) = build_store(builder);
    let derived = store
        .plan_derived_tier_move(
            conservative_policy(),
            ColdDerivedFamilyPolicy::SnapshotFamily,
            &snapshot_id.to_string(),
            PlacementExecutionOrigin::Background,
        )
        .unwrap()
        .tier_move_plan()
        .cloned()
        .unwrap();
    let intent = store.prepare_derived_tier_move(derived).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    store.retire_tier_replica(cutover).unwrap();

    let cold = store
        .plan_cold_recall_lease(
            PlacementBoundArtifactRef::snapshot_family(snapshot_id.to_string()),
            PlacementExecutionOrigin::Foreground,
        )
        .unwrap();
    let recalled = store
        .execute_cold_recall(
            cold.cold_recall_lease().cloned().unwrap(),
            cold.recall_witness().cloned().unwrap(),
        )
        .unwrap();
    assert_eq!(
        recalled.disposition(),
        crate::RecallExecutionDisposition::Executed
    );
    assert!(recalled.completion_witness().is_some());
    (store, snapshot_id)
}
