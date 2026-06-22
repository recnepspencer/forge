use std::path::PathBuf;

use crate::source::{
    WorthUiSourcePackageDiagnosticCode, WorthUiSourcePackageLoader, WorthUiSourcePackageReport,
};

#[test]
fn equivalent_module_graphs_produce_equivalent_package_identity() {
    let workspace_root = PathBuf::from(r"C:\workspace");

    let package_a = WorthUiSourcePackageLoader::from_workspace_root(&workspace_root)
        .register_module_with_imports("app/main.wui", ["app/panels/inspector.wui"])
        .register_module("app/theme/tokens.wui")
        .register_module_with_imports("app/panels/inspector.wui", ["app/theme/tokens.wui"])
        .compile()
        .expect("package a should compile");

    let package_b = WorthUiSourcePackageLoader::from_workspace_root(&workspace_root)
        .register_module_with_imports("app/panels/inspector.wui", ["app/theme/tokens.wui"])
        .register_module_with_imports("app/main.wui", ["app/panels/inspector.wui"])
        .register_module("app/theme/tokens.wui")
        .compile()
        .expect("package b should compile");

    assert_eq!(package_a.digest(), package_b.digest());
    assert_eq!(package_a.digest().raw(), package_b.digest().raw());
    assert_eq!(package_a.module_ids(), package_b.module_ids());
    assert_eq!(package_a.workspace_root(), workspace_root.as_path());

    let main_module = package_a
        .module_record(&package_a.module_ids()[0])
        .expect("main module should be present");
    assert_eq!(main_module.module_id(), &package_a.module_ids()[0]);
    let imports = package_a
        .import_graph()
        .imports_for(main_module.module_id())
        .expect("module imports should be present");
    assert_eq!(imports, main_module.imports());
    assert_eq!(
        package_a.import_graph().adjacency().len(),
        package_a.module_ids().len()
    );
}

#[test]
fn source_text_change_changes_package_digest() {
    let package_a = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", "component Main {}")
        .compile()
        .expect("package a should compile");

    let package_b = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            "component Main { token accent = \"blue\"; }",
        )
        .compile()
        .expect("package b should compile");

    assert_ne!(package_a.digest(), package_b.digest());
    assert_ne!(package_a.digest().raw(), package_b.digest().raw());
}

#[test]
fn canonical_module_identity_ignores_relative_path_spelling_and_duplicate_import_edges() {
    let package_a = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_imports_and_source(
            r".\app\main.wui",
            [r".\app\panels\inspector.wui", r".\app\panels\inspector.wui"],
            "component Main {}",
        )
        .register_module_with_source(r".\app\panels\inspector.wui", "component Inspector {}")
        .compile()
        .expect("package a should compile");

    let package_b = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_imports_and_source(
            "app/main.wui",
            ["app/panels/inspector.wui"],
            "component Main {}",
        )
        .register_module_with_source("app/panels/inspector.wui", "component Inspector {}")
        .compile()
        .expect("package b should compile");

    assert_eq!(package_a.digest(), package_b.digest());
    assert_eq!(package_a.module_ids(), package_b.module_ids());

    let main_module = package_a
        .module_record(&package_a.module_ids()[0])
        .expect("main module should be present");
    assert_eq!(
        main_module.relative_path(),
        PathBuf::from("app/main.wui").as_path()
    );
    assert_eq!(main_module.imports().len(), 1);
}

#[test]
fn cyclic_source_module_import_rejected_before_parsing_progression() {
    let report = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_imports("app/main.wui", ["app/panels/inspector.wui"])
        .register_module_with_imports("app/panels/inspector.wui", ["app/main.wui"])
        .compile()
        .expect_err("cyclic import graph should fail");

    assert!(report
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code()
            == WorthUiSourcePackageDiagnosticCode::CyclicModuleImport));
    assert!(!report.is_empty());
    assert!(report
        .diagnostics()
        .iter()
        .all(|diagnostic| !diagnostic.message().is_empty()));
}

