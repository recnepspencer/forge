use std::collections::BTreeSet;
use std::path::Path;

use crate::workspace_root;

const FORBIDDEN_PATHS: &[&str] = &[
    "crates/worth-store-wal/src/durable_publication/mod.rs",
    "crates/worth-store-wal/src/durable_publication/declaration.rs",
    "crates/worth-store-wal/src/durable_publication/wal_scope.rs",
    "crates/worth-store-wal/src/durable_publication/checkpoint_scope.rs",
    "crates/worth-store-wal/src/durable_publication/tests.rs",
];

const REQUIRED_PATHS: &[&str] = &[
    "crates/worth-store-wal/src/publication_declaration/mod.rs",
    "crates/worth-store-wal/src/publication_declaration/declaration.rs",
    "crates/worth-store-wal/src/publication_declaration/wal_scope.rs",
    "crates/worth-store-wal/src/publication_declaration/checkpoint_scope.rs",
    "crates/worth-store-wal/src/publication_declaration/tests.rs",
];

const FORBIDDEN_TYPE_NAMES: &[&str] = &[
    "DurablePublicationDeclaration",
    "DurablePublicationScope",
    "CheckpointDurablePublicationScope",
    "WalFrameDurablePublicationScope",
];

#[test]
fn wal_declarations_use_publication_only_names_and_topology() {
    inspect_workspace(&workspace_root()).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn declaration_topology_gate_rejects_old_path_type_and_incomplete_destination_mutants() {
    for forbidden in FORBIDDEN_PATHS {
        inspect_paths([*forbidden], REQUIRED_PATHS.iter().copied())
            .expect_err("an old durable-publication path must fail the cutover gate");
    }
    for forbidden in FORBIDDEN_TYPE_NAMES {
        inspect_source("crates/mutant/src/lib.rs", forbidden)
            .expect_err("an old durable-publication type must fail the cutover gate");
    }
    inspect_paths(
        std::iter::empty::<&str>(),
        ["crates/worth-store-wal/src/publication_declaration/mod.rs"],
    )
    .expect_err("an incomplete publication-declaration destination must fail the cutover gate");
}

fn inspect_workspace(root: &Path) -> Result<(), String> {
    inspect_paths(
        FORBIDDEN_PATHS
            .iter()
            .copied()
            .filter(|path| root.join(path).exists()),
        REQUIRED_PATHS
            .iter()
            .copied()
            .filter(|path| root.join(path).exists()),
    )?;
    inspect_sources(root, &root.join("crates"))
}

fn inspect_paths<'path>(
    forbidden_paths: impl IntoIterator<Item = &'path str>,
    required_paths: impl IntoIterator<Item = &'path str>,
) -> Result<(), String> {
    if let Some(path) = forbidden_paths.into_iter().next() {
        return Err(format!("old durable-publication path remains: {path}"));
    }
    let actual = required_paths.into_iter().collect::<BTreeSet<_>>();
    let required = REQUIRED_PATHS.iter().copied().collect::<BTreeSet<_>>();
    if actual != required {
        let missing = required.difference(&actual).copied().collect::<Vec<_>>();
        return Err(format!(
            "publication-declaration destination is incomplete; missing {missing:?}"
        ));
    }
    Ok(())
}

fn inspect_sources(root: &Path, source_root: &Path) -> Result<(), String> {
    let mut pending = vec![source_root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
        {
            let path = entry
                .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
                let source = std::fs::read_to_string(&path)
                    .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
                inspect_source(&relative_path(root, &path), &source)?;
            }
        }
    }
    Ok(())
}

fn inspect_source(path: &str, source: &str) -> Result<(), String> {
    for forbidden in FORBIDDEN_TYPE_NAMES {
        if source.contains(forbidden) {
            return Err(format!(
                "{path} retains old durable-publication type `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
