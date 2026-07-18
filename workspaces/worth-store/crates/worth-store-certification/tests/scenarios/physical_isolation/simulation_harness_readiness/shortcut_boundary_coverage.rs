use worth_store_physical_certification::{
    ForbiddenShortcutKind, ShortcutRejectionBoundary,
};

// store-proof-identity[shortcut_report_still_names_required_shortcut_boundaries]: worth-store-certification::physical_isolation/readiness::shortcut_report::shortcut_report_still_names_required_shortcut_boundaries
#[test]
fn shortcut_report_still_names_required_shortcut_boundaries() {
    let report = super::shortcut_report::complete_shortcut_report();
    assert!(report.all_required_shortcuts_denied());
    for boundary in [
        ShortcutRejectionBoundary::EvidenceLooseLog,
        ShortcutRejectionBoundary::ScenarioJsonAuthority,
        ShortcutRejectionBoundary::EvidenceTerminalProjection,
        ShortcutRejectionBoundary::EvidenceSameRunSelfComparison,
        ShortcutRejectionBoundary::FaultDeliveryPrivateMutation,
        ShortcutRejectionBoundary::OracleFixtureLabel,
        ShortcutRejectionBoundary::TranscriptCopiedFields,
        ShortcutRejectionBoundary::PlanProofProgressionSkipped,
        ShortcutRejectionBoundary::OracleTestSupportVerdict,
    ] {
        assert!(
            report
                .receipts()
                .iter()
                .any(|receipt| receipt.boundary() == boundary),
            "missing shortcut boundary {boundary:?}"
        );
    }
    assert!(report
        .receipts()
        .iter()
        .any(|receipt| receipt.shortcut() == ForbiddenShortcutKind::PrivateMutation));
}
