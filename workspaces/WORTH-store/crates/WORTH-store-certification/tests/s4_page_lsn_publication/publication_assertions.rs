use worth_store_physical_backend::{BackendDurabilityProfileId, PosixFileFsyncDirFsyncProfile};
use worth_store_physical_format::PageGenerationCell;
use worth_store_recovery_physics::{PageFlushRecoveryReceipt, PageLsn, WalBeforeDataOrderingProof};

pub fn assert_ordering_basis(
    ordering: &WalBeforeDataOrderingProof<PosixFileFsyncDirFsyncProfile>,
    page: PageGenerationCell,
    page_lsn: PageLsn,
) {
    assert_eq!(
        ordering.profile_id(),
        BackendDurabilityProfileId::PosixFileFsyncDirFsync
    );
    assert_eq!(ordering.page_generation(), page);
    assert_eq!(ordering.page_lsn(), page_lsn);
    assert!(ordering.counters().wal_before_data_proof_count() > 0);
}

pub fn assert_flush_receipt_basis(
    flush: &PageFlushRecoveryReceipt,
    page: PageGenerationCell,
    page_lsn: PageLsn,
) {
    assert_eq!(
        flush.profile_id(),
        BackendDurabilityProfileId::PosixFileFsyncDirFsync
    );
    assert_eq!(flush.page_generation(), page);
    assert_eq!(flush.page_lsn(), page_lsn);
    assert_eq!(flush.redo_frontier(), page_lsn);
    assert!(flush.counters().page_flush_receipt_count() > 0);
}
