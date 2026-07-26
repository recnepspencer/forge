use crate::facade::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};
use crate::runtime::tests::source_ingress_boundary_test_support::{
    lower_file_submission, lower_rust_submission, source_backed_package_component,
    source_backed_package_region, source_backed_package_sizing,
};
use crate::runtime::{
    WorthUiSourceProvider, WorthUiWatchedCandidateSubmission, WorthUiWatcherEvent,
};

pub(super) fn convergence_submissions() -> (
    WorthUiWatchedCandidateSubmission,
    WorthUiWatchedCandidateSubmission,
) {
    let snapshot = convergence_builder()
        .freeze()
        .expect("convergence snapshot should prepare");
    let file_submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("phase6-file").with_file(
            "app/main.wui",
            "component workspace.component.phase6_convergence {}",
        ),
        [WorthUiWatcherEvent::provider_revision("phase6-file")],
        snapshot.capabilities(),
    );
    let rust_submission = lower_rust_submission(
        WorthUiSourceProvider::rust_authored("phase6-rust").with_rust_authored_input(
            worth_ui_dsl::WorthUiRustAuthoredArtifactInput::from_modules([
                worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                    .with_component("workspace.component.phase6_convergence"),
            ]),
        ),
        [WorthUiWatcherEvent::provider_revision("phase6-rust")],
        snapshot.capabilities(),
    );
    (file_submission, rust_submission)
}

pub(super) fn prepare_convergence_apps(
    file_submission: WorthUiWatchedCandidateSubmission,
    rust_submission: WorthUiWatchedCandidateSubmission,
) -> (WorthUiApp, WorthUiApp) {
    let file_app = convergence_builder()
        .with_candidate_submission(file_submission)
        .freeze()
        .expect("file composition should prepare");
    let rust_app = convergence_builder()
        .with_candidate_submission(rust_submission)
        .freeze()
        .expect("Rust composition should prepare");
    (file_app, rust_app)
}

pub(super) fn prepare_file_authored_package_app() -> WorthUiApp {
    let snapshot = source_backed_package_builder()
        .freeze()
        .expect("application preparation should succeed");
    let submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("source-backed-package")
            .with_file("app/source_backed_package.wui", SOURCE_BACKED_PACKAGE),
        [WorthUiWatcherEvent::provider_revision(
            "source-backed-package",
        )],
        snapshot.capabilities(),
    );
    source_backed_package_builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("the complete watched composition should prepare")
}

fn convergence_builder() -> WorthUiApplicationBuilder {
    WorthUi::app().register_component(source_backed_package_component(
        "workspace.component.phase6_convergence",
    ))
}

fn source_backed_package_builder() -> WorthUiApplicationBuilder {
    WorthUi::app()
        .register_component(source_backed_package_component(
            "workspace.component.workflow_editor",
        ))
        .register_component(source_backed_package_component(
            "workspace.component.workflow_editor.peer_a",
        ))
        .register_component(source_backed_package_component(
            "workspace.component.workflow_editor.peer_b",
        ))
        .register_mosaic_region_kind(source_backed_package_region())
        .register_mosaic_sizing_contract(source_backed_package_sizing())
}

const SOURCE_BACKED_PACKAGE: &str = r#"
component workspace.component.workflow_editor {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}

component workspace.component.workflow_editor.peer_a {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
component workspace.component.workflow_editor.peer_b {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
"#;
