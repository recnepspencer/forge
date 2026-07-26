use std::path::{Component, Path, PathBuf};

pub(super) fn production_rust_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut sources = rust_sources(root)?;
    sources.retain(|path| !is_test_support(path));
    Ok(sources)
}

pub(super) fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?;
        for entry in entries {
            let path = entry
                .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

pub(super) fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))
}

pub(super) fn workspace_relative(path: &Path) -> String {
    path.strip_prefix(super::super::workspace_root())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(super) fn occurrence_count(source: &str, needle: &str) -> usize {
    source.match_indices(needle).count()
}

fn is_test_support(path: &Path) -> bool {
    let filename = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    filename.ends_with("_tests.rs")
        || filename == "tests.rs"
        || path.components().any(|component| {
            matches!(
                component,
                Component::Normal(value)
                    if value == "tests" || value == "test_support"
            )
        })
}
