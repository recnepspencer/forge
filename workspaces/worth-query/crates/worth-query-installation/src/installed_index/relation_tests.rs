use super::{WorthQueryInstalledPackageIndex, WorthQueryInstalledPackageIndexRelation as Relation};
use crate::generation::{WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity};

fn empty_index() -> WorthQueryInstalledPackageIndex {
    WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        std::iter::empty(),
    )
    .unwrap()
}

#[test]
fn rebuild_retains_exact_runtime_generation_and_meaning() {
    let current = empty_index();
    let rebuilt = current.rebuild();

    assert_eq!(
        current.relation_to(&rebuilt),
        Relation::EquivalentGeneration
    );
}

#[test]
fn same_generation_meaning_drift_is_not_an_equivalent_rebuild() {
    let current = empty_index();
    let mut drifted = current.rebuild();
    drifted.identity.push_str("-corrupt");

    assert_eq!(
        current.relation_to(&drifted),
        Relation::SameGenerationMeaningChanged
    );
}

#[test]
fn successor_relation_requires_the_immediate_same_runtime_generation() {
    let current = empty_index();
    let successor = current.successor_generation();
    let skipped = successor.successor_generation();

    assert_eq!(current.relation_to(&successor), Relation::ExactSuccessor);
    assert_eq!(
        current.relation_to(&skipped),
        Relation::NonSuccessorGeneration
    );
    assert_eq!(
        successor.relation_to(&current),
        Relation::NonSuccessorGeneration
    );
}

#[test]
fn equivalent_foreign_index_has_no_installation_relation() {
    let current = empty_index();
    let foreign = empty_index();

    assert_eq!(current.relation_to(&foreign), Relation::ForeignRuntime);
}
