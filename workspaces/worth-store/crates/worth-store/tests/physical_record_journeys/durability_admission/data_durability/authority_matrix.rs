use super::super::super::{configuration, serving_from_initialization, serving_from_open};
use super::mutation_world::{append, synchronize};
use super::oracle::artifact_path;
use worth_store::physical_runtime::{
    PhysicalDataDispatchFailureCause, PhysicalDataDispatchOutcome, PhysicalDataSettlementOutcome,
    PhysicalMutationIdempotencyMaterial, RecordAppendBatch,
};

#[test]
fn foreign_store_rejection_preserves_durable_authority_for_its_exact_owner() {
    let parent = tempfile::tempdir().unwrap();
    let owner_root = parent.path().join("owner");
    let foreign_root = parent.path().join("foreign");
    let owner = serving_from_initialization(&owner_root);
    let foreign = serving_from_initialization(&foreign_root);
    let (_, placement, _) = configuration();
    let owner_submission = owner.record_submission();
    let foreign_submission = foreign.record_submission();
    let appended = append(
        &owner_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([121; 32]),
        RecordAppendBatch::try_from_iter([b"foreign-dispatch-authority".as_slice()]).unwrap(),
    );
    let target = appended.reserved().redo().records()[0].targets()[0]
        .target()
        .coordinate()
        .artifact();
    let identity = appended.mutation_identity();
    let durable = synchronize(&owner_submission, appended);

    let preserved = match foreign_submission.dispatch_wal_durable_data(durable) {
        PhysicalDataDispatchOutcome::NotStarted {
            durable,
            cause: PhysicalDataDispatchFailureCause::ForeignStore,
        } => durable,
        _ => panic!("a foreign Store must reject before any data effect"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert!(!artifact_path(&foreign_root, target).exists());

    let dispatched = match owner_submission.dispatch_wal_durable_data(preserved) {
        PhysicalDataDispatchOutcome::Dispatched(dispatched) => dispatched,
        _ => panic!("the exact owner must retain usable durable authority"),
    };
    assert!(artifact_path(&owner_root, target).exists());
    assert!(matches!(
        dispatched.settle_exact_effects(),
        PhysicalDataSettlementOutcome::Settled(_)
    ));
    owner.close();
    foreign.close();
}

#[test]
fn reopened_runtime_rejects_stale_durable_authority_before_data_effect() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("store");
    let initial = serving_from_initialization(&store_root);
    let (_, placement, _) = configuration();
    let initial_submission = initial.record_submission();
    let appended = append(
        &initial_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([122; 32]),
        RecordAppendBatch::try_from_iter([b"stale-dispatch-authority".as_slice()]).unwrap(),
    );
    let target = appended.reserved().redo().records()[0].targets()[0]
        .target()
        .coordinate()
        .artifact();
    let identity = appended.mutation_identity();
    let durable = synchronize(&initial_submission, appended);
    initial.close();

    let reopened = serving_from_open(&store_root);
    assert_eq!(identity.store_identity(), reopened.store_identity());
    assert_ne!(identity.runtime_identity(), reopened.runtime_identity());
    let preserved = match reopened
        .record_submission()
        .dispatch_wal_durable_data(durable)
    {
        PhysicalDataDispatchOutcome::NotStarted {
            durable,
            cause: PhysicalDataDispatchFailureCause::StaleRuntime,
        } => durable,
        _ => panic!("a reopened runtime must reject stale data authority before effect"),
    };
    assert_eq!(preserved.mutation_identity(), identity);
    assert!(!artifact_path(&store_root, target).exists());
    reopened.close();
}
