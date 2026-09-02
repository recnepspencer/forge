use std::sync::{mpsc, Arc, Barrier};
use std::thread;
use std::time::Duration;

use super::reference_test_fixture as fixture;
use super::{
    ProductBranchReferenceCell, ProductBranchReferenceCellAdmissionDenial,
    ProductBranchReferenceCellDenial, ProductBranchReferenceSnapshot,
};
use crate::branch::observation::ProductBranchObservation;
use crate::history::CompositeHistoryCatalog;
use crate::identity::ProductBranchReferenceGeneration;

fn product_cell(
    snapshot: ProductBranchReferenceSnapshot,
    catalog: &CompositeHistoryCatalog,
) -> ProductBranchReferenceCell {
    let protection = catalog
        .protect_product_head(snapshot.commit())
        .expect("installed product-head protection");
    ProductBranchReferenceCell::new(snapshot, protection).expect("protected cell admission")
}

fn observation(
    cell: &ProductBranchReferenceCell,
    fixture: &fixture::RealReferenceFixture,
    catalog: &CompositeHistoryCatalog,
) -> ProductBranchObservation {
    cell.observe(catalog, &fixture.owner)
        .expect("real managed observation")
}

#[test]
fn construction_requires_exact_product_head_proof_and_returns_it_on_denial() {
    let (mut fixture, catalog, root) = fixture::installed_root();
    let snapshot = fixture::initial_snapshot(&mut fixture, Arc::clone(&root));
    let other = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let before = catalog.counters();
    let wrong_protection = catalog
        .protect_product_head(other.as_ref())
        .expect("other commit is installed");
    let wrong = ProductBranchReferenceCell::new(snapshot.clone(), wrong_protection)
        .expect_err("a proof for another commit cannot admit the cell");
    assert_eq!(
        wrong.denial(),
        ProductBranchReferenceCellAdmissionDenial::ProductHeadCommitMismatch
    );
    drop(wrong.into_product_head_history());
    assert_eq!(
        catalog.counters().direct_protection_releases(),
        before.direct_protection_releases() + 1
    );

    let (mut foreign_fixture, foreign_catalog, foreign_root) = fixture::installed_root();
    let foreign_snapshot =
        fixture::initial_snapshot(&mut foreign_fixture, Arc::clone(&foreign_root));
    let foreign_protection = foreign_catalog
        .protect_product_head(foreign_snapshot.commit())
        .expect("foreign commit is installed in its catalog");
    let foreign = ProductBranchReferenceCell::new(snapshot.clone(), foreign_protection)
        .expect_err("a foreign proof cannot admit the cell");
    assert_eq!(
        foreign.denial(),
        ProductBranchReferenceCellAdmissionDenial::ProductHeadOwnerMismatch
    );
    drop(foreign.into_product_head_history());

    let cell = product_cell(snapshot, &catalog);
    assert_eq!(cell.atomic_snapshot().commit().identity(), root.identity());
}

#[test]
fn stale_expected_head_precedes_successor_validation_and_returns_proof() {
    let (mut fixture, catalog, root) = fixture::installed_root();
    let initial = fixture::initial_snapshot(&mut fixture, Arc::clone(&root));
    let cell = product_cell(initial.clone(), &catalog);
    let expected = observation(&cell, &fixture, &catalog);
    let first = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let first_snapshot = fixture::successor_snapshot(&initial, Arc::clone(&first));
    cell.compare_and_publish(
        &expected,
        first_snapshot.clone(),
        catalog
            .protect_product_head(first.as_ref())
            .expect("first product-head protection"),
    )
    .expect("first movement");

    let malformed_commit = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let malformed = fixture::initial_snapshot(&mut fixture, Arc::clone(&malformed_commit));
    let before_stale_failure = catalog.counters();
    let stale = cell
        .compare_and_publish(
            &expected,
            malformed,
            catalog
                .protect_product_head(malformed_commit.as_ref())
                .expect("malformed candidate is installed"),
        )
        .expect_err("stale expected head wins before malformed successor");
    assert!(matches!(
        stale.denial(),
        ProductBranchReferenceCellDenial::ExpectedHeadMismatch(mismatch)
            if mismatch
                .axes()
                .contains(&crate::branch::observation::ProductBranchObservationMismatchAxis::SelectedCompositeCommit)
    ));
    drop(stale.into_successor_history());
    assert_eq!(
        catalog.counters().direct_protection_releases(),
        before_stale_failure.direct_protection_releases() + 1
    );
    assert_eq!(cell.atomic_snapshot(), first_snapshot);

    let current = observation(&cell, &fixture, &catalog);
    let malformed_commit = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let malformed = fixture::initial_snapshot(&mut fixture, Arc::clone(&malformed_commit));
    let before_successor_failure = catalog.counters();
    let invalid = cell
        .compare_and_publish(
            &current,
            malformed,
            catalog
                .protect_product_head(malformed_commit.as_ref())
                .expect("invalid candidate is installed"),
        )
        .expect_err("successor branch mismatch is denied");
    assert!(matches!(
        invalid.denial(),
        ProductBranchReferenceCellDenial::SuccessorBranchMismatch
    ));
    drop(invalid.into_successor_history());
    assert_eq!(
        catalog.counters().direct_protection_releases(),
        before_successor_failure.direct_protection_releases() + 1
    );
}

