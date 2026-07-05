use std::fs;
use std::path::Path;

use super::dependency_audit::{collect_file_paths, collect_file_use_paths, path_starts_with};
use syn::visit::{self, Visit};
use syn::{ExprMethodCall, ExprPath, File, ImplItem, Item};

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
const PHASE5_DECLARATION_SOURCE_REOPENING_METHODS: &[&str] = &[
    "semantic_artifact",
    "published_aspects",
    "consumed_aspects",
    "structural_tokens",
    "posture_tokens",
    "support_tokens",
];
const PHASE6_DECLARATION_SOURCE_REOPENING_METHODS: &[&str] = &[
    "semantic_artifact",
    "structural_tokens",
    "posture_tokens",
    "support_tokens",
];

pub fn audit_phase5_graph_lookup_lane_does_not_reopen_declaration_source(
    workspace_root: &Path,
) -> Vec<String> {
    let files = [
        "crates/worth-ui-runtime/src/facade/app.rs",
        "crates/worth-ui-runtime/src/facade/obligation_inspection.rs",
        "crates/worth-ui-runtime/src/graph/inspection/graph_lookup_boundary.rs",
        "crates/worth-ui-runtime/src/graph/inspection/graph_node_evidence_index.rs",
    ]
    .iter()
    .map(|relative| workspace_root.join(relative))
    .collect::<Vec<_>>();
    let mut violations = Vec::new();

    for path in files {
        for segments in collect_file_paths(&path)
            .into_iter()
            .chain(collect_file_use_paths(&path))
        {
            if let Some(authority_name) = declaration_semantic_authority_path(&segments) {
                violations.push(format!(
                    "{} reopens declaration meaning by reaching DSL semantic authority type `{authority_name}` inside the phase-5 graph lookup lane",
                    path.display()
                ));
            }
        }

        for method_name in collect_method_names(&path) {
            if PHASE5_DECLARATION_SOURCE_REOPENING_METHODS.contains(&method_name.as_str()) {
                violations.push(format!(
                    "{} reopens declaration meaning through DSL semantic accessor `{method_name}()` inside the phase-5 graph lookup lane",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_phase5_graph_lookup_lane_is_indexed_not_scan_first(
    workspace_root: &Path,
) -> Vec<String> {
    let app = workspace_root.join("crates/worth-ui-runtime/src/facade/app.rs");
    let app_inspection_support =
        workspace_root.join("crates/worth-ui-runtime/src/facade/app_inspection_support.rs");
    let graph_lookup_boundary = workspace_root
        .join("crates/worth-ui-runtime/src/graph/inspection/graph_lookup_boundary.rs");
    let graph_node_evidence_index = workspace_root
        .join("crates/worth-ui-runtime/src/graph/inspection/graph_node_evidence_index.rs");
    let obligation_inspection =
        workspace_root.join("crates/worth-ui-runtime/src/facade/obligation_inspection.rs");
    let app_inspection_support_source =
        fs::read_to_string(&app_inspection_support).expect("source should decode");
    let boundary_source = fs::read_to_string(&graph_lookup_boundary).expect("source should decode");
    let obligation_source =
        fs::read_to_string(&obligation_inspection).expect("source should decode");
    let mut violations = Vec::new();

    if !boundary_source.contains("lookup_graph_node_identity") {
        violations.push(format!(
            "{} no longer routes graph-node inspection through the graph identity evidence index",
            graph_lookup_boundary.display()
        ));
    }

    let inspect_method_names = collect_method_names_for_function(&app, "inspect");
    if inspect_method_names
        .iter()
        .any(|name| name == "rebuild_graph_node_evidence_index_from_authority")
    {
        violations.push(format!(
            "{} appears to rebuild graph-node evidence during the ordinary inspection lane instead of consuming retained derived state",
            app.display()
        ));
    }
    if collect_paths_for_function(&app, "inspect")
        .iter()
        .any(|segments| ends_with_path(segments, &["UiGraphNodeEvidenceIndex", "rebuild"]))
    {
        violations.push(format!(
            "{} appears to call `UiGraphNodeEvidenceIndex::rebuild(...)` directly inside the ordinary inspection lane instead of consuming retained derived state",
            app.display()
        ));
    }
    for (path, source) in [
        (&app_inspection_support, &app_inspection_support_source),
        (&graph_lookup_boundary, &boundary_source),
    ] {
        if source.contains("UiGraphNodeEvidenceIndex::rebuild(") {
            violations.push(format!(
                "{} appears to rebuild graph-node evidence during the ordinary inspection lane instead of consuming retained derived state",
                path.display()
            ));
        }
    }

    let method_names =
        collect_method_names_for_function(&graph_node_evidence_index, "lookup_graph_node_identity");
    if !method_names.iter().any(|name| name == "get") {
        violations.push(format!(
            "{} no longer proves indexed graph lookup because `lookup_graph_node_identity` does not call map lookup `get()`",
            graph_node_evidence_index.display()
        ));
    }
    for forbidden in ["iter", "find", "position", "collect"] {
        if method_names.iter().any(|name| name == forbidden) {
            violations.push(format!(
                "{} appears to scan during ordinary graph lookup because `lookup_graph_node_identity` calls `{forbidden}()`",
                graph_node_evidence_index.display()
            ));
        }
    }

    if obligation_source.contains("declaration_artifacts().iter().find") {
        violations.push(format!(
            "{} reintroduced declaration-artifact scanning into graph-keyed obligation touch recovery",
            obligation_inspection.display()
        ));
    }
    if obligation_source.contains("UiInspectionTarget::GraphNodeIdentity") {
        violations.push(format!(
            "{} still accepts GraphNodeIdentity in the retained obligation helper instead of keeping ordinary graph-node lookup on the graph evidence index lane",
            obligation_inspection.display()
        ));
    }

    let graph_index_source =
        fs::read_to_string(&graph_node_evidence_index).expect("source should decode");
    if !graph_index_source.contains("evidence_index()")
        || !graph_index_source.contains("record.graph_node_digest()")
    {
        violations.push(format!(
            "{} no longer proves graph-node obligation refs are rebuilt into the graph-local neighborhood from authority-backed obligation evidence records",
            graph_node_evidence_index.display()
        ));
    }
    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_phase6_aspect_lookup_lane_does_not_reopen_declaration_source(
    workspace_root: &Path,
) -> Vec<String> {
    let files = [
        "crates/worth-ui-runtime/src/facade/app.rs",
        "crates/worth-ui-runtime/src/facade/app_inspection_support.rs",
        "crates/worth-ui-runtime/src/graph/inspection/aspect/aspect_lookup_boundary.rs",
        "crates/worth-ui-runtime/src/graph/inspection/aspect/published_aspect_evidence_index.rs",
        "crates/worth-ui-runtime/src/graph/inspection/aspect/consumed_aspect_evidence_index.rs",
    ]
    .iter()
    .map(|relative| workspace_root.join(relative))
    .collect::<Vec<_>>();
    let mut violations = Vec::new();

    for path in files {
        for segments in collect_file_paths(&path)
            .into_iter()
            .chain(collect_file_use_paths(&path))
        {
            if let Some(authority_name) = declaration_semantic_authority_path(&segments) {
                violations.push(format!(
                    "{} reopens declaration meaning by reaching DSL semantic authority type `{authority_name}` inside the phase-6 aspect lookup lane",
                    path.display()
                ));
            }
        }

        for method_name in collect_method_names(&path) {
            if PHASE6_DECLARATION_SOURCE_REOPENING_METHODS.contains(&method_name.as_str()) {
                violations.push(format!(
                    "{} reopens declaration meaning through DSL semantic accessor `{method_name}()` inside the phase-6 aspect lookup lane",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_phase6_aspect_lookup_lane_is_indexed_not_scan_first(
    workspace_root: &Path,
) -> Vec<String> {
    let app = workspace_root.join("crates/worth-ui-runtime/src/facade/app.rs");
    let aspect_lookup_boundary = workspace_root
        .join("crates/worth-ui-runtime/src/graph/inspection/aspect/aspect_lookup_boundary.rs");
    let published_aspect_evidence_index = workspace_root.join(
        "crates/worth-ui-runtime/src/graph/inspection/aspect/published_aspect_evidence_index.rs",
    );
    let consumed_aspect_evidence_index = workspace_root.join(
        "crates/worth-ui-runtime/src/graph/inspection/aspect/consumed_aspect_evidence_index.rs",
    );
    let boundary_source =
        fs::read_to_string(&aspect_lookup_boundary).expect("source should decode");
    let mut violations = Vec::new();

    for required_lookup in ["lookup_published_aspect", "lookup_consumed_aspect"] {
        if !boundary_source.contains(required_lookup) {
            violations.push(format!(
                "{} no longer routes aspect inspection through `{required_lookup}` on the retained aspect evidence indexes",
                aspect_lookup_boundary.display()
            ));
        }
    }

    if collect_method_names_for_function(&app, "inspect")
        .iter()
        .any(|name| name == "build_graph_aspect_evidence_indexes")
    {
        violations.push(format!(
            "{} appears to rebuild aspect evidence during the ordinary inspection lane instead of consuming retained derived state",
            app.display()
        ));
    }
    if collect_paths_for_function(&app, "inspect")
        .iter()
        .any(|segments| ends_with_path(segments, &["UiGraphAspectEvidenceIndexes", "rebuild"]))
    {
        violations.push(format!(
            "{} appears to call `UiGraphAspectEvidenceIndexes::rebuild(...)` directly inside the ordinary aspect inspection lane instead of consuming retained derived state",
            app.display()
        ));
    }

    for path in [
        &published_aspect_evidence_index,
        &consumed_aspect_evidence_index,
    ] {
        let method_names = collect_method_names_for_function(path, "lookup");
        if !method_names.iter().any(|name| name == "get") {
            violations.push(format!(
                "{} no longer proves indexed aspect lookup because `lookup` does not call map lookup `get()`",
                path.display()
            ));
        }
        for forbidden in ["iter", "find", "position", "collect"] {
            if method_names.iter().any(|name| name == forbidden) {
                violations.push(format!(
                    "{} appears to scan during ordinary aspect lookup because `lookup` calls `{forbidden}()`",
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

fn collect_method_names(path: &Path) -> Vec<String> {
    let parsed = parse_rust_file(path);
    let mut collector = MethodCallCollector::default();
    collector.visit_file(&parsed);
    collector.method_names
}

fn collect_method_names_for_function(path: &Path, function_name: &str) -> Vec<String> {
    let parsed = parse_rust_file(path);
    let mut collector = MethodCallCollector::default();

    for item in parsed.items {
        match item {
            Item::Fn(item_fn) if item_fn.sig.ident == function_name => {
                collector.visit_block(&item_fn.block);
            }
            Item::Impl(item_impl) => {
                for impl_item in item_impl.items {
                    if let ImplItem::Fn(function) = impl_item {
                        if function.sig.ident == function_name {
                            collector.visit_block(&function.block);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    collector.method_names
}

fn collect_paths_for_function(path: &Path, function_name: &str) -> Vec<Vec<String>> {
    let parsed = parse_rust_file(path);
    let mut collector = PathCollector::default();

    for item in parsed.items {
        match item {
            Item::Fn(item_fn) if item_fn.sig.ident == function_name => {
                collector.visit_block(&item_fn.block);
            }
            Item::Impl(item_impl) => {
                for impl_item in item_impl.items {
                    if let ImplItem::Fn(function) = impl_item {
                        if function.sig.ident == function_name {
                            collector.visit_block(&function.block);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    collector.paths
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

#[derive(Default)]
struct PathCollector {
    paths: Vec<Vec<String>>,
}

impl Visit<'_> for PathCollector {
    fn visit_expr_path(&mut self, expr_path: &ExprPath) {
        self.paths.push(
            expr_path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        );
        visit::visit_expr_path(self, expr_path);
    }
}

fn ends_with_path(segments: &[String], suffix: &[&str]) -> bool {
    segments.len() >= suffix.len()
        && segments[segments.len() - suffix.len()..]
            .iter()
            .map(String::as_str)
            .eq(suffix.iter().copied())
}
