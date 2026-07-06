use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
};
use crate::declaration::UiDeclarationArtifact;
use crate::facade::{
    ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, WorthUi, WorthUiApp,
};
use crate::runtime::candidate::rust_authored_replacement_candidate;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::runtime_test_modules::allocation_planning_test_support::allocation_planning;
use crate::runtime::{
    WorthUiCandidateAdmission, WorthUiComponentLoweringHook, WorthUiDiagnosticProjectionHook,
    WorthUiDurableStateFamily, WorthUiExecutionLaneSupport, WorthUiExecutionPlanInspection,
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameReceipt, WorthUiPendingActivation,
    WorthUiPlanLoweringDenial, WorthUiPlanNodeInputFamily, WorthUiReplacementCandidate,
    WorthUiReplacementCause, WorthUiRuntimeDiagnosticReport, WorthUiRuntimeHost,
    WorthUiRuntimeLaunch,
};
use crate::source::{
    WorthUiArtifact, WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler,
    WorthUiIdentitySeedLowerer, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiStructuralLegalityLowerer,
};
use std::borrow::Borrow;
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiTouchOriginFixtureVariant {
    Baseline,
    OverlayArtifact,
    SameArtifactExtraPlanHook,
}

pub struct WorthUiTouchOriginCertificationFixture {
    pub app: WorthUiApp,
    pub runtime: WorthUiRuntimeHost,
    pub inspection: WorthUiExecutionPlanInspection,
    pub frame_receipt: WorthUiOrdinaryLaneFrameReceipt,
    pub intent_candidate: WorthUiReplacementCandidate,
    pub diagnostic_report: WorthUiRuntimeDiagnosticReport,
}

impl WorthUiTouchOriginCertificationFixture {
    pub fn control_artifact(&self) -> &UiDeclarationArtifact {
        &self.app.declaration_artifacts()[0]
    }

    pub fn region_artifact(&self) -> &UiDeclarationArtifact {
        &self.app.declaration_artifacts()[1]
    }

    pub fn control_graph_node_identity(&self) -> crate::graph::UiGraphNodeIdentity {
        graph_node_identity(&self.app, self.control_artifact())
    }

    pub fn region_graph_node_identity(&self) -> crate::graph::UiGraphNodeIdentity {
        graph_node_identity(&self.app, self.region_artifact())
    }
}

pub fn runtime_origin_fixture(
    variant: WorthUiTouchOriginFixtureVariant,
) -> WorthUiTouchOriginCertificationFixture {
    let app = touch_runtime_app();
    let active = active_runtime_artifact(&app, variant);
    let mut runtime = launch_runtime(&app, active);
    let intent_candidate = replacement_candidate(&app, variant);
    let admitted =
        WorthUiCandidateAdmission::for_active_basis(runtime.replacement_admission_basis())
            .admit(replacement_candidate(&app, variant))
            .expect("candidate admits against active runtime");
    let comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity matching succeeds");
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement classification succeeds");
    let inventory = runtime
        .durable_state_inventory()
        .register_platform_family(WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(WorthUiDurableStateFamily::selection_range())
        .register_platform_family(WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(WorthUiDurableStateFamily::tab_state())
        .register_platform_family(WorthUiDurableStateFamily::panel_visibility())
        .build_for_replacement(&node_plan)
        .expect("durable state inventory succeeds");
    let reconciliation = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("durable state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("query rebind planning succeeds");
    let pending_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation,
        &query_rebind,
    );
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation),
            Some(&query_rebind),
            Some(&pending_input),
        )
        .expect("activation staging succeeds");
    let plan_input = prepare_execution_plan_input_with_component_hooks(
        &runtime,
        &pending,
        &component_hooks_for_variant(variant),
    )
    .expect("plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "touch-origin.fixture");
    let allocation = runtime
        .allocate_runtime_handles(&planning)
        .expect("runtime handle allocation succeeds");
    let lane_admission = runtime
        .admit_execution_lanes(&planning, &WorthUiExecutionLaneSupport::platform_default())
        .expect("lane admission succeeds");
    let plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &planning,
            &allocation,
            &lane_admission,
        )
        .expect("execution plan topology assembles");
    let inspection = runtime
        .inspect_execution_plan(&plan, &planning)
        .expect("plan inspection succeeds");
    let ordinary_plan = runtime
        .prepare_ordinary_lane_plan(&plan, &lane_admission)
        .expect("ordinary lane plan prepares");
    let frame_receipt = runtime
        .execute_ordinary_lane_frame(&ordinary_plan, WorthUiOrdinaryFrameTarget::root_shell())
        .expect("ordinary lane frame executes");
    let ready = runtime
        .prepare_ready_activation(pending, &plan_input, &allocation, &plan, None)
        .expect("ready activation prepares");
    runtime
        .swap_ready_activation_at_frame_boundary(ready, plan, runtime.safe_frame_boundary())
        .expect("ready activation swaps candidate plan into active runtime truth");
    let diagnostic_report = runtime
        .diagnostics()
        .for_projection_hook(&WorthUiDiagnosticProjectionHook::projection(
            "touch.origin.diagnostics",
        ))
        .materialize();

    WorthUiTouchOriginCertificationFixture {
        app,
        runtime,
        inspection,
        frame_receipt,
        intent_candidate,
        diagnostic_report,
    }
}

