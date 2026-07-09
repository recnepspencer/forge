use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::public_surface_audit::collect_query_lane_public_surface_names;
use syn::visit::{self, Visit};
use syn::{File, Item, ItemUse, UseTree};

fn collect_rust_files(root: &Path, output: &mut Vec<PathBuf>) {
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

fn collect_file_paths(path: &Path) -> Vec<Vec<String>> {
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

fn path_matches(segments: &[String], crate_name: &str, internal_root: &str) -> bool {
    segments.len() >= 2 && segments[0] == crate_name && segments[1] == internal_root
}

pub fn audit_non_dsl_crates_do_not_reach_dsl_internals(workspace_root: &Path) -> Vec<String> {
    let crate_paths = [
        "crates/worth-ui/src",
        "crates/worth-ui-runtime/src",
        "crates/worth-ui-inspection/src",
        "crates/worth-ui-query-binding/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
        "crates/worth-ui-certification/src",
    ];
    let mut violations = Vec::new();
    let mut files = Vec::new();

    for crate_path in crate_paths {
        collect_rust_files(&workspace_root.join(crate_path), &mut files);
    }

    for file in files {
        for segments in collect_file_paths(&file) {
            if path_matches(&segments, "worth_ui_dsl", "package")
                || path_matches(&segments, "worth_ui_dsl", "support")
            {
                violations.push(format!(
                    "{} reaches worth-ui-dsl internals instead of admitted DSL boundary types",
                    file.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_public_surfaces_do_not_recreate_query_owned_lanes(
    workspace_root: &Path,
) -> Vec<String> {
    let entrypoints = [
        "crates/worth-ui/src/facade/mod.rs",
        "crates/worth-ui-runtime/src/facade/mod.rs",
        "crates/worth-ui-inspection/src/lib.rs",
    ];
    let mut violations = Vec::new();

    for relative_path in entrypoints {
        let path = workspace_root.join(relative_path);
        let names = collect_query_lane_public_surface_names(&path);
        for (name, surface_path) in names {
            if public_name_recreates_query_owned_lane(&name) {
                violations.push(format!(
                    "{} publicly exposes `{name}`, which recreates a Query-owned support/async-result/inspection/causal-explanation/projection-fact lane on a Worth-local surface",
                    surface_path.display(),
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_preboundary_receipt_and_posture_files_do_not_lower_to_foundational(
    workspace_root: &Path,
) -> Vec<String> {
    let mut violations = Vec::new();
    let mut guarded_files = Vec::new();

    for guarded_root in [
        "crates/worth-ui-inspection/src/posture",
        "crates/worth-ui-inspection/src/receipt",
    ] {
        collect_rust_files(&workspace_root.join(guarded_root), &mut guarded_files);
    }

    for guarded_root in [
        "crates/worth-ui-runtime/src/lifecycle",
        "crates/worth-ui-runtime/src/facade",
    ] {
        let root = workspace_root.join(guarded_root);
        let mut files = Vec::new();
        collect_rust_files(&root, &mut files);
        guarded_files.extend(files.into_iter().filter(|path| {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            name.contains("support")
                || name.contains("inspection")
                || name.contains("receipt")
                || name.contains("closure")
                || name.contains("posture")
        }));
    }

    for path in guarded_files {
        for segments in collect_file_paths(&path) {
            if segments
                .first()
                .is_some_and(|segment| segment == "worth_foundational")
            {
                violations.push(format!(
                    "{} lowers runtime-local receipt/support/posture truth into worth_foundational before a real boundary",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_required_runtime_lifecycle_aggregates_do_not_cheat_with_default_or_option(
    workspace_root: &Path,
) -> Vec<String> {
    let aggregate_roots = [
        "crates/worth-ui-runtime/src/facade",
        "crates/worth-ui-runtime/src/lifecycle",
        "crates/worth-ui-inspection/src/facade",
    ];
    let mut violations = Vec::new();
    let mut files = Vec::new();

    for aggregate_root in aggregate_roots {
        collect_rust_files(&workspace_root.join(aggregate_root), &mut files);
    }

    for path in files {
        let parsed = parse_rust_file(&path);
        let aggregate_structs = collect_lifecycle_aggregate_struct_names(&parsed);

        for struct_name in aggregate_structs {
            if struct_derives_default(&parsed, struct_name.as_str())
                || struct_has_default_impl(&parsed, struct_name.as_str())
            {
                violations.push(format!(
                    "{} allows `{struct_name}` to default required lifecycle state instead of forcing explicit propagation",
                    path.display()
                ));
            }

            if struct_uses_optional_or_map_fields(&parsed, struct_name.as_str()) {
                violations.push(format!(
                    "{} allows `{struct_name}` to weaken required lifecycle state with Option/map storage",
                    path.display()
                ));
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn public_name_recreates_query_owned_lane(name: &str) -> bool {
    let query_lane_fragments = [
        "QuerySupport",
        "QueryStatus",
        "QueryInspection",
        "AsyncResult",
        "CausalExplanation",
        "ProjectionFact",
    ];
    let worth_or_ui_lane_fragments = ["AsyncResult", "CausalExplanation", "ProjectionFact"];

    (name.starts_with("WorthUiQuery")
        && query_lane_fragments
            .iter()
            .any(|fragment| name.contains(fragment)))
        || ((name.starts_with("WorthUi") || name.starts_with("Ui"))
            && worth_or_ui_lane_fragments
                .iter()
                .any(|fragment| name.contains(fragment)))
}

fn collect_lifecycle_aggregate_struct_names(parsed: &File) -> Vec<String> {
    parsed
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(item_struct)
                if is_lifecycle_aggregate_struct_name(&item_struct.ident.to_string()) =>
            {
                Some(item_struct.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

fn is_lifecycle_aggregate_struct_name(name: &str) -> bool {
    name == "WorthUiBuilder"
        || name.contains("Bootstrap")
        || name.contains("FreezeCore")
        || name.contains("SupportInventory")
        || name.contains("ScopeInventory")
}

fn struct_derives_default(parsed: &File, struct_name: &str) -> bool {
    parsed.items.iter().any(|item| match item {
        Item::Struct(item_struct) if item_struct.ident == struct_name => {
            item_struct.attrs.iter().any(|attr| {
                attr.path().is_ident("derive")
                    && matches!(
                        &attr.meta,
                        syn::Meta::List(list) if list.tokens.to_string().contains("Default")
                    )
            })
        }
        _ => false,
    })
}

fn struct_has_default_impl(parsed: &File, struct_name: &str) -> bool {
    parsed.items.iter().any(|item| match item {
        Item::Impl(item_impl) => {
            item_impl.trait_.as_ref().is_some_and(|(_, path, _)| {
                path.segments
                    .last()
                    .is_some_and(|segment| segment.ident == "Default")
            }) && type_is_named(item_impl.self_ty.as_ref(), struct_name)
        }
        _ => false,
    })
}

fn struct_uses_optional_or_map_fields(parsed: &File, struct_name: &str) -> bool {
    parsed.items.iter().any(|item| match item {
        Item::Struct(item_struct) if item_struct.ident == struct_name => item_struct
            .fields
            .iter()
            .any(|field| type_uses_optional_or_map_storage(&field.ty)),
        _ => false,
    })
}

fn type_uses_optional_or_map_storage(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path.path.segments.last().is_some_and(|segment| {
            let ident = segment.ident.to_string();
            ident == "Option" || ident == "HashMap" || ident == "BTreeMap"
        }),
        _ => false,
    }
}

fn type_is_named(ty: &syn::Type, type_name: &str) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == type_name),
        _ => false,
    }
}
