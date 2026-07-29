use crate::facade::WorthUi;
use crate::runtime::{WorthUiRuntimeLaunch, WorthUiSourceProvider};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiStructuralLegalityLowerer,
};
use worth_ui_dsl::{WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule};

pub(crate) fn file_import_provider() -> WorthUiSourceProvider {
    file_import_provider_for("app/panels/inspector.wui")
}

pub(crate) fn file_import_provider_for(target_module_path: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::in_memory("source-ingress-file-authored")
        .with_file("app/main.wui", format!(r#"import "{target_module_path}";"#))
        .with_file(target_module_path, "")
}

pub(crate) fn rust_import_provider() -> WorthUiSourceProvider {
    rust_import_provider_for("app/panels/inspector.wui")
}

pub(crate) fn rust_import_provider_for(target_module_path: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::rust_authored("rust-authored")
        .with_rust_authored_input(rust_import_input_for(target_module_path))
}

pub(crate) fn rust_import_input() -> WorthUiRustAuthoredArtifactInput {
    rust_import_input_for("app/panels/inspector.wui")
}

pub(crate) fn rust_import_input_for(target_module_path: &str) -> WorthUiRustAuthoredArtifactInput {
    WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_import(target_module_path),
        WorthUiRustAuthoredArtifactInputModule::new(target_module_path),
    ])
}

pub(crate) fn rust_import_artifact() -> WorthUiArtifact {
    rust_import_artifact_for("app/panels/inspector.wui")
}

pub(crate) fn rust_import_artifact_for(target_module_path: &str) -> WorthUiArtifact {
    canonical_artifact_from_input(crate::source::test_compilation::compile_rust_authored(
        &rust_import_input_for(target_module_path),
    ))
}

pub(crate) fn empty_artifact() -> WorthUiArtifact {
    canonical_artifact_from_rust_modules([WorthUiRustAuthoredArtifactInputModule::new(
        "app/main.wui",
    )])
}

pub(crate) fn runtime_from_artifact(artifact: WorthUiArtifact) -> crate::runtime::WorthUiRuntime {
    framework_from_artifact(artifact).into_runtime()
}

pub(crate) fn framework_from_artifact(
    artifact: WorthUiArtifact,
) -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let candidate = crate::runtime::replacement::candidate::rust_authored_replacement_candidate(
        artifact,
        app.capabilities().digest(),
        crate::runtime::WorthUiReplacementCause::rust_authored_input_change(1),
    )
    .expect("production candidate lowers");
    app.launch_runtime(WorthUiRuntimeLaunch::from_candidate(candidate))
        .expect("runtime launches")
}

fn canonical_artifact_from_rust_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    canonical_artifact_from_input(artifact_input)
}

fn canonical_artifact_from_input(
    artifact_input: worth_ui_dsl::WorthUiSealedSemanticPackage,
) -> WorthUiArtifact {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let snapshot = app.capabilities();
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("artifact input resolves");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("structure lowers");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("binding semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact assembles")
}
