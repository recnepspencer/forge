use std::fs;
use std::path::Path;

#[test]
fn production_query_graph_uses_single_execution_adapter_boundary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime/query_graph");
    let offenders = rust_sources(&root)
        .into_iter()
        .filter(|path| !path.ends_with(Path::new("execution/adapter.rs")))
        .filter(|path| {
            fs::read_to_string(path)
                .expect("query graph source should be readable")
                .contains("ForgeQueryGraphObligationInMemoryTestWorkspace")
        })
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "production Query in-memory workspace use must stay sealed behind execution/adapter.rs: {offenders:?}"
    );
}

#[test]
fn production_query_graph_does_not_execute_query_workspace_directly() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime/query_graph");
    let offenders = rust_sources(&root)
        .into_iter()
        .filter(|path| !path.ends_with(Path::new("execution/adapter.rs")))
        .filter(|path| {
            fs::read_to_string(path)
                .expect("query graph source should be readable")
                .contains(".prove_execution(")
        })
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "Query workspace execution must flow through WorthUiQueryGraphExecutionAdapter: {offenders:?}"
    );
}

#[test]
fn production_query_graph_does_not_preselect_obligation_support_in_worth_tables() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime/query_graph");
    let forbidden_sources = [
        "support_posture.rs",
        "WorthUiQueryGraphSupportPosture",
        "for_mounted_interaction(",
        "for_primitive_event_dispatch(",
        "for_primitive_content_anatomy(",
    ];
    let offenders = rust_sources(&root)
        .into_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(&path).expect("query graph source should be readable");
            forbidden_sources
                .iter()
                .any(|forbidden| source.contains(forbidden))
                .then_some(path)
        })
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "Worth query graph must declare touch operations and let Query select support rows: {offenders:?}"
    );
}

fn rust_sources(root: &Path) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    collect_rust_sources(root, &mut paths);
    paths
}

fn collect_rust_sources(root: &Path, paths: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(root).expect("source directory should be readable") {
        let entry = entry.expect("source entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, paths);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            paths.push(path);
        }
    }
}
