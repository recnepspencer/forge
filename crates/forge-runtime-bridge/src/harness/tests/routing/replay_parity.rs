use crate::facade::BridgeRouteRequest;
use crate::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyDeclaration,
    BridgePolicyDeclarationIdentity, BridgeRequestKind,
};

use super::super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

#[test]
fn replayed_slice_route_matches_original_canonical_slice_artifact() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("slice route should plan"),
        )
        .expect("slice route should deliver");
    let canonical = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("canonical route record should be retained");
    let replay = runtime
        .replay_canonical_record(&canonical)
        .expect("canonical slice route should replay");

    assert_eq!(
        replay.subscription_slice_identity(),
        result.result_summary().subscription_slice_identity()
    );
    assert_eq!(
        replay.route_identity(),
        result.result_summary().route_identity()
    );
    assert_eq!(
        replay.invalidation_identity(),
        result.result_summary().invalidation_identity()
    );
}

#[test]
fn replayed_policy_scoped_route_preserves_route_policy_digest_in_route_record() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:route-record-visible"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Standard,
        false,
        true,
    );
    let contract = runtime
        .admit_policy_declaration(declaration)
        .expect("policy declaration should admit");
    let lowered = runtime.lower_admitted_policy(&contract);
    let route_policy = runtime
        .project_route_planning_policy(&lowered)
        .expect("route planning policy should project");

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch_with_route_policy(
                    BridgeRouteRequest::for_commit("commit-a"),
                    &route_policy,
                )
                .expect("policy scoped route should plan"),
        )
        .expect("policy scoped route should deliver");
    let canonical = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("canonical route record should be retained");
    let replay = runtime
        .replay_canonical_record(&canonical)
        .expect("policy scoped route should replay");
    let record = runtime
        .diagnostics()
        .last_route_record()
        .expect("route record should be retained");

    assert_eq!(
        result.result_summary().route_planning_policy_digest(),
        Some(route_policy.digest())
    );
    assert_eq!(
        record.route_planning_policy_digest(),
        Some(route_policy.digest())
    );
    assert_eq!(
        canonical
            .decode()
            .expect("canonical route record should decode")
            .route_planning_policy_digest(),
        Some(route_policy.digest())
    );
    assert_eq!(
        replay.route_identity(),
        result.result_summary().route_identity()
    );
}
