use crate::consumer_kit::support_pinning::{
    load_support_pin_contract_document, support_pinning_contract, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture, ForgeQuerySupportPinContractSchemaVersion,
    ForgeQuerySupportPinningErrorKind,
};
use crate::runtime::ForgeQueryRuntimeFacadeFamily;

use super::{empty_family_snapshot, scaffold_snapshot, write_deferred_snapshot};

type TerminalSupportPinContractDocumentJson = serde_json::Value;

#[test]
fn assert_satisfied_returns_typed_error_for_blocking_findings() {
    let basis = scaffold_snapshot();
    let drifted = write_deferred_snapshot();
    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();

    let error = contract
        .evaluate_snapshot(&drifted)
        .unwrap()
        .assert_satisfied()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::BlockingFindings
    );
    assert_eq!(error.consumer_name(), Some("worth-kernel"));
    assert!(error.report_digest().is_some());
    assert!(error.blocking_findings().iter().any(|finding| {
        finding.family() == Some(ForgeQueryRuntimeFacadeFamily::Write)
            && finding.expected() == Some("supported")
            && finding.found() == Some("deferred-debt")
    }));
}

#[test]
fn missing_required_row_fails_typed_at_evaluation() {
    let basis = scaffold_snapshot();
    let missing_write = empty_family_snapshot();
    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();

    let report = contract.evaluate_snapshot(&missing_write).unwrap();

    assert!(!report.satisfied());
    assert_eq!(report.finding_count(), 2);
    assert!(report.findings().iter().any(|finding| {
        finding.kind()
            == crate::consumer_kit::support_pinning::ForgeQuerySupportPinFindingKind::RequiredRowMissing
            && finding.family() == Some(ForgeQueryRuntimeFacadeFamily::Write)
            && finding.blocking()
    }));
}

#[test]
fn stale_vocabulary_document_fails_typed_at_load() {
    let basis = scaffold_snapshot();
    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();
    let mut document = terminal_support_pin_contract_document_json(&contract);
    document["pinned_vocabulary_identity"] = terminal_pin_document_string("stale");
    let json = terminal_support_pin_contract_json(document);

    let error = load_support_pin_contract_document(
        &json,
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::VocabularyMismatch
    );
    assert_eq!(error.found(), Some("stale"));
}

#[test]
fn tampered_contract_document_fails_digest_validation() {
    let basis = scaffold_snapshot();
    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();
    let mut document = terminal_support_pin_contract_document_json(&contract);
    document["requirements"][0]["required_status"] = terminal_pin_document_string("unsupported");
    let json = terminal_support_pin_contract_json(document);

    let error = load_support_pin_contract_document(
        &json,
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::ContractDigestMismatch
    );
    assert!(error.expected().is_some());
    assert_eq!(error.found(), Some(contract.contract_digest()));
}

#[test]
fn invalid_document_family_fails_typed_at_load() {
    let basis = scaffold_snapshot();
    let contract = support_pinning_contract("worth-kernel")
        .against_snapshot(&basis)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .seal()
        .unwrap();
    let mut document = terminal_support_pin_contract_document_json(&contract);
    document["requirements"][0]["family"] = terminal_pin_document_string("made-up");
    let json = terminal_support_pin_contract_json(document);

    let error = load_support_pin_contract_document(
        &json,
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::InvalidFacadeFamily
    );
    assert_eq!(error.family(), Some("made-up"));
}

#[test]
fn duplicate_pin_declarations_fail_before_sealing() {
    let snapshot = scaffold_snapshot();
    let error = support_pinning_contract("worth-kernel")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::DuplicateRequiredFamily
    );
    assert_eq!(error.family(), Some("write"));
}

fn terminal_support_pin_contract_document_json(
    contract: &crate::consumer_kit::support_pinning::ForgeQuerySupportPinContract,
) -> TerminalSupportPinContractDocumentJson {
    serde_json::from_str(&contract.to_canonical_json().unwrap()).unwrap()
}

fn terminal_support_pin_contract_json(document: TerminalSupportPinContractDocumentJson) -> String {
    serde_json::to_string_pretty(&document).unwrap()
}

fn terminal_pin_document_string(
    value: impl Into<String>,
) -> TerminalSupportPinContractDocumentJson {
    TerminalSupportPinContractDocumentJson::String(value.into())
}