#[test]
fn real_same_head_contention_has_one_winner_and_no_torn_image() {
    let (mut fixture, catalog, root) = fixture::installed_root();
    let initial = fixture::initial_snapshot(&mut fixture, Arc::clone(&root));
    let cell = product_cell(initial.clone(), &catalog);
    let expected = Arc::new(observation(&cell, &fixture, &catalog));
    let first = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let second = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let first_snapshot = fixture::successor_snapshot(&initial, Arc::clone(&first));
    let second_snapshot = fixture::successor_snapshot(&initial, Arc::clone(&second));
    let first_protection = catalog
        .protect_product_head(first.as_ref())
        .expect("first contention protection");
    let second_protection = catalog
        .protect_product_head(second.as_ref())
        .expect("second contention protection");
    let start = Arc::new(Barrier::new(3));
    let (results, receive) = mpsc::channel();

    let first_cell = cell.clone();
    let first_expected = Arc::clone(&expected);
    let first_start = Arc::clone(&start);
    let first_results = results.clone();
    let first_worker = thread::spawn(move || {
        first_start.wait();
        first_results
            .send(
                first_cell
                    .compare_and_publish(first_expected.as_ref(), first_snapshot, first_protection)
                    .is_ok(),
            )
            .expect("first contention result");
    });
    let second_cell = cell.clone();
    let second_expected = Arc::clone(&expected);
    let second_start = Arc::clone(&start);
    let second_worker = thread::spawn(move || {
        second_start.wait();
        results
            .send(
                second_cell
                    .compare_and_publish(
                        second_expected.as_ref(),
                        second_snapshot,
                        second_protection,
                    )
                    .is_ok(),
            )
            .expect("second contention result");
    });
    start.wait();
    let first_won = receive
        .recv_timeout(Duration::from_secs(1))
        .expect("first contention worker completes");
    let second_won = receive
        .recv_timeout(Duration::from_secs(1))
        .expect("second contention worker completes");
    first_worker.join().expect("first contention worker");
    second_worker.join().expect("second contention worker");
    assert_ne!(first_won, second_won);

    let selected = cell.atomic_snapshot();
    assert_eq!(selected.generation().get(), 1);
    assert!(
        selected.commit().identity() == first.identity()
            || selected.commit().identity() == second.identity()
    );
    assert!(crate::basis::compare_exact(selected.commit().basis(), &fixture.basis).is_ok());
    assert_eq!(
        selected.generation(),
        ProductBranchReferenceGeneration::initial()
            .advance()
            .unwrap()
    );
}

#[test]
fn unrelated_product_reference_progress_does_not_wait() {
    let (mut fixture, catalog, root) = fixture::installed_root();
    let first_snapshot = fixture::initial_snapshot(&mut fixture, Arc::clone(&root));
    let second_snapshot = fixture::initial_snapshot(&mut fixture, Arc::clone(&root));
    let first = product_cell(first_snapshot, &catalog);
    let second = product_cell(second_snapshot.clone(), &catalog);
    let expected = observation(&second, &fixture, &catalog);
    let successor = fixture::install_ordinary(&mut fixture, &catalog, root.as_ref());
    let successor_snapshot = fixture::successor_snapshot(&second_snapshot, Arc::clone(&successor));
    let successor_protection = catalog
        .protect_product_head(successor.as_ref())
        .expect("unrelated successor protection");
    let held = first.hold_for_test();
    let (completed, receive) = mpsc::channel();
    let worker = thread::spawn(move || {
        completed
            .send(
                second
                    .compare_and_publish(&expected, successor_snapshot, successor_protection)
                    .is_ok(),
            )
            .expect("unrelated completion result");
    });
    assert!(receive
        .recv_timeout(Duration::from_secs(1))
        .expect("unrelated cell progresses while first is locked"));
    drop(held);
    worker.join().expect("unrelated worker");
}
