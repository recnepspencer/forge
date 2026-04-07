use crate::facade::{BridgeProducerAuthorityKind, BridgeRouteRequest};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration,
};

#[test]
fn bridge_delivery_and_result_surfaces_expose_planning_and_lowering_proof_contracts() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime_with_aspects(
        source,
        sink.clone(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan route with proof metadata");
    let result = runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver route with proof metadata");
    let delivery = sink.last_delivery().expect("recorded sink delivery");

    assert_eq!(
        result.result_summary().producer_metadata().authority_kind(),
        BridgeProducerAuthorityKind::BridgeHarnessFixture
    );
    assert!(result
        .result_summary()
        .planning_provenance_digest()
        .starts_with("planning-provenance:sha256:"));
    assert!(result
        .result_summary()
        .planning_summary_digest()
        .starts_with("planning-summary:sha256:"));
    assert!(result
        .result_summary()
        .lowering_provenance_digest()
        .starts_with("lowering-provenance:sha256:"));
    assert!(result
        .result_summary()
        .lowering_summary_digest()
        .starts_with("lowering-summary:sha256:"));
    assert_eq!(
        result.result_summary().planning_provenance_digest(),
        delivery.delivery.planning_provenance_digest()
    );
    assert_eq!(
        result.result_summary().lowering_summary_digest(),
        delivery.delivery.lowering_summary_digest()
    );
    assert_eq!(
        result.result_summary().mapping_context_digest(),
        delivery.delivery.mapping_context_digest()
    );
    assert_eq!(
        delivery.delivery.producer_metadata().authority_kind(),
        BridgeProducerAuthorityKind::BridgeHarnessFixture
    );
}
