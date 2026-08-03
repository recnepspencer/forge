use super::dependency_audit::path_starts_with;
use super::workspace_source_inventory::WorkspaceSourceInventory;

mod declaration_residue_ast;

use declaration_residue_ast::{
    audit_files_do_not_reopen_declaration_source, collect_file_paths, collect_file_use_paths,
    collect_method_names, collect_method_names_for_function, production_source_text,
    starts_with_declaration_surface,
};

const DECLARATION_SOURCE_REOPENING_ALLOWED_FILES: &[&str] = &[
    "crates/worth-ui-runtime/src/declaration/artifact/ui_declaration_lowering.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/contract.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/aspect_name.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/consumed.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/published.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/admission.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/measurement_policy/admission.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/measurement_policy/basis_source.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/measurement_policy/constraint_modifier.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/measurement_policy/evidence_requirement.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/measurement_policy/mode.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/measurement_policy/ownership_posture.rs",
    "crates/worth-ui-runtime/src/declaration/family/admission.rs",
    "crates/worth-ui-runtime/src/declaration/intent/authored_material.rs",
    "crates/worth-ui-runtime/src/declaration/rust_authored_declaration_fixture.rs",
    "crates/worth-ui-runtime/src/source/lower/artifact_dependency/worth_ui_subtree_digest_basis.rs",
    "crates/worth-ui-runtime/src/source/lower/artifact_equivalence/worth_ui_artifact_descriptor_basis.rs",
    "crates/worth-ui-runtime/src/declaration/structural_semantics/admission.rs",
];

const DECLARATION_SEMANTIC_AUTHORITY_TYPES: &[&str] = &[
    "UiDslAspectName",
    "UiDslLoweringReceipt",
    "UiDslSemanticArtifact",
    "UiDslSemanticFamily",
    "UiDslSemanticKey",
    "UiDslStructuralToken",
    "UiDslPostureToken",
    "UiDslSupportToken",
];
const DECLARATION_SOURCE_REOPENING_METHODS: &[&str] = &[
    "semantic_artifact",
    "structural_tokens",
    "posture_tokens",
    "support_tokens",
];
const DECLARATION_SEMANTIC_TOKEN_MARKERS: &[&str] = &[
    "\"page:",
    "\"page-set:",
    "\"region:",
    "\"mosaic:",
    "\"local-composition:",
    "\"control:",
    "\"diagnostic-surface:",
    "\"query-binding:",
    "\"intent:",
    "\"service:",
    "\"touch:",
    "\"measurement:",
    "\"host-capability:",
    "\"appearance.",
    "\"content.",
    "\"interaction.",
    "\"structure.",
];

