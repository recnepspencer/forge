use super::edge_splitting_replay_parity_support::{
    assert_checkpoint_parity_is_retained_replay_backed,
    assert_foreign_retained_replay_receipt_is_rejected,
    assert_replay_parity_certifies_split_products, assert_reversed_source_sense_is_covered,
    build_edge_split_replay_parity_subject, replay_parity_report,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use super::reduced_pair_support;
use worth_kernel::workload_composition::WorkloadCatalog;

#[test]
fn edge_split_replay_preserves_split_ledger_digest_and_downstream_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 edge split replay ledger parity");
        let replay_subject = build_edge_split_replay_parity_subject(&subject);
        let report = replay_parity_report(&replay_subject);

        assert_replay_parity_certifies_split_products(&replay_subject, &report);
    });
}

#[test]
fn edge_split_reversed_source_edge_sense_preserves_canonical_fragment_set() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify(
            "phase7.3 edge split replay reversed source sense",
        );
        let replay_subject = build_edge_split_replay_parity_subject(&subject);
        let report = replay_parity_report(&replay_subject);

        assert_reversed_source_sense_is_covered(&replay_subject, &report);
    });
}

#[test]
fn edge_split_checkpoint_and_non_checkpoint_paths_preserve_split_decisions() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 edge split replay checkpoint parity");
        let replay_subject = build_edge_split_replay_parity_subject(&subject);
        let report = replay_parity_report(&replay_subject);

        assert_checkpoint_parity_is_retained_replay_backed(&replay_subject, &report);
    });
}

#[test]
fn edge_split_replay_rejects_foreign_retained_replay_receipt() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 edge split replay lineage denial");
        let replay_subject = build_edge_split_replay_parity_subject(&subject);
        let foreign_replay_receipts = WorkloadCatalog::planar_boolean_coplanar_overlap_pair()
            .build()
            .expect("foreign coplanar overlap pair should build")
            .left()
            .replay_receipts()
            .expect("foreign retained replay receipt should exist")
            .clone();

        assert_foreign_retained_replay_receipt_is_rejected(
            &replay_subject,
            &foreign_replay_receipts,
        );
    });
}
