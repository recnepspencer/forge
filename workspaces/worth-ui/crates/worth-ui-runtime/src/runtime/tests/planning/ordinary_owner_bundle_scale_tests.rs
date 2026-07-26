use crate::capability::{
    CommandDescriptor, CommandId, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::planning::plan_topology::{
    WorthUiPlanRegionIdentity, WorthUiPlanRegionMutation, WorthUiPlanRegionSchema,
    WorthUiPlanRegionStore, WorthUiPlanRegionStoreDenial,
};
use crate::runtime::WorthUiPlanNodeInputFamily;
use worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule;

const COMMAND_COUNT: usize = 64;
const CHANGED_COMMAND: usize = 31;
const SURFACE_ID: &str = "workspace.surface.owner_bundle";

#[test]
fn one_real_command_change_is_owner_bounded_and_preserves_unrelated_handles() {
    let predecessor = command_store("Save");
    let candidate = command_store("Save changed");
    let (root, candidate_schemas) = candidate_owner_bundle(&candidate);
    let changed_command = changed_command_identity(&predecessor, &candidate_schemas);
    let unchanged_command = candidate_schemas
        .iter()
        .find(|schema| {
            schema.input().family() == WorthUiPlanNodeInputFamily::Command
                && schema.identity() != &changed_command
        })
        .expect("the wide owner has an unchanged sibling command")
        .identity()
        .clone();

    let small = scaled_predecessor(&predecessor, 0);
    let large = scaled_predecessor(&predecessor, 512);
    let unrelated = WorthUiPlanRegionIdentity::from_exact_basis("unrelated.region.0511");
    let unrelated_before = large
        .handle_for(&unrelated)
        .expect("large predecessor has the unrelated sentinel")
        .clone();
    let small_successor = small
        .try_successor(vec![WorthUiPlanRegionMutation::OwnerBundle {
            root: root.clone(),
            schemas: candidate_schemas.clone(),
        }])
        .expect("small owner replacement should seal");
    let large_successor = large
        .try_successor(vec![WorthUiPlanRegionMutation::OwnerBundle {
            root,
            schemas: candidate_schemas,
        }])
        .expect("large owner replacement should seal");

    assert_eq!(
        small_successor.counters().exact_comparison_count(),
        large_successor.counters().exact_comparison_count()
    );
    assert_eq!(
        small_successor.counters().region_construction_count(),
        large_successor.counters().region_construction_count()
    );
    assert_eq!(
        small_successor.counters().trie_node_copy_count(),
        large_successor.counters().trie_node_copy_count()
    );
    assert_eq!(
        large_successor.counters().exact_comparison_count(),
        COMMAND_COUNT + 2,
        "work is one root, one range, and the owner's command rows"
    );

    let large_successor = large_successor.into_store();
    let changed_before = predecessor.handle_for(&changed_command).unwrap();
    let changed_after = large_successor.handle_for(&changed_command).unwrap();
    assert_eq!(changed_before.stable_slot(), changed_after.stable_slot());
    assert_eq!(
        changed_before.slot_generation() + 1,
        changed_after.slot_generation()
    );
    assert_eq!(
        predecessor.handle_for(&unchanged_command),
        large_successor.handle_for(&unchanged_command),
        "an unrelated command in the same owner retains its exact handle"
    );
    assert_eq!(
        large_successor.handle_for(&unrelated),
        Some(&unrelated_before),
        "unrelated predecessor scale remains untouched"
    );
}

#[test]
fn owner_bundle_member_cannot_claim_foreign_ownership() {
    let predecessor = command_store("Save");
    let candidate = command_store("Save changed");
    let (root, mut schemas) = candidate_owner_bundle(&candidate);
    let member = schemas
        .iter_mut()
        .find(|schema| schema.identity() != &root)
        .expect("owner bundle has a member");
    *member = WorthUiPlanRegionSchema::from_node_input(
        member
            .input()
            .clone()
            .with_owner_identity_basis_for_test("foreign.owner"),
    );

    assert_eq!(
        predecessor
            .try_successor(vec![WorthUiPlanRegionMutation::OwnerBundle {
                root,
                schemas
            }])
            .unwrap_err(),
        WorthUiPlanRegionStoreDenial::OwnerManifestMismatch
    );
}

fn command_store(changed_label: &str) -> WorthUiPlanRegionStore {
    let app = command_app(changed_label);
    let artifact = super::replacement_impact_test_support::artifact_from_modules(
        &app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_surface(SURFACE_ID)],
    );
    let runtime = super::replacement_impact_test_support::launch_runtime(&app, artifact);
    runtime
        .active
        .active_plan()
        .exact_plan()
        .region_store()
        .clone()
}

