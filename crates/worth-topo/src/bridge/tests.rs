use std::sync::Arc;

use forge_relational::facade::history::BranchId;
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeSignalInvalidationDelivery, BridgeTruthViewEvaluationRequest,
    InvalidationSink, SignalBridgeSinkError, TruthBranchIdentity,
};
use worth_schema::facade::{
    explain_bridge_trace, seed_minimal_topology, worth_milestone_two_invalidation_declarations,
    WorthBridgeTraceAnchor, WorthDerivedInvalidationTarget, WorthDerivedTruthSurfaceKind,
};

use crate::bridge::{
    build_worth_milestone_one_bridge, worth_milestone_one_bridge_aspect_registrations,
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
    let declarations = worth_milestone_two_invalidation_declarations();

    assert_eq!(mappings.len(), declarations.len());
    assert_eq!(aspects.len(), declarations.len());
    for declaration in declarations {
        assert!(mappings.iter().any(|registration| {
            registration.mapping_id().as_str() == format!("worth:m2:{}", declaration.declaration_id)
                && registration.signal_scope().as_str() == declaration.target.bridge_scope()
                && registration.truth_scope().aspect_selector()
                    == &forge_runtime_bridge::facade::MappingSelector::exact(
                        declaration.truth_patch_field,
                    )
        }));
        assert!(aspects.iter().any(|registration| {
            registration.registration_id().as_str()
                == format!("worth:m2:aspect:{}", declaration.declaration_id)
                && registration.truth_scope().aspect_selector()
                    == &forge_runtime_bridge::facade::MappingSelector::exact(
                        declaration.truth_patch_field,
                    )
                && registration.truth_surface_kind() == match declaration.truth_surface_kind {
                    WorthDerivedTruthSurfaceKind::EntityField => {
                        forge_runtime_bridge::facade::TruthDeltaSurfaceKind::EntityField
                    }
                    WorthDerivedTruthSurfaceKind::EntityRelationEndpoint => {
                        forge_runtime_bridge::facade::TruthDeltaSurfaceKind::EntityRelationEndpoint
                    }
                }
        }));
    }
}

#[test]
fn milestone_two_bridge_target_vocabulary_remains_canonical() {
    let declarations = worth_milestone_two_invalidation_declarations();

    assert!(
        declarations
            .iter()
            .any(|declaration| declaration.target
                == WorthDerivedInvalidationTarget::TopologyStructure)
    );
    assert!(
        declarations
            .iter()
            .any(|declaration| declaration.target
                == WorthDerivedInvalidationTarget::TopologyOwnership)
    );
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::TopologyBoundary));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::TopologyRadial));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target
            == WorthDerivedInvalidationTarget::NamingPersistentName));
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
    assert_eq!(
        bridge.diagnostics().historical_evaluation_records().len(),
        1
    );
}

#[test]
fn bridge_trace_explanation_queries_real_runtime_diagnostics() {
    let mut runtime = crate::runtime_invariants::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();

    let _seeded =
        seed_minimal_topology(&mut runtime, "bridge-worth-explained").expect("seed worth topology");
    let history = runtime.history();
    let head_commit_id = history
        .branch_head(&BranchId("main".to_string()))
        .expect("seed should publish a main-branch head")
        .commit_id;

    let runtime = Arc::new(runtime);
    let bridge = build_worth_milestone_one_bridge(Arc::clone(&runtime), RecordingSink)
        .expect("worth bridge should build");
    let _route = bridge
        .route(format!("commit-{}", head_commit_id.0))
        .expect("worth bridge should route a seeded commit");
    let _evaluation = bridge
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new("main"),
        ))
        .expect("worth bridge should evaluate current main branch head");

    let route_records = bridge.diagnostics().route_records();
    let historical_records = bridge.diagnostics().historical_evaluation_records();
    let anchor = WorthBridgeTraceAnchor::new(
        route_records
            .iter()
            .map(|record| record.route_identity().as_str().to_string()),
        route_records
            .iter()
            .map(|record| record.invalidation_identity().as_str().to_string()),
        route_records
            .iter()
            .map(|record| record.source_snapshot().as_str().to_string())
            .chain(historical_records.iter().map(|record| {
                record
                    .decision_log()
                    .snapshot_identity()
                    .as_str()
                    .to_string()
            })),
        historical_records
            .iter()
            .map(|record| record.record_identity().as_str().to_string()),
    );

    let narrative = explain_bridge_trace(bridge.diagnostics().raw(), &anchor, None);

    assert_eq!(narrative.route_count, 1);
    assert_eq!(narrative.historical_record_count, 1);
    assert!(narrative.headline.contains("Bridge retained"));
    assert!(narrative.routes[0]
        .summary
        .contains("lowered one truth event"));
    assert!(narrative.historical_records[0]
        .summary
        .contains("Historical evaluation"));
    assert_eq!(narrative.query_hints.len(), 2);
}
