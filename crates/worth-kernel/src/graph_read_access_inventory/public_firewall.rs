use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn old_graph_read_adoption_scaffolding_is_not_production_authority() {
    for file in worth_kernel_firewall_rust_files() {
        let source = fs::read_to_string(&file).expect("production source should be readable");
        for forbidden in FORBIDDEN_OLD_GRAPH_READ_AUTHORITY_PATTERNS {
            assert!(
                !source.contains(forbidden),
                "old graph-read adoption authority pattern `{forbidden}` survived in {}",
                file.display()
            );
        }
    }
}

#[test]
fn old_graph_read_adoption_scaffolding_directory_is_deleted() {
    let deleted_scaffolding_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("query_adoption")
        .join("graph_read_access");

    assert!(
        !deleted_scaffolding_path.exists(),
        "old graph-read adoption scaffolding directory survived at {}",
        deleted_scaffolding_path.display()
    );
}

fn worth_kernel_firewall_rust_files() -> Vec<PathBuf> {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    rust_files_below(&src_root)
        .into_iter()
        .filter(|path| firewall_scan_should_include(path))
        .collect()
}

fn rust_files_below(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = fs::read_dir(root).expect("source directory should be readable");
    for entry in entries {
        let path = entry.expect("source entry should be readable").path();
        if path.is_dir() {
            files.extend(rust_files_below(&path));
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}

fn firewall_scan_should_include(path: &Path) -> bool {
    let path_text = path.to_string_lossy();
    !path_text.contains("certification\\public_facade_contracts\\compile_fail")
        && !path_text.contains("certification/public_facade_contracts/compile_fail")
        && !path_text.ends_with("graph_read_access_inventory\\public_firewall.rs")
        && !path_text.ends_with("graph_read_access_inventory/public_firewall.rs")
}

const FORBIDDEN_OLD_GRAPH_READ_AUTHORITY_PATTERNS: &[&str] = &[
    "pub mod graph_read_access;",
    "pub use graph_read_access::",
    "query_adoption::graph_read_access",
    "current_worth_kernel_construction_graph_read_access_adoption",
    "WorthKernelGraphReadAccessAdoptionReport",
    "WorthKernelGraphReadAccessAdoptionError",
    "OldGraphReadAdoption",
    "old_graph_read_adoption",
];
