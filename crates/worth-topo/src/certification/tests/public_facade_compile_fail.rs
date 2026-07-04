#[test]
fn topology_compiled_product_family_public_boundary_compile_fail_fixtures_execute() {
    use std::collections::BTreeSet;

    let test_cases = trybuild::TestCases::new();
    let fences =
        crate::certification::public_facade_contracts::phase_fifteen_topology_compile_fail_fences();
    let unique_fixture_paths = fences
        .iter()
        .map(|fence| fence.fixture_path())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unique_fixture_paths.len(),
        fences.len(),
        "each certified topology fence class must map to its own executed compile-fail fixture"
    );
    let mut executed_fixture_count = 0usize;
    for fixture_path in &unique_fixture_paths {
        test_cases.compile_fail(fixture_path);
        executed_fixture_count += 1;
    }
    assert_eq!(executed_fixture_count, unique_fixture_paths.len());
}
