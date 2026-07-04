use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use super::dependency_audit::{collect_rust_files, path_starts_with};
use syn::visit::{self, Visit};
use syn::{ExprMethodCall, File, ImplItem, Item, ItemExternCrate, ItemUse, UseTree};

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

        if should_skip_non_owner_audit_file(&relative_text) {
            continue;
        }

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

        let source = production_source_text(&path);
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
    workspace_root: &Path,
) -> Vec<String> {
    let scoped_files = [
        "crates/worth-ui-runtime/src/facade/app.rs",
        "crates/worth-ui-runtime/src/facade/inspection/authored_lookup_boundary.rs",
        "crates/worth-ui-runtime/src/declaration/inspection/index/authored_evidence_index.rs",
    ];
    let files = scoped_files
        .iter()
        .map(|relative| workspace_root.join(relative))
        .collect::<Vec<_>>();

    audit_files_do_not_reopen_declaration_source(workspace_root, &files)
}

pub fn audit_phase4_authored_lookup_lane_is_indexed_not_scan_first(
    workspace_root: &Path,
) -> Vec<String> {
    let authored_lookup_boundary = workspace_root
        .join("crates/worth-ui-runtime/src/facade/inspection/authored_lookup_boundary.rs");
    let authored_evidence_index = workspace_root.join(
        "crates/worth-ui-runtime/src/declaration/inspection/index/authored_evidence_index.rs",
    );
    let boundary_source =
        fs::read_to_string(&authored_lookup_boundary).expect("source file should decode");
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
        let method_names = collect_method_names_for_function(&authored_evidence_index, function_name);
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

fn parse_rust_file(path: &Path) -> File {
    let text = source_without_test_module_tail(path);
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}

fn production_source_text(path: &Path) -> String {
    let text = fs::read_to_string(path).expect("source file should decode");
    if let Some(cfg_test_start) = text.find("#[cfg(test)]") {
        text[..cfg_test_start].to_string()
    } else {
        text
    }
}

fn source_without_test_module_tail(path: &Path) -> String {
    let text = fs::read_to_string(path).expect("source file should decode");
    if let Some(test_module_start) = text.find("#[cfg(test)]\nmod tests")
        .or_else(|| text.find("#[cfg(test)]\r\nmod tests"))
    {
        text[..test_module_start].to_string()
    } else {
        text
    }
}

fn should_skip_non_owner_audit_file(relative_text: &str) -> bool {
    relative_text.contains("_test_support.rs")
        || relative_text.contains("_certification_support.rs")
}

#[derive(Default)]
struct AliasCollector {
    use_aliases: HashMap<String, Vec<String>>,
}

impl Visit<'_> for AliasCollector {
    fn visit_item_extern_crate(&mut self, item_extern_crate: &ItemExternCrate) {
        let alias = item_extern_crate
            .rename
            .as_ref()
            .map(|(_, ident)| ident)
            .unwrap_or(&item_extern_crate.ident)
            .to_string();
        self.use_aliases
            .insert(alias, vec![item_extern_crate.ident.to_string()]);
        visit::visit_item_extern_crate(self, item_extern_crate);
    }

    fn visit_item_use(&mut self, item_use: &ItemUse) {
        collect_use_aliases(&item_use.tree, &mut Vec::new(), &mut self.use_aliases);
        visit::visit_item_use(self, item_use);
    }
}

fn collect_use_aliases(
    tree: &UseTree,
    prefix: &mut Vec<String>,
    aliases: &mut HashMap<String, Vec<String>>,
) {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_aliases(&path.tree, prefix, aliases);
            prefix.pop();
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_aliases(item, prefix, aliases);
            }
        }
        UseTree::Name(name) => {
            if !prefix.is_empty() {
                let mut full_path = prefix.clone();
                full_path.push(name.ident.to_string());
                aliases.insert(name.ident.to_string(), full_path);
            }
        }
        UseTree::Rename(rename) => {
            let mut full_path = prefix.clone();
            full_path.push(rename.ident.to_string());
            aliases.insert(rename.rename.to_string(), full_path);
        }
        _ => {}
    }
}

struct PathCollector<'a> {
    use_aliases: &'a HashMap<String, Vec<String>>,
    collected_paths: Vec<Vec<String>>,
}

impl Visit<'_> for PathCollector<'_> {
    fn visit_path(&mut self, path: &syn::Path) {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();
        self.collected_paths
            .push(expand_use_alias_path(segments, self.use_aliases));
        visit::visit_path(self, path);
    }
}

fn expand_use_alias_path(
    mut segments: Vec<String>,
    use_aliases: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut expanded_aliases = HashSet::new();

    loop {
        let Some(first) = segments.first().cloned() else {
            return segments;
        };
        let Some(alias_path) = use_aliases.get(&first) else {
            return segments;
        };
        if !expanded_aliases.insert(first) {
            return segments;
        }

        let mut expanded = alias_path.clone();
        expanded.extend(segments.into_iter().skip(1));
        segments = expanded;
    }
}

fn collect_file_paths(path: &Path) -> Vec<Vec<String>> {
    let parsed = parse_rust_file(path);
    let mut alias_collector = AliasCollector::default();
    alias_collector.visit_file(&parsed);

    let mut path_collector = PathCollector {
        use_aliases: &alias_collector.use_aliases,
        collected_paths: Vec::new(),
    };
    path_collector.visit_file(&parsed);
    path_collector.collected_paths
}

#[derive(Default)]
struct UsePathCollector {
    collected_paths: Vec<Vec<String>>,
}

impl Visit<'_> for UsePathCollector {
    fn visit_item_use(&mut self, item_use: &ItemUse) {
        collect_use_paths(&item_use.tree, Vec::new(), &mut self.collected_paths);
        visit::visit_item_use(self, item_use);
    }
}

fn collect_file_use_paths(path: &Path) -> Vec<Vec<String>> {
    let parsed = parse_rust_file(path);
    let mut collector = UsePathCollector::default();
    collector.visit_file(&parsed);
    collector.collected_paths
}

fn collect_use_paths(tree: &UseTree, prefix: Vec<String>, output: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix;
            next.push(path.ident.to_string());
            collect_use_paths(&path.tree, next, output);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_paths(item, prefix.clone(), output);
            }
        }
        UseTree::Name(name) => {
            let mut next = prefix;
            next.push(name.ident.to_string());
            output.push(next);
        }
        UseTree::Rename(rename) => {
            let mut next = prefix;
            next.push(rename.ident.to_string());
            output.push(next);
        }
        UseTree::Glob(_) => output.push(prefix),
    }
}

fn audit_files_do_not_reopen_declaration_source(
    workspace_root: &Path,
    files: &[std::path::PathBuf],
) -> Vec<String> {
    let mut violations = Vec::new();

    for path in files {
        let relative = path
            .strip_prefix(workspace_root)
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

        for segments in collect_file_paths(path)
            .into_iter()
            .chain(collect_file_use_paths(path))
        {
            if let Some(authority_name) = declaration_semantic_authority_path(&segments) {
                violations.push(format!(
                    "{} reopens declaration meaning by reaching DSL semantic authority type `{authority_name}` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }

        for method_name in collect_method_names(path) {
            if DECLARATION_SOURCE_REOPENING_METHODS.contains(&method_name.as_str()) {
                violations.push(format!(
                    "{} reopens declaration meaning through DSL semantic accessor `{method_name}()` outside the owning declaration lowering/admission lanes",
                    path.display()
                ));
            }
        }

        let source = production_source_text(path);
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
