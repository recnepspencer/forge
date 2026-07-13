use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::UiDeclaredMeasurementBasisSource;
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
use crate::runtime::candidate::rust_authored_replacement_candidate;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmission, WorthUiPendingActivation,
    WorthUiReplacementCause, WorthUiRuntime, WorthUiRuntimeLaunch,
};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

use super::fixture_support::{
    artifact_from_modules, capability_report, container_basis, control_app,
    declaration_identity_for, graph_node_identity_for_provenance, host_portal_anchor_result,
    host_scroll_viewport_result, intrinsic_basis, measurement_policy, peer_app, query_app,
    snapshot_with_admitted_layout,
};
use crate::runtime::WorthUiAllocationPlanning;

#[derive(Clone, Copy)]
pub(super) enum CertificationScenarioShape {
    Control { nodes: usize, bounded: bool },
    Peer { nodes: usize, bounded: bool },
    Intrinsic { nodes: usize, bounded: bool },
}

pub(super) fn planning_pair_from_runtime_fixture(
    operator_token: &str,
    shape: CertificationScenarioShape,
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
) -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    let (runtime, pending) = planning_runtime_fixture();
    let (first_basis, first_snapshot, first_selected) =
        planning_scenario(operator_token, shape, basis_source);
    let (second_basis, second_snapshot, second_selected) =
        planning_scenario(operator_token, shape, basis_source);
    (
        runtime
            .plan_allocation(
                runtime
                    .admit_planning_lane_input(
                        &pending,
                        &first_snapshot,
                        first_basis,
                        &first_selected,
                    )
                    .expect("certification planning input admits through graph authority"),
            )
            .planning()
            .clone(),
        runtime
            .plan_allocation(
                runtime
                    .admit_planning_lane_input(
                        &pending,
                        &second_snapshot,
                        second_basis,
                        &second_selected,
                    )
                    .expect("certification replay input admits through graph authority"),
            )
            .planning()
            .clone(),
    )
}

fn planning_scenario(
    operator_token: &str,
    shape: CertificationScenarioShape,
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
) -> (
    crate::evidence::UiMeasurementBasis,
    crate::graph::UiGraphSnapshot,
    crate::obligations::selection::UiSelectedObligationSet,
) {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let (_, _, world_profile) = crate::evidence::measurement::projection::query_context_test_support::
        display_field_projection_context(operator_token);
    let (app, nodes, bounded) = match shape {
        CertificationScenarioShape::Control { nodes, bounded } => (
            control_app(world_profile, operator_token, nodes, bounded),
            nodes,
            bounded,
        ),
        CertificationScenarioShape::Peer { nodes, bounded } => (
            peer_app(world_profile, operator_token, nodes, bounded),
            nodes,
            bounded,
        ),
        CertificationScenarioShape::Intrinsic { nodes, bounded } => (
            control_app(world_profile, operator_token, nodes, bounded),
            nodes,
            bounded,
        ),
    };
    let root = graph_node_identity_for_provenance(&app, 0);
    let admitted = (0..nodes)
        .map(|index| graph_node_identity_for_provenance(&app, index))
        .collect::<Vec<_>>();
    let snapshot = snapshot_with_admitted_layout(&app, &admitted);
    let basis = match (shape, basis_source) {
        (_, Some(UiDeclaredMeasurementBasisSource::ScrollViewport)) => {
            scroll_viewport_basis(&app, root, generation)
        }
        (_, Some(UiDeclaredMeasurementBasisSource::PortalAnchor)) => {
            portal_anchor_basis(&app, root, generation)
        }
        (CertificationScenarioShape::Intrinsic { .. }, None) => {
            intrinsic_basis(&app, root, nodes, generation, bounded)
        }
        _ => container_basis(&app, root, generation, bounded),
    };
    let touch = app
        .try_query_touch_for_node(root)
        .expect("certification planning root must admit a query touch");
    let selected = app.admission().select_obligations(&touch);
    (basis, snapshot, selected)
}

fn scroll_viewport_basis(
    app: &crate::facade::WorthUiApp,
    root: crate::graph::UiGraphNodeIdentity,
    generation: UiEvidenceAuthorityGeneration,
) -> crate::evidence::UiMeasurementBasis {
    let report = capability_report(91);
    let viewport = host_scroll_viewport_result(911, &report, generation);
    admit_measurement_basis(
        declaration_identity_for(app, 0),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &measurement_policy(Some(UiDeclaredMeasurementBasisSource::ScrollViewport), true),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&viewport),
        ],
    )
}

fn portal_anchor_basis(
    app: &crate::facade::WorthUiApp,
    root: crate::graph::UiGraphNodeIdentity,
    generation: UiEvidenceAuthorityGeneration,
) -> crate::evidence::UiMeasurementBasis {
    let report = capability_report(97);
    let anchor = host_portal_anchor_result(977, &report, generation);
    admit_measurement_basis(
        declaration_identity_for(app, 0),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &measurement_policy(Some(UiDeclaredMeasurementBasisSource::PortalAnchor), true),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::host_measurement_result(&anchor),
        ],
    )
}

fn planning_runtime_fixture() -> (crate::runtime::WorthUiRuntime, WorthUiPendingActivation) {
    let app = query_app();
    let active = artifact_from_modules(
        &app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_binding("workspace.view_binding.selection")],
    );
    let candidate = artifact_from_modules(
        &app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_binding("workspace.view_binding.selection")],
    );
    let runtime = app
        .launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(active))
        .expect("suite runtime launches");
    let candidate = rust_authored_replacement_candidate(
        candidate,
        app.capabilities().digest(),
        WorthUiReplacementCause::rust_authored_input_change(17),
    )
    .expect("suite candidate seals");
    let admitted =
        WorthUiCandidateAdmission::for_active_basis(runtime.replacement_admission_basis())
            .admit(candidate)
            .expect("suite candidate admits");
    let pending = stage_pending_activation(&runtime, admitted);
    (runtime, pending)
}

fn stage_pending_activation(
    runtime: &WorthUiRuntime,
    admitted: WorthUiAdmittedReplacementCandidate,
) -> WorthUiPendingActivation {
    let comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("suite comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&comparison, &admitted)
        .expect("suite impact succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("suite narrowing succeeds");
    let identity = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("suite identity succeeds");
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity)
        .expect("suite node plan succeeds");
    let inventory = runtime
        .durable_state_inventory()
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::focus_chain())
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::scroll_anchor())
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::selection_range())
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::text_edit_buffer())
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::splitter_position())
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::tab_state())
        .register_platform_family(crate::runtime::WorthUiDurableStateFamily::panel_visibility())
        .build_for_replacement(&node_plan)
        .expect("suite inventory builds");
    let reconciliation = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("suite reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("suite query comparison succeeds");
    let query_rebind = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("suite query rebind succeeds");
    let pending_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation,
        &query_rebind,
    );
    runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation),
            Some(&query_rebind),
            Some(&pending_input),
        )
        .expect("suite activation staging succeeds")
}
