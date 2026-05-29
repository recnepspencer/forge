use std::sync::Arc;

use forge_relational::facade::history::BranchId;
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeSignalInvalidationDelivery, BridgeTruthViewEvaluationRequest,
    InvalidationSink, SignalBridgeSinkError, TruthBranchIdentity,
};
use schema::facade::topology_authoring::seed_minimal_topology;
use schema::facade::platform::authority::{
    milestone_two_invalidation_declarations, DerivedInvalidationTarget, DerivedTruthSurfaceKind,
};

use crate::certification::BridgeTraceAnchor;
use crate::projection::runtime_boundary::bridge::{
    build_milestone_one_bridge, milestone_one_bridge_aspect_registrations,
    milestone_one_bridge_mapping_registrations,
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
    let mappings = milestone_one_bridge_mapping_registrations();
    let aspects = milestone_one_bridge_aspect_registrations();
    let declarations = milestone_two_invalidation_declarations();

    assert_eq!(mappings.len(), declarations.len());
    assert_eq!(aspects.len(), declarations.len());
    for declaration in declarations {
        assert!(mappings.iter().any(|registration| {
            registration.mapping_id().as_str() == format!(":m2:{}", declaration.declaration_id)
                && registration.signal_scope().as_str() == declaration.target.bridge_scope()
                && registration.truth_scope().aspect_selector()
                    == &forge_runtime_bridge::facade::MappingSelector::exact(
                        declaration.truth_patch_field,
                    )
        }));
        assert!(aspects.iter().any(|registration| {
            registration.registration_id().as_str()
                == format!(":m2:aspect:{}", declaration.declaration_id)
                && registration.truth_scope().aspect_selector()
                    == &forge_runtime_bridge::facade::MappingSelector::exact(
                        declaration.truth_patch_field,
                    )
                && registration.truth_surface_kind() == match declaration.truth_surface_kind {
                    DerivedTruthSurfaceKind::EntityField => {
                        forge_runtime_bridge::facade::TruthDeltaSurfaceKind::EntityField
                    }
                    DerivedTruthSurfaceKind::EntityRelationEndpoint => {
                        forge_runtime_bridge::facade::TruthDeltaSurfaceKind::EntityRelationEndpoint
                    }
                }
        }));
    }
}

#[test]
fn milestone_two_bridge_target_vocabulary_remains_canonical() {
    let declarations = milestone_two_invalidation_declarations();

    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == DerivedInvalidationTarget::TopologyStructure));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == DerivedInvalidationTarget::TopologyOwnership));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == DerivedInvalidationTarget::TopologyBoundary));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == DerivedInvalidationTarget::TopologyRadial));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == DerivedInvalidationTarget::NamingPersistentName));
}

#[test]
fn milestone_one_bridge_builder_registers_mapping_pack() {
    let runtime =
        Arc::new(crate::validation::reference_integrity::build_milestone_one_runtime().unwrap());

    let _bridge = build_milestone_one_bridge(runtime, RecordingSink).unwrap();
}

#[test]
fn milestone_one_bridge_routes_and_evaluates_seeded_commit() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let _seeded = seed_minimal_topology(&mut runtime, "bridge--seeded").expect("seed  topology");
    let history = runtime.history();
    let head_commit_id = history
        .branch_head(&BranchId("main".to_string()))
        .expect("seed should publish a main-branch head")
        .commit_id;

    let runtime = Arc::new(runtime);
    let bridge = build_milestone_one_bridge(Arc::clone(&runtime), RecordingSink)
        .expect(" bridge should build");

    let route = bridge
        .route(format!("commit-{}", head_commit_id.0))
        .expect(" bridge should route a seeded commit");
    let evaluation = bridge
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new("main"),
        ))
        .expect(" bridge should evaluate current main branch head");

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
fn bridge_trace_anchor_tracks_real_runtime_diagnostics() {
    let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let _seeded = seed_minimal_topology(&mut runtime, "bridge--explained").expect("seed  topology");
    let history = runtime.history();
    let head_commit_id = history
        .branch_head(&BranchId("main".to_string()))
        .expect("seed should publish a main-branch head")
        .commit_id;

    let runtime = Arc::new(runtime);
    let bridge = build_milestone_one_bridge(Arc::clone(&runtime), RecordingSink)
        .expect(" bridge should build");
    let _route = bridge
        .route(format!("commit-{}", head_commit_id.0))
        .expect(" bridge should route a seeded commit");
    let _evaluation = bridge
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new("main"),
        ))
        .expect(" bridge should evaluate current main branch head");

    let route_records = bridge.diagnostics().route_records();
    let historical_records = bridge.diagnostics().historical_evaluation_records();
    let anchor = BridgeTraceAnchor::new(
        route_records
            .iter()
            .map(|record| record.route_identity().as_str().to_string())
            .collect(),
        route_records
            .iter()
            .map(|record| record.invalidation_identity().as_str().to_string())
            .collect(),
        route_records
            .iter()
            .map(|record| record.source_snapshot().as_str().to_string())
            .chain(historical_records.iter().map(|record| {
                record
                    .decision_log()
                    .snapshot_identity()
                    .as_str()
                    .to_string()
            }))
            .collect(),
        historical_records
            .iter()
            .map(|record| record.record_identity().as_str().to_string())
            .collect(),
    );
    assert_eq!(anchor.route_identities.len(), 1);
    assert_eq!(anchor.invalidation_identities.len(), 1);
    assert_eq!(anchor.historical_record_identities.len(), 1);
    assert!(!anchor.snapshot_identities.is_empty());
}




