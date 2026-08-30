use std::path::{Path, PathBuf};

#[test]
fn recovery_has_no_raw_wal_inspector_entry() {
    let recovery = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let physics = recovery
        .parent()
        .unwrap()
        .join("worth-store-recovery-physics");
    for source_root in [recovery.join("src"), physics.join("src")] {
        let offenders = rust_sources(&source_root)
            .into_iter()
            .filter(|path| {
                let source = std::fs::read_to_string(path).unwrap();
                source.contains("inspect_physical_wal_artifacts")
                    || source.contains("inspect_bounded_wal_active_tail_with_evidence")
            })
            .collect::<Vec<_>>();
        assert!(
            offenders.is_empty(),
            "raw recovery WAL inspector routes remain: {offenders:?}"
        );
    }
}

#[test]
fn recovery_wal_route_names_c4_binding_c9_admission_and_store_interpretation() {
    let recovery = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let admission = read(&recovery.join("src/orchestration/discovery/wal/admission.rs"));
    assert!(admission.contains("validate_wal_frame_prefix"));
    assert!(admission.contains("IntegrityAdmittedRecoveryArtifact::bind_wal_frame"));
    assert!(admission.contains("into_store_admission"));

    let conclusion = read(&recovery.join("src/orchestration/discovery/wal/conclusion.rs"));
    assert!(conclusion.contains("classify_admitted_wal_segment"));
    assert!(conclusion.contains("retain_admitted_recovery_wal_segment"));

    let planning = read(&recovery.join("src/orchestration/planning/admitted_basis.rs"));
    assert!(planning.contains("integrity"));
    assert!(planning.contains("admitted_wal"));
    assert!(planning.contains("selected_frames"));
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}
