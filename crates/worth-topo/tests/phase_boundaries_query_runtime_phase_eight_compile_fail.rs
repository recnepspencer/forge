#[test]
fn query_runtime_phase_eight_compile_fail_boundaries_hold() {
    let workspace_temp = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("trybuild-query-runtime-phase-eight");
    std::fs::create_dir_all(&workspace_temp).expect("trybuild temp directory");
    std::env::set_var("HOME", &workspace_temp);
    std::env::set_var("USERPROFILE", &workspace_temp);
    std::env::set_var("TMP", &workspace_temp);
    std::env::set_var("TEMP", &workspace_temp);
    std::env::set_var("CARGO_TARGET_DIR", workspace_temp.join("cargo-target"));

    let t = trybuild::TestCases::new();
    for golden in topology::facade::topology_query_runtime_phase_eight_golden_paths() {
        t.pass(golden.path());
    }
    for target in topology::facade::topology_query_runtime_phase_eight_compile_fail_targets() {
        t.compile_fail(target.path());
    }
}

#[test]
fn query_runtime_phase_eight_manifest_counts_hold() {
    assert_eq!(
        topology::facade::topology_query_runtime_phase_eight_compile_fail_targets().len(),
        topology::facade::TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_COMPILE_FAIL_TARGET_COUNT
    );
    assert_eq!(
        topology::facade::topology_query_runtime_phase_eight_golden_paths().len(),
        topology::facade::TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_GOLDEN_PATH_COUNT
    );
}

#[test]
fn query_runtime_phase_eight_folklore_inventory_scan_holds() {
    use std::path::Path;

    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for relative_path in topology::facade::PHASE_EIGHT_QUERY_RUNTIME_SCAN_PATHS {
        let scan_root = crate_root.join(relative_path);
        if scan_root.is_dir() {
            scan_directory_for_forbidden_patterns(&scan_root, relative_path);
            continue;
        }
        assert_forbidden_patterns_absent(&scan_root, relative_path);
    }
}

fn scan_directory_for_forbidden_patterns(dir: &std::path::Path, relative_prefix: &str) {
    for entry in std::fs::read_dir(dir).expect("folklore scan directory should be readable") {
        let entry = entry.expect("folklore scan directory entry");
        let path = entry.path();
        if path.is_dir() {
            let nested = format!(
                "{}/{}",
                relative_prefix,
                path.file_name().expect("directory name").to_string_lossy()
            );
            scan_directory_for_forbidden_patterns(&path, &nested);
            continue;
        }
        if path.extension().is_some_and(|ext| ext == "rs") {
            let relative = format!(
                "{}/{}",
                relative_prefix,
                path.file_name().expect("file name").to_string_lossy()
            );
            assert_forbidden_patterns_absent(&path, &relative);
        }
    }
}

fn assert_forbidden_patterns_absent(path: &std::path::Path, relative_path: &str) {
    if topology::facade::PHASE_EIGHT_EXCLUDED_FOLKLORE_PATHS
        .iter()
        .any(|excluded| relative_path.starts_with(excluded.trim_end_matches('/')))
    {
        return;
    }
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("folklore scan could not read `{relative_path}`: {error}"));
    for pattern in topology::facade::PHASE_EIGHT_FORBIDDEN_SUBSTITUTION_PATTERNS {
        assert!(
            !content.contains(pattern),
            "forbidden folklore pattern `{pattern}` found in `{relative_path}`"
        );
    }
}
