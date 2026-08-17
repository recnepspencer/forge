use worth_ui::facade::{
    app::WorthUi,
    query_binding::WorthUiInstalledSnapshotQueryView,
    source::{
        WorthUiFilesystemSourceProvider, WorthUiSourceEventIngress, WorthUiSourceProvider,
        WorthUiWatcherEvent,
    },
};
use worth_ui_dsl::{WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule};

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

pub(super) fn file_authored_query_app(
    view: WorthUiInstalledSnapshotQueryView,
) -> worth_ui::facade::app::WorthUiApp {
    let capability_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(view.clone())
        .expect("the public builder registers installed authority")
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("capability snapshot preparation should succeed");
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(view)
        .expect("the public builder registers installed authority")
        .with_candidate_submission(query_bound_submission(capability_app.capabilities()))
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

pub(super) fn file_authored_two_query_view_app(
    first: WorthUiInstalledSnapshotQueryView,
    second: WorthUiInstalledSnapshotQueryView,
) -> worth_ui::facade::app::WorthUiApp {
    let capability_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(first.clone())
        .expect("the first installed view registers")
        .register_query_view(second.clone())
        .expect("the second installed view registers")
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("two-view capability snapshot preparation should succeed");
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(first)
        .expect("the first application view registers")
        .register_query_view(second)
        .expect("the second application view registers")
        .with_candidate_submission(two_query_binding_submission(capability_app.capabilities()))
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("two-view application preparation should succeed")
}

pub(super) fn query_bound_submission(
    capabilities: &worth_ui::facade::diagnostics::CapabilitySnapshot,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    let provider_id = "query-consumer-kit-source";
    let filesystem = FilesystemContractWorkspace::new(provider_id);
    filesystem.write("app/main.wui", "binding inspector.measurements {}");
    let source_snapshot = WorthUiFilesystemSourceProvider::new(filesystem.root())
        .read()
        .expect("production filesystem acquisition reads real Query-bound .wui bytes");
    filesystem.close();
    source_snapshot
        .attempt_candidate_for_certification(capabilities)
        .expect("Query-bound source lowers to one inseparable candidate submission")
}

pub(super) fn query_free_app() -> worth_ui::facade::app::WorthUiApp {
    let capability_app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("Query-free capability snapshot preparation should succeed");
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_candidate_submission(file_submission(
            "query-free-consumer-kit-source",
            "\n",
            capability_app.capabilities(),
        ))
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("Query-free application preparation should succeed")
}

fn file_submission(
    provider_id: &str,
    source: &str,
    capabilities: &worth_ui::facade::diagnostics::CapabilitySnapshot,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    let filesystem = FilesystemContractWorkspace::new(provider_id);
    filesystem.write("app/main.wui", source);
    let source_snapshot = WorthUiFilesystemSourceProvider::new(filesystem.root())
        .read()
        .expect("production filesystem acquisition reads real .wui bytes");
    filesystem.close();
    source_snapshot
        .attempt_candidate_for_certification(capabilities)
        .expect("the real file source lowers to one candidate submission")
}

fn two_query_binding_submission(
    capabilities: &worth_ui::facade::diagnostics::CapabilitySnapshot,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    let provider_id = "two-query-binding-source";
    let filesystem = FilesystemContractWorkspace::new(provider_id);
    filesystem.write(
        "app/main.wui",
        "binding inspector.measurements {}\nbinding inspector.secondary {}",
    );
    let source_snapshot = WorthUiFilesystemSourceProvider::new(filesystem.root())
        .read()
        .expect("production filesystem acquisition reads both Query bindings");
    filesystem.close();
    source_snapshot
        .attempt_candidate_for_certification(capabilities)
        .expect("two Query bindings lower as one candidate submission")
}

pub(super) fn query_bound_rust_submission(
    capabilities: &worth_ui::facade::diagnostics::CapabilitySnapshot,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    let provider_id = "query-consumer-kit-rust-source";
    let mut ingress = WorthUiSourceEventIngress::new(
        WorthUiSourceProvider::rust_authored(provider_id).with_rust_authored_input(
            WorthUiRustAuthoredArtifactInput::from_modules([
                WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                    .with_binding("inspector.measurements"),
            ]),
        ),
    )
    .start();
    ingress
        .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
        .expect("Rust-authored Query source settles")
        .attempt_candidate_for_certification(capabilities)
        .expect("Rust-authored Query source lowers to one candidate submission")
}
