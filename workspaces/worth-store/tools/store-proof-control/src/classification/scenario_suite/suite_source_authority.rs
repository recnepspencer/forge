use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::super::ClassifiedInventory;

pub(super) fn suite_source_fingerprints(
    workspace_root: &Path,
    inventory: &ClassifiedInventory,
    target_identity: &str,
    violations: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let Some(target) = inventory
        .discovered
        .targets
        .iter()
        .find(|target| target.identity == target_identity)
    else {
        return BTreeMap::new();
    };
    let admitted_root = canonical(&workspace_root.join("crates/worth-store-certification/tests"));
    let mut pending = vec![(PathBuf::from(&target.source_path), true)];
    let mut fingerprints = BTreeMap::new();
    while let Some((source_path, is_crate_root)) = pending.pop() {
        let canonical_source = canonical(&source_path);
        if fingerprints.contains_key(&canonical_source) {
            continue;
        }
        if !canonical_source.starts_with(&format!("{admitted_root}/")) {
            violations.push(format!(
                "suite {target_identity} compiles source outside certification test ownership: {canonical_source}"
            ));
            continue;
        }
        let Ok(source) = fs::read_to_string(&source_path) else {
            violations.push(format!(
                "suite {target_identity} cannot read registered source: {canonical_source}"
            ));
            continue;
        };
        fingerprints.insert(
            super::source_identity::repository_relative(workspace_root, &source_path),
            crate::evidence::sha256_bytes(source.as_bytes()),
        );
        pending.extend(declared_module_sources(
            &source_path,
            &source,
            is_crate_root,
        ));
    }
    fingerprints
}

fn declared_module_sources(
    source_path: &Path,
    source: &str,
    is_crate_root: bool,
) -> Vec<(PathBuf, bool)> {
    let parent = source_path.parent().unwrap_or_else(|| Path::new("."));
    let module_parent = module_parent(source_path, parent, is_crate_root);
    let mut declared_path = None;
    let mut paths = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(path) = path_attribute(trimmed) {
            declared_path = Some(path);
            continue;
        }
        let Some(module) = module_declaration(trimmed) else {
            continue;
        };
        if let Some(path) = declared_path.take() {
            paths.push((parent.join(path), true));
        } else if let Some(path) = conventional_module_path(&module_parent, module) {
            paths.push((path, false));
        }
    }
    paths
}

fn module_parent(source_path: &Path, parent: &Path, is_crate_root: bool) -> PathBuf {
    match (
        is_crate_root,
        source_path.file_stem().and_then(|stem| stem.to_str()),
    ) {
        (true, _) => parent.to_path_buf(),
        (_, Some("lib" | "main" | "mod") | None) => parent.to_path_buf(),
        (_, Some(stem)) => parent.join(stem),
    }
}

fn path_attribute(line: &str) -> Option<String> {
    line.strip_prefix("#[path = \"")?
        .strip_suffix("\"]")
        .map(str::to_owned)
}

fn module_declaration(line: &str) -> Option<&str> {
    let declaration = line
        .strip_prefix("pub ")
        .or_else(|| line.strip_prefix("pub(crate) "))
        .unwrap_or(line);
    declaration
        .strip_prefix("mod ")?
        .strip_suffix(';')
        .map(str::trim)
}

fn conventional_module_path(parent: &Path, module: &str) -> Option<PathBuf> {
    let sibling = parent.join(format!("{module}.rs"));
    let child = parent.join(module).join("mod.rs");
    if sibling.exists() {
        Some(sibling)
    } else {
        child.exists().then_some(child)
    }
}

fn canonical(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace("\\\\?\\", "")
        .replace('\\', "/")
}
