use forge_foundational::Milestone2DigestReadinessNote;

#[test]
fn milestone_2_readiness_note_names_deferred_digest_work() {
    let note = Milestone2DigestReadinessNote::new();

    assert_eq!(
        note.owns(),
        "canonical semantic ordering and equality basis"
    );
    assert!(note.deferred().contains("final digest algorithms"));
}
