use std::sync::Arc;

use super::unpublished_from_cas_loss;
use crate::branch::reference_test_fixture as fixture;
use crate::branch::{ProductBranchReferenceCell, ProductBranchReferenceSnapshot};
use crate::history::CompositeHistoryReclamationRequest;
use crate::publication::{
    CompositeAttemptProgress, RelationalAttemptProgress, SignalAttemptProgress,
};
use crate::recovery::ProductUnpublishedCause;
use crate::retention::ComponentBasisDependencyClass;

fn product_cell(
    snapshot: ProductBranchReferenceSnapshot,
    catalog: &crate::history::CompositeHistoryCatalog,
    fixture: &fixture::RealReferenceFixture,
) -> ProductBranchReferenceCell {
    ProductBranchReferenceCell::new(fixture::product_head_protection(fixture, catalog, snapshot))
        .expect("protected product cell")
}

#[test]
fn real_cas_loss_retains_exact_winner_owner_progress_and_successor_custody() {
    let (mut fixture, catalog, root) = fixture::installed_root();
    let initial = fixture::initial_snapshot(&mut fixture, Arc::clone(&root));
    let cell = product_cell(initial.clone(), &catalog, &fixture);
    let expected = cell
        .observe(&catalog, fixture.retention_owner())
        .expect("real managed expected observation");
    let expected_guard = expected.clone();

    let winner = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let winner_snapshot = fixture::successor_snapshot(&initial, Arc::clone(&winner));
    cell.compare_and_publish(
        &expected,
        fixture::product_head_protection(&fixture, &catalog, winner_snapshot.clone()),
    )
    .expect("winner crosses the real branch-cell CAS");

    let loser = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let loser_snapshot = fixture::successor_snapshot(&initial, Arc::clone(&loser));
    let failure = cell
        .compare_and_publish(
            &expected,
            fixture::product_head_protection(&fixture, &catalog, loser_snapshot),
        )
        .expect_err("the stale contender loses the real branch-cell CAS");

    let performed = fixture.perform_relational_owner_change();
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::performed(performed),
        SignalAttemptProgress::untouched(),
    );
    let attempt_identity = fixture.next_publication_attempt();
    let unpublished_identity = fixture.next_product_unpublished();
    let owner_identity = fixture.owner_identity();
    let recovery = fixture::recovery_catalog(owner_identity);
    let recovery_slot = recovery
        .reserve_product_unpublished(owner_identity)
        .expect("bounded recovery slot");
    let retention_before = fixture.retention_owner().cost_snapshot();
    let active_before = fixture
        .retention_owner()
        .active_component_obligation_count();
    let history_before = catalog.counters();

    let effects = unpublished_from_cas_loss(
        failure,
        attempt_identity,
        unpublished_identity,
        expected,
        progress,
        Arc::clone(&loser),
        recovery_slot,
        None,
    )
    .expect("real CAS loss becomes retained recovery");

    assert_eq!(
        effects.cause(),
        ProductUnpublishedCause::ProductPublicationLost
    );
    assert_eq!(effects.expected_head().selected_commit(), root.identity());
    let observed = effects
        .last_observed_head()
        .expect("CAS loss carries its exact observed winner");
    assert_eq!(observed, &winner_snapshot);
    assert_eq!(observed.selected_commit(), winner.identity());
    assert_eq!(effects.successor_basis(), Some(loser.basis()));
    assert_eq!(effects.successor_commit(), loser.identity());
    assert_eq!(effects.owner_effect_count(), 1);
    assert_eq!(effects.live_obligation_count(), 3);
    assert_eq!(
        effects.retention_obligation().relational().dependency(),
        ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
    );
    assert_eq!(
        effects.retention_obligation().signal().dependency(),
        ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
    );
    assert_eq!(
        effects.successor_history_protection().commit_identity(),
        loser.identity()
    );
    assert_eq!(
        fixture
            .retention_owner()
            .active_component_obligation_count(),
        active_before
    );
    let retention_after = fixture.retention_owner().cost_snapshot();
    assert_eq!(
        retention_after.owner_acquisition_contacts(),
        retention_before.owner_acquisition_contacts(),
        "custody transition must not contact either component owner"
    );
    assert_eq!(
        catalog.counters().direct_protection_acquisitions(),
        history_before.direct_protection_acquisitions(),
        "History custody is reclassified, not reacquired"
    );

    let blocked = catalog
        .reclaim_batch(CompositeHistoryReclamationRequest::new(
            owner_identity,
            vec![loser.identity().clone()],
            1,
            1,
        ))
        .expect("live recovery history custody blocks reclamation");
    assert_eq!(blocked.skipped_protected(), 1);

    let releases_before_drop = catalog.counters().direct_protection_releases();
    drop(effects);
    assert_eq!(
        fixture
            .retention_owner()
            .active_component_obligation_count(),
        active_before - 2,
        "dropping recovery releases exactly its two retained component claims"
    );
    assert_eq!(
        catalog.counters().direct_protection_releases(),
        releases_before_drop + 1,
        "dropping recovery releases exactly its successor History protection"
    );
    let reclaimed = catalog
        .reclaim_batch(CompositeHistoryReclamationRequest::new(
            owner_identity,
            vec![loser.identity().clone()],
            1,
            1,
        ))
        .expect("released recovery custody permits exact loser reclamation");
    assert_eq!(reclaimed.reclaimed_commits(), &[loser.identity().clone()]);

    drop(expected_guard);
}
