use std::collections::BTreeSet;

use super::super::documents::{read_repository_document, split_csv, QA_AUDITS};
use super::audit_source_manifest::validate_source_manifests;

const REQUIRED_FINDINGS: &[&str] = &[
    "C8-P1-F01",
    "C8-P1-F02",
    "C8-P1-F03",
    "C8-P1-F04",
    "C8-P1-F05",
    "C8-P1-F06",
    "C8-P1-F07",
    "C8-P1-F08",
    "C8-P1-F09",
    "C8-P1-F10",
    "C8-P1-F11",
    "C8-P1-F12",
    "C8-P1-F13",
    "C8-P1-F14",
    "C8-P1-F15",
    "C8-P1-F16",
    "C8-P1-F17",
    "C8-P1-F18",
    "C8-P1-F19",
    "C8-P1-F20",
    "C8-P1-F21",
    "C8-P1-F22",
    "C8-P1-F23",
    "C8-P1-F24",
    "C8-P1-F25",
    "C8-P1-F26",
    "C8-P1-F27",
    "C8-P1-F28",
    "C8-P1-F29",
    "C8-P1-F30",
    "C8-P1-F31",
    "C8-P1-F32",
    "C8-P1-F33",
    "C8-P1-F34",
    "C8-P1-F35",
    "C8-P1-F36",
    "C8-P1-F37",
    "C8-P1-F38",
    "C8-P1-F39",
    "C8-P1-F40",
    "C8-P1-F41",
    "C8-P1-F42",
];
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
    let revision = "c960d5593a23d8a0d09ad5cc795e9e605f55e250";
    let expected = BTreeSet::from([
        audit_contract(
            "/root/c8_phase1_critic",
            &format!("{revision} plus audit-start dirty snapshot"),
            "87ddf09b1f7235fe9f317173b6857b946a7d70ac4348571b9bb557ea78aaf1e2",
            "Read-only falsify specification fidelity architecture authority topology tests and ledger closure; retain exact evidence and do not edit",
            6..=13,
            "audit-start findings closed",
            "focused boundary suite plus four inherited C7 trybuild attacks",
        ),
        audit_contract(
            "/root/c8_phase1_test_critic",
            &format!("{revision} plus frozen corrected dirty snapshot"),
            "ceddb3f5156790ff9fb6345b3790e0a16466ca4194d9110cc29e88e8a3886f6a",
            "Read-only falsify test realism oracle independence mutation strength and evidence causality; retain exact evidence and do not edit",
            14..=18,
            "frozen-snapshot findings closed",
            "21-test warnings-denied boundary suite plus four inherited C7 trybuild attacks",
        ),
        audit_contract(
            "/root/c8_phase1_test_closure_critic",
            &format!("{revision} plus closure-candidate dirty snapshot"),
            "45282d1c3089a9a8b553ca3e9a3621822bf4543814a9b51a7a0b1413d3adb913",
            "Read-only falsify persisted producer coupling topology path semantics API trait reachability cutover aliases causal closures audit retention and documentation equality; retain exact evidence and do not edit",
            19..=25,
            "closure-candidate findings closed",
            "22-test warnings-denied boundary suite plus four inherited C7 trybuild attacks",
        ),
        audit_contract(
            "/root/c8_phase1_final_critic",
            &format!("{revision} plus final closure-candidate dirty snapshot"),
            "10a02362ff7ac6439ee907e33e231b5eea911660e7926f8800e39343faf15343",
            "Read-only falsify final Phase 1 API impl reachability grouped aliases audit binding and composition; retain exact evidence and do not edit",
            26..=28,
            "final-critic findings closed",
            "read-only Rust scrutiny plus git diff and dirty Rust line-cap checks; focused boundary suite timed out before result",
        ),
        audit_contract(
            "/root/c8_phase1_postfix_closure_critic",
            &format!("{revision} plus post-F35 closure-candidate dirty snapshot"),
            "03dfafe7f05b3105f70c300de083594a69b286f7ddeace14638cc1a49687ff79",
            "Read-only falsify closure of parent alias API semantic persisted producer and codec chains performed effect homes complete dirty source manifests and causal ledger history; retain exact evidence and do not edit",
            29..=35,
            "postfix findings closed",
            "26-test warnings-denied boundary suite plus four inherited C7 trybuild attacks",
        ),
        audit_contract(
            "/root/c8_phase1_absolute_final_critic",
            &format!("{revision} plus post-F42 closure-candidate review snapshot"),
        "f82c8051f1207ae8f74a5c3d0bcc77588f8c6762ae11b524c695b203bda3eb60",
            "Read-only falsify final Phase 1 persisted syntax freshness topology destination API audit reproducibility and commit-stable source closure; retain exact evidence and do not edit",
            36..=42,
            "absolute-final findings closed",
            "28-test warnings-denied boundary suite plus four inherited C7 trybuild attacks and constitutional checks",
        ),
    ]);
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

fn audit_contract(
    reviewer: &str,
    revision: &str,
    snapshot: &str,
    prompt: &str,
    findings: std::ops::RangeInclusive<u8>,
    disposition: &str,
    verification: &str,
) -> AuditRecord {
    (
        reviewer.into(),
        "gpt-5.6-sol high".into(),
        revision.into(),
        snapshot.into(),
        prompt.into(),
        findings.map(|id| format!("C8-P1-F{id:02}")).collect(),
        disposition.into(),
        verification.into(),
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