fn touch_runtime_app() -> WorthUiApp {
    WorthUi::app()
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.dashboard").expect("valid component id"),
            ComponentPropSchema::named("workspace.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new("workspace.surface.command_save").expect("valid surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new("workspace.component.dashboard").expect("valid component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::restorable(),
        ))
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.touch-origin-receipts")
                .with_semantic_artifact_spec(control_spec())
                .with_semantic_artifact_spec(region_spec()),
        )
        .freeze()
}

fn active_runtime_artifact(
    app: &WorthUiApp,
    variant: WorthUiTouchOriginFixtureVariant,
) -> WorthUiArtifact {
    match variant {
        WorthUiTouchOriginFixtureVariant::OverlayArtifact => {
            artifact_from_modules(app, [runtime_surface_module(), runtime_overlay_module()])
        }
        WorthUiTouchOriginFixtureVariant::Baseline
        | WorthUiTouchOriginFixtureVariant::SameArtifactExtraPlanHook => {
            artifact_from_modules(app, [runtime_surface_module()])
        }
    }
}

fn replacement_candidate(
    app: &WorthUiApp,
    variant: WorthUiTouchOriginFixtureVariant,
) -> WorthUiReplacementCandidate {
    rust_authored_replacement_candidate(
        active_runtime_artifact(app, variant),
        app.capabilities().digest(),
        WorthUiReplacementCause::manual_refresh(41),
    )
    .expect("replacement candidate seals")
}

fn component_hooks_for_variant(
    variant: WorthUiTouchOriginFixtureVariant,
) -> Vec<WorthUiComponentLoweringHook> {
    match variant {
        WorthUiTouchOriginFixtureVariant::Baseline
        | WorthUiTouchOriginFixtureVariant::OverlayArtifact => Vec::new(),
        WorthUiTouchOriginFixtureVariant::SameArtifactExtraPlanHook => {
            vec![WorthUiComponentLoweringHook::registered(
                "touch.origin.extra",
                WorthUiPlanNodeInputFamily::DiagnosticsRef,
            )]
        }
    }
}

fn prepare_execution_plan_input_with_component_hooks<P>(
    runtime: &WorthUiRuntimeHost,
    pending_activation: P,
    component_hooks: &[WorthUiComponentLoweringHook],
) -> Result<crate::runtime::WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
where
    P: Borrow<WorthUiPendingActivation>,
{
    WorthUiExecutionPlanInputPreparer::prepare(
        pending_activation.borrow(),
        runtime.inspect_active().frame_epoch(),
        component_hooks,
    )
}

fn launch_runtime(app: &WorthUiApp, artifact: WorthUiArtifact) -> WorthUiRuntimeHost {
    app.launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(artifact))
        .expect("runtime launches")
}

fn artifact_from_modules<const N: usize>(
    app: &WorthUiApp,
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let snapshot = app.capabilities();
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("artifact input resolves");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("structure lowers");
    let bound =
        WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).expect("semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact assembles")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/graph_touch_origin_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
    .with_posture_token(UiDslPostureToken::new("measurement:hug-height"))
    .with_posture_token(UiDslPostureToken::new("host-capability:text-input"))
}

fn region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.sidebar"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/graph_touch_origin_runtime.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}

fn graph_node_identity(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> crate::graph::UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should materialize one graph node")
}

fn runtime_surface_module() -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("worth-ui.runtime.bootstrap")
        .with_surface("workspace.surface.command_save")
}

fn runtime_overlay_module() -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("worth-ui.runtime.overlay")
        .with_surface("workspace.surface.command_save")
}
