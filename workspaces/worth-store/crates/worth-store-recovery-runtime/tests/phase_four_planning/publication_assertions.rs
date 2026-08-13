use worth_store_recovery_runtime::{PlannedPhysicalRecovery, RecoveryPublicationAction};

pub(super) fn assert_exact_publication_plan(planned: &PlannedPhysicalRecovery) {
    let publication = planned.publication_plan();
    assert!(!publication.candidates().is_empty());
    assert_eq!(
        publication.actions().len(),
        publication.candidates().len() * 2 + 2
    );
    for (candidate, actions) in publication
        .candidates()
        .iter()
        .zip(publication.actions().chunks_exact(2))
    {
        assert_eq!(
            actions,
            [
                RecoveryPublicationAction::MaterializeRootCandidate {
                    artifact: candidate.artifact(),
                },
                RecoveryPublicationAction::SynchronizeRootCandidate {
                    artifact: candidate.artifact(),
                },
            ]
        );
    }
    assert_eq!(
        &publication.actions()[publication.actions().len() - 2..],
        [
            RecoveryPublicationAction::ReplaceRootProtocol,
            RecoveryPublicationAction::SynchronizeStoreNamespace,
        ]
    );
}
