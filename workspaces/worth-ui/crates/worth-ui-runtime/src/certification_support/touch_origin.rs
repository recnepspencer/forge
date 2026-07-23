//! SUPPORT AUTHORITY ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â runtime-origin touch fixture for certification consumers.

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
use crate::runtime::{
    WorthUiCandidateAdmission, WorthUiDiagnosticProjectionHook, WorthUiDurableStateFamily,
    WorthUiExecutionPlanInspection, WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameReceipt,
    WorthUiReplacementCandidate, WorthUiRuntimeDiagnosticReport,
};
use worth_ui_host_contract::{
    WorthUiHostCapability, WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

pub use crate::certification_support::touch_origin_source::WorthUiTouchOriginFixtureVariant;

pub struct WorthUiTouchOriginCertificationFixture {
    pub app: WorthUiApp,
    pub runtime: crate::runtime::WorthUiRuntime,
    pub inspection: WorthUiExecutionPlanInspection,
    pub frame_receipt: WorthUiOrdinaryLaneFrameReceipt,
    pub intent_candidate: WorthUiReplacementCandidate,
    pub diagnostic_report: WorthUiRuntimeDiagnosticReport,
    pub allocation_receipt: crate::runtime::UiAllocationReceipt,
    pub allocation_inspection: crate::evidence::UiAllocationReceiptInspectionReceipt,
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
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            crate::runtime::WorthUiActivationStagingPlans {
                reconciliation_plan: Some(&reconciliation),
                query_rebind_plan: Some(&query_rebind),
            },
        )
        .expect("activation staging succeeds");
    let (planning_snapshot, planning_basis, planning_obligations) =
        honest_planning_catalog_basis(&app);
    let admitted_catalog = planning_snapshot
        .admit_allocation_catalog_basis_set(vec![(planning_basis, planning_obligations)])
        .expect("graph admits complete catalog basis");
    let mut inspection = None;
    let mut committed_allocation_receipt = None;
    runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted_catalog,
            |runtime, allocation_receipt, plan, lowering_facts| {
                committed_allocation_receipt = Some(allocation_receipt.clone());
                inspection = Some(
                    runtime
                        .inspect_execution_plan(plan, lowering_facts)
                        .map_err(|_| crate::runtime::WorthUiAllocationCatalogActivationDenial::CertificationBoundary("plan inspection"))?,
                );
                let execution = runtime
                    .execute_framework_turn(|_| {})
                    .into_execution()
                    .map_err(|_| crate::runtime::WorthUiAllocationCatalogActivationDenial::CertificationBoundary("framework turn"))?;
                Ok((execution.into_activation_boundary(), None))
            },
        )
        .expect("production catalog activation swaps active runtime truth");
    let inspection = inspection.expect("canonical activation inspects the candidate plan");
    let frame_receipt = runtime
        .execute_framework_turn(|_| {})
        .into_execution()
        .expect("active runtime opens a framework execution turn")
        .execute_active_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
        .expect("the published active bundle executes its ordinary frame");
    let diagnostic_report = runtime
        .diagnostics()
        .for_projection_hook(&WorthUiDiagnosticProjectionHook::projection(
            "touch.origin.diagnostics",
        ))
        .materialize();
    let allocation_receipt =
        committed_allocation_receipt.expect("activation exposes its committed receipt");
    let allocation_inspection = allocation_receipt.inspection_receipt();

    WorthUiTouchOriginCertificationFixture {
        app,
        runtime,
        inspection,
        frame_receipt,
        intent_candidate,
        diagnostic_report,
        allocation_receipt,
        allocation_inspection,
    }
}

/// Production `plan_allocation` with real declaration identity from the frozen app.
fn honest_planning_catalog_basis(
    app: &WorthUiApp,
) -> (
    crate::graph::UiGraphSnapshot,
    crate::evidence::UiMeasurementBasis,
    crate::obligations::selection::UiSelectedObligationSet,
) {
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
    let origin = app
        .graph()
        .touches()
        .declaration_change_receipt(control)
        .expect("touch-origin declaration must admit structural change authority");
    let touch = app
        .graph()
        .touches()
        .from_node(
            origin,
            crate::obligations::touch::UiGraphTouchTiming::PostMutation,
            root,
            crate::obligations::touch::UiGraphTouchAspects::new()
                .structural(crate::obligations::touch::UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("touch-origin structural change must admit a graph touch");
    let selected = app.admission().select_obligations(&touch);
    (snapshot, basis, selected)
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
