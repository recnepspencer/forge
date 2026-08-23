use std::collections::BTreeSet;
use std::path::Path;

pub(super) fn add(
    root: &Path,
    paths: &mut BTreeSet<String>,
    gate_root: &str,
    api_inventory: &str,
) -> Result<(), String> {
    insert(paths, api_inventory);
    insert(
        paths,
        "workspaces/worth-store/tools/store-test-runner/Cargo.toml",
    );
    collect_rust_sources(root, &format!("{gate_root}/facade_inventory"), paths)?;
    collect_rust_sources(
        root,
        "workspaces/worth-store/crates/worth-store-recovery-physics/src",
        paths,
    )?;
    for destination in [
        "workspaces/worth-store/crates/worth-store-offline-verifier/src/truth_composition/candidate_evaluation/mod.rs",
        "workspaces/worth-store/crates/worth-store-offline-verifier/src/truth_composition/candidate_evaluation/candidate_set.rs",
        "workspaces/worth-store/crates/worth-store-physical-isolation/src/publication/recovery_replay.rs",
    ] {
        insert(paths, destination);
    }
    Ok(())
}

fn collect_rust_sources(
    root: &Path,
    relative_root: &str,
    paths: &mut BTreeSet<String>,
) -> Result<(), String> {
    let mut pending = vec![root.join(relative_root)];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?
        {
            let path = entry
                .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|error| format!("API source escaped repository root: {error}"))?;
                insert(paths, &relative.to_string_lossy());
            }
        }
    }
    Ok(())
}

fn insert(paths: &mut BTreeSet<String>, path: &str) {
    paths.insert(path.replace('\\', "/"));
}
