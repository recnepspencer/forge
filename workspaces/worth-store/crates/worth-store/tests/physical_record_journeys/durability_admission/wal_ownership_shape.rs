use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn wal_and_checkpoint_append_effects_have_exact_semantic_owners() {
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("physical_runtime");
    let sources = rust_sources(&runtime);

    assert_exact_owners(
        &runtime,
        &sources,
        ".append_scheduled_artifact_exact_at(",
        &[
            "instance/executor/checkpoint.rs",
            "instance/executor/wal_append.rs",
        ],
    );
    assert_single_owner(
        &runtime,
        &sources,
        "fn dispatch_wal_append(",
        "instance/executor/wal_append.rs",
    );
    assert_single_owner(
        &runtime,
        &sources,
        "fn dispatch_checkpoint(",
        "instance/executor/checkpoint.rs",
    );
    assert_single_owner(
        &runtime,
        &sources,
        "PhysicalWalAppendSettlement::completed_append(",
        "durability/wal/port.rs",
    );
    assert_single_owner(
        &runtime,
        &sources,
        "PhysicalWalAppendSettlement::completed_segment_create(",
        "durability/wal/port.rs",
    );
    assert_single_owner(
        &runtime,
        &sources,
        "WalAppendedPhysicalMutation::new(",
        "durability/wal/port.rs",
    );
    assert!(
        sources
            .iter()
            .all(|(_, source)| !source.contains(".append_artifact_exact_at(")),
        "ordinary Store code must not bypass scheduled WAL append execution"
    );
}

#[test]
fn wal_segment_creation_has_one_scheduled_owner() {
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("physical_runtime");
    let sources = rust_sources(&runtime);
    let create_owner = read(&runtime.join("instance/executor/wal_segment_create.rs"));
    assert_eq!(
        create_owner.matches(".write_scheduled_new_exact(").count(),
        1
    );
    assert!(
        sources
            .iter()
            .filter(|(path, _)| {
                path.to_string_lossy()
                    .replace('\\', "/")
                    .contains("durability/wal")
            })
            .all(|(_, source)| !source.contains(".write_new(")),
        "the WAL owner must not create an empty or unscheduled segment artifact"
    );
}

#[test]
fn displaced_wal_executors_are_absent_from_default_facades() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("worth-store is inside the crates directory");
    let recovery_root = read(
        &crates
            .join("worth-store-recovery-physics")
            .join("src")
            .join("lib.rs"),
    );
    let displaced_recovery_wal = crates
        .join("worth-store-recovery-physics")
        .join("src")
        .join("wal_durability");
    assert!(
        !displaced_recovery_wal.exists(),
        "the recovery-owned WAL execution directory must remain deleted"
    );
    let recovery_wal_basis = read(
        &crates
            .join("worth-store-recovery-physics")
            .join("src")
            .join("wal_recovery_basis")
            .join("mod.rs"),
    );
    for displaced in [
        "execute_wal_durability",
        "ExecutedWalDurabilityOutcome",
        "WalDurabilityExecutionError",
    ] {
        assert!(!recovery_root.contains(displaced));
        assert!(!recovery_wal_basis.contains(displaced));
    }
    for descriptive in [
        "WalAppendReceipt",
        "WalDurabilityObservation",
        "WalDurabilityCrashBasis",
    ] {
        assert!(recovery_wal_basis.contains(descriptive));
    }

    let wal_root = read(&crates.join("worth-store-wal").join("src").join("lib.rs"));
    let guarded_wal_planner =
        "#[cfg(feature = \"certification-authority\")]\npub use artifact_store::WalAppendPlanner;";
    assert!(has_only_guarded_surface(
        &wal_root,
        guarded_wal_planner,
        "WalAppendPlanner",
    ));
}

#[test]
fn cleanup_gate_rejects_unguarded_duplicate_exports() {
    let guarded_wal_planner =
        "#[cfg(feature = \"certification-authority\")]\npub use artifact_store::WalAppendPlanner;";
    let wal_mutant = format!("{guarded_wal_planner}\npub use artifact_store::WalAppendPlanner;");
    assert!(!has_only_guarded_surface(
        &wal_mutant,
        guarded_wal_planner,
        "WalAppendPlanner",
    ));
}

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read production source {}: {error}", path.display()))
        .replace("\r\n", "\n")
}

fn assert_single_owner(
    root: &Path,
    sources: &[(PathBuf, String)],
    needle: &str,
    expected_owner: &str,
) {
    let owners = sources
        .iter()
        .flat_map(|(path, source)| std::iter::repeat_n(path, source.matches(needle).count()))
        .collect::<Vec<_>>();
    assert_eq!(
        owners.len(),
        1,
        "{needle} must have exactly one production owner, found {owners:?}"
    );
    let relative = owners[0]
        .strip_prefix(root)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    assert_eq!(relative, expected_owner);
}

fn assert_exact_owners(
    root: &Path,
    sources: &[(PathBuf, String)],
    needle: &str,
    expected_owners: &[&str],
) {
    let mut owners = sources
        .iter()
        .flat_map(|(path, source)| std::iter::repeat_n(path, source.matches(needle).count()))
        .map(|path| {
            path.strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect::<Vec<_>>();
    owners.sort();
    assert_eq!(owners, expected_owners, "unexpected owners for {needle}");
}

fn rust_sources(root: &Path) -> Vec<(PathBuf, String)> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push((
                    path.clone(),
                    fs::read_to_string(path).unwrap().replace("\r\n", "\n"),
                ));
            }
        }
    }
    sources
}

fn has_only_guarded_surface(source: &str, guarded: &str, symbol: &str) -> bool {
    source.matches(guarded).count() == 1 && !source.replacen(guarded, "", 1).contains(symbol)
}
