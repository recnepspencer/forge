use super::support::{completed_loop_replay_closeout_chain, run_with_large_stack};

#[test]
fn loop_reconstruction_phase_ten_contract_preserves_real_reconstructed_loop_truth() {
    run_with_large_stack(|| {
        let chain =
            completed_loop_replay_closeout_chain("phase7.4 public reconstructed loop contract");
        let original = chain
            .original
            .products()
            .expect("original loop handoff should retain canonical loop products");
        let replayed = chain
            .replayed
            .products()
            .expect("replayed loop handoff should retain canonical loop products");

        assert!(!original
            .candidate_boundary()
            .loop_candidates()
            .rows()
            .is_empty());
        assert!(!replayed
            .candidate_boundary()
            .loop_candidates()
            .rows()
            .is_empty());
        assert_eq!(
            original
                .reconstructed_boundary()
                .reconstructed_loops()
                .rows(),
            replayed
                .reconstructed_boundary()
                .reconstructed_loops()
                .rows()
        );
        assert_eq!(
            original.reconstructed_boundary().born_loops().rows(),
            replayed.reconstructed_boundary().born_loops().rows()
        );

        let replay = &chain.replay_parity;
        assert!(!replay.replay_identity().is_empty());
        assert!(!replay.checkpoint_receipt().checkpoint_identity().is_empty());
        assert_eq!(replay.rows().len(), 11);
        assert_eq!(replay.counters().compared_loop_evidence_receipts_count(), 1);
    });
}
