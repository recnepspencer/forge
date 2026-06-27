use super::dependency_fixture_support::{
    dependency_basis_from_imported_modules, dependency_basis_from_reordered_modules,
};

#[test]
fn equivalent_artifacts_produce_equivalent_dependency_metadata() {
    let baseline = dependency_basis_from_imported_modules();
    let reordered = dependency_basis_from_reordered_modules();

    assert_eq!(baseline, reordered);
}
