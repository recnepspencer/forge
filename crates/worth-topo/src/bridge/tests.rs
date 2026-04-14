use std::sync::Arc;

use forge_relational::facade::history::BranchId;
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeSignalInvalidationDelivery, BridgeTruthViewEvaluationRequest,
    InvalidationSink, SignalBridgeSinkError, TruthBranchIdentity,
};
use worth_schema::facade::seed_minimal_topology;

use crate::bridge::{
    worth_milestone_one_bridge_aspect_registrations,
    build_worth_milestone_one_bridge,
    worth_milestone_one_bridge_mapping_registrations,
};

#[derive(Clone)]
struct RecordingSink;

impl InvalidationSink for RecordingSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

#[test]
fn milestone_one_bridge_registration_packs_cover_topology_and_naming_aspects() {
    let mappings = worth_milestone_one_bridge_mapping_registrations();
    let aspects = worth_milestone_one_bridge_aspect_registrations();

    assert_eq!(mappings.len(), 8);
    assert_eq!(aspects.len(), 8);
}

#[test]
fn milestone_one_bridge_builder_registers_worth_mapping_pack() {
    let runtime = Arc::new(crate::runtime_invariants::build_worth_milestone_one_runtime().unwrap());

    let _bridge = build_worth_milestone_one_bridge(runtime, RecordingSink).unwrap();
}

#[test]
fn milestone_one_bridge_routes_and_evaluates_seeded_worth_commit() {
    let mut runtime = crate::runtime_invariants::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let _seeded =
        seed_minimal_topology(&mut runtime, "bridge-worth-seeded").expect("seed worth topology");
    let history = runtime.history();
    let head_commit_id = history
        .branch_head(&BranchId("main".to_string()))
        .expect("seed should publish a main-branch head")
        .commit_id;

    let runtime = Arc::new(runtime);
    let bridge = build_worth_milestone_one_bridge(Arc::clone(&runtime), RecordingSink)
        .expect("worth bridge should build");

    let route = bridge
        .route(format!("commit-{}", head_commit_id.0))
        .expect("worth bridge should route a seeded commit");
    let evaluation = bridge
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new("main"),
        ))
        .expect("worth bridge should evaluate current main branch head");

    assert_eq!(
        route.result().receipt().snapshot_identity(),
        evaluation.snapshot_identity()
    );
    assert_eq!(bridge.diagnostics().route_records().len(), 1);
    assert_eq!(bridge.diagnostics().historical_evaluation_records().len(), 1);
}
