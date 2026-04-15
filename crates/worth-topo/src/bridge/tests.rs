use std::sync::Arc;

use forge_relational::facade::history::BranchId;
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeSignalInvalidationDelivery, BridgeTruthViewEvaluationRequest,
    InvalidationSink, SignalBridgeSinkError, TruthBranchIdentity,
};
use worth_schema::facade::{
    seed_minimal_topology, worth_milestone_two_invalidation_declarations,
    WorthDerivedInvalidationTarget, WorthDerivedTruthSurfaceKind,
};

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
                && registration.truth_surface_kind()
                    == match declaration.truth_surface_kind {
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

    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::TopologyStructure));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::TopologyOwnership));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::TopologyBoundary));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::TopologyRadial));
    assert!(declarations
        .iter()
        .any(|declaration| declaration.target == WorthDerivedInvalidationTarget::NamingPersistentName));
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
