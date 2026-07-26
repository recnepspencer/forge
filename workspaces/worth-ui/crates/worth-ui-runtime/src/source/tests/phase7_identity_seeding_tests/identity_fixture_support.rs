use crate::source::{
    WorthUiArtifactIdentitySeedKind, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
    WorthUiIdentitySeedLowerer, WorthUiIdentitySeededArtifactInput,
    WorthUiIdentitySeededArtifactInputBindingNode, WorthUiIdentitySeededArtifactInputComponentNode,
    WorthUiIdentitySeededArtifactInputImportNode, WorthUiIdentitySeededArtifactInputNode,
    WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiIdentitySeededArtifactInputThemeTokenNode, WorthUiIdentitySeedingDiagnosticCode,
    WorthUiIdentitySeedingMetrics, WorthUiIdentitySeedingReport, WorthUiStructuralLegalityLowerer,
};
use worth_ui_dsl::{
    WorthUiArtifactInputBodyAtom, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule,
};

use super::identity_app_fixture::identity_test_app;

pub(super) fn identity_seeded_from_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> (
    WorthUiIdentitySeededArtifactInput,
    WorthUiIdentitySeedingMetrics,
) {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("phase 5 legality");
    let bound =
        WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).expect("phase 6 binding");
    WorthUiIdentitySeedLowerer::lower(&bound).expect("phase 7 identity seeding")
}

pub(super) fn identity_seeding_report_from_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiIdentitySeedingReport {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("phase 5 legality");
    let bound =
        WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).expect("phase 6 binding");
    WorthUiIdentitySeedLowerer::lower(&bound).expect_err("phase 7 identity seeding should fail")
}

pub(super) fn authored_component_module(
    authored_identity: &str,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms_and_authored_identity(
            "workspace.component.dashboard",
            authored_identity,
            standard_component_body_atoms(),
        )
}

pub(crate) fn structural_component_module(
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms("workspace.component.dashboard", body_atoms)
}

