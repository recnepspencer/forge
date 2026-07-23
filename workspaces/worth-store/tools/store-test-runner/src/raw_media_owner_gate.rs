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