pub fn audit_non_owner_code_does_not_reopen_declaration_source(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let scoped_roots = [
        "crates/worth-ui/src",
        "crates/worth-ui-runtime/src",
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
    ];
    let mut violations = Vec::new();
    let files = scoped_roots
        .into_iter()
        .flat_map(|scoped_root| inventory.rust_files_under(scoped_root))
        .collect::<Vec<_>>();

    for source_file in files {
        let path = source_file.absolute_path();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if file_name == "tests.rs" || file_name.ends_with("_tests.rs") {
            continue;
        }

        let relative = path
            .strip_prefix(inventory.root())
            .expect("workspace file should strip to relative path");
        let relative_text = relative.to_string_lossy().replace('\\', "/");

        if should_skip_non_owner_audit_file(&relative_text) {
            continue;
        }

        if DECLARATION_SOURCE_REOPENING_ALLOWED_FILES
            .iter()
            .any(|allowed| *allowed == relative_text)
        {
            continue;
        }

        for segments in collect_file_paths(inventory, path)
            .into_iter()
            .chain(collect_file_use_paths(inventory, path))
        {
            if let Some(authority_name) = declaration_semantic_authority_path(&segments) {
                violations.push(format!(
                    "{} reopens declaration meaning by reaching DSL semantic authority type `{authority_name}` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }

        for method_name in collect_method_names(inventory, path) {
            if DECLARATION_SOURCE_REOPENING_METHODS.contains(&method_name.as_str()) {
                violations.push(format!(
                    "{} reopens declaration meaning through DSL semantic accessor `{method_name}()` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }

        let source = production_source_text(inventory, path);
        for marker in DECLARATION_SEMANTIC_TOKEN_MARKERS {
            if source.contains(marker) {
                violations.push(format!(
                    "{} reinterprets declaration semantics through raw declaration token vocabulary `{marker}` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_phase4_authored_lookup_lane_does_not_reopen_declaration_source(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let scoped_files = [
        "crates/worth-ui-runtime/src/facade/entry/app.rs",
        "crates/worth-ui-runtime/src/facade/inspection/authored_lookup_boundary.rs",
        "crates/worth-ui-runtime/src/declaration/inspection/index/authored_evidence_index.rs",
    ];
    let files = scoped_files
        .iter()
        .map(|relative| inventory.absolute_path(relative))
        .collect::<Vec<_>>();

    audit_files_do_not_reopen_declaration_source(inventory, &files)
}

pub fn audit_phase4_authored_lookup_lane_is_indexed_not_scan_first(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let authored_lookup_boundary = inventory
        .absolute_path("crates/worth-ui-runtime/src/facade/inspection/authored_lookup_boundary.rs");
    let authored_evidence_index = inventory.absolute_path(
        "crates/worth-ui-runtime/src/declaration/inspection/index/authored_evidence_index.rs",
    );
    let boundary_source = inventory.text(&authored_lookup_boundary);
    let mut violations = Vec::new();

    for required in ["lookup_declaration_identity", "lookup_authored_provenance"] {
        if !boundary_source.contains(required) {
            violations.push(format!(
                "{} no longer routes authored lookup through the costed authored index access `{required}`",
                authored_lookup_boundary.display()
            ));
        }
    }

    for function_name in ["lookup_declaration_identity", "lookup_authored_provenance"] {
        let method_names =
            collect_method_names_for_function(inventory, &authored_evidence_index, function_name);
        if !method_names.iter().any(|name| name == "get") {
            violations.push(format!(
                "{} no longer proves indexed authored lookup because `{function_name}` does not call map lookup `get()` on the authoritative authored index",
                authored_evidence_index.display()
            ));
        }
        for forbidden in ["iter", "find", "position", "collect"] {
            if method_names.iter().any(|name| name == forbidden) {
                violations.push(format!(
                    "{} appears to scan during ordinary authored lookup because `{function_name}` calls `{forbidden}()` instead of staying on direct authored index access",
                    authored_evidence_index.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_host_and_inspection_layers_do_not_import_declaration_authority(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let scoped_roots = [
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
    ];
    let mut violations = Vec::new();
    let files = scoped_roots
        .into_iter()
        .flat_map(|scoped_root| inventory.rust_files_under(scoped_root))
        .collect::<Vec<_>>();

    for source_file in files {
        let path = source_file.absolute_path();
        for segments in collect_file_paths(inventory, path) {
            if starts_with_declaration_surface(&segments) {
                violations.push(format!(
                    "{} imports declaration authority into a host/inspection layer instead of consuming lowered receipts or host-contract facts",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn declaration_semantic_authority_path(segments: &[String]) -> Option<&str> {
    if !path_starts_with(segments, "worth_ui_dsl") {
        return None;
    }

    DECLARATION_SEMANTIC_AUTHORITY_TYPES
        .iter()
        .copied()
        .find(|name| segments.iter().any(|segment| segment == name))
}

fn should_skip_non_owner_audit_file(relative_text: &str) -> bool {
    relative_text.contains("_test_support.rs")
        || relative_text.contains("_certification_support.rs")
        || relative_text.contains("/certification_support/")
}
