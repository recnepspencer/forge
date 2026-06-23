use std::path::Path;

#[test]
fn spatial_public_boundary_rejects_internal_constructor_bypass() {
    let compile_fail_root = Path::new("src/certification/public_facade_contracts/compile_fail");
    let fixtures = compile_fail_fixture_paths(compile_fail_root);

    assert!(
        !fixtures.is_empty(),
        "public-boundary compile-fail suite must discover fixtures"
    );

    let test_cases = trybuild::TestCases::new();
    for fixture in fixtures {
        test_cases.compile_fail(fixture);
    }
}

fn compile_fail_fixture_paths(compile_fail_root: &Path) -> Vec<String> {
    let mut fixtures = Vec::new();
    collect_compile_fail_fixtures(compile_fail_root, &mut fixtures);
    fixtures.sort();
    fixtures
}

fn collect_compile_fail_fixtures(directory: &Path, fixtures: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).expect("compile-fail fixture directory must exist") {
        let path = entry
            .expect("compile-fail fixture entry must be readable")
            .path();
        if path.is_dir() {
            collect_compile_fail_fixtures(&path, fixtures);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            fixtures.push(path.to_string_lossy().replace('\\', "/"));
        }
    }
}
