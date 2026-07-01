use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::{self, Visit};
use syn::{File, ItemUse, UseTree};

pub(crate) fn collect_rust_files(root: &Path, output: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("read_dir should succeed") {
        let entry = entry.expect("dir entry should load");
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, output);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            output.push(path);
        }
    }
}

fn parse_rust_file(path: &Path) -> File {
    let text = fs::read_to_string(path).expect("source file should decode");
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}

#[derive(Default)]
struct AliasCollector {
    crate_aliases: HashMap<String, String>,
}

impl Visit<'_> for AliasCollector {
    fn visit_item_use(&mut self, item_use: &ItemUse) {
        collect_use_aliases(&item_use.tree, &mut Vec::new(), &mut self.crate_aliases);
        visit::visit_item_use(self, item_use);
    }
}

fn collect_use_aliases(
    tree: &UseTree,
    prefix: &mut Vec<String>,
    aliases: &mut HashMap<String, String>,
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
        UseTree::Rename(rename) => {
            if prefix.len() == 1 {
                aliases.insert(rename.rename.to_string(), prefix[0].clone());
            }
        }
        _ => {}
    }
}

struct PathCollector<'a> {
    crate_aliases: &'a HashMap<String, String>,
    collected_paths: Vec<Vec<String>>,
}

impl<'a> Visit<'_> for PathCollector<'a> {
    fn visit_path(&mut self, path: &syn::Path) {
        let mut segments = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>();
        if let Some(first) = segments.first_mut() {
            if let Some(crate_name) = self.crate_aliases.get(first) {
                *first = crate_name.clone();
            }
        }
        self.collected_paths.push(segments);
        visit::visit_path(self, path);
    }
}

