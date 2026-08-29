use crate::facade::history::BranchId;
use crate::facade::mvcc::{
    PreparedRelationalCommitCandidate, RelationalInterruptionBoundary, RelationalOperationControl,
    RelationalOperationInterruption, RelationalTransactionIntent,
};
use crate::tests::support::*;

/// The pending-settlement record is installed before the branch reference can
/// move, so a stale attempt must give back the exact reservation it installed.
#[test]
fn stale_publication_releases_its_pending_settlement_reservation() {
    let mut runtime = runtime_with_test_schema();
    create_entity(&mut runtime, "stale-reservation-anchor");
    let loser = prepared_main_write(&mut runtime, "stale-loser");
    let winner = prepared_main_write(&mut runtime, "stale-winner");

    let crate::mvcc::RelationalPublicationOutcome::Performed(performed) =
        runtime.publication_port().compare_and_publish(winner)
    else {
        panic!("the first publisher from a shared basis performs");
    };
    assert_eq!(
        runtime.publication_binding().pending_settlement_count(),
        1,
        "movement is reported only after its settlement record exists"
    );
    let settled = runtime
        .settle_performed_publication(performed)
        .expect("the winner settles");
    assert_eq!(runtime.publication_binding().pending_settlement_count(), 0);

    let reference_before = runtime
        .branch_reference_state(&BranchId("main".to_owned()))
        .unwrap();
    let contacts_before = runtime
        .publication_binding()
        .pending_settlement_contact_count();
    let crate::mvcc::RelationalPublicationOutcome::Stale(_) =
        runtime.publication_port().compare_and_publish(loser)
    else {
        panic!("the second publisher from the same basis is stale");
    };

    assert_eq!(
        runtime
            .branch_reference_state(&BranchId("main".to_owned()))
            .unwrap(),
        reference_before,
        "a stale attempt performs no movement"
    );
    assert_eq!(
        runtime.publication_binding().pending_settlement_count(),
        0,
        "a stale attempt releases the exact reservation it installed"
    );
    assert!(
        runtime
            .publication_binding()
            .pending_settlement_contact_count()
            > contacts_before,
        "the stale attempt did reach the registry before releasing"
    );
    release_test_commit_snapshot(&mut runtime, &settled);
}

/// Interruption between the pre-effect reservation and the linearization point
/// is a no-movement path, so it must not retain settlement state either.
#[test]
fn interrupted_publication_releases_its_pending_settlement_reservation() {
    for interruption in [
        RelationalOperationInterruption::Cancelled,
        RelationalOperationInterruption::TimedOut,
    ] {
        let mut runtime = runtime_with_test_schema();
        create_entity(&mut runtime, "interrupted-reservation-anchor");
        let identity = runtime.main_branch_identity();
        let (_, basis) = runtime.observe_branch(&identity).unwrap();
        let control = RelationalOperationControl::uninterrupted().with_injected_interruption(
            RelationalInterruptionBoundary::BeforeCriticalSection,
            interruption,
            1,
        );
        let mut transaction = runtime
            .begin_branch_transaction_with_control(
                &basis,
                RelationalTransactionIntent::ordinary(),
                control,
            )
            .unwrap();
        transaction
            .push_batch(batch_create("interrupted-before-critical-section"))
            .unwrap();
        let candidate = runtime.prepare_branch_transaction(transaction).unwrap();
        let reference_before = runtime
            .branch_reference_state(&BranchId("main".to_owned()))
            .unwrap();

        let crate::mvcc::RelationalPublicationOutcome::Interrupted(event) =
            runtime.publication_port().compare_and_publish(candidate)
        else {
            panic!("an injected critical-section interruption must not move the reference");
        };

        assert_eq!(event.interruption(), interruption);
        assert_eq!(
            event.boundary(),
            RelationalInterruptionBoundary::BeforeCriticalSection
        );
        assert_eq!(
            runtime
                .branch_reference_state(&BranchId("main".to_owned()))
                .unwrap(),
            reference_before,
        );
        assert_eq!(
            runtime.publication_binding().pending_settlement_count(),
            0,
            "an interrupted attempt leaks no pre-effect settlement reservation"
        );
    }
}

/// Published-snapshot capacity bounds the pending registry as well. Exhausting
/// it is a no-movement deferral, so no settlement state may survive it.
#[test]
fn published_snapshot_capacity_exhaustion_retains_no_pending_settlement() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .publication(PublicationConfig {
            coherent_publication_required: true,
            max_patch_records_per_commit: 4096,
            max_published_snapshot_handles: 2,
            max_active_snapshot_handles: 4_096,
            max_transaction_overlay_bytes: 1_048_576,
            max_transaction_footprint_loci: 1_024,
            max_transaction_savepoints: 8,
            max_prepared_candidates: 8,
            candidate_max_lifetime_millis: 30_000,
            max_prepared_root_bytes: 268_435_456,
        })
        .build();
    let first = create_entity_outcome(&mut runtime, "capacity-first");
    let second = create_entity_outcome(&mut runtime, "capacity-second");
    let before = test_owner_main_basis(&runtime).unwrap();

    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction
        .push_batch(batch_create("capacity-third"))
        .unwrap();
    assert!(matches!(
        transaction.commit(&mut runtime),
        Err(
            crate::transactions::data::TransactionCommitError::PublicationDeferred {
                deferred:
                    crate::mvcc::RelationalPublicationDeferred::PublishedSnapshotCapacityExhausted {
                        maximum_handles: 2,
                    },
                ..
            }
        )
    ));

    assert_eq!(
        runtime.publication_binding().pending_settlement_count(),
        0,
        "capacity exhaustion retains no settlement reservation"
    );
    assert_eq!(
        test_owner_main_basis(&runtime).unwrap().descriptor(),
        before.descriptor(),
        "capacity exhaustion performs no movement"
    );
    release_test_commit_snapshot(&mut runtime, &first);
    release_test_commit_snapshot(&mut runtime, &second);
}

fn prepared_main_write(
    runtime: &mut RelationalRuntime,
    name: &str,
) -> PreparedRelationalCommitCandidate {
    let basis = test_owner_main_basis(runtime).expect("main basis is admitted");
    let mut transaction = runtime
        .begin_branch_transaction(&basis, RelationalTransactionIntent::ordinary())
        .expect("transaction binds");
    transaction
        .push_batch(batch_create(name))
        .expect("test staging stays within configured resource budgets");
    runtime
        .prepare_branch_transaction(transaction)
        .expect("candidate prepares")
}
