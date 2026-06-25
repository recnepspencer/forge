#[test]
fn phase_five_sources_do_not_claim_fallback_walks_receipts_or_plan_consumption() {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("graph_read_access_declarations");
    let forbidden = [
        "graph_read_access_plan_consumption",
        "ephemeral_graph_index_receipt",
        "graph_read_streaming_receipt",
        "live_graph_read_access",
        "local_graph_walk",
        "fallback_graph_walk",
        "adjacency_loop",
        "broad_scan",
        "visited_set",
        "dedup_set",
        "result_buffer",
        "increase_limit_and_retry",
    ];
    let mut offenders = Vec::new();
    for path in rust_sources_under(&source_root).into_iter().filter(|path| {
        !path
            .components()
            .any(|component| component.as_os_str() == "tests")
            && path
                .file_name()
                .is_some_and(|file_name| file_name != "tests.rs")
            && !is_source_firewall_pattern_catalog(path)
    }) {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        for needle in forbidden {
            if text.contains(needle) {
                offenders.push(format!("{} contains {needle}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Phase 5 must not grow fallback traversal or receipt/plan-consumption claims: {offenders:#?}"
    );
}

fn is_source_firewall_pattern_catalog(path: &std::path::Path) -> bool {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();
    components.windows(3).any(|window| {
        window[0] == "deletion_firewall"
            && window[1] == "source_firewall"
            && window[2] == "forbidden_pattern.rs"
    })
}

fn rust_sources_under(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut sources = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        for entry in std::fs::read_dir(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
        {
            let entry = entry.expect("directory entry should be readable");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}
