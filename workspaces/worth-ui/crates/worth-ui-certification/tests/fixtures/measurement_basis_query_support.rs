#[path = "measurement_basis_projection_support.rs"]
mod projection_consumption_support;

use worth_query::facade::read::{project_facts, WorthQueryProjectionOutcome};
use worth_query::facade::runtime::WorthQueryAuthoredAspectValue;
use worth_ui::facade::admission::{UiAdmissionTarget, UiAdmissionWorld};
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
use worth_ui_query_binding::WorthUiQueryAuthorityHandle;

use self::projection_consumption_support::{
    aspect_touch, measurement_projection_workspace, projection_consumption_attempt,
    title_value_field_path,
};

pub fn query_measurement_app(world_profile: UiGraphWorldProfile) -> WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.measurement-basis-query")
                .with_semantic_artifact_spec(query_measurement_spec())
                .with_semantic_artifact_spec(portal_measurement_spec()),
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

pub fn target_bound_to_projection_consumption(
    touch: &UiGraphTouchDescriptor,
    authority: &WorthUiQueryAuthorityHandle,
) -> UiAdmissionTarget {
    available_measurement_target(touch)
        .with_query_prerequisites_from_query_authority(authority.authority())
        .expect("query-backed measurement target should bind real projection consumption authority")
}

pub fn display_field_projection_consumption(
    lane_label: &str,
) -> (UiGraphWorldProfile, WorthUiQueryAuthorityHandle) {
    let (mut workspace, schema_basis_authority, _) = measurement_projection_workspace(lane_label);
    query_authority(projection_consumption_attempt(
        &mut workspace,
        schema_basis_authority,
        project_facts().display_field(title_value_field_path()),
    ))
}

pub fn display_projection_consumptions_across_basis_generations(
    lane_label: &str,
) -> (
    (UiGraphWorldProfile, WorthUiQueryAuthorityHandle),
    (UiGraphWorldProfile, WorthUiQueryAuthorityHandle),
) {
    let (mut workspace, schema_basis_authority, entity_identity) =
        measurement_projection_workspace(lane_label);
    let current = projection_consumption_attempt(
        &mut workspace,
        schema_basis_authority.clone(),
        project_facts().display_field(title_value_field_path()),
    );
    workspace
        .update(entity_identity, |task| {
            task.set_aspect(
                aspect_touch("size.value"),
                WorthQueryAuthoredAspectValue::native(
                    worth_foundational::facade::AspectValue::Float32(
                        worth_foundational::facade::CanonicalF32::from_f32(241.0),
                    ),
                ),
            )
        })
        .expect("fixture workspace should admit the follow-up size update");
    let next = projection_consumption_attempt(
        &mut workspace,
        schema_basis_authority,
        project_facts().display_field(title_value_field_path()),
    );
    (query_authority(current), query_authority(next))
}

fn query_authority(
    (world, outcome): (UiGraphWorldProfile, WorthQueryProjectionOutcome),
) -> (UiGraphWorldProfile, WorthUiQueryAuthorityHandle) {
    let (authority, _) = WorthUiQueryAuthorityHandle::from_outcome(outcome)
        .expect("certification fixture must mint authority through Query");
    (world, authority)
}

fn available_measurement_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_host_capability_report(WorthUiHostCapabilityReport::from_contract(
        WorthUiHostContract::egui(),
    ))
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
