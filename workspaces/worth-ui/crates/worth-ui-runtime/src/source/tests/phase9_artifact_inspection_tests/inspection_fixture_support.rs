use crate::source::{
    WorthUiArtifact, WorthUiArtifactHandle, WorthUiArtifactInspection,
    WorthUiArtifactInspectionBasis, WorthUiArtifactInspectionBasisBuilder,
    WorthUiArtifactInspectionDeriver, WorthUiArtifactInspectionMetrics, WorthUiArtifactNode,
    WorthUiCanonicalArtifactAssembler,
};
use worth_ui_dsl::{WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;
use super::super::phase7_identity_seeding_tests::identity_fixture_support::{
    imported_identity_modules, standard_component_body_atoms, structural_component_module,
};

pub(super) fn rust_inspection_subject_from_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> (
    WorthUiArtifact,
    WorthUiArtifactInspectionBasis,
    WorthUiArtifactInspection,
    WorthUiArtifactInspectionMetrics,
) {
    let identity_seeded = identity_seeded_from_rust_modules(modules);
    let artifact = WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact assembly should succeed");
    let basis = WorthUiArtifactInspectionBasisBuilder::build(&artifact, &identity_seeded)
        .expect("phase 9 basis build should succeed");
    let (inspection, metrics) =
        WorthUiArtifactInspectionDeriver::derive_with_metrics(&artifact, &basis)
            .expect("phase 9 inspection derivation should succeed");
    (artifact, basis, inspection, metrics)
}

pub(super) fn inspection_basis_from_rust_modules<const N: usize>(
    artifact: &WorthUiArtifact,
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> Result<WorthUiArtifactInspectionBasis, crate::source::WorthUiArtifactInspectionReport> {
    let identity_seeded = identity_seeded_from_rust_modules(modules);
    WorthUiArtifactInspectionBasisBuilder::build(artifact, &identity_seeded)
}

pub(super) fn file_authored_inspection_subject(
    main_module_source: &str,
    inspector_module_source: &str,
) -> (
    WorthUiArtifact,
    WorthUiArtifactInspectionBasis,
    WorthUiArtifactInspection,
) {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let artifact_input = crate::source::test_compilation::compile_source([
        ("app/main.wui", main_module_source),
        ("app/panels/inspector.wui", inspector_module_source),
    ]);
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    let structured = crate::source::WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 legality should succeed");
    let bound = crate::source::WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("phase 6 binding should succeed");
    let identity_seeded = crate::source::WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("phase 7 identity should succeed")
        .0;
    let artifact = WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact should succeed");
    let basis = WorthUiArtifactInspectionBasisBuilder::build(&artifact, &identity_seeded)
        .expect("phase 9 basis should succeed");
    let inspection = WorthUiArtifactInspectionDeriver::derive(&artifact, &basis)
        .expect("phase 9 inspection should succeed");
    (artifact, basis, inspection)
}

pub(super) fn imported_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    imported_identity_modules()
}

pub(super) fn structureful_component_modules() -> [WorthUiRustAuthoredArtifactInputModule; 1] {
    [structural_component_module(standard_component_body_atoms())]
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

pub(super) fn same_shape_but_misaligned_rust_authored_modules(
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

pub(super) fn first_handle(artifact: &WorthUiArtifact) -> WorthUiArtifactHandle {
    artifact
        .module(artifact.module_ids().first().expect("module id"))
        .expect("artifact module")
        .nodes()
        .first()
        .expect("artifact node")
        .handle()
        .clone()
}

pub(super) fn node_handle_by_kind_and_id(
    artifact: &WorthUiArtifact,
    expected_kind: crate::source::WorthUiArtifactNodeKind,
    expected_id: &str,
) -> WorthUiArtifactHandle {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Import(node)
                if expected_kind == crate::source::WorthUiArtifactNodeKind::Import
                    && node.target().authored_text() == expected_id =>
            {
                Some(node.handle().clone())
            }
            WorthUiArtifactNode::Component(node)
                if expected_kind == crate::source::WorthUiArtifactNodeKind::Component
                    && node.component().id().as_str() == expected_id =>
            {
                Some(node.handle().clone())
            }
            WorthUiArtifactNode::Surface(node)
                if expected_kind == crate::source::WorthUiArtifactNodeKind::Surface
                    && node.surface().id().as_str() == expected_id =>
            {
                Some(node.handle().clone())
            }
            WorthUiArtifactNode::Binding(node)
                if expected_kind == crate::source::WorthUiArtifactNodeKind::Binding
                    && node.view_binding_reference().view_binding().id().as_str()
                        == expected_id =>
            {
                Some(node.handle().clone())
            }
            WorthUiArtifactNode::Token(node)
                if expected_kind == crate::source::WorthUiArtifactNodeKind::Token
                    && node.theme_token().id().as_str() == expected_id =>
            {
                Some(node.handle().clone())
            }
            _ => None,
        })
        .expect("artifact handle should exist")
}

pub(super) fn inspection_semantic_summary(
    inspection: &WorthUiArtifactInspection,
) -> Vec<(
    crate::source::WorthUiArtifactNodeKind,
    Vec<String>,
    Vec<String>,
)> {
    inspection
        .handles()
        .iter()
        .map(|handle| {
            let node = inspection.node(handle).expect("inspection node");
            let capability_summary = node
                .capability_references()
                .iter()
                .map(|reference| format!("{:?}:{:?}", reference.role(), reference.reference()))
                .collect::<Vec<_>>();
            let query_summary = node
                .query_inspection_links()
                .iter()
                .map(|link| {
                    format!(
                        "{:?}:{}:{}",
                        link.role(),
                        link.view_binding().id().as_str(),
                        link.definition().digest().as_u64()
                    )
                })
                .collect::<Vec<_>>();
            (node.node_kind(), capability_summary, query_summary)
        })
        .collect()
}

fn identity_seeded_from_rust_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> crate::source::WorthUiIdentitySeededArtifactInput {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    let structured = crate::source::WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 legality should succeed");
    let bound = crate::source::WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("phase 6 binding should succeed");
    crate::source::WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("phase 7 identity should succeed")
        .0
}
