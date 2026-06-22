#[test]
fn runtime_sources_do_not_depend_on_consumer_support_snapshots() {
    let runtime_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("runtime");
    let mut offending_files = Vec::new();
    collect_runtime_support_snapshot_imports(&runtime_dir, &mut offending_files);

    assert!(
        offending_files.is_empty(),
        "runtime authority must not import or mention consumer support snapshots: {offending_files:?}"
    );
}

fn collect_runtime_support_snapshot_imports(
    directory: &std::path::Path,
    offending_files: &mut Vec<String>,
) {
    for entry in std::fs::read_dir(directory).expect("runtime directory should be readable") {
        let entry = entry.expect("runtime directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_runtime_support_snapshot_imports(&path, offending_files);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            let source = std::fs::read_to_string(&path).expect("runtime source should be readable");
            if source.contains("consumer_kit::support_snapshot")
                || source.contains("project_support_snapshot")
                || source.contains("ForgeQuerySupportSnapshot")
            {
                offending_files.push(path.display().to_string());
            }
        }
    }
}
