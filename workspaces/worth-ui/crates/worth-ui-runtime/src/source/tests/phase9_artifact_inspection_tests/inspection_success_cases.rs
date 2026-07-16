use crate::source::{
    WorthUiArtifactCapabilityReference, WorthUiArtifactCapabilityReferenceRole,
    WorthUiArtifactNodeKind, WorthUiArtifactSourceOrigin, WorthUiQueryInspectionLinkRole,
};

use super::inspection_fixture_support::{
    first_handle, imported_modules, inspection_basis_from_rust_modules, node_handle_by_kind_and_id,
    rust_inspection_subject_from_modules, same_shape_but_misaligned_rust_authored_modules,
    structureful_component_modules,
};

#[test]
fn artifact_inspection_explains_source_and_capability_origin() {
    let (artifact, _, inspection, metrics) =
        rust_inspection_subject_from_modules(imported_modules());

    let import_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Import,
        "app/panels/inspector.wui",
    );
    let binding_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Binding,
        "workspace.view_binding.selection",
    );
    let surface_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Surface,
        "workspace.surface.inspector",
    );

    let import_inspection = inspection.node(&import_handle).expect("import inspection");
    let binding_inspection = inspection
        .node(&binding_handle)
        .expect("binding inspection");
    let surface_inspection = inspection
        .node(&surface_handle)
        .expect("surface inspection");

    assert!(matches!(
        import_inspection.source_origin(),
        WorthUiArtifactSourceOrigin::RustAuthoredDeclaration { authored_module_path, .. }
            if authored_module_path == "app/main.wui"
    ));
    assert!(binding_inspection
        .capability_references()
        .iter()
        .any(|reference| {
            reference.role() == WorthUiArtifactCapabilityReferenceRole::BoundViewBinding
                && matches!(
                    reference.reference(),
                    WorthUiArtifactCapabilityReference::ViewBinding(binding)
                        if binding.id().as_str() == "workspace.view_binding.selection"
                )
        }));
    assert!(surface_inspection
        .capability_references()
        .iter()
        .any(|reference| {
            reference.role() == WorthUiArtifactCapabilityReferenceRole::SurfaceCommand
                && matches!(
                    reference.reference(),
                    WorthUiArtifactCapabilityReference::Command(command)
                        if command.id().as_str() == "workspace.command.inspect"
                )
        }));
    assert!(binding_inspection
        .query_inspection_links()
        .iter()
        .any(|link| {
            link.role() == WorthUiQueryInspectionLinkRole::BindingViewBindingQuery
                && link.view_binding().id().as_str() == "workspace.view_binding.selection"
                && link.definition().digest().as_u64() != 0
        }));
    assert_eq!(surface_inspection.handle(), &surface_handle);
    assert_eq!(inspection.provenance_map().handles(), inspection.handles());
    assert_eq!(
        inspection
            .provenance_map()
            .source_origin(&surface_handle)
            .expect("surface provenance map origin"),
        surface_inspection.source_origin()
    );
    assert_eq!(metrics.modules_inspected(), artifact.module_ids().len());
    assert_eq!(metrics.broad_registry_scans(), 0);
}

#[test]
fn import_nodes_remain_inspectable_without_fake_capability_references() {
    let (artifact, _, inspection, _) = rust_inspection_subject_from_modules(imported_modules());
    let import_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Import,
        "app/panels/inspector.wui",
    );
    let import_inspection = inspection.node(&import_handle).expect("import inspection");

    assert!(import_inspection.capability_references().is_empty());
    assert!(import_inspection.query_inspection_links().is_empty());
}

#[test]
fn structure_capability_references_are_derived_from_canonical_artifact_structure() {
    let (artifact, _, inspection, metrics) =
        rust_inspection_subject_from_modules(structureful_component_modules());
    let component_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Component,
        "workspace.component.dashboard",
    );
    let component_inspection = inspection
        .node(&component_handle)
        .expect("component inspection");

    assert!(component_inspection
        .capability_references()
        .iter()
        .any(|reference| reference.role()
            == WorthUiArtifactCapabilityReferenceRole::StructureRegionKind));
    assert!(component_inspection
        .capability_references()
        .iter()
        .any(|reference| reference.role()
            == WorthUiArtifactCapabilityReferenceRole::StructureMountSurface));
    assert!(metrics.capability_references_recorded() > 1);
}

#[test]
fn missing_source_origin_fails_closed_at_inspection_boundary() {
    let (artifact, basis, _, _) = rust_inspection_subject_from_modules(imported_modules());
    let incomplete_basis = basis.without_handle(&first_handle(&artifact));

    let report =
        crate::source::WorthUiArtifactInspectionDeriver::derive(&artifact, &incomplete_basis)
            .expect_err("missing source origin should fail inspection derivation");

    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].code(),
        crate::source::WorthUiArtifactInspectionDiagnosticCode::MissingArtifactSourceOrigin
    );
    assert!(report.metrics().nodes_inspected() > 0);
}

#[test]
fn multiple_missing_source_origins_report_in_stable_handle_order() {
    let (artifact, basis, _, _) = rust_inspection_subject_from_modules(imported_modules());
    let import_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Import,
        "app/panels/inspector.wui",
    );
    let token_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Token,
        "theme.text.default",
    );
    let incomplete_basis = basis
        .without_handle(&token_handle)
        .without_handle(&import_handle);

    let report =
        crate::source::WorthUiArtifactInspectionDeriver::derive(&artifact, &incomplete_basis)
            .expect_err("multiple missing source origins should fail inspection derivation");

    assert_eq!(report.diagnostics().len(), 2);
    assert_eq!(report.diagnostics()[0].handle(), Some(&import_handle));
    assert_eq!(report.diagnostics()[1].handle(), Some(&token_handle));
}

#[test]
fn shape_compatible_seeded_input_cannot_silently_rebind_provenance() {
    let (artifact, _, _, _) = rust_inspection_subject_from_modules(imported_modules());
    let report = inspection_basis_from_rust_modules(
        &artifact,
        same_shape_but_misaligned_rust_authored_modules(),
    )
    .expect_err("shape-compatible but semantically different basis should fail closed");

    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].code(),
        crate::source::WorthUiArtifactInspectionDiagnosticCode::ArtifactBasisAlignmentMismatch
    );
}

#[test]
fn query_link_semantics_survive_on_the_inspection_lane() {
    let (artifact, _, inspection, metrics) =
        rust_inspection_subject_from_modules(imported_modules());
    let binding_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Binding,
        "workspace.view_binding.selection",
    );
    let surface_handle = node_handle_by_kind_and_id(
        &artifact,
        WorthUiArtifactNodeKind::Surface,
        "workspace.surface.inspector",
    );
    let binding_link = inspection
        .node(&binding_handle)
        .expect("binding inspection")
        .query_inspection_links()
        .first()
        .expect("binding query link");
    let surface_link = inspection
        .node(&surface_handle)
        .expect("surface inspection")
        .query_inspection_links()
        .first()
        .expect("surface query link");

    assert_eq!(binding_link.definition(), surface_link.definition());
    assert_eq!(
        binding_link.denial_presentation(),
        surface_link.denial_presentation()
    );
    assert_eq!(metrics.query_links_recorded(), 2);
}
