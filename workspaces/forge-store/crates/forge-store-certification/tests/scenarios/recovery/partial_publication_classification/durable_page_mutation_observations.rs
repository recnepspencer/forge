#[allow(dead_code)]
#[path = "../page_lsn_publication/dirty_publication_paths.rs"]
mod dirty_publication_paths;
#[allow(dead_code)]
#[path = "../page_lsn_publication/wal_durability_paths.rs"]
mod wal_durability_paths;

use forge_store_physical_backend::PosixFileFsyncDirFsyncProfile;
use forge_store_recovery_physics::{
    AcknowledgmentPrecondition, DirtyPublicationEvidence, DurableAckReceipt,
    NoUndoPublicationProof, PageFlushRecoveryReceipt, PageLsn, PartialPublicationObservationSet,
    RollbackImagePublicationDeclaration, UnadmittedDirtyPagePublicationDenial,
    WalBeforeDataOrderingProof,
};

use dirty_publication_paths::scheduled_dirty_publication;
use wal_durability_paths::completed_posix_receipt_for_range;

pub(crate) fn missing_rollback_image_observations() -> PartialPublicationObservationSet {
    PartialPublicationObservationSet::new()
        .with_unadmitted_durable_page_mutation(missing_rollback_image_denial())
}

pub(crate) fn rollback_image_protected_observations() -> PartialPublicationObservationSet {
    PartialPublicationObservationSet::new()
        .with_page_flush_recovery_receipt(rollback_image_protected_page_flush_receipt())
}

pub(crate) fn admitted_redo_only_observations() -> PartialPublicationObservationSet {
    PartialPublicationObservationSet::new()
        .with_page_flush_recovery_receipt(admitted_redo_only_page_flush_receipt())
}

fn admitted_redo_only_page_flush_receipt() -> PageFlushRecoveryReceipt {
    let ack = durable_ack_for_range(20, 21);
    let dirty = dirty_evidence(&ack, b"phase8-admitted-redo-only");
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty, &ack).unwrap();
    PageFlushRecoveryReceipt::publish_admitted_redo_only(ordering)
}

fn rollback_image_protected_page_flush_receipt() -> PageFlushRecoveryReceipt {
    let ack = durable_ack_for_range(20, 21);
    let dirty = dirty_evidence(&ack, b"phase8-rollback-image");
    let declaration = RollbackImagePublicationDeclaration::declare(
        dirty.dirty_identity(),
        dirty.page_generation(),
        dirty.page_lsn(),
        "phase8-rollback-image",
    );
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty, &ack).unwrap();
    PageFlushRecoveryReceipt::publish_rollback_image_protected(ordering, declaration).unwrap()
}

fn missing_rollback_image_denial() -> UnadmittedDirtyPagePublicationDenial {
    let ack = durable_ack_for_range(20, 21);
    let dirty = dirty_evidence(&ack, b"phase8-missing-rollback-image");
    let ordering =
        WalBeforeDataOrderingProof::<PosixFileFsyncDirFsyncProfile>::prove(dirty, &ack).unwrap();
    NoUndoPublicationProof::deny_missing_required_rollback_image(ordering).unwrap_err()
}

fn durable_ack_for_range(
    start: u64,
    end_exclusive: u64,
) -> DurableAckReceipt<PosixFileFsyncDirFsyncProfile> {
    DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt_for_range(
            start,
            end_exclusive,
        ))
        .unwrap(),
    )
}

fn dirty_evidence(
    ack: &DurableAckReceipt<PosixFileFsyncDirFsyncProfile>,
    payload: &[u8],
) -> DirtyPublicationEvidence {
    DirtyPublicationEvidence::from_physical_substrate_publication(
        scheduled_dirty_publication(payload),
        PageLsn::from_lsn(ack.ack_basis().lsn_range().start()),
    )
}
