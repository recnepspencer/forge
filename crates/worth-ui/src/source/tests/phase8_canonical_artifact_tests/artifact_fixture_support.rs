use std::collections::BTreeMap;
use std::path::Path;

use crate::source::{
    WorthUiArtifact, WorthUiArtifactBindingNode, WorthUiArtifactComponentNode,
    WorthUiArtifactHandle, WorthUiArtifactInputImportNode, WorthUiArtifactInputProvenance,
    WorthUiArtifactInputReference, WorthUiArtifactNode, WorthUiArtifactSurfaceNode,
    WorthUiArtifactThemeTokenNode, WorthUiCanonicalArtifactAssembler,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
    WorthUiIdentitySeedLowerer, WorthUiIdentitySeededArtifactInput,
    WorthUiIdentitySeededArtifactInputImportNode, WorthUiIdentitySeededArtifactInputModule,
    WorthUiIdentitySeededArtifactInputNode, WorthUiSourceModuleId,
};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;
use super::super::phase7_identity_seeding_tests::identity_fixture_support::{
    imported_identity_modules, reordered_imported_identity_modules, standard_component_body_atoms,
    structural_component_module,
};

pub(super) fn assembled_artifact_from_modules<const N: usize>(
    modules: [crate::source::WorthUiRustAuthoredArtifactInputModule; N],
) -> (
    WorthUiArtifact,
    crate::source::WorthUiArtifactAssemblyMetrics,
) {
    let identity_seeded = identity_seeded_from_modules(modules);
    WorthUiCanonicalArtifactAssembler::assemble_with_metrics(&identity_seeded)
        .expect("phase 8 artifact assembly should succeed")
}

pub(super) fn identity_seeded_from_modules<const N: usize>(
    modules: [crate::source::WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiIdentitySeededArtifactInput {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let artifact_input = crate::source::WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &crate::source::WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution");
    let structured = crate::source::WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 legality");
    let bound = crate::source::WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("phase 6 binding");

    WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("phase 7 identity seeding")
        .0
}

pub(super) fn imported_modules() -> [crate::source::WorthUiRustAuthoredArtifactInputModule; 2] {
    imported_identity_modules()
}

pub(super) fn reordered_modules() -> [crate::source::WorthUiRustAuthoredArtifactInputModule; 2] {
    reordered_imported_identity_modules()
}

pub(super) fn canonical_declaration_module(
) -> [crate::source::WorthUiRustAuthoredArtifactInputModule; 1] {
    [
        crate::source::WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("workspace.component.dashboard")
            .with_surface("workspace.surface.inspector")
            .with_binding("workspace.view_binding.selection")
            .with_token("theme.text.default", "theme.text.primary"),
    ]
}

pub(super) fn structureful_component_module(
) -> [crate::source::WorthUiRustAuthoredArtifactInputModule; 1] {
    [structural_component_module(standard_component_body_atoms())]
}

pub(super) fn canonical_identity_seeded_declaration_input() -> WorthUiIdentitySeededArtifactInput {
    identity_seeded_from_modules(canonical_declaration_module())
}

pub(super) fn reordered_identity_seeded_declaration_input() -> WorthUiIdentitySeededArtifactInput {
    let canonical = canonical_identity_seeded_declaration_input();
    let module_id = canonical.module_ids().first().expect("module id").clone();
    let module = canonical.module(&module_id).expect("seeded module");
    let mut reversed_nodes = module.nodes().to_vec();
    reversed_nodes.reverse();

    let mut modules = BTreeMap::new();
    modules.insert(
        module_id.clone(),
        WorthUiIdentitySeededArtifactInputModule::new(module_id.clone(), reversed_nodes),
    );

    WorthUiIdentitySeededArtifactInput::new(modules, vec![module_id])
}

pub(super) fn artifact_component_node<'a>(
    artifact: &'a WorthUiArtifact,
    component_id: &str,
) -> &'a WorthUiArtifactComponentNode {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Component(node)
                if node.component().id().as_str() == component_id =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("artifact component node should exist")
}

pub(super) fn artifact_surface_node<'a>(
    artifact: &'a WorthUiArtifact,
    surface_id: &str,
) -> &'a WorthUiArtifactSurfaceNode {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Surface(node) if node.surface().id().as_str() == surface_id => {
                Some(node)
            }
            _ => None,
        })
        .expect("artifact surface node should exist")
}

pub(super) fn artifact_binding_node<'a>(
    artifact: &'a WorthUiArtifact,
    binding_id: &str,
) -> &'a WorthUiArtifactBindingNode {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Binding(node)
                if node.view_binding_reference().view_binding().id().as_str() == binding_id =>
            {
                Some(node)
            }
            _ => None,
        })
        .expect("artifact binding node should exist")
}

pub(super) fn artifact_token_node<'a>(
    artifact: &'a WorthUiArtifact,
    token_id: &str,
) -> &'a WorthUiArtifactThemeTokenNode {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Token(node) if node.theme_token().id().as_str() == token_id => {
                Some(node)
            }
            _ => None,
        })
        .expect("artifact theme token node should exist")
}

pub(super) fn handle_round_trip(
    artifact: &WorthUiArtifact,
    handle: &WorthUiArtifactHandle,
) -> bool {
    artifact.node(handle).is_some()
}

pub(super) fn import_only_identity_seeded_input(
    declaration_index: usize,
) -> WorthUiIdentitySeededArtifactInput {
    identity_seeded_from_manual_nodes(vec![manual_import_node(
        "app/shared.wui",
        "module:app/main.wui|import:app/shared.wui",
        declaration_index,
    )])
}

pub(super) fn duplicate_import_identity_seeded_input() -> WorthUiIdentitySeededArtifactInput {
    identity_seeded_from_manual_nodes(vec![
        manual_import_node(
            "app/shared.wui",
            "module:app/main.wui|import:app/shared.wui",
            0,
        ),
        manual_import_node(
            "app/shared.wui",
            "module:app/main.wui|import:app/shared.wui",
            1,
        ),
    ])
}

fn identity_seeded_from_manual_nodes(
    nodes: Vec<WorthUiIdentitySeededArtifactInputNode>,
) -> WorthUiIdentitySeededArtifactInput {
    let module_id = module_id("app/main.wui");
    let mut modules = BTreeMap::new();
    modules.insert(
        module_id.clone(),
        WorthUiIdentitySeededArtifactInputModule::new(module_id.clone(), nodes),
    );
    WorthUiIdentitySeededArtifactInput::new(modules, vec![module_id])
}

fn manual_import_node(
    target: &str,
    seed_basis: &str,
    declaration_index: usize,
) -> WorthUiIdentitySeededArtifactInputNode {
    WorthUiIdentitySeededArtifactInputNode::Import(
        WorthUiIdentitySeededArtifactInputImportNode::new(
            WorthUiArtifactInputImportNode::new(
                WorthUiArtifactInputReference::new(target),
                WorthUiArtifactInputProvenance::rust_authored("app/main.wui", declaration_index),
            ),
            crate::source::WorthUiArtifactIdentitySeed::structural_fallback(seed_basis.to_owned()),
            WorthUiDurableStateEligibility::Ineligible {
                reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
            },
        ),
    )
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("module id")
}
