use crate::facade::WorthUiApp;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigest, WorthUiArtifactEquivalence,
    WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceComparator, WorthUiArtifactInput,
    WorthUiArtifactInputResolver, WorthUiArtifactInputTokenNode, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer, WorthUiIdentitySeedingReport,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiResolutionReport,
    WorthUiRustCompositionInput, WorthUiRustCompositionModule,
    WorthUiRustCompositionToArtifactInputLowerer, WorthUiSourcePackageLoader, WorthUiSourceParser,
    WorthUiStructuralLegalityLowerer,
};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;

pub(super) fn equivalent_rust_composition() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules([
        WorthUiRustCompositionModule::new("app/main.wui")
            .import("app/panels/inspector.wui")
            .component_body_atoms("workspace.component.dashboard", [])
            .surface("workspace.surface.inspector")
            .binding("workspace.view_binding.selection")
            .token("theme.text.default", "theme.text.primary"),
        WorthUiRustCompositionModule::new("app/panels/inspector.wui")
            .component("workspace.component.inspector_panel"),
    ])
}

pub(super) fn reordered_rust_composition() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules([
        WorthUiRustCompositionModule::new("app/panels/inspector.wui")
            .component("workspace.component.inspector_panel"),
        WorthUiRustCompositionModule::new("app/main.wui")
            .token("theme.text.default", "theme.text.primary")
            .binding("workspace.view_binding.selection")
            .surface("workspace.surface.inspector")
            .component("workspace.component.dashboard")
            .import("app/panels/inspector.wui"),
    ])
}

pub(super) fn missing_component_rust_composition() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules([
        WorthUiRustCompositionModule::new("app/main.wui").component("workspace.component.missing")
    ])
}

pub(super) fn duplicate_authored_identity_rust_composition() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules([WorthUiRustCompositionModule::new("app/main.wui")
        .component_authored_identity("workspace.component.dashboard", "duplicate.identity")
        .component_authored_identity("workspace.component.inspector_panel", "duplicate.identity")])
}

pub(super) fn equivalent_file_artifact() -> WorthUiArtifact {
    artifact_from_file_sources(main_file_source(), inspector_file_source())
}

pub(super) fn artifact_from_composition(
    composition: &WorthUiRustCompositionInput,
) -> WorthUiArtifact {
    artifact_from_input(
        WorthUiRustCompositionToArtifactInputLowerer::lower(composition),
        identity_test_app(),
    )
}

pub(super) fn resolution_report_from_composition(
    composition: &WorthUiRustCompositionInput,
) -> WorthUiResolutionReport {
    let artifact_input = WorthUiRustCompositionToArtifactInputLowerer::lower(composition);
    WorthUiArtifactInputResolver::resolve(&artifact_input, identity_test_app().capabilities())
        .expect_err("rust composition should fail at snapshot resolution")
}

pub(super) fn artifact_input_from_composition(
    composition: &WorthUiRustCompositionInput,
) -> WorthUiArtifactInput {
    WorthUiRustCompositionToArtifactInputLowerer::lower(composition)
}

pub(super) fn identity_seeding_report_from_composition(
    composition: &WorthUiRustCompositionInput,
) -> WorthUiIdentitySeedingReport {
    let artifact_input = WorthUiRustCompositionToArtifactInputLowerer::lower(composition);
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("resolution should succeed");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("structural legality should succeed");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("binding should succeed");

    WorthUiIdentitySeedLowerer::lower(&bound)
        .expect_err("rust composition identity conflict should fail at identity seeding")
}

pub(super) fn semantic_digest(artifact: &WorthUiArtifact) -> WorthUiArtifactDigest {
    crate::source::WorthUiArtifactDigestor::digest(
        artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    )
}

pub(super) fn semantic_equivalence(
    left: &WorthUiArtifact,
    right: &WorthUiArtifact,
) -> WorthUiArtifactEquivalence {
    WorthUiArtifactEquivalenceComparator::compare(
        left,
        right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    )
}

pub(super) fn first_token_node(
    artifact_input: &WorthUiArtifactInput,
) -> &WorthUiArtifactInputTokenNode {
    artifact_input
        .module(&artifact_input.module_ids()[0])
        .expect("artifact input module")
        .nodes()
        .iter()
        .find_map(|node| match node {
            crate::source::WorthUiArtifactInputNode::Token(token) => Some(token),
            _ => None,
        })
        .expect("token node")
}

pub(super) fn artifact_node_count(artifact: &WorthUiArtifact) -> usize {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .map(|module| module.nodes().len())
        .sum()
}

fn artifact_from_file_sources(main_source: &str, inspector_source: &str) -> WorthUiArtifact {
    let source_package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", main_source)
        .register_module_with_source("app/panels/inspector.wui", inspector_source)
        .compile()
        .expect("file source package should compile");
    let parsed_source_package =
        WorthUiSourceParser::parse_package(&source_package).expect("file source should parse");
    artifact_from_input(
        WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_source_package)
            .expect("authoring entry should lower to artifact input"),
        identity_test_app(),
    )
}

fn artifact_from_input(artifact_input: WorthUiArtifactInput, app: WorthUiApp) -> WorthUiArtifact {
    let snapshot = app.capabilities();
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("resolution should succeed");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("structural legality should succeed");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("binding should succeed");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeding should succeed")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact should assemble")
}

fn main_file_source() -> &'static str {
    r#"
    import "app/panels/inspector.wui";
    component workspace.component.dashboard {}
    surface workspace.surface.inspector {}
    binding workspace.view_binding.selection {}
    token theme.text.default = "theme.text.primary";
    "#
}

fn inspector_file_source() -> &'static str {
    r#"
    component workspace.component.inspector_panel {}
    "#
}
