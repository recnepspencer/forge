#[test]
fn phase_five_lsm_invariants_cover_replay_lookup_tombstones_and_compaction() {
    use crate::strategy::tests_support::admit_lsm_wal_strategy;
    use crate::S8LsmLookupDisposition;
    use forge_store_wal::layout_access::baseline_lsm_invariant_proof::{
        prove_baseline_lsm_invariants, prove_baseline_lsm_older_run_lookup,
        prove_baseline_lsm_tombstone_blocked_lookup,
    };

    let strategy = admit_lsm_wal_strategy();
    let suite = strategy.invariant_suite().require_lsm_suite().unwrap();
    let execution = executed_wal_lsm();
    let proof = prove_baseline_lsm_invariants(&execution);

    assert_eq!(
        suite.verify_lookup_proof(proof.lookup()).unwrap(),
        S8LsmLookupDisposition::NewestRun
    );
    assert_eq!(
        suite
            .verify_lookup_proof(prove_baseline_lsm_older_run_lookup(&execution))
            .unwrap(),
        S8LsmLookupDisposition::OlderRun
    );
    assert_eq!(
        suite
            .verify_lookup_proof(prove_baseline_lsm_tombstone_blocked_lookup(&execution))
            .unwrap(),
        S8LsmLookupDisposition::NotFound
    );
    suite.verify_publication_proof(proof.publication()).unwrap();
    suite.verify_recovery_proof(proof.recovery()).unwrap();
    suite
        .verify_owner_mutation_and_compaction(execution.compaction_publication_receipt())
        .unwrap();
}

fn executed_wal_lsm(
) -> forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmExecutionWitness {
    use forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmPhysicalPublicationBinding;
    forge_store_wal::layout_access::execute_baseline_lsm_persisted_fixture(
        BaselineLsmPhysicalPublicationBinding::new(2, 2, 2).unwrap(),
    )
}
