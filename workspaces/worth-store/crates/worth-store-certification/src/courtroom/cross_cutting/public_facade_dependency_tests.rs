use crate::courtroom::source_tree::repository_source;
use std::fs;

#[test]
fn store_public_facade_exports_aspect_native_boundary_only() {
    let facade = workspace_file("workspaces/worth-store/crates/worth-store/src/lib.rs");
    let aspect_native = facade_module_body(&facade, "aspect_native");
    let certification = facade_module_body(&facade, "certification");
    let terminal_projection = facade_module_body(&facade, "terminal_projection");

    assert_module_exports(
        "aspect_native",
        aspect_native,
        &[
            "StoreAspectBoundaryFact",
            "StorePhysicalBoundaryWitness",
            "StorePhysicalAuthorityWitness",
        ],
    );
    assert_module_exports(
        "certification",
        certification,
        &[
            "certify_store_json_residue_inventory",
            "StoreJsonResidueInventory",
            "StoreJsonResidueDenial",
        ],
    );
    assert_module_exports(
        "terminal_projection",
        terminal_projection,
        &[
            "project_store_boundary_fact_to_terminal_json",
            "readmit_external_terminal_projection_document_as_store_aspect_state",
            "StoreTerminalJsonProjection",
        ],
    );
    assert_no_json_or_serde_export("aspect_native", aspect_native);
    assert_no_json_or_serde_export("certification", certification);
}

#[test]
fn workspace_serde_json_dependency_is_terminal_only() {
    let workspace_manifest = workspace_file("workspaces/worth-store/Cargo.toml");
    let facade_manifest = workspace_file("workspaces/worth-store/crates/worth-store/Cargo.toml");
    let aspect_native_manifest =
        workspace_file("workspaces/worth-store/crates/worth-store-aspect-native/Cargo.toml");
    let certification_manifest =
        workspace_file("workspaces/worth-store/crates/worth-store-certification/Cargo.toml");

    assert!(!workspace_manifest.contains("serde_json"));
    assert_manifest_dep_section_lacks_serde_json(&facade_manifest);
    assert_manifest_dev_dep_section_contains_serde_json(&facade_manifest);
    assert!(aspect_native_manifest.contains("serde_json ="));
    assert_manifest_dep_section_lacks_serde_json(&certification_manifest);
    assert_manifest_dev_dep_section_contains_serde_json(&certification_manifest);
}

#[test]
fn generic_serde_authority_helpers_are_rejected() {
    for path in public_authority_source_paths() {
        let source = workspace_file(path);
        for (line_number, line) in source.lines().enumerate() {
            if is_terminal_or_readmission_path(path) {
                continue;
            }
            assert!(
                !contains_forbidden_public_serde_surface(line),
                "{path}:{} exposes a forbidden JSON/serde authority surface: {line}",
                line_number + 1
            );
        }
    }
}

fn public_authority_source_paths() -> Vec<&'static str> {
    vec![
        "workspaces/worth-store/crates/worth-store/src/lib.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/authority/authoritative_state.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/authority/authoritative_patch.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/contract_admission.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/value_admission.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/physical_witness.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/terminal_json_projection.rs",
        "workspaces/worth-store/crates/worth-store-aspect-native/src/json_ingress_readmission.rs",
    ]
}

fn is_terminal_or_readmission_path(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-aspect-native/src/terminal_json_projection.rs"
            | "workspaces/worth-store/crates/worth-store-aspect-native/src/json_ingress_readmission.rs"
    )
}

fn contains_forbidden_public_serde_surface(line: &str) -> bool {
    line.contains("T: Serialize")
        || line.contains("T : Serialize")
        || line.contains("DeserializeOwned")
        || line.contains("serde_json::Value")
        || line.contains("serde_json :: Value")
        || line.contains("JsonDocument")
        || line.contains("json_document")
        || line.contains("pub fn from_json")
        || line.contains("pub fn from_raw_json")
}

fn assert_manifest_dep_section_lacks_serde_json(manifest: &str) {
    let dependencies = manifest_section(manifest, "[dependencies]");
    assert!(!dependencies.contains("serde_json"));
}

fn assert_manifest_dev_dep_section_contains_serde_json(manifest: &str) {
    let dev_dependencies = manifest_section(manifest, "[dev-dependencies]");
    assert!(dev_dependencies.contains("serde_json"));
}

fn manifest_section<'a>(manifest: &'a str, heading: &str) -> &'a str {
    let Some(start) = manifest.find(heading) else {
        return "";
    };
    let after_heading = &manifest[start + heading.len()..];
    let end = after_heading.find("\n[").unwrap_or(after_heading.len());
    &after_heading[..end]
}

fn facade_module_body<'a>(facade: &'a str, module_name: &str) -> &'a str {
    let module_header = format!("pub mod {module_name} {{");
    let Some(module_start) = facade.find(&module_header) else {
        panic!("facade is missing public module {module_name}");
    };
    let body_start = module_start + module_header.len();
    let mut depth = 1usize;
    for (offset, character) in facade[body_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &facade[body_start..body_start + offset];
                }
            }
            _ => {}
        }
    }
    panic!("facade module {module_name} is not closed")
}

fn assert_module_exports(module_name: &str, module_body: &str, expected_exports: &[&str]) {
    for expected in expected_exports {
        assert!(
            module_body.contains(expected),
            "facade module {module_name} does not export {expected}"
        );
    }
}

fn assert_no_json_or_serde_export(module_name: &str, module_body: &str) {
    for line in module_body.lines() {
        assert!(
            !contains_forbidden_public_serde_surface(line) && !line.contains("json!"),
            "facade module {module_name} exposes JSON/serde residue: {line}"
        );
    }
}

fn workspace_file(relative: &str) -> String {
    fs::read_to_string(repository_source(relative)).unwrap()
}
