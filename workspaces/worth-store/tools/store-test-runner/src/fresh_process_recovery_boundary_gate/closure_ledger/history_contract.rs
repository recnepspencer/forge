use std::collections::BTreeSet;

mod audit_contracts;
#[path = "history_contract/finding_inventory.rs"]
mod finding_inventory;

use super::super::documents::{read_repository_document, split_csv, QA_AUDITS};
use super::audit_source_manifest::validate_source_manifests;
use audit_contracts::{AuditContract, AUDIT_CONTRACTS};
use finding_inventory::REQUIRED_FINDINGS;

const FINDING_GUARANTEES: &[(&str, &str)] = &[
    (
        "C8-P1-F01",
        "C8-P1-API-01 C8-P1-CUTOVER-01 C8-P1-DEPENDENCY-01",
    ),
    (
        "C8-P1-F02",
        "C8-P1-AUTHORITY-01 C8-P1-SESSION-01 C8-P1-PROTOCOL-01",
    ),
    ("C8-P1-F03", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    ("C8-P1-F04", "C8-P1-TRUTH-01 C8-P1-ENTRY-01"),
    ("C8-P1-F05", "C8-P1-AUTHORITY-01 C8-P1-CUTOVER-01"),
    ("C8-P1-F06", "C8-P1-API-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F07", "C8-P1-CUTOVER-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F08", "C8-P1-TOPOLOGY-01 C8-P1-SESSION-01"),
    (
        "C8-P1-F09",
        "C8-P1-AUTHORITY-01 C8-P1-EFFECT-01 C8-P1-FRESHNESS-01",
    ),
    (
        "C8-P1-F10",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F11", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    ("C8-P1-F12", "C8-P1-COMPILE-01 C8-P1-TRUTH-01"),
    ("C8-P1-F13", "C8-P1-DOCUMENTATION-01"),
    (
        "C8-P1-F14",
        "C8-P1-TRUTH-01 C8-P1-ENTRY-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F15", "C8-P1-CUTOVER-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F16", "C8-P1-TOPOLOGY-01"),
    ("C8-P1-F17", "C8-P1-API-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F18", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F19",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F20", "C8-P1-TOPOLOGY-01 C8-P1-LEDGER-02"),
    ("C8-P1-F21", "C8-P1-API-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F22", "C8-P1-CUTOVER-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F23", "C8-P1-LEDGER-02"),
    ("C8-P1-F24", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    ("C8-P1-F25", "C8-P1-DOCUMENTATION-01"),
    ("C8-P1-F26", "C8-P1-API-01 C8-P1-CLEANUP-01 C8-P1-LEDGER-02"),
    ("C8-P1-F27", "C8-P1-CUTOVER-01 C8-P1-CLEANUP-01"),
    ("C8-P1-F28", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    ("C8-P1-F29", "C8-P1-API-01 C8-P1-CLEANUP-01"),
    (
        "C8-P1-F30",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F31", "C8-P1-TOPOLOGY-01 C8-P1-EFFECT-01"),
    ("C8-P1-F32", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F33",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F34", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F35",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F36",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F37",
        "C8-P1-FRESHNESS-01 C8-P1-TOPOLOGY-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F38", "C8-P1-API-01 C8-P1-LEDGER-02"),
    ("C8-P1-F39", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F40",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F41",
        "C8-P1-API-01 C8-P1-FRESHNESS-01 C8-P1-TOPOLOGY-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F42", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F43",
        "C8-P1-API-01 C8-P1-TOPOLOGY-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F44", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F45",
        "C8-P1-API-01 C8-P1-TOPOLOGY-01 C8-P1-ENTRY-01 C8-P1-LEDGER-02",
    ),
    ("C8-P1-F46", "C8-P1-LEDGER-01 C8-P1-LEDGER-02"),
    (
        "C8-P1-F47",
        "C8-P1-DEPENDENCY-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F48",
        "C8-P1-API-01 C8-P1-AUTHORITY-01 C8-P1-TOPOLOGY-01 C8-P1-ENTRY-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F49",
        "C8-P1-API-01 C8-P1-EFFECT-01 C8-P1-FRESHNESS-01 C8-P1-TOPOLOGY-01 C8-P1-DEPENDENCY-01 C8-P1-CUTOVER-01 C8-P1-CLEANUP-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F50",
        "C8-P1-API-01 C8-P1-AUTHORITY-01 C8-P1-SESSION-01 C8-P1-EFFECT-01 C8-P1-FRESHNESS-01 C8-P1-TOPOLOGY-01 C8-P1-DEPENDENCY-01 C8-P1-CUTOVER-01 C8-P1-CLEANUP-01 C8-P1-DOCUMENTATION-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F51",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F52",
        "C8-P1-TRUTH-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F53",
        "C8-P1-DOCUMENTATION-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F54",
        "C8-P1-API-01 C8-P1-AUTHORITY-01 C8-P1-SESSION-01 C8-P1-EFFECT-01 C8-P1-TOPOLOGY-01 C8-P1-CLEANUP-01 C8-P1-DOCUMENTATION-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F55",
        "C8-P1-API-01 C8-P1-TOPOLOGY-01 C8-P1-CUTOVER-01 C8-P1-CLEANUP-01 C8-P1-DOCUMENTATION-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F56",
        "C8-P1-API-01 C8-P1-DOCUMENTATION-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F57",
        "C8-P1-API-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F58",
        "C8-P1-TRUTH-01 C8-P1-API-01 C8-P1-TOPOLOGY-01 C8-P1-CUTOVER-01 C8-P1-CLEANUP-01 C8-P1-DOCUMENTATION-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F59",
        "C8-P1-TRUTH-01 C8-P1-API-01 C8-P1-TOPOLOGY-01 C8-P1-CUTOVER-01 C8-P1-CLEANUP-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F60",
        "C8-P1-TRUTH-01 C8-P1-API-01 C8-P1-CUTOVER-01 C8-P1-DOCUMENTATION-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F61",
        "C8-P1-TRUTH-01 C8-P1-API-01 C8-P1-AUTHORITY-01 C8-P1-SESSION-01 C8-P1-EFFECT-01 C8-P1-TOPOLOGY-01 C8-P1-CUTOVER-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F62",
        "C8-P1-API-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
    (
        "C8-P1-F63",
        "C8-P1-TRUTH-01 C8-P1-TOPOLOGY-01 C8-P1-CUTOVER-01 C8-P1-DOCUMENTATION-01 C8-P1-PERSISTED-01 C8-P1-LEDGER-01 C8-P1-LEDGER-02",
    ),
];

pub(super) type AuditRecord = (
    String,
    String,
    String,
    String,
    String,
    BTreeSet<String>,
    String,
    String,
);

pub(super) fn validate_finding_history(
    document: &str,
    guarantees: &BTreeSet<&str>,
) -> Result<BTreeSet<String>, String> {
    let row_list = document
        .lines()
        .filter(|line| {
            line.trim()
                .trim_matches('|')
                .split('|')
                .next()
                .map(str::trim)
                .is_some_and(|id| {
                    id.strip_prefix("C8-P1-F").is_some_and(|suffix| {
                        !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
                    })
                })
        })
        .map(|line| {
            let columns = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            if columns.len() != 6 {
                return Err("C.8 finding row must have six columns".to_owned());
            }
            validate_finding_row(&columns, guarantees)?;
            Ok(columns[0].to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let rows = row_list.iter().cloned().collect::<BTreeSet<_>>();
    let required = REQUIRED_FINDINGS
        .iter()
        .map(|id| (*id).to_owned())
        .collect::<BTreeSet<_>>();
    (rows == required && row_list.len() == required.len())
        .then_some(rows)
        .ok_or_else(|| "C.8 finding history is incomplete or duplicated".to_owned())
}

pub(super) fn finding_applies(finding: &str, guarantee: &str) -> bool {
    FINDING_GUARANTEES
        .iter()
        .find_map(|(id, guarantees)| (*id == finding).then_some(*guarantees))
        .is_some_and(|guarantees| guarantees.split_whitespace().any(|id| id == guarantee))
}

fn validate_finding_row(columns: &[&str], guarantees: &BTreeSet<&str>) -> Result<(), String> {
    let expected_guarantees = FINDING_GUARANTEES
        .iter()
        .find_map(|(id, expected)| (*id == columns[0]).then_some(*expected))
        .ok_or_else(|| format!("unknown C.8 finding {}", columns[0]))?;
    if columns[2] != expected_guarantees {
        return Err(format!(
            "C.8 finding {} changed affected guarantees",
            columns[0]
        ));
    }
    if columns[5]
        .split_whitespace()
        .map(|word| word.trim_matches(|character: char| !character.is_ascii_alphanumeric()))
        .any(|word| word.eq_ignore_ascii_case("pending") || word.eq_ignore_ascii_case("open"))
    {
        return Err(format!("C.8 finding {} is not closed", columns[0]));
    }
    if columns[2]
        .split_whitespace()
        .any(|id| !guarantees.contains(id))
        || columns[3].len() < 20
        || columns[4].len() < 20
        || columns[5].len() < 20
    {
        return Err(format!("C.8 finding {} lacks causal closure", columns[0]));
    }
    Ok(())
}

pub(super) fn validate_audit_history(document: &str) -> Result<(), String> {
    let audit_document = read_repository_document(QA_AUDITS)?;
    validate_audit_records(&audit_document, document)
}

pub(super) fn validate_audit_records(audit_document: &str, document: &str) -> Result<(), String> {
    let mut lines = audit_document.lines();
    if lines.next()
        != Some(
            "reviewer,model,revision,source_snapshot,prompt,finding_ids,disposition,verification",
        )
    {
        return Err("C.8 QA audit artifact has an invalid schema".into());
    }
    let mut audits = BTreeSet::new();
    let mut audit_count = 0;
    for line in lines.filter(|line| !line.trim().is_empty()) {
        audit_count += 1;
        audits.insert(parse_audit_row(line)?);
    }
    let expected = expected_audit_records();
    if audits != expected || audit_count != expected.len() {
        return Err("C.8 structured QA audit history is incomplete or altered".into());
    }
    validate_source_manifests(&audits)?;
    validate_audit_summary(document, &expected)
}

pub(super) fn parse_audit_row(line: &str) -> Result<AuditRecord, String> {
    let columns = split_csv(line, 8)?;
    if columns[3].len() != 64 || !columns[3].chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!(
            "C.8 QA audit {} lacks source-bound evidence",
            columns[0]
        ));
    }
    Ok((
        columns[0].into(),
        columns[1].into(),
        columns[2].into(),
        columns[3].into(),
        columns[4].into(),
        columns[5].split(';').map(str::to_owned).collect(),
        columns[6].into(),
        columns[7].into(),
    ))
}

fn expected_audit_records() -> BTreeSet<AuditRecord> {
    AUDIT_CONTRACTS.iter().map(audit_contract).collect()
}

fn audit_contract(contract: &AuditContract) -> AuditRecord {
    let revision = "c960d5593a23d8a0d09ad5cc795e9e605f55e250";
    (
        contract.reviewer.into(),
        "gpt-5.6-sol high".into(),
        format!("{revision} {}", contract.revision_suffix),
        contract.snapshot.into(),
        contract.prompt.into(),
        (contract.findings.0..=contract.findings.1)
            .map(|id| format!("C8-P1-F{id:02}"))
            .collect(),
        contract.disposition.into(),
        contract.verification.into(),
    )
}

fn validate_audit_summary(document: &str, expected: &BTreeSet<AuditRecord>) -> Result<(), String> {
    let summary_list = document
        .lines()
        .filter(|line| line.trim_start().starts_with("| /root/c8_phase1_"))
        .map(|line| {
            let columns = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>();
            if columns.len() != 6 || columns[1] != "gpt-5.6-sol high" {
                return Err("C.8 audit summary row is malformed".to_owned());
            }
            Ok((
                columns[0].to_owned(),
                columns[2].to_owned(),
                columns[5].to_owned(),
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let summaries = summary_list.iter().cloned().collect::<BTreeSet<_>>();
    let required = expected
        .iter()
        .map(|(reviewer, _, revision, _, _, _, disposition, _)| {
            (reviewer.clone(), revision.clone(), disposition.clone())
        })
        .collect::<BTreeSet<_>>();
    (summaries == required && summary_list.len() == required.len())
        .then_some(())
        .ok_or_else(|| "C.8 audit summary diverges from structured records".to_owned())
}
