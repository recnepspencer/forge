#[test]
fn btree_invariants_cover_lookup_mutation_integrity_and_recovery() {
    use crate::strategy::tests_support::admit_btree_page_strategy;
    use crate::{BTreeCorruptionRegion, BTreeLookupBranch};

    let strategy = admit_btree_page_strategy();
    let suite = strategy.invariant_suite().require_btree_suite().unwrap();

    assert_eq!(
        suite.verify_declared_baseline_lookup().unwrap(),
        BTreeLookupBranch::Left
    );
    assert_eq!(
        suite
            .verify_declared_baseline_mutation_and_integrity()
            .unwrap(),
        BTreeCorruptionRegion::Header
    );
    suite.verify_declared_baseline_publication().unwrap();
    suite.verify_declared_baseline_recovery().unwrap();
}

#[test]
fn btree_separator_corruption_is_denied() {
    use crate::strategy::tests_support::{admit_btree_page_strategy, admitted_page_key_bytes};
    use crate::{BTreeLookupBranch, StrategyDenial};

    let suite = admit_btree_page_strategy()
        .invariant_suite()
        .require_btree_suite()
        .unwrap();
    assert_eq!(
        suite.search_path_law().verify_search_and_insertion_path(
            &admitted_page_key_bytes(1, 5),
            &admitted_page_key_bytes(1, 30),
            &admitted_page_key_bytes(1, 20),
            &admitted_page_key_bytes(1, 30),
            BTreeLookupBranch::Left,
        ),
        Err(StrategyDenial::ComparatorOrderViolation)
    );
}
