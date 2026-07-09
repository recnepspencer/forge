use crate::consumer_kit::support_pinning::{
    support_pinning_contract, WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture,
    WorthQuerySupportPinFindingKind,
};
use crate::runtime::WorthQueryRuntimeFacadeFamily;

use super::{scaffold_snapshot, write_deferred_snapshot};

#[test]
fn drift_fails_only_consumers_pinned_to_regressed_row() {
    let basis = scaffold_snapshot();
    let drifted = write_deferred_snapshot();

    let write_consumer = support_pinning_contract("write-consumer")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(WorthQueryRuntimeFacadeFamily::Write, |row| {
            row.status(WorthQueryPinnedSupportStatus::Supported)
                .teaching_posture(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();
    let inspect_consumer = support_pinning_contract("inspect-consumer")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(WorthQueryRuntimeFacadeFamily::Inspect, |row| {
            row.status(WorthQueryPinnedSupportStatus::Supported)
                .teaching_posture(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();

    let write_report = write_consumer.evaluate_snapshot(&drifted).unwrap();
    let inspect_report = inspect_consumer.evaluate_snapshot(&drifted).unwrap();

    assert!(!write_report.satisfied());
    assert_eq!(write_report.finding_count(), 4);
    assert_eq!(write_report.blocking_finding_count(), 3);
    assert_write_finding(
        &write_report,
        WorthQuerySupportPinFindingKind::StatusMismatch,
        Some("supported"),
        Some("deferred-debt"),
        true,
    );
    assert_write_finding(
        &write_report,
        WorthQuerySupportPinFindingKind::TeachingPostureMismatch,
        Some("ordinary-runtime-dx"),
        Some("visible-but-deferred"),
        true,
    );
    assert_write_finding(
        &write_report,
        WorthQuerySupportPinFindingKind::LiveRowDigestMismatch,
        None,
        None,
        true,
    );

    assert!(inspect_report.satisfied());
    assert_eq!(inspect_report.finding_count(), 1);
    assert_eq!(
        inspect_report.findings()[0].kind(),
        WorthQuerySupportPinFindingKind::SourceMatrixDigestChanged
    );
    assert!(!inspect_report.findings()[0].blocking());
}

#[test]
fn observed_row_drift_is_localized_but_nonblocking() {
    let basis = scaffold_snapshot();
    let drifted = write_deferred_snapshot();
    let observer = support_pinning_contract("write-observer")
        .against_snapshot(&basis)
        .unwrap()
        .observe_family(WorthQueryRuntimeFacadeFamily::Write)
        .unwrap()
        .seal()
        .unwrap();

    let report = observer.evaluate_snapshot(&drifted).unwrap();

    assert!(report.satisfied());
    assert_eq!(report.finding_count(), 4);
    assert_eq!(report.blocking_finding_count(), 0);
    assert_write_finding(
        &report,
        WorthQuerySupportPinFindingKind::ObservedStatusChanged,
        Some("supported"),
        Some("deferred-debt"),
        false,
    );
    assert_write_finding(
        &report,
        WorthQuerySupportPinFindingKind::ObservedTeachingPostureChanged,
        Some("ordinary-runtime-dx"),
        Some("visible-but-deferred"),
        false,
    );
    assert_write_finding(
        &report,
        WorthQuerySupportPinFindingKind::ObservedLiveRowDigestChanged,
        None,
        None,
        false,
    );
}

fn assert_write_finding(
    report: &crate::consumer_kit::support_pinning::WorthQuerySupportPinReport,
    kind: WorthQuerySupportPinFindingKind,
    expected: Option<&str>,
    found: Option<&str>,
    blocking: bool,
) {
    let finding = report
        .findings()
        .iter()
        .find(|finding| {
            finding.kind() == kind && finding.family() == Some(WorthQueryRuntimeFacadeFamily::Write)
        })
        .expect("expected write finding should exist");
    if let Some(expected) = expected {
        assert_eq!(finding.expected(), Some(expected));
    }
    if let Some(found) = found {
        assert_eq!(finding.found(), Some(found));
    }
    assert_eq!(finding.blocking(), blocking);
}
