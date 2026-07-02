use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use syn::{File, Item, UseTree, Visibility};

pub(crate) fn collect_public_names(path: &Path) -> BTreeSet<String> {
    let parsed = parse_rust_file(path);
    let mut names = BTreeSet::new();

    for item in parsed.items {
        match item {
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                collect_public_use_names(&item_use.tree, &mut names);
            }
            Item::Struct(item_struct) if matches!(item_struct.vis, Visibility::Public(_)) => {
                names.insert(item_struct.ident.to_string());
            }
            Item::Enum(item_enum) if matches!(item_enum.vis, Visibility::Public(_)) => {
                names.insert(item_enum.ident.to_string());
            }
            Item::Fn(item_fn) if matches!(item_fn.vis, Visibility::Public(_)) => {
                names.insert(item_fn.sig.ident.to_string());
            }
            Item::Const(item_const) if matches!(item_const.vis, Visibility::Public(_)) => {
                names.insert(item_const.ident.to_string());
            }
            _ => {}
        }
    }

    names
}

pub(crate) fn collect_query_lane_public_surface_names(entrypoint: &Path) -> Vec<(String, PathBuf)> {
    let mut visited = BTreeSet::new();
    let mut output = Vec::new();
    collect_query_lane_public_surface_names_from_file(entrypoint, &mut visited, &mut output);
    output.sort();
    output.dedup();
    output
}

fn collect_query_lane_public_surface_names_from_file(
    path: &Path,
    visited: &mut BTreeSet<PathBuf>,
    output: &mut Vec<(String, PathBuf)>,
) {
    let canonical = path.to_path_buf();
    if !visited.insert(canonical.clone()) {
        return;
    }

    let parsed = parse_rust_file(path);
    output.extend(
        collect_public_names(path)
            .into_iter()
            .map(|name| (name, canonical.clone())),
    );

    let module_names = collect_query_lane_surface_module_names(&parsed);
    let module_dir = if path.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
        path.parent()
            .expect("mod.rs should have a parent")
            .to_path_buf()
    } else {
        path.with_extension("")
    };

    for module_name in module_names {
        if let Some(module_path) = resolve_module_file(&module_dir, &module_name) {
            collect_query_lane_public_surface_names_from_file(&module_path, visited, output);
        }
    }
}

fn collect_query_lane_surface_module_names(parsed: &File) -> BTreeSet<String> {
    let mut modules = BTreeSet::new();

    for item in &parsed.items {
        match item {
            Item::Mod(item_mod) if matches!(item_mod.vis, Visibility::Public(_)) => {
                modules.insert(item_mod.ident.to_string());
            }
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                collect_glob_surface_modules(&item_use.tree, &mut modules);
            }
            _ => {}
        }
    }

    modules
}

fn collect_public_use_names(tree: &UseTree, output: &mut BTreeSet<String>) {
    match tree {
        UseTree::Name(name) => {
            output.insert(name.ident.to_string());
        }
        UseTree::Rename(rename) => {
            output.insert(rename.rename.to_string());
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_public_use_names(item, output);
            }
        }
        UseTree::Path(path) => collect_public_use_names(&path.tree, output),
        UseTree::Glob(_) => {}
    }
}

fn collect_glob_surface_modules(tree: &UseTree, output: &mut BTreeSet<String>) {
    match tree {
        UseTree::Path(path) => {
            if matches!(&*path.tree, UseTree::Glob(_)) {
                output.insert(path.ident.to_string());
            } else {
                collect_glob_surface_modules(&path.tree, output);
            }
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_glob_surface_modules(item, output);
            }
        }
        _ => {}
    }
}

fn resolve_module_file(module_dir: &Path, module_name: &str) -> Option<PathBuf> {
    let file_path = module_dir.join(format!("{module_name}.rs"));
    if file_path.exists() {
        return Some(file_path);
    }

    let mod_path = module_dir.join(module_name).join("mod.rs");
    mod_path.exists().then_some(mod_path)
}

fn parse_rust_file(path: &Path) -> File {
    let text = fs::read_to_string(path).expect("source file should decode");
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}
