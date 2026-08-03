//! Application authority used by obligation-dispatch prerequisite scenarios.

use crate::{WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture};
use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken,
};
use worth_ui_test_support::WorthUiApplicationBuilderCertificationExt;

use super::query_prerequisites::settled_query_world_profile;

pub fn structural_touch_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-dispatch.structural",
            )
            .with_semantic_artifact_spec(control_spec())
            .with_semantic_artifact_spec(service_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

pub fn query_touch_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_graph_world_profile(settled_query_world_profile(
            "snapshot:phase5-dispatch-prereq",
            ["worth-ui.phase5", "dispatch", "query-prereq"],
        ))
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-dispatch.query",
            )
            .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

pub fn service_touch_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-dispatch.service",
            )
            .with_semantic_artifact_spec(service_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

pub fn focus_touch_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-dispatch.focus",
            )
            .with_semantic_artifact_spec(focus_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

pub fn motion_touch_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.obligation-dispatch.motion",
            )
            .with_semantic_artifact_spec(motion_control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_dispatch_prereq_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn service_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.portal"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_dispatch_service_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:portal"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn focus_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.focus"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_dispatch_focus_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:focus"))
    .with_posture_token(UiDslPostureToken::new("service:focus-routing"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}

fn motion_control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.motion"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_dispatch_motion_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:motion"))
    .with_posture_token(UiDslPostureToken::new("service:motion"))
    .with_posture_token(UiDslPostureToken::new("touch:press"))
}
