use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime;
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::host::WorthUiOperationalHostAdapter;
use worth_ui::facade::query_binding::{
    worth_ui_domain_package, worth_ui_native_aspect_contracts, WorthUiInstalledLiveQueryView,
};
use worth_ui::facade::registry::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MeasurementConstraint, MeasurementValue, MosaicChildRule,
    MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority,
    MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission,
    MosaicScrollOwnership, MosaicSizingBehavior, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken, SurfacePlacementClass,
};
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiWatchedCandidateSubmission,
};
use worth_ui_query_binding::certification::worth_ui_query_snapshot_prerequisites;

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

pub(super) const ACTIVE_COMPONENT: &str = "workspace.component.query_lifecycle_active";
pub(super) const NEXT_COMPONENT: &str = "workspace.component.query_lifecycle_next";
pub(super) const FIRST_VIEW: &str = "inspector.first";
pub(super) const SECOND_VIEW: &str = "inspector.second";
const REGION: &str = "workspace.region.query_lifecycle";
const SIZING: &str = "workspace.sizing.query_lifecycle";

pub(super) fn application(
    first: WorthUiInstalledLiveQueryView,
    second: WorthUiInstalledLiveQueryView,
) -> worth_ui::facade::app::WorthUiApp {
    let snapshot = capability_application(first.clone(), second.clone());
    builder(first, second)
        .with_candidate_submission(submission(
            "query-lifecycle-active",
            ACTIVE_COMPONENT,
            &[FIRST_VIEW],
            snapshot.capabilities(),
        ))
        .freeze()
        .expect("source-backed Query app")
}

pub(super) fn capability_application(
    first: WorthUiInstalledLiveQueryView,
    second: WorthUiInstalledLiveQueryView,
) -> worth_ui::facade::app::WorthUiApp {
    builder(first, second)
        .freeze()
        .expect("capability snapshot")
}

pub(super) fn application_with_submission_and_host<Host>(
    first: WorthUiInstalledLiveQueryView,
    second: WorthUiInstalledLiveQueryView,
    submission: WorthUiWatchedCandidateSubmission,
    host: Host,
) -> worth_ui::facade::app::WorthUiApp
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    builder(first, second)
        .with_host(host)
        .with_candidate_submission(submission)
        .freeze()
        .expect("source-backed Query application")
}

pub(super) fn submission(
    provider_id: &str,
    component: &str,
    bindings: &[&str],
    capabilities: &worth_ui::facade::diagnostics::CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    let mut source = format!("component {component} {{ region {REGION} {{ sizing {SIZING}; }} }}");
    for binding in bindings {
        source.push_str(&format!("\nbinding {binding} {{}}"));
    }
    let workspace = FilesystemContractWorkspace::new(provider_id);
    workspace.write("app/main.wui", &source);
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem acquisition reads real Query-bound .wui bytes");
    workspace.close();
    snapshot
        .lower_to_candidate_submission(capabilities)
        .expect("filesystem source lowers against exact capabilities")
}

pub(super) fn installed_workspace(label: &str) -> runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiMeasurement")
        .aspect_contracts(worth_ui_native_aspect_contracts())
        .expect("native contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("measurement.value", "measurement.value")
        .expect("measurement aspect");
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
        .workspace(label)
        .expect("installed Query workspace")
}

fn builder(
    first: WorthUiInstalledLiveQueryView,
    second: WorthUiInstalledLiveQueryView,
) -> worth_ui::facade::app::WorthUiBuilder {
    WorthUi::app()
        .with_graph_world_profile(
            worth_ui::facade::graph::UiGraphWorldProfile::query_snapshot_basis(
                worth_ui_query_snapshot_prerequisites(
                    "query-replacement-lifecycle",
                    ["worth-ui.phase8", "query", "replacement"],
                ),
            ),
        )
        .register_component(component(ACTIVE_COMPONENT))
        .register_component(component(NEXT_COMPONENT))
        .register_mosaic_region_kind(
            MosaicRegionKindDescriptor::new(
                MosaicRegionKindId::new(REGION).expect("region id"),
                MosaicRegionRole::primary(),
            )
            .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
            .with_scroll_ownership(MosaicScrollOwnership::region_owned())
            .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
            .with_child_rule(MosaicChildRule::accepts_surfaces())
            .with_allowed_surface_class(SurfacePlacementClass::primary_region())
            .with_persistence(MosaicRegionPersistence::restorable())
            .with_clipping(MosaicClippingPosture::clip_to_region())
            .with_hit_test(MosaicHitTestPosture::participates()),
        )
        .register_mosaic_sizing_contract(
            MosaicSizingContractDescriptor::new(
                MosaicSizingContractId::new(SIZING).expect("sizing id"),
                MosaicSizingKind::fill(),
            )
            .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
            .with_resize_permission(MosaicResizePermission::user_resizable())
            .with_persistence(MosaicSizingPersistence::restorable())
            .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
            .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
            .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
            .with_named_measurement(NamedMeasurementDefinition::new(
                NamedMeasurementToken::new("workspace.measurement.query_lifecycle")
                    .expect("measurement token"),
                MeasurementValue::logical_pixels(320),
                MeasurementConstraint::between(
                    MeasurementValue::logical_pixels(200),
                    MeasurementValue::logical_pixels(640),
                ),
            )),
        )
        .register_query_view(first)
        .expect("first Query view registration")
        .register_query_view(second)
        .expect("second Query view registration")
}

fn component(identity: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(identity).expect("component id"),
        ComponentPropSchema::named(format!("{identity}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}
