use std::path::{Path, PathBuf};

use super::{rust_sources, workspace_root};

#[test]
fn raw_media_owner_entrypoint_has_one_workspace_consumer() {
    let root = workspace_root();
    let mut callsites = Vec::new();
    for source in production_sources(root).expect("read Store production sources") {
        let text = std::fs::read_to_string(&source).expect("read Store workspace source");
        for (line_index, line) in text.lines().enumerate() {
            if line.contains("qualify_filesystem_media(") {
                callsites.push((source.clone(), line_index + 1));
            }
        }
    }
    assert_eq!(
        callsites.len(),
        1,
        "raw backend qualification escaped Store"
    );
    assert!(callsites[0]
        .0
        .ends_with("worth-store/src/physical_runtime/media_ownership/admission.rs"));
}

fn production_sources(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut sources = Vec::new();
    for package_directory in [root.join("crates"), root.join("tools")] {
        for entry in std::fs::read_dir(package_directory)? {
            let package_root = entry?.path();
            let package_name = package_root.file_name();
            if !package_root.is_dir()
                || package_name.is_some_and(|name| {
                    name == std::ffi::OsStr::new("worth-store-physical-backend")
                })
                || package_name
                    .is_some_and(|name| name == std::ffi::OsStr::new("store-test-runner"))
            {
                continue;
            }
            let source_root = package_root.join("src");
            if source_root.is_dir() {
                sources.extend(rust_sources(&source_root)?);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

#[test]
fn raw_media_owner_feature_has_one_manifest_consumer() {
    let root = workspace_root();
    let mut consumers = Vec::new();
    for manifest in cargo_manifests(root).expect("read Store workspace manifests") {
        let text = std::fs::read_to_string(&manifest).expect("read Store workspace manifest");
        if text.contains("store-runtime-owner") {
            consumers.push(manifest);
        }
    }
    consumers.sort();
    let mut expected = vec![
        root.join("crates/worth-store-physical-backend/Cargo.toml"),
        root.join("crates/worth-store/Cargo.toml"),
    ];
    expected.sort();
    assert_eq!(
        consumers, expected,
        "only the unpublished backend may define and Store may enable raw owner qualification"
    );
    let backend =
        std::fs::read_to_string(root.join("crates/worth-store-physical-backend/Cargo.toml"))
            .unwrap();
    assert!(backend.contains("publish = false"));
}

fn cargo_manifests(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut pending = vec![root.to_path_buf()];
    let mut manifests = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "target") {
                    continue;
                }
                pending.push(path);
            } else if path.file_name().is_some_and(|name| name == "Cargo.toml") {
                manifests.push(path);
            }
        }
    }
    manifests.sort();
    Ok(manifests)
}

#[test]
fn recovery_cleanup_filesystem_mechanics_remain_in_the_c4_backend() {
    let root = workspace_root();
    let backend = std::fs::read_to_string(
        root.join("crates/worth-store-physical-backend/src/recovery_media/cleanup.rs"),
    )
    .expect("read backend cleanup owner");
    assert!(backend.contains("remove_file_durably_observed"));
    assert!(backend.contains("execute_recovery_cleanup_removal"));
    assert!(!backend.contains("BackendRecoveryCleanupEffectAuthority"));
    assert!(!backend.contains("BackendRecoveryCleanupEffectIssuer"));
    assert!(!backend.contains("VerifiedCheckpointStream"));
    assert!(!backend.contains("VerifiedWalArtifact"));

    let callsites = production_sources(&root)
        .expect("read Store workspace sources")
        .into_iter()
        .filter(|source| {
            std::fs::read_to_string(source)
                .is_ok_and(|text| text.contains("execute_recovery_cleanup_removal("))
        })
        .collect::<Vec<_>>();
    assert_eq!(callsites.len(), 1, "cleanup C.4 entry escaped Store");
    assert!(callsites[0].ends_with(
        "worth-store/src/physical_runtime/recovery_coordination/cleanup/removal/execution.rs"
    ));

    let store_cleanup = root.join("crates/worth-store/src/physical_runtime");
    let mut forbidden = Vec::new();
    for source in rust_sources(&store_cleanup).expect("read Store physical runtime") {
        let path = source.to_string_lossy().replace('\\', "/");
        if !path.contains("/recovery_coordination/cleanup/")
            && !path.contains("/media_ownership/recovery_cleanup/")
        {
            continue;
        }
        let text = std::fs::read_to_string(&source).expect("read Store cleanup source");
        for token in [
            "cap_std",
            "open_ambient_dir",
            "remove_file(",
            "synchronize_directory(",
        ] {
            if text.contains(token) {
                forbidden.push((source.clone(), token));
            }
        }
    }
    assert!(
        forbidden.is_empty(),
        "Store cleanup policy regained raw filesystem mechanics: {forbidden:?}"
    );
}

#[test]
fn cleanup_backend_is_explicitly_one_unpublished_trusted_c4_boundary() {
    let root = workspace_root();
    let manifest =
        std::fs::read_to_string(root.join("crates/worth-store-physical-backend/Cargo.toml"))
            .expect("read backend manifest");
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains("recovery-runtime-owner = []"));
    assert!(manifest.contains("store-runtime-owner = []"));

    let facade =
        std::fs::read_to_string(root.join("crates/worth-store-physical-backend/src/facade.rs"))
            .expect("read backend facade");
    assert!(facade.contains(
        "cfg(all(feature = \"recovery-runtime-owner\", feature = \"store-runtime-owner\"))"
    ));

    let topology = std::fs::read_to_string(
        root.join("../../_docs/worth-store/physical-reconstruction-c8-destination-topology.csv"),
    )
    .expect("read destination topology");
    let trusted = topology
        .lines()
        .filter(|line| line.contains("trusted-unpublished-c4-media-boundary"))
        .collect::<Vec<_>>();
    assert_eq!(
        trusted.len(),
        2,
        "cleanup and revalidation form one trusted C4 family"
    );
    assert!(trusted.iter().all(|line| {
        line.contains("crates/worth-store-physical-backend/src/recovery_media/cleanup")
    }));
}
