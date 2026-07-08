#[test]
fn phase_five_btree_invariants_cover_lookup_mutation_integrity_and_recovery() {
    use crate::strategy::tests_support::admit_btree_page_strategy;
    use crate::{S8BTreeCorruptionRegion, S8BTreeLookupBranch};

    let strategy = admit_btree_page_strategy();
    let suite = strategy.invariant_suite().require_btree_suite().unwrap();

    assert_eq!(
        suite.verify_baseline_lookup().unwrap(),
        S8BTreeLookupBranch::Left
    );
    assert_eq!(
        suite.verify_baseline_mutation_and_integrity().unwrap(),
        S8BTreeCorruptionRegion::Header
    );
    suite.verify_baseline_publication().unwrap();
    suite.verify_baseline_recovery().unwrap();
}
