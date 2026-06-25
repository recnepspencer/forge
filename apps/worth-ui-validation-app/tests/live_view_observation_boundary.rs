use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn live_view_denials_stay_typed_until_runtime_mounted_evidence() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        "last_live_view_edit_denial: Option<String>",
        "last_live_view_submission_denial: Option<String>",
        "format!(\"{denial:?}\")",
    ];
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            forbidden.iter().any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "live-view edit/submission denials must stay typed until Worth UI mounts evidence rows: {offenders:?}"
    );
}

#[test]
fn validation_app_asks_runtime_to_mount_live_view_observations() {
    let evidence_renderer = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("app")
        .join("live_view")
        .join("evidence_rendering.rs");
    let text = fs::read_to_string(evidence_renderer).expect("evidence renderer should exist");

    assert!(
        text.contains("mount_live_view_observation_evidence("),
        "live-view observation evidence must be mounted by Worth UI runtime authority"
    );
}

fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files(root, &mut files);
    files
}

fn collect_rust_files(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_file() {
        if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path.to_path_buf());
        }
        return;
    }
    for entry in fs::read_dir(path).expect("directory should be readable") {
        collect_rust_files(
            &entry.expect("directory entry should be readable").path(),
            files,
        );
    }
}