fn command_app(changed_label: &str) -> WorthUiApp {
    let component_id = ComponentId::new("workspace.component.owner_bundle").unwrap();
    let mut builder = WorthUi::app().register_component(ComponentDescriptor::new(
        component_id.clone(),
        ComponentPropSchema::named("workspace.props.owner_bundle"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    ));
    let mut surface = SurfaceDescriptor::new(
        SurfaceId::new(SURFACE_ID).unwrap(),
        SurfaceKind::primary_content(),
        component_id,
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    );
    for index in 0..COMMAND_COUNT {
        let id =
            CommandId::new(format!("workspace.command.owner_bundle.command_{index:02}")).unwrap();
        let label = if index == CHANGED_COMMAND {
            changed_label
        } else {
            "Save"
        };
        builder = builder.register_command(CommandDescriptor::new(id.clone(), label));
        surface = surface.with_command_slot(id);
    }
    builder
        .register_surface(surface)
        .freeze()
        .expect("owner-bundle command capabilities should prepare")
}

fn candidate_owner_bundle(
    store: &WorthUiPlanRegionStore,
) -> (WorthUiPlanRegionIdentity, Vec<WorthUiPlanRegionSchema>) {
    let root = store
        .canonical_identities()
        .into_iter()
        .find(|identity| {
            store.schema_for(identity).is_some_and(|schema| {
                schema.input().family() == WorthUiPlanNodeInputFamily::LayoutRegion
                    && schema.input().owner_identity_basis().is_none()
            })
        })
        .expect("the surface owner root is present");
    let root_schema = store.schema_for(&root).unwrap();
    let mut schemas = vec![root_schema.clone()];
    schemas.extend(
        root_schema
            .input()
            .owned_region_identity_bases()
            .iter()
            .map(|identity| {
                store
                    .schema_for(&WorthUiPlanRegionIdentity::from_exact_basis(identity))
                    .expect("owner manifest member is present")
                    .clone()
            }),
    );
    (root, schemas)
}

fn changed_command_identity(
    predecessor: &WorthUiPlanRegionStore,
    candidate: &[WorthUiPlanRegionSchema],
) -> WorthUiPlanRegionIdentity {
    candidate
        .iter()
        .find(|schema| {
            schema.input().family() == WorthUiPlanNodeInputFamily::Command
                && predecessor
                    .schema_for(schema.identity())
                    .is_some_and(|previous| !previous.exactly_matches(schema))
        })
        .expect("one command descriptor changed")
        .identity()
        .clone()
}

fn scaled_predecessor(
    predecessor: &WorthUiPlanRegionStore,
    count: usize,
) -> WorthUiPlanRegionStore {
    let seed = super::plan_topology_test_support::topology_fixture()
        .1
        .node_inputs()[0]
        .clone();
    let mutations = (0..count)
        .map(|index| {
            WorthUiPlanRegionMutation::Upsert(WorthUiPlanRegionSchema::from_node_input(
                seed.clone()
                    .with_identity_basis_for_test(format!("unrelated.region.{index:04}")),
            ))
        })
        .collect();
    predecessor
        .try_successor(mutations)
        .expect("unrelated predecessor scale should seal")
        .into_store()
}
