use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use super::read;

const LEGACY_FEATURES: &[(&str, &str)] = &[
    ("legacy-s2-models", "legacy-s2-module-closure"),
    (
        "legacy-certification-models",
        "legacy-certification-module-closure",
    ),
];

pub(super) fn discover(
    workspace: &Path,
    sources: &[PathBuf],
) -> Result<BTreeMap<PathBuf, BTreeSet<String>>, String> {
    let mut closure = BTreeMap::<PathBuf, BTreeSet<String>>::new();
    for source in sources
        .iter()
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("rs"))
    {
        let text = read(source)?;
        for declaration in gated_module_declarations(&text) {
            let root = resolve_module_root(workspace, source, &declaration)?;
            for descendant in module_descendants(workspace, &root)? {
                closure
                    .entry(descendant)
                    .or_default()
                    .insert(declaration.family.clone());
            }
        }
    }
    Ok(closure)
}

struct GatedModuleDeclaration {
    name: String,
    path_override: Option<String>,
    family: String,
}

fn gated_module_declarations(source: &str) -> Vec<GatedModuleDeclaration> {
    let mut declarations = Vec::new();
    let mut families = BTreeSet::new();
    let mut path_override = None;
    let mut attribute = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if !attribute.is_empty() {
            attribute.push(' ');
            attribute.push_str(trimmed);
            if trimmed.ends_with(']') {
                classify_attribute(&attribute, &mut families, &mut path_override);
                attribute.clear();
            }
            continue;
        }
        if trimmed.starts_with("#[") {
            if trimmed.ends_with(']') {
                classify_attribute(trimmed, &mut families, &mut path_override);
            } else {
                attribute.push_str(trimmed);
            }
            continue;
        }
        if let Some(name) = module_name(trimmed) {
            for family in &families {
                declarations.push(GatedModuleDeclaration {
                    name: name.clone(),
                    path_override: path_override.clone(),
                    family: family.clone(),
                });
            }
        }
        if !trimmed.is_empty() && !trimmed.starts_with("//") {
            families.clear();
            path_override = None;
        }
    }
    declarations
}

fn classify_attribute(
    attribute: &str,
    families: &mut BTreeSet<String>,
    path_override: &mut Option<String>,
) {
    if is_cfg_attribute(attribute) {
        let compact = attribute
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        for (feature, family) in LEGACY_FEATURES {
            if compact.contains(&format!("feature=\"{feature}\"")) {
                families.insert((*family).to_owned());
            }
        }
    }
    if let Some(path) = attribute_path(attribute) {
        *path_override = Some(path);
    }
}

fn is_cfg_attribute(attribute: &str) -> bool {
    let Some(body) = attribute.strip_prefix("#[") else {
        return false;
    };
    let body = body.trim_start();
    body.starts_with("cfg(") || body.starts_with("cfg (")
}

fn attribute_path(attribute: &str) -> Option<String> {
    if !attribute.starts_with("#[path") {
        return None;
    }
    let start = attribute.find('"')? + 1;
    let end = attribute.rfind('"')?;
    (start <= end).then(|| attribute[start..end].to_owned())
}

fn module_name(declaration: &str) -> Option<String> {
    if declaration.contains('{') || !declaration.ends_with(';') {
        return None;
    }
    let tokens = declaration.split_whitespace().collect::<Vec<_>>();
    let index = tokens.iter().position(|token| *token == "mod")?;
    let name = tokens.get(index + 1)?.trim_end_matches(';');
    (!name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_'))
    .then(|| name.to_owned())
}

fn resolve_module_root(
    workspace: &Path,
    source: &Path,
    declaration: &GatedModuleDeclaration,
) -> Result<PathBuf, String> {
    let base = module_base(source)?;
    let candidates = match &declaration.path_override {
        Some(path) => vec![base.join(path)],
        None => vec![
            base.join(format!("{}.rs", declaration.name)),
            base.join(&declaration.name).join("mod.rs"),
        ],
    };
    let present = candidates
        .into_iter()
        .filter(|candidate| candidate.is_file())
        .collect::<Vec<_>>();
    if present.len() != 1 {
        return Err(format!(
            "legacy module `{}` declared at {} resolves to {} roots",
            declaration.name,
            source.display(),
            present.len()
        ));
    }
    let canonical_root = std::fs::canonicalize(
        present.into_iter().next().expect("one module root"),
    )
    .map_err(|error| {
        format!(
            "cannot canonicalize legacy module `{}` from {}: {error}",
            declaration.name,
            source.display()
        )
    })?;
    let canonical_workspace = std::fs::canonicalize(workspace)
        .map_err(|error| format!("cannot canonicalize {}: {error}", workspace.display()))?;
    let relative = canonical_root
        .strip_prefix(&canonical_workspace)
        .map_err(|_| {
            format!(
                "legacy module `{}` escapes Store workspace at {}",
                declaration.name,
                canonical_root.display()
            )
        })?;
    Ok(workspace.join(relative))
}

fn module_base(source: &Path) -> Result<PathBuf, String> {
    let parent = source
        .parent()
        .ok_or_else(|| format!("module source has no parent: {}", source.display()))?;
    let file_name = source.file_name().and_then(|name| name.to_str());
    if matches!(file_name, Some("lib.rs" | "main.rs" | "mod.rs")) {
        return Ok(parent.to_path_buf());
    }
    let stem = source
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("module source has no UTF-8 stem: {}", source.display()))?;
    Ok(parent.join(stem))
}

fn module_descendants(workspace: &Path, root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut descendants = vec![root.to_path_buf()];
    let directory = if root.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
        root.parent().map(Path::to_path_buf)
    } else {
        root.parent().and_then(|parent| {
            root.file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| parent.join(stem))
        })
    };
    if let Some(directory) = directory.filter(|directory| directory.is_dir()) {
        collect_rust_sources(workspace, &directory, &mut descendants)?;
    }
    descendants.sort();
    descendants.dedup();
    Ok(descendants)
}

fn collect_rust_sources(
    workspace: &Path,
    directory: &Path,
    descendants: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if !directory.starts_with(workspace) {
        return Err(format!(
            "legacy module directory escapes Store workspace: {}",
            directory.display()
        ));
    }
    let mut pending = vec![directory.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
        {
            let entry = entry
                .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?;
            let file_type = entry.file_type().map_err(|error| {
                format!(
                    "cannot inspect type for {}: {error}",
                    entry.path().display()
                )
            })?;
            if file_type.is_symlink() {
                return Err(format!(
                    "legacy module closure contains unsupported symlink: {}",
                    entry.path().display()
                ));
            }
            let path = entry.path();
            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file()
                && path.extension().and_then(|extension| extension.to_str()) == Some("rs")
            {
                descendants.push(path);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
