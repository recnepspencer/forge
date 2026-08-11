use bank_server::{BankCommitReceipt, BankMutationOutcome, BankMutationStatus};
use worth_query_host::facade::publication::application_aftermath::WorthQueryPublishedApplicationCommitKind;
use worth_query_host::facade::publication::domain_computation::WorthQueryPublishedApplicationCommitAttemptReleasePosture;

pub(super) fn assert_committed(outcome: BankMutationOutcome) {
    let BankMutationStatus::Committed(receipt) = outcome.status() else {
        panic!("unexpected mutation outcome: {outcome:?}");
    };
    assert_fresh_publication(receipt);
    assert!(outcome.metadata().projection_work().is_some());
}

pub(super) fn assert_emitting_commit(outcome: BankMutationOutcome) {
    let BankMutationStatus::Committed(receipt) = outcome.status() else {
        panic!("unexpected mutation outcome: {outcome:?}");
    };
    assert!(receipt.emitted_effect_count() > 0);
    assert_fresh_publication(receipt);
    assert!(outcome.metadata().provider_work_units() > 0);
}

pub(super) fn assert_fresh_publication(receipt: &BankCommitReceipt) {
    let publication = receipt.publication().inspect();
    assert_eq!(
        publication.kind(),
        WorthQueryPublishedApplicationCommitKind::Executed
    );
    let work = publication
        .mutation_work()
        .expect("a fresh commit publication retains actual mutation work");
    assert!(work.decision_fact_count() > 0);
    assert!(work.proposed_fact_count() > 0);
    assert!(work.relational_invariant_execution_count() > 0);
    assert_eq!(
        publication.attempt_release(),
        WorthQueryPublishedApplicationCommitAttemptReleasePosture::Released
    );
    assert_eq!(publication.publication_canonical_entries(), 0);
    assert_eq!(publication.publication_sha256_compression_blocks(), 0);
    assert_eq!(publication.publication_identity_text_materializations(), 0);
}
