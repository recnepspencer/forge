use worth_store_physical_backend::PosixFileFsyncDirFsyncProfile;
use worth_store_recovery_physics::{
    NoUndoPublicationProof, PageFlushRecoveryReceipt, UnadmittedDirtyPagePublicationDenial,
    UnadmittedDirtyPagePublicationDenialKind, WalBeforeDataOrderingProof,
};

pub fn assert_page_flush_before_wal_denial(
    result: Result<
        WalBeforeDataOrderingProof<PosixFileFsyncDirFsyncProfile>,
        UnadmittedDirtyPagePublicationDenial,
    >,
) {
    let denial = result.unwrap_err();
    assert_eq!(
        denial.kind(),
        UnadmittedDirtyPagePublicationDenialKind::PageFlushBeforeWalDurability
    );
    assert!(denial.wal_frontier().is_some());
    assert!(denial.page_lsn().is_some());
    assert!(denial.counters().wal_before_data_denial_count() > 0);
}

pub fn assert_no_undo_rollback_required_denial(
    result: Result<
        NoUndoPublicationProof<PosixFileFsyncDirFsyncProfile>,
        UnadmittedDirtyPagePublicationDenial,
    >,
) {
    let denial = result.unwrap_err();
    assert_eq!(
        denial.kind(),
        UnadmittedDirtyPagePublicationDenialKind::RollbackImageRequired
    );
    assert!(denial.counters().no_undo_denial_count() > 0);
}

pub fn assert_no_undo_rollback_mismatch_denial(
    result: Result<PageFlushRecoveryReceipt, UnadmittedDirtyPagePublicationDenial>,
) {
    let denial = result.unwrap_err();
    assert_eq!(
        denial.kind(),
        UnadmittedDirtyPagePublicationDenialKind::RollbackImageDeclarationMismatch
    );
    assert!(denial.expected_page().is_some());
    assert!(denial.observed_page().is_some());
    assert!(denial.counters().no_undo_denial_count() > 0);
}