pub(crate) fn primary_only_component_body_atoms() -> Vec<WorthUiArtifactInputBodyAtom> {
    vec![
        ident("region"),
        ident("workspace.region.primary"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.fill"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.region_scroll"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("mount"),
        ident("workspace.surface.main"),
        ident("placement"),
        ident("workspace.placement.primary"),
        ident("state"),
        ident("workspace.state.primary_surface"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]
}

pub(crate) fn standard_component_body_atoms() -> Vec<WorthUiArtifactInputBodyAtom> {
    vec![
        ident("region"),
        ident("workspace.region.primary"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.fill"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.region_scroll"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("mount"),
        ident("workspace.surface.main"),
        ident("placement"),
        ident("workspace.placement.primary"),
        ident("state"),
        ident("workspace.state.primary_surface"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
        ident("region"),
        ident("workspace.region.overlay"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("state"),
        ident("workspace.state.overlay_pinned"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("mount"),
        ident("workspace.surface.overlay"),
        ident("placement"),
        ident("workspace.placement.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]
}

pub(crate) fn imported_identity_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    [
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/panels/inspector.wui")
            .with_component("workspace.component.dashboard")
            .with_surface("workspace.surface.inspector")
            .with_binding("workspace.view_binding.selection")
            .with_token("theme.text.default", "theme.text.primary"),
        WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
            .with_component("workspace.component.inspector_panel"),
    ]
}

pub(crate) fn reordered_imported_identity_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    let [main, inspector] = imported_identity_modules();
    [inspector, main]
}

pub(super) fn authored_surface_binding_and_token_module() -> WorthUiRustAuthoredArtifactInputModule
{
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_surface_authored_identity("workspace.surface.inspector", "surface.identity")
        .with_binding_authored_identity("workspace.view_binding.selection", "binding.identity")
        .with_token_authored_identity("theme.text.default", "token.identity", "theme.text.primary")
}

pub(super) fn duplicate_component_authored_identity_modules(
) -> [WorthUiRustAuthoredArtifactInputModule; 1] {
    [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_authored_identity("workspace.component.dashboard", "duplicate.identity")
        .with_component_authored_identity(
            "workspace.component.inspector_panel",
            "duplicate.identity",
        )]
}

pub(super) fn duplicate_surface_binding_and_token_identity_modules(
) -> [WorthUiRustAuthoredArtifactInputModule; 1] {
    [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_surface_authored_identity("workspace.surface.main", "surface.duplicate")
        .with_surface_authored_identity("workspace.surface.inspector", "surface.duplicate")
        .with_binding_authored_identity("workspace.view_binding.selection", "binding.duplicate")
        .with_binding_authored_identity("workspace.view_binding.selection", "binding.duplicate")
        .with_token_authored_identity(
            "theme.text.primary",
            "token.duplicate",
            "theme.text.primary",
        )
        .with_token_authored_identity(
            "theme.text.default",
            "token.duplicate",
            "theme.text.primary",
        )]
}

pub(super) fn component_node<'a>(
    package: &'a WorthUiIdentitySeededArtifactInput,
    component_id: &str,
) -> &'a WorthUiIdentitySeededArtifactInputComponentNode {
    package
        .module_ids()
        .iter()
        .filter_map(|module_id| package.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiIdentitySeededArtifactInputNode::Component(node)
                if node.component().id().as_str() == component_id =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("component node should exist")
}

pub(super) fn import_node<'a>(
    package: &'a WorthUiIdentitySeededArtifactInput,
    target: &str,
) -> &'a WorthUiIdentitySeededArtifactInputImportNode {
    package
        .module_ids()
        .iter()
        .filter_map(|module_id| package.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiIdentitySeededArtifactInputNode::Import(node)
                if node.target().authored_text() == target =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("import node should exist")
}

pub(super) fn surface_node<'a>(
    package: &'a WorthUiIdentitySeededArtifactInput,
    surface_id: &str,
) -> &'a WorthUiIdentitySeededArtifactInputSurfaceNode {
    package
        .module_ids()
        .iter()
        .filter_map(|module_id| package.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiIdentitySeededArtifactInputNode::Surface(node)
                if node.surface().id().as_str() == surface_id =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("surface node should exist")
}

pub(super) fn binding_node<'a>(
    package: &'a WorthUiIdentitySeededArtifactInput,
    binding_id: &str,
) -> &'a WorthUiIdentitySeededArtifactInputBindingNode {
    package
        .module_ids()
        .iter()
        .filter_map(|module_id| package.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiIdentitySeededArtifactInputNode::Binding(node)
                if node.view_binding_reference().view_binding().id().as_str() == binding_id =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("binding node should exist")
}

pub(super) fn token_node<'a>(
    package: &'a WorthUiIdentitySeededArtifactInput,
    token_id: &str,
) -> &'a WorthUiIdentitySeededArtifactInputThemeTokenNode {
    package
        .module_ids()
        .iter()
        .filter_map(|module_id| package.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiIdentitySeededArtifactInputNode::Token(node)
                if node.theme_token().id().as_str() == token_id =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("theme token node should exist")
}

pub(super) fn assert_authored_seed_kind(node: &WorthUiIdentitySeededArtifactInputComponentNode) {
    assert_eq!(
        node.identity_seed().kind(),
        &WorthUiArtifactIdentitySeedKind::Authored
    );
    assert!(node.identity_seed().is_stable());
}

pub(super) fn assert_durable_eligible_count(
    eligibility: &WorthUiDurableStateEligibility,
    expected_count: usize,
) {
    assert_eq!(
        eligibility,
        &WorthUiDurableStateEligibility::Eligible {
            restorable_state_slot_count: expected_count,
        }
    );
}

pub(super) fn assert_ineligible_reason(
    eligibility: &WorthUiDurableStateEligibility,
    expected_reason: WorthUiDurableStateIneligibilityReason,
) {
    assert_eq!(
        eligibility,
        &WorthUiDurableStateEligibility::Ineligible {
            reason: expected_reason,
        }
    );
}

pub(super) fn assert_duplicate_authored_identity_report(
    report: &WorthUiIdentitySeedingReport,
    semantic_locus: &str,
    conflicting_locus: &str,
) {
    assert_eq!(report.metrics().authored_seed_count(), 2);
    assert_eq!(report.diagnostics().len(), 1);
    let diagnostic = &report.diagnostics()[0];
    assert_eq!(
        diagnostic.code(),
        WorthUiIdentitySeedingDiagnosticCode::DuplicateAuthoredIdentitySeed
    );
    assert_eq!(diagnostic.authored_identity(), "duplicate.identity");
    assert_eq!(diagnostic.semantic_locus(), semantic_locus);
    assert_eq!(diagnostic.conflicting_locus(), conflicting_locus);
}

pub(super) fn assert_multi_family_duplicate_authored_identity_report(
    report: &WorthUiIdentitySeedingReport,
) {
    assert_eq!(report.metrics().authored_seed_count(), 6);
    assert_eq!(report.diagnostics().len(), 3);

    let diagnostics = report.diagnostics();
    assert_eq!(diagnostics[0].authored_identity(), "binding.duplicate");
    assert_eq!(
        diagnostics[0].semantic_locus(),
        "binding:workspace.view_binding.selection"
    );
    assert_eq!(
        diagnostics[0].conflicting_locus(),
        "binding:workspace.view_binding.selection"
    );

    assert_eq!(diagnostics[1].authored_identity(), "surface.duplicate");
    assert_eq!(
        diagnostics[1].semantic_locus(),
        "surface:workspace.surface.main"
    );
    assert_eq!(
        diagnostics[1].conflicting_locus(),
        "surface:workspace.surface.inspector"
    );

    assert_eq!(diagnostics[2].authored_identity(), "token.duplicate");
    assert_eq!(diagnostics[2].semantic_locus(), "token:theme.text.primary");
    assert_eq!(
        diagnostics[2].conflicting_locus(),
        "token:theme.text.default"
    );
}

fn ident(text: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(text.to_owned())
}
