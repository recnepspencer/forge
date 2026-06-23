pub fn worth_graph_authority_audited_source_roots() -> &'static [&'static str] {
    &[
        "crates/worth-topo/src/topology_operators/adoption",
        "crates/worth-topo/src/topology_operators/edge_split_blueprint",
        "crates/worth-topo/src/topology_operators/loop_reconstruction_blueprint",
        "crates/worth-topo/src/construction/query_native_boundary",
        "crates/worth-spatial/src/query_adoption",
        "crates/worth-spatial/src/workload_platform/evidence_ledger",
        "crates/worth-kernel/src/query_adoption",
        "crates/worth-kernel/src/construction/query_authority",
        "crates/worth-kernel/src/construction/graph_obligation_adoption",
        "crates/worth-kernel/src/construction/phase_chain",
        "crates/worth-kernel/src/construction/result_surface",
        "crates/worth-kernel/src/workload_composition",
        "crates/forge-query/src",
    ]
}

pub fn current_worth_graph_authority_audited_source_paths() -> Vec<String> {
    let workspace_root = workspace_root();
    let mut audited_source_paths = Vec::new();
    for source_root in worth_graph_authority_audited_source_roots() {
        collect_rust_sources(&workspace_root.join(source_root), &mut audited_source_paths);
    }
    audited_source_paths.sort();
    audited_source_paths
}

fn collect_rust_sources(source_root: &std::path::Path, audited_source_paths: &mut Vec<String>) {
    let entries = std::fs::read_dir(source_root).unwrap_or_else(|error| {
        panic!("failed to read audited source root {source_root:?}: {error}")
    });
    for entry in entries {
        let entry = entry.expect("failed to read audited source directory entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, audited_source_paths);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            audited_source_paths.push(normalize_workspace_source_path(&path));
        }
    }
}

fn normalize_workspace_source_path(path: &std::path::Path) -> String {
    path.strip_prefix(workspace_root())
        .expect("audited source path should be under the workspace root")
        .to_string_lossy()
        .replace('\\', "/")
}

fn workspace_root() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-kernel should live two levels below the workspace root")
}