#[test]
fn duplicate_module_identity_rejected() {
    let report = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module(r".\app\main.wui")
        .register_module("app/main.wui")
        .compile()
        .expect_err("duplicate canonical module identity should fail");

    let duplicate_diagnostic = find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::DuplicateModuleIdentity,
    )
    .expect("duplicate-module diagnostic should be present");
    assert_eq!(
        duplicate_diagnostic.module_path(),
        Some(&PathBuf::from("app/main.wui"))
    );
    assert_eq!(duplicate_diagnostic.module_id_text(), Some("app/main.wui"));
    assert!(!report.is_empty());
}

#[test]
fn unknown_import_target_rejected() {
    let report = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_imports("app/main.wui", ["app/unknown.wui"])
        .compile()
        .expect_err("unknown import target should fail");

    let unknown_target_diagnostic = find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::UnknownImportTarget,
    )
    .expect("unknown-import diagnostic should be present");
    assert_eq!(
        unknown_target_diagnostic.module_path(),
        Some(&PathBuf::from("app/main.wui"))
    );
    assert_eq!(
        unknown_target_diagnostic.module_id_text(),
        Some("app/main.wui")
    );
    assert_eq!(
        unknown_target_diagnostic.related_module_id_text(),
        Some("app/unknown.wui")
    );
    assert!(!report.is_empty());
}

#[test]
fn module_path_cannot_escape_workspace_relative_package_boundary() {
    let report = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module("../outside.wui")
        .compile()
        .expect_err("module path escaping the package root should fail");

    let invalid_path_diagnostic = find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::InvalidModulePath,
    )
    .expect("invalid-path diagnostic should be present");
    assert_eq!(
        invalid_path_diagnostic.module_path(),
        Some(&PathBuf::from("../outside.wui"))
    );
    assert!(!report.is_empty());
}

#[test]
fn self_import_is_rejected_as_a_cycle() {
    let report = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_imports("app/main.wui", ["app/main.wui"])
        .compile()
        .expect_err("self-import cycle should fail");

    let cycle_diagnostic = find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::CyclicModuleImport,
    )
    .expect("cycle diagnostic should be present");
    assert_eq!(cycle_diagnostic.module_id_text(), Some("app/main.wui"));
    assert_eq!(cycle_diagnostic.related_module_id_text(), None);
}

#[test]
fn invalid_and_unknown_import_diagnostics_accumulate_before_cycle_analysis() {
    let report = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_imports(
            "app/main.wui",
            [
                "../outside.wui",
                "app/missing.wui",
                "app/panels/inspector.wui",
            ],
        )
        .register_module_with_imports("app/panels/inspector.wui", ["app/main.wui"])
        .compile()
        .expect_err("invalid import package should fail before cycle analysis");

    assert_eq!(report.diagnostics().len(), 2);
    let invalid_path_diagnostic = find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::InvalidModulePath,
    )
    .expect("invalid-path diagnostic should be present");
    let unknown_target_diagnostic = find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::UnknownImportTarget,
    )
    .expect("unknown-target diagnostic should be present");

    assert_eq!(
        invalid_path_diagnostic.module_path(),
        Some(&PathBuf::from("../outside.wui"))
    );
    assert_eq!(
        invalid_path_diagnostic.module_id_text(),
        Some("app/main.wui")
    );
    assert_eq!(
        unknown_target_diagnostic.related_module_id_text(),
        Some("app/missing.wui")
    );
    assert!(find_diagnostic(
        &report,
        WorthUiSourcePackageDiagnosticCode::CyclicModuleImport
    )
    .is_none());
}

fn find_diagnostic(
    report: &WorthUiSourcePackageReport,
    code: WorthUiSourcePackageDiagnosticCode,
) -> Option<&crate::source::WorthUiSourcePackageDiagnostic> {
    report
        .diagnostics()
        .iter()
        .find(|diagnostic| diagnostic.code() == code)
}
