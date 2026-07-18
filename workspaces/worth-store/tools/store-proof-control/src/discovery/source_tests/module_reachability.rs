use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::super::cargo_surface::{normalized, TestTargetIdentity};

pub(super) fn reachable_target_files(
    target: &TestTargetIdentity,
    source_texts: &mut std::collections::BTreeMap<String, String>,
) -> Result<BTreeSet<String>, String> {
    let mut discovered = BTreeSet::new();
    let mut pending = vec![(PathBuf::from(&target.source_path), true)];
    while let Some((source, is_crate_root)) = pending.pop() {
        let normalized_source = normalized(&source);
        if !discovered.insert(normalized_source.clone()) || !source.exists() {
            continue;
        }
        if !source_texts.contains_key(&normalized_source) {
            let text = std::fs::read_to_string(&source)
                .map_err(|error| format!("could not read {}: {error}", source.display()))?;
            source_texts.insert(normalized_source.clone(), text);
        }
        let text = source_texts
            .get(&normalized_source)
            .expect("reachable source was inserted into the shared cache");
        pending.extend(declared_module_paths(&source, &text, is_crate_root));
    }
    Ok(discovered)
}

fn declared_module_paths(source: &Path, text: &str, is_crate_root: bool) -> Vec<(PathBuf, bool)> {
    let parent = source.parent().unwrap_or_else(|| Path::new("."));
    let module_parent = match (
        is_crate_root,
        source.file_stem().and_then(|value| value.to_str()),
    ) {
        (true, _) => parent.to_path_buf(),
        (_, Some("lib" | "main" | "mod") | None) => parent.to_path_buf(),
        (_, Some(stem)) => parent.join(stem),
    };
    let mut paths = Vec::new();
    let mut declared_path = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(path) = path_attribute(trimmed) {
            declared_path = Some(path);
            continue;
        }
        if let Some(module) = module_declaration(trimmed) {
            if let Some(path) = declared_path.take() {
                paths.push((parent.join(path), true));
                continue;
            }
            let sibling = module_parent.join(format!("{module}.rs"));
            let child = module_parent.join(module).join("mod.rs");
            if sibling.exists() {
                paths.push((sibling, false));
            } else if child.exists() {
                paths.push((child, false));
            }
        }
    }
    paths
}

fn path_attribute(line: &str) -> Option<String> {
    let value = line.strip_prefix("#[path")?.split_once('"')?.1;
    Some(value.split_once('"')?.0.to_owned())
}

fn module_declaration(line: &str) -> Option<String> {
    let line = line.strip_prefix("pub ").unwrap_or(line);
    let name = line.strip_prefix("mod ")?.strip_suffix(';')?.trim();
    (!name.is_empty()).then(|| name.to_owned())
}
