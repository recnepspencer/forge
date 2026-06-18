use crate::consumer_kit::support_pinning::{
    load_support_pin_contract_document, support_pinning_contract, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture, ForgeQuerySupportPinContractSchemaVersion,
};
use crate::runtime::ForgeQueryRuntimeFacadeFamily;

use super::scaffold_snapshot;

#[test]
fn required_write_and_inspect_pins_pass_against_current_snapshot() {
    let snapshot = scaffold_snapshot();

    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Inspect, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .observe_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .unwrap()
        .seal()
        .unwrap();

    let report = contract.evaluate_snapshot(&snapshot).unwrap();

    assert!(report.satisfied());
    assert_eq!(report.requirement_count(), 2);
    assert_eq!(report.observed_count(), 1);
    assert_eq!(report.matched_required_count(), 2);
    assert_eq!(report.snapshot_row_count(), snapshot.rows().len());
    assert_eq!(report.blocking_finding_count(), 0);
    assert!(!report.report_digest().is_empty());
    report.assert_satisfied().unwrap();
}

#[test]
fn durable_contract_document_round_trips_and_still_pins_snapshot() {
    let snapshot = scaffold_snapshot();
    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Inspect, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();
    let json = contract.to_canonical_json().unwrap();

    let loaded = load_support_pin_contract_document(
        &json,
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
    .unwrap();
    let report = loaded.evaluate_snapshot(&snapshot).unwrap();

    assert_eq!(loaded.contract_digest(), contract.contract_digest());
    assert!(report.satisfied());
}
