use forge_harness::facade::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
use forge_harness::runtime::HarnessAdapter;

use crate::facade::{BridgeMappingContext, BridgeRouteRequest};

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::{BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, committed_patch, committed_patch_items, registration, snapshot,
};

#[test]
fn bridge_prepared_delivery_is_equivalent_to_one_shot_delivery() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let one_shot = left_runtime
        .deliver_invalidation(
            left_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("one-shot route should plan"),
        )
        .expect("one-shot delivery should succeed");
    let prepared = right_runtime.prepare_delivery(
        right_runtime
            .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
            .expect("prepared route should plan"),
    );
    let staged = right_runtime
        .deliver_prepared(prepared)
        .expect("prepared delivery should succeed");

    assert_eq!(
        one_shot.result_summary().route_identity(),
        staged.result_summary().route_identity()
    );
    assert_eq!(
        one_shot.result_summary().invalidation_identity(),
        staged.result_summary().invalidation_identity()
    );
    assert_eq!(
        one_shot.result_summary().subscription_slice_identity(),
        staged.result_summary().subscription_slice_identity()
    );
    assert_eq!(one_shot.counters(), staged.counters());
}

#[test]
fn bridge_empty_mapping_context_is_equivalent_to_default_planning_path() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let default_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("default planning should succeed");
    let explicit_route = right_runtime
        .plan_committed_patch_with_mapping_context(
            BridgeRouteRequest::for_commit("commit-a"),
            BridgeMappingContext::empty(),
        )
        .expect("explicit empty mapping context planning should succeed");

    assert_eq!(default_route.route_identity(), explicit_route.route_identity());
    assert_eq!(default_route.source_digest(), explicit_route.source_digest());
    assert_eq!(
        default_route.planning_provenance().digest(),
        explicit_route.planning_provenance().digest()
    );
    assert_eq!(
        default_route.lowering_provenance().digest(),
        explicit_route.lowering_provenance().digest()
    );
    assert_eq!(default_route.read_packet(), explicit_route.read_packet());
    assert_eq!(default_route.counters(), explicit_route.counters());
}

#[test]
fn bridge_route_identity_is_stable_across_equivalent_surface_spellings() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "field:name"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left_route = left_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("unprefixed field route should plan");
    let right_route = right_runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("prefixed field route should plan");

    assert_eq!(left_route.route_identity(), right_route.route_identity());
    assert_eq!(left_route.read_packet(), right_route.read_packet());
    assert_eq!(
        left_route.lowering_summary().subscription_slice_identity(),
        right_route.lowering_summary().subscription_slice_identity()
    );
}

#[test]
fn bridge_route_identity_is_stable_when_patch_items_arrive_out_of_order_with_duplicates() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-canonical-patch-order",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                "commit-a",
                "patch-a",
                "snapshot-a",
                vec![
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                ],
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut left = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut left, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut left, &fixture)
        .expect("bridge harness load fixture");
    let left_run = adapter
        .execute(&mut left, &fixture, &request, &profile)
        .expect("bridge harness execute");

    let reordered_fixture = ScenarioPlan::new(
        "bridge-canonical-patch-order-reordered",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_items(
                "commit-a",
                "patch-a",
                "snapshot-a",
                vec![
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                    crate::facade::BridgeCommittedPatchItem::new("user", "profile", "name"),
                ],
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mut right = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut right, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut right, &reordered_fixture)
        .expect("bridge harness load fixture");
    let right_run = adapter
        .execute(&mut right, &reordered_fixture, &request, &profile)
        .expect("bridge harness execute");

    assert_eq!(
        left_run.summary["route_identity"],
        right_run.summary["route_identity"]
    );
}
