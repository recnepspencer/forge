use crate::capability::{
    ComponentCanvasSpatialContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::planning::plan_topology::{
    WorthUiPlanRegionMutation, WorthUiPlanRegionTransition,
};
use crate::runtime::{WorthUiExecutablePlanDecisionKind, WorthUiPlanNodeInputFamily};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

use super::replacement_impact_test_support::{artifact_from_modules, launch_runtime};

const CANVAS_COUNT: usize = 256;
const CHANGED_INDEX: usize = 137;

#[test]
fn one_changed_canvas_region_rebuilds_without_copying_unaffected_regions() {
    let active_app = canvas_app(2_048);
    let candidate_app = canvas_app(4_096);
    let active_artifact = canvas_artifact(&active_app);
    let candidate_artifact = canvas_artifact(&candidate_app);
    let active_runtime = launch_runtime(&active_app, active_artifact);
    let candidate_runtime = launch_runtime(&candidate_app, candidate_artifact);
    let active_plan = active_runtime.active.active_plan();
    let candidate_plan = candidate_runtime.active.active_plan();
    let active_store = active_plan.exact_plan().region_store();
    let candidate_store = candidate_plan.exact_plan().region_store();
    let changed_identity = candidate_store
        .canonical_identities()
        .into_iter()
        .find(|identity| {
            candidate_store
                .schema_for(identity)
                .and_then(|schema| schema.input().spatial_meaning_reference())
                .is_some_and(|meaning| meaning.contract().visible_primitive_limit() == 4_096)
        })
        .expect("candidate identifies the one changed canvas region");
    let candidate_schema = candidate_store
        .schema_for(&changed_identity)
        .expect("candidate contains the changed canvas region")
        .clone();

    let successor =
        active_store.successor(vec![WorthUiPlanRegionMutation::Upsert(candidate_schema)]);
    assert_eq!(successor.counters().region_construction_count(), 1);
    assert_eq!(successor.counters().reuse_count(), 0);
    assert_eq!(
        successor.evidence()[0].transition(),
        WorthUiPlanRegionTransition::Replaced
    );
    let successor_store = successor.into_store();
    assert!(!active_store.shares_exact_region_storage_with(&successor_store, &changed_identity));

    let unchanged = active_store
        .canonical_identities()
        .into_iter()
        .filter(|region| region != &changed_identity)
        .filter(|region| {
            active_store
                .schema_for(region)
                .is_some_and(|schema| schema.input().spatial_meaning_reference().is_some())
        })
        .collect::<Vec<_>>();
    assert_eq!(unchanged.len(), CANVAS_COUNT - 1);
    assert!(unchanged
        .iter()
        .all(|region| active_store.shares_exact_region_storage_with(&successor_store, region)));
}

#[test]
fn ordinary_to_canvas_transition_is_non_equivalent_and_leaves_no_ordinary_residue() {
    const ID: &str = "workspace.component.lane_transition";
    let ordinary_app = WorthUi::app()
        .register_component(ordinary_descriptor(ID))
        .freeze()
        .expect("ordinary predecessor freezes");
    let canvas_app = WorthUi::app()
        .register_component(ordinary_descriptor(ID).with_canvas_spatial_contract(
            ComponentCanvasSpatialContract::new(64, 2, 1).expect("canvas successor is bounded"),
        ))
        .freeze()
        .expect("canvas successor freezes");
    let artifact = |app: &WorthUiApp| {
        artifact_from_modules(
            app,
            [
                WorthUiRustAuthoredArtifactInputModule::new("app/lane-transition.wui")
                    .with_component(ID),
            ],
        )
    };
    let predecessor = launch_runtime(&ordinary_app, artifact(&ordinary_app));
    let successor = launch_runtime(&canvas_app, artifact(&canvas_app));
    let predecessor_plan = predecessor.active.active_plan();
    let successor_plan = successor.active.active_plan();
    let equivalence =
        crate::runtime::planning::plan_equivalence::WorthUiExecutionPlanDigestor::compare(
            predecessor_plan.exact_plan(),
            successor_plan.exact_plan(),
        );
    assert_eq!(
        equivalence.decision_kind(),
        WorthUiExecutablePlanDecisionKind::RebuildRequired
    );

    let successor_store = successor_plan.exact_plan().region_store();
    let identity = successor_store
        .canonical_identities()
        .into_iter()
        .next()
        .expect("successor has one component region");
    let successor_schema = successor_store
        .schema_for(&identity)
        .expect("successor schema")
        .clone();
    let rebuilt = predecessor_plan
        .exact_plan()
        .region_store()
        .successor(vec![WorthUiPlanRegionMutation::Upsert(successor_schema)])
        .into_store();
    assert_eq!(
        rebuilt.family_count(WorthUiPlanNodeInputFamily::ComponentInvocation),
        0
    );
    assert_eq!(
        rebuilt.family_count(WorthUiPlanNodeInputFamily::CanvasSpatial),
        1
    );
    let rebuilt_schema = rebuilt
        .schema_for(&identity)
        .expect("rebuilt canvas schema");
    assert!(rebuilt_schema.input().ordinary_meaning().is_none());
    assert!(rebuilt_schema.input().spatial_meaning_reference().is_some());
}

fn canvas_app(changed_primitive_limit: u32) -> WorthUiApp {
    let mut builder = WorthUi::app();
    for index in 0..CANVAS_COUNT {
        let primitive_limit = if index == CHANGED_INDEX {
            changed_primitive_limit
        } else {
            2_048
        };
        builder = builder.register_component(canvas_descriptor(index, primitive_limit));
    }
    builder.freeze().expect("large canvas capabilities freeze")
}

fn canvas_artifact(app: &WorthUiApp) -> crate::source::WorthUiArtifact {
    let mut module = WorthUiRustAuthoredArtifactInputModule::new("app/canvas.wui");
    for index in 0..CANVAS_COUNT {
        module = module.with_component(component_id(index));
    }
    artifact_from_modules(app, [module])
}

fn canvas_descriptor(index: usize, primitive_limit: u32) -> ComponentDescriptor {
    ordinary_descriptor(component_id(index)).with_canvas_spatial_contract(
        ComponentCanvasSpatialContract::new(primitive_limit, 8, 4)
            .expect("canvas scale limit is positive"),
    )
}

fn ordinary_descriptor(id: impl Into<String>) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id.into()).expect("component id is valid"),
        ComponentPropSchema::named("canvas.region.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn component_id(index: usize) -> String {
    format!("workspace.component.canvas_region_{index:04}")
}
