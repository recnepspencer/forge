#[test]
fn phase_five_lsm_invariants_cover_replay_lookup_tombstones_and_compaction() {
    use crate::strategy::tests_support::admit_lsm_wal_strategy;
    use crate::S8LsmLookupDisposition;
    use forge_store_wal::layout_access::baseline_lsm_invariant_proof::{
        prove_baseline_lsm_older_run_lookup, prove_baseline_lsm_tombstone_blocked_lookup,
    };

    let strategy = admit_lsm_wal_strategy();
    let suite = strategy.invariant_suite().require_lsm_suite().unwrap();

    assert_eq!(
        suite.verify_baseline_lookup().unwrap(),
        S8LsmLookupDisposition::NewestRun
    );
    assert_eq!(
        suite
            .verify_lookup_proof(prove_baseline_lsm_older_run_lookup())
            .unwrap(),
        S8LsmLookupDisposition::OlderRun
    );
    assert_eq!(
        suite
            .verify_lookup_proof(prove_baseline_lsm_tombstone_blocked_lookup())
            .unwrap(),
        S8LsmLookupDisposition::NotFound
    );
    suite.verify_baseline_publication().unwrap();
    suite.verify_baseline_recovery().unwrap();
    suite.verify_baseline_mutation_and_compaction().unwrap();
}
