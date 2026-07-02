use std::path::Path;

use super::dependency_audit::{collect_file_paths, collect_file_use_paths, collect_rust_files};
use syn::visit::{self, Visit};
use syn::{ExprMethodCall, File};

const OBLIGATION_DECLARATION_REOPENING_ROOTS: &[&str] = &[
    "crates/worth-ui/src",
    "crates/worth-ui-inspection/src",
    "crates/worth-ui-runtime/src/obligations",
];
const OBLIGATION_DECLARATION_AUTHORITY_TYPES: &[&str] = &[
    "UiDslAspectName",
    "UiDslLoweringReceipt",
    "UiDslSemanticArtifact",
    "UiDslSemanticFamily",
    "UiDslSemanticKey",
    "UiDslStructuralToken",
    "UiDslPostureToken",
    "UiDslSupportToken",
];
const OBLIGATION_DECLARATION_ACCESSORS: &[&str] = &[
    "semantic_artifact",
    "published_aspects",
    "consumed_aspects",
    "structural_tokens",
    "posture_tokens",
    "support_tokens",
];
const LEGALITY_OWNER_FILES: &[&str] = &[
    "crates/worth-ui-runtime/src/admission/boundary/ui_admission_boundary.rs",
    "crates/worth-ui-runtime/src/admission/legality/ui_legality_decision.rs",
    "crates/worth-ui-runtime/src/admission/report/evidence_index_builders.rs",
    "crates/worth-ui-runtime/src/admission/report/ui_admission_report.rs",
];
const LEGALITY_RESOLUTION_ROOTS: &[&str] = &[
    "crates/worth-ui-runtime/src",
    "crates/worth-ui/src",
    "crates/worth-ui-inspection/src",
];

pub fn audit_non_owner_code_does_not_reopen_obligation_declaration_source(
    workspace_root: &Path,
) -> Vec<String> {
    let files = collect_scoped_rust_files(workspace_root, OBLIGATION_DECLARATION_REOPENING_ROOTS);
    let mut violations = Vec::new();

    for path in files {
        if is_test_file(&path) {
            continue;
        }

        for segments in collect_file_paths(&path)
            .into_iter()
            .chain(collect_file_use_paths(&path))
        {
            if let Some(type_name) = obligation_declaration_authority_name(&segments) {
                violations.push(format!(
                    "{} reopens declaration semantics inside the obligation boundary via DSL authority type `{type_name}`",
                    path.display()
                ));
            }
        }

        for method_name in collect_method_names(&path) {
            if OBLIGATION_DECLARATION_ACCESSORS.contains(&method_name.as_str()) {
                violations.push(format!(
                    "{} reopens declaration semantics inside the obligation boundary via DSL accessor `{method_name}()`",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_legality_resolution_stays_in_admission_owner_lane(
    workspace_root: &Path,
) -> Vec<String> {
    let files = collect_scoped_rust_files(workspace_root, LEGALITY_RESOLUTION_ROOTS);
    let mut violations = Vec::new();

    for path in files {
        if is_test_file(&path) {
            continue;
        }

        let relative = path
            .strip_prefix(workspace_root)
            .expect("workspace file should strip to relative path")
            .to_string_lossy()
            .replace('\\', "/");
        if LEGALITY_OWNER_FILES
            .iter()
            .any(|allowed| *allowed == relative)
        {
            continue;
        }

        for segments in collect_file_paths(&path)
            .into_iter()
            .chain(collect_file_use_paths(&path))
        {
            if let Some(reason) = legality_resolution_edge(&segments) {
                violations.push(format!(
                    "{} resolves legality outside the admission owner lane via {reason}",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn collect_scoped_rust_files(workspace_root: &Path, roots: &[&str]) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    for scoped_root in roots {
        let path = workspace_root.join(scoped_root);
        if path.exists() {
            collect_rust_files(&path, &mut files);
        }
    }

    files
}

fn is_test_file(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    file_name == "tests.rs" || file_name.ends_with("_tests.rs")
}

fn obligation_declaration_authority_name(segments: &[String]) -> Option<&str> {
    if !segments
        .first()
        .is_some_and(|segment| segment == "worth_ui_dsl")
    {
        return None;
    }

    OBLIGATION_DECLARATION_AUTHORITY_TYPES
        .iter()
        .copied()
        .find(|name| segments.iter().any(|segment| segment == name))
}

fn legality_resolution_edge(segments: &[String]) -> Option<String> {
    let last = segments.last()?;

    if segments.iter().any(|segment| segment == "UiLegalityReason") && last != "UiLegalityReason" {
        return Some(format!(
            "legality reason variant path `{}`",
            segments.join("::")
        ));
    }

    if segments
        .iter()
        .any(|segment| segment == "UiLegalityPosture")
        && last != "UiLegalityPosture"
    {
        return Some(format!(
            "legality posture variant path `{}`",
            segments.join("::")
        ));
    }

    None
}

fn collect_method_names(path: &Path) -> Vec<String> {
    let parsed = parse_rust_file(path);
    let mut collector = MethodCallCollector::default();
    collector.visit_file(&parsed);
    collector.method_names
}

fn parse_rust_file(path: &Path) -> File {
    let text = std::fs::read_to_string(path).expect("source file should decode");
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}

#[derive(Default)]
struct MethodCallCollector {
    method_names: Vec<String>,
}

impl Visit<'_> for MethodCallCollector {
    fn visit_expr_method_call(&mut self, method_call: &ExprMethodCall) {
        self.method_names.push(method_call.method.to_string());
        visit::visit_expr_method_call(self, method_call);
    }
}
