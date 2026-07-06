use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::projection_fact_test_support::{
    capability_report, display_field_projection_context, host_font_metrics_policy,
    host_result_font_metrics, scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiAllocationNeighborhood, UiMeasurementBasis,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;
use crate::graph::UiGraphNodeIdentity;
use crate::runtime::{WorthUiAllocationPlanning, WorthUiExecutionPlanInput, WorthUiRuntimeHost};

pub(crate) fn admitted_measurement_basis(label: &str) -> UiMeasurementBasis {
    measurement_basis_with_font_seed(label, 100, admitted_operator_token())
}

pub(crate) fn admitted_measurement_basis_with_generation(
    label: &str,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementBasis {
    measurement_basis_with_authority_generation(label, 100, admitted_operator_token(), generation)
}

pub(crate) fn admitted_measurement_basis_with_font_seed(
    label: &str,
    request_seed: u64,
) -> UiMeasurementBasis {
    measurement_basis_with_font_seed(label, request_seed, admitted_operator_token())
}

pub(crate) fn changed_measurement_basis(label: &str) -> UiMeasurementBasis {
    measurement_basis_with_font_seed(label, 240, changed_operator_token())
}

pub(crate) fn denied_measurement_basis(label: &str) -> UiMeasurementBasis {
    let (app, root_node) = planning_graph_fixture(label, denied_operator_token());
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity(label);
    let (prerequisites, attempt, _) = display_field_projection_context(label);
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability = capability_report(77);
    let font_metrics = host_result_font_metrics(100, &capability, generation);
    admit_measurement_basis(
        declaration_identity,
        root_node,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
        ],
    )
}

pub(crate) fn admitted_allocation_neighborhood(label: &str) -> UiAllocationNeighborhood {
    admitted_neighborhood_for(
        label,
        admitted_measurement_basis(label),
        admitted_operator_token(),
    )
}

pub(crate) fn changed_allocation_neighborhood(label: &str) -> UiAllocationNeighborhood {
    admitted_neighborhood_for(
        label,
        changed_measurement_basis(label),
        changed_operator_token(),
    )
}

pub(crate) fn denied_allocation_neighborhood(label: &str) -> UiAllocationNeighborhood {
    admitted_neighborhood_for(
        label,
        denied_measurement_basis(label),
        denied_operator_token(),
    )
}

pub(crate) fn admitted_allocation_neighborhood_for_basis(
    label: &str,
    measurement_basis: UiMeasurementBasis,
) -> UiAllocationNeighborhood {
    admitted_neighborhood_for(label, measurement_basis, admitted_operator_token())
}

pub(crate) fn allocation_planning(
    runtime: &WorthUiRuntimeHost,
    plan_input: &WorthUiExecutionPlanInput,
    label: &str,
) -> WorthUiAllocationPlanning {
    let measurement_basis = admitted_measurement_basis(label);
    let allocation_neighborhood =
        admitted_neighborhood_for(label, measurement_basis.clone(), admitted_operator_token());
    runtime.plan_allocation_for_lowered_input_for_test(
        plan_input.clone(),
        &measurement_basis,
        &allocation_neighborhood,
    )
}

fn measurement_basis_with_font_seed(
    label: &str,
    request_seed: u64,
    operator_token: &str,
) -> UiMeasurementBasis {
    measurement_basis_with_authority_generation(
        label,
        request_seed,
        operator_token,
        UiEvidenceAuthorityGeneration::new(17),
    )
}

fn measurement_basis_with_authority_generation(
    label: &str,
    request_seed: u64,
    operator_token: &str,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementBasis {
    let (app, root_node) = planning_graph_fixture(label, operator_token);
    let declaration_identity = synthetic_declaration_identity(label);
    let capability = capability_report(77);
    let font_metrics = host_result_font_metrics(request_seed, &capability, generation);
    admit_measurement_basis(
        declaration_identity,
        root_node,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &host_font_metrics_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&capability),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
        ],
    )
}

fn admitted_neighborhood_for(
    label: &str,
    measurement_basis: UiMeasurementBasis,
    operator_token: &str,
) -> UiAllocationNeighborhood {
    let (app, root_node) = planning_graph_fixture(label, operator_token);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node]);
    measurement_basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("allocation neighborhood should admit from graph")
}

fn admitted_operator_token() -> &'static str {
    "operator:stack"
}

fn changed_operator_token() -> &'static str {
    "operator:grid"
}

fn denied_operator_token() -> &'static str {
    "operator:stack"
}

fn planning_graph_fixture(label: &str, operator_token: &str) -> (WorthUiApp, UiGraphNodeIdentity) {
    let (_, _, world_profile) = display_field_projection_context(label);
    let app = WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.runtime.allocation-planning")
                .with_semantic_artifact_spec(control_spec(
                    "allocation_planning.control.primary",
                    operator_token,
                )),
        )
        .freeze();
    let root_node = graph_node_identity_for_provenance(&app);
    (app, root_node)
}

fn control_spec(semantic_key: &str, operator_token: &str) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/allocation_planning_test_support.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:primary"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_structural_token(UiDslStructuralToken::new(operator_token))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn graph_node_identity_for_provenance(app: &WorthUiApp) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_planning_test_support.wui"
                && provenance.declaration_index() == 0
        })
        .expect("expected declaration artifact for runtime planning fixture");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("fixture declaration should project one graph node")
}
