use std::fs;
use std::path::Path;

use super::dependency_audit::{
    collect_file_paths, collect_file_use_paths, collect_rust_files, path_starts_with,
};
use syn::visit::{self, Visit};
use syn::{ExprMethodCall, File};

const DECLARATION_SOURCE_REOPENING_ALLOWED_FILES: &[&str] = &[
    "crates/worth-ui-runtime/src/declaration/artifact/ui_declaration_lowering.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/aspect_contract.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/aspect_name.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/consumed.rs",
    "crates/worth-ui-runtime/src/declaration/aspect_contract/published.rs",
    "crates/worth-ui-runtime/src/declaration/declared_posture/admission.rs",
    "crates/worth-ui-runtime/src/declaration/family/admission.rs",
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
    "published_aspects",
    "consumed_aspects",
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
    workspace_root: &Path,
) -> Vec<String> {
    let scoped_roots = [
        "crates/worth-ui/src",
        "crates/worth-ui-runtime/src",
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
    ];
    let mut violations = Vec::new();
    let mut files = Vec::new();

    for scoped_root in scoped_roots {
        collect_rust_files(&workspace_root.join(scoped_root), &mut files);
    }

    for path in files {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if file_name == "tests.rs" || file_name.ends_with("_tests.rs") {
            continue;
        }

        let relative = path
            .strip_prefix(workspace_root)
            .expect("workspace file should strip to relative path");
        let relative_text = relative.to_string_lossy().replace('\\', "/");

        if DECLARATION_SOURCE_REOPENING_ALLOWED_FILES
            .iter()
            .any(|allowed| *allowed == relative_text)
        {
            continue;
        }

        for segments in collect_file_paths(&path)
            .into_iter()
            .chain(collect_file_use_paths(&path))
        {
            if let Some(authority_name) = declaration_semantic_authority_path(&segments) {
                violations.push(format!(
                    "{} reopens declaration meaning by reaching DSL semantic authority type `{authority_name}` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }

        for method_name in collect_method_names(&path) {
            if DECLARATION_SOURCE_REOPENING_METHODS.contains(&method_name.as_str()) {
                violations.push(format!(
                    "{} reopens declaration meaning through DSL semantic accessor `{method_name}()` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }

        let source = fs::read_to_string(&path).expect("source file should decode");
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

pub fn audit_host_and_inspection_layers_do_not_import_declaration_authority(
    workspace_root: &Path,
) -> Vec<String> {
    let scoped_roots = [
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
    ];
    let mut violations = Vec::new();
    let mut files = Vec::new();

    for scoped_root in scoped_roots {
        collect_rust_files(&workspace_root.join(scoped_root), &mut files);
    }

    for path in files {
        for segments in collect_file_paths(&path) {
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

fn starts_with_declaration_surface(segments: &[String]) -> bool {
    (path_starts_with(segments, "worth_ui")
        && segments.get(1).is_some_and(|segment| segment == "facade")
        && segments
            .get(2)
            .is_some_and(|segment| segment == "declaration"))
        || (path_starts_with(segments, "worth_ui_runtime")
            && ((segments
                .get(1)
                .is_some_and(|segment| segment == "declaration"))
                || (segments.get(1).is_some_and(|segment| segment == "facade")
                    && segments
                        .get(2)
                        .is_some_and(|segment| segment == "declaration"))))
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

fn collect_method_names(path: &Path) -> Vec<String> {
    let parsed = parse_rust_file(path);
    let mut collector = MethodCallCollector::default();
    collector.visit_file(&parsed);
    collector.method_names
}

fn parse_rust_file(path: &Path) -> File {
    let text = fs::read_to_string(path).expect("source file should decode");
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
