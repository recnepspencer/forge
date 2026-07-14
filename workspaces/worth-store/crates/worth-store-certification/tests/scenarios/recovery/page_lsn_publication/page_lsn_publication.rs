#[path = "adversarial_publication_denials.rs"]
mod adversarial_publication_denials;
use worth_store_test_support::harness::recovery::dirty_publication as dirty_publication_paths;
#[path = "page_generation_paths.rs"]
mod page_generation_paths;
use worth_store_test_support::harness::recovery::wal_durability as wal_durability_paths;

use worth_store_physical_backend::PosixFileFsyncDirFsyncProfile;
use worth_store_recovery_physics::{
    DirtyPublicationEvidence, NoUndoPublicationProof, PageFlushRecoveryReceipt, PageLsn,
    RollbackImagePublicationDeclaration, WalBeforeDataOrderingProof,
};

use adversarial_publication_denials::{
    assert_no_undo_rollback_mismatch_denial, assert_no_undo_rollback_required_denial,
    assert_page_flush_before_wal_denial,
};
use dirty_publication_paths::scheduled_dirty_publication;
use page_generation_paths::page_generation;
use wal_durability_paths::{completed_posix_receipt, completed_posix_receipt_for_range};

#[test]
fn public_wal_before_data_denies_page_lsn_not_covered_by_durable_wal_range() {
    let ack = worth_store_recovery_physics::DurableAckReceipt::acknowledge(
        worth_store_recovery_physics::AcknowledgmentPrecondition::from_append_receipt(
            completed_posix_receipt(),
        )
        .unwrap(),
    );
    let too_new = DirtyPublicationEvidence::from_physical_substrate_publication(
        scheduled_dirty_publication(b"too-new"),
        PageLsn::from_lsn(worth_store_recovery_physics::LogSequenceNumber::new(999)),
    );
    assert_page_flush_before_wal_denial(
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(too_new, &ack),
    );

    let unrelated_later_ack = worth_store_recovery_physics::DurableAckReceipt::acknowledge(
        worth_store_recovery_physics::AcknowledgmentPrecondition::from_append_receipt(
            completed_posix_receipt_for_range(200, 201),
        )
        .unwrap(),
    );
    let old_page_lsn = DirtyPublicationEvidence::from_physical_substrate_publication(
        scheduled_dirty_publication(b"old-lsn-not-in-ack-range"),
        PageLsn::from_lsn(worth_store_recovery_physics::LogSequenceNumber::new(100)),
    );
    assert_page_flush_before_wal_denial(
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(
            old_page_lsn,
            &unrelated_later_ack,
        ),
    );
}

#[test]
fn public_no_undo_surface_denies_missing_required_rollback_image() {
    let ack = worth_store_recovery_physics::DurableAckReceipt::acknowledge(
        worth_store_recovery_physics::AcknowledgmentPrecondition::from_append_receipt(
            completed_posix_receipt(),
        )
        .unwrap(),
    );
    let dirty = DirtyPublicationEvidence::from_physical_substrate_publication(
        scheduled_dirty_publication(b"needs-rollback-image"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
    );
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty, &ack).unwrap();

    assert_no_undo_rollback_required_denial(
        NoUndoPublicationProof::deny_missing_required_rollback_image(ordering),
    );
}

#[test]
fn public_rollback_protected_publication_requires_matching_declaration() {
    let ack = worth_store_recovery_physics::DurableAckReceipt::acknowledge(
        worth_store_recovery_physics::AcknowledgmentPrecondition::from_append_receipt(
            completed_posix_receipt(),
        )
        .unwrap(),
    );
    let dirty = DirtyPublicationEvidence::from_physical_substrate_publication(
        scheduled_dirty_publication(b"rollback-declaration"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
    );
    let declaration = RollbackImagePublicationDeclaration::declare(
        dirty.dirty_identity(),
        dirty.page_generation(),
        dirty.page_lsn(),
        "rollback-declaration-digest",
    );
    let mismatched_declaration = RollbackImagePublicationDeclaration::declare(
        dirty.dirty_identity(),
        page_generation(9, 2),
        dirty.page_lsn(),
        "wrong-page-rollback-declaration",
    );
    let matching_ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty.clone(), &ack)
            .unwrap();
    let mismatched_ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty, &ack).unwrap();

    let receipt =
        PageFlushRecoveryReceipt::publish_rollback_image_protected(matching_ordering, declaration)
            .unwrap();
    assert_eq!(
        receipt.page_lsn(),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start())
    );

    assert_no_undo_rollback_mismatch_denial(
        PageFlushRecoveryReceipt::publish_rollback_image_protected(
            mismatched_ordering,
            mismatched_declaration,
        ),
    );
}

#[test]
fn public_page_flush_publication_requires_wal_before_data_ordering_proof() {
    let ack = worth_store_recovery_physics::DurableAckReceipt::acknowledge(
        worth_store_recovery_physics::AcknowledgmentPrecondition::from_append_receipt(
            completed_posix_receipt(),
        )
        .unwrap(),
    );
    let dirty = DirtyPublicationEvidence::from_physical_substrate_publication(
        scheduled_dirty_publication(b"public-page-flush-proof"),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
    );
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty, &ack).unwrap();

    let receipt = PageFlushRecoveryReceipt::publish_admitted_redo_only(ordering);

    assert_eq!(
        receipt.page_lsn(),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start())
    );
    assert!(receipt.counters().page_flush_receipt_count() > 0);
}
