use std::fs;
use std::path::{Path, PathBuf};

pub fn rust_files(root: &Path) -> Vec<PathBuf> {
    all_paths(root)
        .into_iter()
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect()
}

pub fn all_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    collect_paths(root, &mut paths);
    paths
}

pub fn file_contains(path: &Path, needle: &str) -> bool {
    fs::read_to_string(path)
        .expect("source file should be readable")
        .contains(needle)
}

pub fn strip_toml_comment(line: &str) -> &str {
    line.split_once('#')
        .map_or(line, |(before_comment, _)| before_comment)
        .trim()
}

pub fn line_declares_forge_query_dependency(line: &str) -> bool {
    line.starts_with("[dependencies.forge-query]")
        || line.starts_with("[dev-dependencies.forge-query]")
        || line.contains(".dependencies.forge-query]")
        || cargo_key_matches_forge_query(line)
        || line.contains("package = \"forge-query\"")
        || line.contains("package=\"forge-query\"")
}

pub fn is_runtime_receipt_adapter(path: &Path, src_root: &Path) -> bool {
    let relative = relative_source_path(path, src_root);
    matches!(
        relative.as_str(),
        "reload\\validation_runtime_change_evidence.rs" | "runtime_workbench\\rebind_execution.rs"
    )
}

pub fn is_native_egui_boundary_file(
    path: &Path,
    src_root: &Path,
    admitted_boundary_files: &[&str],
) -> bool {
    let relative = relative_source_path(path, src_root);
    admitted_boundary_files.contains(&relative.as_str())
}

fn collect_paths(root: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("source directory should be readable") {
        let path = entry.expect("directory entry should be readable").path();
        paths.push(path.clone());
        if path.is_dir() {
            collect_paths(&path, paths);
        }
    }
}

fn cargo_key_matches_forge_query(line: &str) -> bool {
    line.split_once('=')
        .is_some_and(|(key, _)| key.trim() == "forge-query")
}

fn relative_source_path(path: &Path, src_root: &Path) -> String {
    path.strip_prefix(src_root)
        .expect("path should be under src root")
        .display()
        .to_string()
}
