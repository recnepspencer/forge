pub(super) fn collect_runtime_support_snapshot_imports(
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
                || source.contains("WorthQuerySupportSnapshot")
            {
                offending_files.push(path.display().to_string());
            }
        }
    }
}

pub(super) fn collect_production_serde_json_residue(
    directory: &std::path::Path,
    crate_root: &std::path::Path,
    files_with_residue: &mut Vec<String>,
) {
    collect_production_marker_residue(directory, crate_root, "serde_json", files_with_residue);
}

pub(super) fn collect_production_string_map_residue(
    directory: &std::path::Path,
    crate_root: &std::path::Path,
    files_with_residue: &mut Vec<String>,
) {
    for marker in ["BTreeMap<String", "HashMap<String"] {
        collect_production_marker_residue(directory, crate_root, marker, files_with_residue);
    }
    files_with_residue.sort();
    files_with_residue.dedup();
}

fn collect_production_marker_residue(
    directory: &std::path::Path,
    crate_root: &std::path::Path,
    marker: &str,
    files_with_residue: &mut Vec<String>,
) {
    collect_rust_source_marker_residue(directory, crate_root, marker, files_with_residue);
    files_with_residue.retain(|path| !is_test_source_path(path));
}

pub(super) fn collect_rust_source_marker_residue(
    directory: &std::path::Path,
    crate_root: &std::path::Path,
    marker: &str,
    files_with_residue: &mut Vec<String>,
) {
    let entries = std::fs::read_dir(directory).unwrap_or_else(|error| {
        panic!(
            "source directory {} should be readable: {error}",
            directory.display()
        )
    });
    for entry in entries {
        let entry = entry.expect("source directory entry should be readable");
        let path = entry.path();
        let file_type = entry.file_type().unwrap_or_else(|error| {
            panic!("source entry {} should be typed: {error}", path.display())
        });
        if file_type.is_dir() {
            collect_rust_source_marker_residue(&path, crate_root, marker, files_with_residue);
        } else if file_type.is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("rs")
        {
            let source = std::fs::read_to_string(&path).expect("Rust source should be readable");
            if source.contains(marker) {
                files_with_residue.push(relative_source_path(crate_root, &path));
            }
        }
    }
    files_with_residue.sort();
}

fn is_test_source_path(relative_path: &str) -> bool {
    relative_path.contains("/tests/") || relative_path.ends_with("_tests.rs")
}

fn relative_source_path(crate_root: &std::path::Path, path: &std::path::Path) -> String {
    path.strip_prefix(crate_root)
        .expect("source file should live under crate root")
        .iter()
        .map(|component| component.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
