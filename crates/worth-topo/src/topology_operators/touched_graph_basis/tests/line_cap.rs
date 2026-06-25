use std::path::PathBuf;

#[test]
fn touched_graph_basis_files_satisfy_workspace_line_cap() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let basis_dir = manifest_dir.join("src/topology_operators/touched_graph_basis");
    let mut rust_files = std::fs::read_dir(&basis_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect::<Vec<_>>();
    rust_files.sort();

    for path in rust_files {
        let relative = path
            .strip_prefix(&manifest_dir)
            .unwrap()
            .display()
            .to_string();
        let contents = std::fs::read_to_string(&path).unwrap();
        let line_count = contents.lines().count();
        assert!(
            line_count <= 400,
            "{} has {} lines, above the workspace cap",
            relative,
            line_count
        );
    }
}

#[test]
fn production_touched_graph_basis_has_no_spatial_geometry_admission_bridge() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let basis_dir = manifest_dir.join("src/topology_operators/touched_graph_basis");
    let forbidden_patterns = [
        "WorthGeometryOnlyEvidence",
        "GeometryOnlyEvidence",
        "geometry_only_evidence",
        "spatial_sealed_receipt_admission",
        "from_spatial_boolean_receipt",
        "type_name::<",
    ];

    for entry in std::fs::read_dir(&basis_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        if path
            .file_name()
            .is_some_and(|file_name| file_name == "tests.rs")
        {
            continue;
        }
        let relative = path
            .strip_prefix(&manifest_dir)
            .unwrap()
            .display()
            .to_string();
        let contents = std::fs::read_to_string(&path).unwrap();
        for forbidden_pattern in forbidden_patterns {
            assert!(
                !contents.contains(forbidden_pattern),
                "{relative} still contains forbidden topology geometry admission pattern {forbidden_pattern}"
            );
        }
    }
}
