use forge_query::facade::consumer_kit::hard_prohibition_compile_fail_fixtures;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const REGISTRY_ROW_CONSTRUCTOR_FIXTURE: &str =
    "tests/ui/prohibition_registry/registry_row_constructor_private.rs";

#[test]
fn hard_prohibition_seams_are_not_consumer_reachable() {
    let t = trybuild::TestCases::new();
    for fixture in hard_prohibition_compile_fail_fixtures() {
        t.compile_fail(fixture.fixture_path());
    }
    t.compile_fail(REGISTRY_ROW_CONSTRUCTOR_FIXTURE);
}

#[test]
fn prohibition_registry_compile_fail_directory_has_no_unregistered_fixture() {
    let expected = hard_prohibition_compile_fail_fixtures()
        .iter()
        .map(|fixture| fixture.fixture_path().to_string())
        .chain([REGISTRY_ROW_CONSTRUCTOR_FIXTURE.to_string()])
        .collect::<BTreeSet<_>>();
    let found = fs::read_dir("tests/ui/prohibition_registry")
        .expect("prohibition registry fixture directory should exist")
        .filter_map(|entry| {
            let entry = entry.expect("fixture directory entry should be readable");
            (entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str())
                == Some("rs"))
            .then(|| normalize_path(&entry.path()))
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(found, expected);
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
