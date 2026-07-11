use forge_store_recovery_physics::{
    PageRedoApplicationBasis, PageRedoDigestState, PageRedoEligibility, PageRedoEligibilityKind,
    StalePageRecoveryClassification, StalePageRecoveryClassificationKind,
};

pub fn assert_stale_page_requires_redo(classification: &StalePageRecoveryClassification) {
    assert_eq!(
        classification.kind(),
        StalePageRecoveryClassificationKind::RedoRequired
    );
    assert!(classification.counters().stale_page_redo_required_count() > 0);
}

pub fn assert_current_page_skips_redo(classification: &StalePageRecoveryClassification) {
    assert_eq!(
        classification.kind(),
        StalePageRecoveryClassificationKind::Current
    );
    assert!(classification.counters().current_page_redo_skip_count() > 0);
}

pub fn assert_idempotent_redo_converges(
    redo: &PageRedoEligibility,
    applied_page: PageRedoDigestState,
    basis: &PageRedoApplicationBasis,
) {
    assert_eq!(redo.kind(), PageRedoEligibilityKind::ApplyRedo);
    let reapplied = redo
        .apply_idempotent_redo(applied_page.clone(), basis)
        .unwrap();
    assert_eq!(applied_page, reapplied);
    assert!(
        redo.record_idempotent_redo_application()
            .idempotent_redo_application_count()
            > 0
    );
}
