use crate::{PlacementExecutionOrigin, PlacementObservationScopeClass, WORTHStoreBuilder};

use super::super::harness::fixtures::stores::unique_test_store_path;
use super::world::{conservative_policy, tiering_phase3_local_fixture};

#[test]
fn local_file_reopen_restores_manifest_bounded_truth_after_cutover() {
    let path = unique_test_store_path("worth-store-tiering-phase3-local");
    let (mut store, _, _, snapshot_id) = tiering_phase3_local_fixture(path.clone());
    let snapshot_basis_label = format!("snapshot:{snapshot_id}");

    let report = store
        .plan_authoritative_tier_move(
            conservative_policy(),
            PlacementObservationScopeClass::RetainedBasis,
            &snapshot_basis_label,
            PlacementExecutionOrigin::Background,
        )
        .unwrap();
    let plan = report
        .tier_move_plan()
        .cloned()
        .expect("authoritative plan");
    let intent = store.prepare_authoritative_tier_move(plan).unwrap();
    let transferred = store.transfer_tier_replica(intent).unwrap();
    let verified = store.verify_tier_replica(transferred).unwrap();
    let cutover = store.cutover_tier_replica(verified).unwrap();
    assert_eq!(
        cutover.canonical_residence(),
        crate::TierResidenceClass::Warm
    );
    drop(store);

    let reopened = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let manifest = reopened.canonical_residency_manifest();
    let recovered = reopened.recover_tiering_state().unwrap();

    assert_eq!(
        manifest.resident_artifact_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &[format!("retained_authority:{snapshot_basis_label}")]
    );
    assert_eq!(manifest, recovered);

    let counters = reopened.milestone_13_counter_contract();
    assert_eq!(counters.placement_state_manifest_load_count, 1);
    assert_eq!(counters.placement_state_recovery_count, 1);
}