pub(crate) fn collect_file_paths(path: &Path) -> Vec<Vec<String>> {
    let parsed = parse_rust_file(path);
    let mut alias_collector = AliasCollector::default();
    alias_collector.visit_file(&parsed);

    let mut path_collector = PathCollector {
        crate_aliases: &alias_collector.crate_aliases,
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

pub(crate) fn collect_file_use_paths(path: &Path) -> Vec<Vec<String>> {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ManifestDependency {
    pub(crate) key: String,
    pub(crate) package: String,
}

pub(crate) fn manifests_dependencies(path: &Path) -> Vec<ManifestDependency> {
    let text = fs::read_to_string(path).expect("manifest should decode");
    let manifest = text
        .parse::<toml::Value>()
        .unwrap_or_else(|error| panic!("{} should parse as TOML: {error}", path.display()));

    ["dependencies", "dev-dependencies", "build-dependencies"]
        .into_iter()
        .filter_map(|section| manifest.get(section))
        .filter_map(toml::Value::as_table)
        .flat_map(|table| {
            table.iter().map(|(key, value)| ManifestDependency {
                key: key.clone(),
                package: value
                    .as_table()
                    .and_then(|entry| entry.get("package"))
                    .and_then(toml::Value::as_str)
                    .unwrap_or(key)
                    .to_string(),
            })
        })
        .collect()
}

pub(crate) fn manifest_dependency_crate_aliases(path: &Path) -> HashMap<String, String> {
    manifests_dependencies(path)
        .into_iter()
        .map(|dependency| {
            (
                dependency.key.replace('-', "_"),
                dependency.package.replace('-', "_"),
            )
        })
        .collect()
}

fn path_matches(segments: &[String], crate_name: &str, internal_root: &str) -> bool {
    segments.len() >= 2 && segments[0] == crate_name && segments[1] == internal_root
}

pub(crate) fn path_starts_with(segments: &[String], crate_name: &str) -> bool {
    segments
        .first()
        .is_some_and(|segment| segment == crate_name)
}

pub fn audit_no_cross_crate_deep_imports(workspace_root: &Path) -> Vec<String> {
    let crate_paths = [
        "crates/worth-ui/src",
        "crates/worth-ui-dsl/src",
        "crates/worth-ui-runtime/src",
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-query-binding/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
        "crates/worth-ui-certification/src",
    ];
    let forbidden_boundaries = [
        ("worth_ui_runtime", "lifecycle"),
        ("worth_ui_runtime", "source"),
        ("worth_ui_runtime", "host"),
        ("worth_ui_inspection", "facade"),
        ("worth_ui_inspection", "query"),
        ("worth_ui_inspection", "target"),
        ("worth_ui_inspection", "scope"),
        ("worth_ui_inspection", "receipt"),
        ("worth_ui_inspection", "posture"),
        ("worth_ui_dsl", "package"),
        ("worth_ui_dsl", "support"),
        ("worth_ui_host_contract", "runtime"),
        ("worth_ui_host_contract", "inspection"),
        ("worth_ui_query_binding", "facade"),
    ];
    let mut violations = Vec::new();
    let mut files = Vec::new();

    for crate_path in crate_paths {
        collect_rust_files(&workspace_root.join(crate_path), &mut files);
    }

    for file in files {
        for segments in collect_file_paths(&file)
            .into_iter()
            .chain(collect_file_use_paths(&file))
        {
            for (crate_name, internal_root) in forbidden_boundaries {
                if path_matches(&segments, crate_name, internal_root) {
                    violations.push(format!(
                        "{} deep-imports `{crate_name}::{internal_root}` through structured Rust paths",
                        file.display()
                    ));
                }
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_non_product_crates_route_declaration_through_worth_ui_facade(
    workspace_root: &Path,
) -> Vec<String> {
    let crate_paths = [
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
        "crates/worth-ui-query-binding/src",
    ];
    let mut violations = Vec::new();
    let mut files = Vec::new();

    for crate_path in crate_paths {
        collect_rust_files(&workspace_root.join(crate_path), &mut files);
    }

    for file in files {
        for segments in collect_file_paths(&file)
            .into_iter()
            .chain(collect_file_use_paths(&file))
        {
            if path_matches(&segments, "worth_ui_runtime", "facade")
                && segments
                    .get(2)
                    .is_some_and(|segment| segment == "declaration")
            {
                violations.push(format!(
                    "{} bypasses the product declaration facade and reaches `worth_ui_runtime::facade::declaration`; declaration consumers must enter through `worth_ui::facade::declaration`",
                    file.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_host_egui_dependency_boundary(workspace_root: &Path) -> Vec<String> {
    let mut violations = Vec::new();
    let cargo_toml = workspace_root.join("crates/worth-ui-host-egui/Cargo.toml");
    let dependencies = manifests_dependencies(&cargo_toml);

    for forbidden_dep in ["worth-ui", "worth-ui-runtime", "worth-ui-inspection"] {
        if dependencies
            .iter()
            .any(|dependency| dependency.package == forbidden_dep)
        {
            violations.push(format!(
                "worth-ui-host-egui manifest must not depend on `{forbidden_dep}`"
            ));
        }
    }

    let mut rust_files = Vec::new();
    collect_rust_files(
        &workspace_root.join("crates/worth-ui-host-egui/src"),
        &mut rust_files,
    );
    let manifest_aliases = manifest_dependency_crate_aliases(&cargo_toml);

    for file in rust_files {
        for segments in collect_file_paths(&file) {
            let normalized_segments = normalize_manifest_alias_path(&segments, &manifest_aliases);
            if path_matches(&normalized_segments, "worth_ui_runtime", "lifecycle")
                || path_matches(&normalized_segments, "worth_ui_runtime", "source")
                || path_matches(&normalized_segments, "worth_ui_runtime", "host")
            {
                violations.push(format!(
                    "{} reaches worth-ui-runtime internals through structured Rust paths",
                    file.display()
                ));
            }
            if ["facade", "query", "target", "scope", "receipt", "posture"]
                .into_iter()
                .any(|module| path_matches(&normalized_segments, "worth_ui_inspection", module))
            {
                violations.push(format!(
                    "{} reaches worth-ui-inspection internals through structured Rust paths",
                    file.display()
                ));
            }
            if path_matches(&normalized_segments, "worth_ui", "runtime") {
                violations.push(format!(
                    "{} reaches the worth-ui shadow runtime module through structured Rust paths",
                    file.display()
                ));
            }
            if path_starts_with(&normalized_segments, "worth_ui") {
                violations.push(format!(
                    "{} reaches the worth-ui product facade; host adapters must stay on host-contract-only surfaces",
                    file.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub(crate) fn normalize_manifest_alias_path(
    segments: &[String],
    manifest_aliases: &HashMap<String, String>,
) -> Vec<String> {
    let mut normalized = segments.to_vec();
    if let Some(first) = normalized.first_mut() {
        if let Some(package_name) = manifest_aliases.get(first) {
            *first = package_name.clone();
        }
    }
    normalized
}
