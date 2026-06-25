use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn validation_app_renders_mounted_product_views_not_raw_mounted_views() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        "mount_live_view_projection(",
        "WorthUiMountedViewReceipt",
        "mounted_product_view().nodes()",
        "mounted_product_view()\r\n        .nodes()",
        "mounted_product_view()\n        .nodes()",
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
        "validation app must render runtime-proved mounted product views, not raw mounted views: {offenders:?}"
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
