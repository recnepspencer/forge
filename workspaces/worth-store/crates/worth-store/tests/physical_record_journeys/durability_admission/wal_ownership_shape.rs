use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
fn wal_append_effect_and_typed_promotion_each_have_one_production_owner() {
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("physical_runtime");
    let sources = rust_sources(&runtime);

    assert_single_owner(
        &runtime,
        &sources,
        "append_scheduled_artifact_exact_at(",
        "instance/executor/wal_append.rs",
    );
    assert_single_owner(
        &runtime,
        &sources,
        "PhysicalWalAppendSettlement::completed(",
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
    let recovery_wal = read(
        &crates
            .join("worth-store-recovery-physics")
            .join("src")
            .join("wal_durability")
            .join("mod.rs"),
    );
    for displaced in [
        "execute_wal_durability",
        "ExecutedWalDurabilityOutcome",
        "WalDurabilityExecutionError",
    ] {
        assert!(!recovery_root.contains(displaced));
        assert!(!recovery_wal.contains(displaced));
    }
    assert!(recovery_wal
        .contains("#[cfg(feature = \"certification-test-authority\")]\nmod certification_probe;"));

    let backend_facade = read(
        &crates
            .join("worth-store-physical-backend")
            .join("src")
            .join("facade.rs"),
    );
    let guarded_backend_append = "#[cfg(feature = \"certification-test-authority\")]\npub use crate::durability_ordering::StoreDurabilityAppendInput;";
    assert!(has_only_guarded_surface(
        &backend_facade,
        guarded_backend_append,
        "StoreDurabilityAppendInput",
    ));

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
    let guarded_backend_append = "#[cfg(feature = \"certification-test-authority\")]\npub use crate::durability_ordering::StoreDurabilityAppendInput;";
    let backend_mutant = format!(
        "{guarded_backend_append}\npub use crate::durability_ordering::StoreDurabilityAppendInput;"
    );
    assert!(!has_only_guarded_surface(
        &backend_mutant,
        guarded_backend_append,
        "StoreDurabilityAppendInput",
    ));

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
