use super::super::super::{configuration, serving_from_initialization, serving_from_open};
use super::mutation_world::{append, synchronize};
use super::oracle::artifact_path;
use worth_store::physical_runtime::{
    PhysicalDataDispatchFailureCause, PhysicalDataDispatchOutcome, PhysicalDataSettlementOutcome,
    PhysicalMutationIdempotencyMaterial, PhysicalWalGroupBarrierDeclarationDenial,
    PhysicalWalGroupBarrierFailureCause, PhysicalWalGroupBarrierOutcome, RecordAppendBatch,
};
use worth_store_physical_backend::MediaOperationRole;

#[test]
fn foreign_store_cannot_synchronize_an_appended_member_or_spend_its_authority() {
    let parent = tempfile::tempdir().unwrap();
    let owner_root = parent.path().join("barrier-owner");
    let foreign_root = parent.path().join("barrier-foreign");
    let owner = serving_from_initialization(&owner_root);
    let foreign = serving_from_initialization(&foreign_root);
    let (_, placement, _) = configuration();
    let owner_submission = owner.certification_record_submission();
    let appended = append(
        &owner_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([119; 32]),
        RecordAppendBatch::try_from_iter([b"foreign-barrier-authority".as_slice()]).unwrap(),
    );
    let identity = appended.members()[0].mutation().mutation_identity();
    let before = foreign
        .media_counters()
        .identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState);

    let preserved = match foreign
        .certification_record_submission()
        .synchronize_appended_wal_group(appended)
    {
        PhysicalWalGroupBarrierOutcome::BarrierNotStarted {
            appended,
            cause:
                PhysicalWalGroupBarrierFailureCause::Declaration(
                    PhysicalWalGroupBarrierDeclarationDenial::PolicyOrRuntimeMismatch,
                ),
        } => appended,
        _ => panic!("a foreign Store must reject before synchronization"),
    };
    assert_eq!(
        preserved.members()[0].mutation().mutation_identity(),
        identity
    );
    assert_eq!(
        foreign
            .media_counters()
            .identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState),
        before
    );

    let durable = synchronize(&owner_submission, preserved);
    assert_eq!(durable.mutation_identity(), identity);
    owner.close();
    foreign.close();
}

#[test]
fn reopened_runtime_cannot_synchronize_a_stale_appended_member() {
    let parent = tempfile::tempdir().unwrap();
    let store_root = parent.path().join("barrier-stale");
    let initial = serving_from_initialization(&store_root);
    let (_, placement, _) = configuration();
    let appended = append(
        &initial.certification_record_submission(),
        placement,
        PhysicalMutationIdempotencyMaterial::new([120; 32]),
        RecordAppendBatch::try_from_iter([b"stale-barrier-authority".as_slice()]).unwrap(),
    );
    let identity = appended.members()[0].mutation().mutation_identity();
    initial.close();

    let reopened = serving_from_open(&store_root);
    let before = reopened
        .media_counters()
        .identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState);
    let preserved = match reopened
        .certification_record_submission()
        .synchronize_appended_wal_group(appended)
    {
        PhysicalWalGroupBarrierOutcome::BarrierNotStarted {
            appended,
            cause:
                PhysicalWalGroupBarrierFailureCause::Declaration(
                    PhysicalWalGroupBarrierDeclarationDenial::PolicyOrRuntimeMismatch,
                ),
        } => appended,
        _ => panic!("a reopened runtime must reject stale barrier authority"),
    };
    assert_eq!(
        preserved.members()[0].mutation().mutation_identity(),
        identity
    );
    assert_eq!(
        reopened
            .media_counters()
            .identified_operation_attempts_for(MediaOperationRole::SynchronizeFileState),
        before
    );
    reopened.close();
}

#[test]
fn foreign_store_rejection_preserves_durable_authority_for_its_exact_owner() {
    let parent = tempfile::tempdir().unwrap();
    let owner_root = parent.path().join("owner");
    let foreign_root = parent.path().join("foreign");
    let owner = serving_from_initialization(&owner_root);
    let foreign = serving_from_initialization(&foreign_root);
    let (_, placement, _) = configuration();
    let owner_submission = owner.certification_record_submission();
    let foreign_submission = foreign.certification_record_submission();
    let appended = append(
        &owner_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([121; 32]),
        RecordAppendBatch::try_from_iter([b"foreign-dispatch-authority".as_slice()]).unwrap(),
    );
    let appended_member = appended.members()[0].mutation();
    let target = appended_member.reserved().redo().records()[0].targets()[0]
        .target()
        .coordinate()
        .artifact();
    let identity = appended_member.mutation_identity();
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
    let initial_submission = initial.certification_record_submission();
    let appended = append(
        &initial_submission,
        placement,
        PhysicalMutationIdempotencyMaterial::new([122; 32]),
        RecordAppendBatch::try_from_iter([b"stale-dispatch-authority".as_slice()]).unwrap(),
    );
    let appended_member = appended.members()[0].mutation();
    let target = appended_member.reserved().redo().records()[0].targets()[0]
        .target()
        .coordinate()
        .artifact();
    let identity = appended_member.mutation_identity();
    let durable = synchronize(&initial_submission, appended);
    initial.close();

    let reopened = serving_from_open(&store_root);
    assert_eq!(identity.store_identity(), reopened.store_identity());
    assert_ne!(identity.runtime_identity(), reopened.runtime_identity());
    let preserved = match reopened
        .certification_record_submission()
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
