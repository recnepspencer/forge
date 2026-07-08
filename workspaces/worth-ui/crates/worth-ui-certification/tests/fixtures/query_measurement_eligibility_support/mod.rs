mod projection_consumption_support;

use forge_query::facade::{
    ForgeQueryAuthoredAspectValue, ProjectMaterializedFacts, ProjectionFactConsumptionAttempt,
};
use worth_ui::facade::admission::{UiAdmissionQueryBasis, UiAdmissionTarget, UiAdmissionWorld};
use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDescriptor, UiGraphTouchTiming,
    UiGraphWorldProfile,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_host_contract::{WorthUiHostCapabilityReport, WorthUiHostContract};
use worth_ui_query_binding::{
    WorthUiQueryBasisPosture, WorthUiQueryBindingSubsystem, WorthUiQueryCausalExplanationLane,
    WorthUiQueryInspectionLane, WorthUiQueryPrerequisiteEvidence,
    WorthUiQueryProjectionConsumptionLane,
};

use self::projection_consumption_support::{
    aspect_touch, identity_only_family_graph, measurement_projection_workspace,
    measurement_projection_workspace_with_graph, projection_consumption_attempt,
    title_value_field_path,
};

pub fn query_measurement_app(world_profile: UiGraphWorldProfile) -> WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.query-measurement-eligibility")
                .with_semantic_artifact_spec(query_measurement_spec())
                .with_semantic_artifact_spec(portal_measurement_spec()),
        )
        .freeze()
}

pub fn query_only_measurement_app(world_profile: UiGraphWorldProfile) -> WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.query-measurement-query-only")
                .with_semantic_artifact_spec(query_only_measurement_spec()),
        )
        .freeze()
}

pub fn measurement_touch(app: &WorthUiApp, declaration_index: usize) -> UiGraphTouchDescriptor {
    let artifact = artifact_from_index(app, declaration_index);
    app.graph()
        .touches()
        .from_node(
            app.graph()
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            graph_node_identity(app, artifact),
            UiGraphTouchAspects::new().measurement(UiGraphTouchAspectPosture::Read),
        )
        .expect("measurement touch should admit")
}

pub fn available_measurement_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::egui(),
    ))
}

pub fn target_bound_to_projection_consumption(
    touch: &UiGraphTouchDescriptor,
    consumption: &ProjectionFactConsumptionAttempt,
) -> UiAdmissionTarget {
    available_measurement_target(touch)
        .with_query_prerequisites_from_projection_consumption(consumption)
        .expect("query-backed measurement target should bind real projection consumption authority")
}

pub fn synthetic_query_prerequisites_for_world(
    world_profile: &UiGraphWorldProfile,
    query_basis: UiAdmissionQueryBasis,
) -> WorthUiQueryPrerequisiteEvidence {
    let UiGraphWorldProfile::QuerySnapshotBasis {
        basis,
        resolution_report,
    } = world_profile
    else {
        panic!("query measurement eligibility tests require query snapshot worlds");
    };

    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .assemble(
            basis.clone(),
            resolution_report.clone(),
            match query_basis {
                UiAdmissionQueryBasis::GraphAligned => WorthUiQueryBasisPosture::GraphAligned,
                UiAdmissionQueryBasis::WrongWorldProjection => {
                    WorthUiQueryBasisPosture::WrongWorldProjection
                }
                UiAdmissionQueryBasis::RebindRequired => WorthUiQueryBasisPosture::RebindRequired,
                UiAdmissionQueryBasis::StaleReceipt => WorthUiQueryBasisPosture::StaleReceipt,
                UiAdmissionQueryBasis::AmbiguousSources => {
                    WorthUiQueryBasisPosture::AmbiguousSources
                }
            },
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::NotRequested,
            WorthUiQueryCausalExplanationLane::NotRequested,
        )
        .expect("query prerequisite assembly should admit")
}

pub fn display_field_projection_consumption(
    lane_label: &str,
) -> (UiGraphWorldProfile, ProjectionFactConsumptionAttempt) {
    let (mut workspace, family, _) = measurement_projection_workspace(lane_label);
    projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().display_field_path(title_value_field_path()),
    )
}

pub fn denied_display_field_projection_consumption(
    lane_label: &str,
) -> (UiGraphWorldProfile, ProjectionFactConsumptionAttempt) {
    let (mut workspace, family, _) =
        measurement_projection_workspace_with_graph(lane_label, identity_only_family_graph);
    projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().display_field_path(title_value_field_path()),
    )
}

pub fn view_local_only_projection_consumption(
    lane_label: &str,
) -> (UiGraphWorldProfile, ProjectionFactConsumptionAttempt) {
    let (mut workspace, family, _) = measurement_projection_workspace(lane_label);
    projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().entity_identities(),
    )
}

pub fn display_and_view_local_projection_consumptions(
    lane_label: &str,
) -> (
    UiGraphWorldProfile,
    ProjectionFactConsumptionAttempt,
    ProjectionFactConsumptionAttempt,
) {
    let (mut workspace, family, _) = measurement_projection_workspace(lane_label);
    let (world_profile, display_consumption) = projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().display_field_path(title_value_field_path()),
    );
    let (_, view_local_consumption) = projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().entity_identities(),
    );
    (world_profile, display_consumption, view_local_consumption)
}

pub fn display_projection_consumptions_across_basis_generations(
    lane_label: &str,
) -> (
    (UiGraphWorldProfile, ProjectionFactConsumptionAttempt),
    (UiGraphWorldProfile, ProjectionFactConsumptionAttempt),
) {
    let (mut workspace, family, entity_identity) = measurement_projection_workspace(lane_label);
    let current = projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().display_field_path(title_value_field_path()),
    );
    workspace
        .update(entity_identity, |task| {
            task.set_aspect(
                aspect_touch("size.value"),
                ForgeQueryAuthoredAspectValue::string(format!("24{lane_label}")),
            )
        })
        .expect("fixture workspace should admit the follow-up size update");
    let next = projection_consumption_attempt(
        &mut workspace,
        &family,
        ProjectMaterializedFacts::declare().display_field_path(title_value_field_path()),
    );
    (current, next)
}

fn query_measurement_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.query_measurement"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/query_measurement_eligibility.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:query-measurement"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
    .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
    .with_posture_token(UiDslPostureToken::new(
        "measurement:evidence:font-metrics-required",
    ))
}

fn query_only_measurement_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.query_measurement_query_only"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/query_measurement_eligibility.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:query-measurement"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:mode:hug-height"))
    .with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"))
    .with_posture_token(UiDslPostureToken::new("measurement:scroll-owned"))
}

fn portal_measurement_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.portal_measurement"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/query_measurement_eligibility.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("control:portal-measurement"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
    .with_posture_token(UiDslPostureToken::new("measurement:portal-anchored"))
}

fn artifact_from_index(app: &WorthUiApp, declaration_index: usize) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/query_measurement_eligibility.wui"
                && provenance.declaration_index() == declaration_index
        })
        .expect("test declaration should exist")
}

fn graph_node_identity(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}
