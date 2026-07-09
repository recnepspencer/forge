//! SUPPORT AUTHORITY â€” runtime-origin touch fixture for certification consumers.

use crate::certification_support::layout_admission::snapshot_after_layout_admission_support;
use crate::certification_support::touch_origin_source::{
    active_runtime_artifact, launch_runtime, replacement_candidate, touch_runtime_app,
};
use crate::declaration::{
    UiDeclarationArtifact, UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
use crate::facade::entry::WorthUiApp;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiCandidateAdmission, WorthUiComponentLoweringHook,
    WorthUiDiagnosticProjectionHook, WorthUiDurableStateFamily, WorthUiExecutionLaneSupport,
    WorthUiExecutionPlanInspection, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameReceipt,
    WorthUiPendingActivation, WorthUiPlanLoweringDenial, WorthUiPlanNodeInputFamily,
    WorthUiReplacementCandidate, WorthUiRuntimeDiagnosticReport, WorthUiRuntimeHost,
};
use std::borrow::Borrow;
use worth_ui_host_contract::{
    WorthUiHostCapability, WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

pub use crate::certification_support::touch_origin_source::WorthUiTouchOriginFixtureVariant;

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
        artifact_from_file_provenance(&self.app, "app/graph_touch_origin_runtime.wui", 0)
    }

    pub fn region_artifact(&self) -> &UiDeclarationArtifact {
        artifact_from_file_provenance(&self.app, "app/graph_touch_origin_runtime.wui", 1)
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
    let planning = honest_planning_for_pending(&app, &runtime, &pending);
    let plan_input = prepare_execution_plan_input_with_component_hooks(
        &runtime,
        &pending,
        &component_hooks_for_variant(variant),
    )
    .expect("plan input prepares");
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

/// Production `plan_allocation` with real declaration identity from the frozen app.
fn honest_planning_for_pending(
    app: &WorthUiApp,
    runtime: &WorthUiRuntimeHost,
    pending: &WorthUiPendingActivation,
) -> WorthUiAllocationPlanning {
    let control = artifact_from_file_provenance(app, "app/graph_touch_origin_runtime.wui", 0);
    let root = graph_node_identity(app, control);
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let report = WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::TextIntrinsicMeasurement,
        WorthUiHostCapability::ScrollContainerObservation,
        WorthUiHostCapability::PortalAnchorObservation,
    ])
    .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(77));
    let policy = UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("touch-origin measurement policy should admit");
    let basis = admit_measurement_basis(
        control.identity().clone(),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &policy,
        &[MeasurementEvidenceInput::host_capability_report(&report)],
    );
    let snapshot = snapshot_after_layout_admission_support(app, &[root]);
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect(
            "touch-origin neighborhood admits through measurement basis + layout-admitted graph",
        );
    runtime.plan_allocation(pending, &basis, &neighborhood)
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

fn artifact_from_file_provenance<'a>(
    app: &'a WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .expect("fixture declaration should exist")
}
