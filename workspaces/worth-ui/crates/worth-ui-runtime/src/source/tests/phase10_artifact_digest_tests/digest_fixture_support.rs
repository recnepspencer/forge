use crate::facade::WorthUiApp;
use crate::source::{
    WorthUiArtifact, WorthUiCanonicalArtifactAssembler, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiSourcePackageLoader, WorthUiSourceParser,
};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;
use super::super::phase7_identity_seeding_tests::identity_fixture_support::{
    imported_identity_modules, reordered_imported_identity_modules,
};
pub(super) use super::variant_app_fixture::{
    component_descriptor_variant_app, surface_descriptor_variant_app,
    theme_token_alias_chain_variant_app,
};

pub(super) fn artifact_from_rust_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    artifact_from_rust_modules_with_app(modules, identity_test_app())
}

pub(super) fn artifact_from_rust_modules_with_app<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
    app: WorthUiApp,
) -> WorthUiArtifact {
    let snapshot = app.capabilities();
    let artifact_input = crate::source::WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    let structured = crate::source::WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 legality should succeed");
    let bound = crate::source::WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("phase 6 binding should succeed");
    let identity_seeded = crate::source::WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("phase 7 identity should succeed")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact should succeed")
}

pub(super) fn artifact_from_file_sources(
    main_module_source: &str,
    inspector_module_source: &str,
) -> WorthUiArtifact {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let source_package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", main_module_source)
        .register_module_with_source("app/panels/inspector.wui", inspector_module_source)
        .compile()
        .expect("file-authored package should compile");
    let parsed_source_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source package should parse");
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_source_package);
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    let structured = crate::source::WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 legality should succeed");
    let bound = crate::source::WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("phase 6 binding should succeed");
    let identity_seeded = crate::source::WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("phase 7 identity should succeed")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact should succeed")
}

pub(super) fn imported_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    imported_identity_modules()
}

pub(super) fn reordered_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    reordered_imported_identity_modules()
}

pub(super) fn same_shape_but_different_surface_modules(
) -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    [
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/panels/inspector.wui")
            .with_component("workspace.component.dashboard")
            .with_surface("workspace.surface.main")
            .with_binding("workspace.view_binding.selection")
            .with_token("theme.text.default", "theme.text.primary"),
        WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
            .with_component("workspace.component.inspector_panel"),
    ]
}

pub(super) fn token_difference_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    [
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/panels/inspector.wui")
            .with_component("workspace.component.dashboard")
            .with_surface("workspace.surface.inspector")
            .with_binding("workspace.view_binding.selection")
            .with_token("theme.text.primary", "theme.text.primary"),
        WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
            .with_component("workspace.component.inspector_panel"),
    ]
}

pub(super) fn equivalent_file_authored_main_module_source() -> &'static str {
    r#"
    import "app/panels/inspector.wui";
    component workspace.component.dashboard {}
    token theme.text.default = "theme.text.primary";
    "#
}

pub(super) fn equivalent_file_authored_inspector_module_source() -> &'static str {
    r#"
    component workspace.component.inspector_panel {}
    "#
}

pub(super) fn equivalent_rust_authored_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    [
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/panels/inspector.wui")
            .with_component("workspace.component.dashboard")
            .with_token("theme.text.default", "theme.text.primary"),
        WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
            .with_component("workspace.component.inspector_panel"),
    ]
}
