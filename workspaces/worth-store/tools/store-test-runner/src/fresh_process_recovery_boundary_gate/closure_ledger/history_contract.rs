use std::collections::BTreeSet;

mod audit_contracts;
#[path = "history_contract/finding_inventory.rs"]
mod finding_inventory;

use super::super::documents::{read_repository_document, split_csv, QA_AUDITS};
use super::audit_source_manifest::validate_source_manifests;
use audit_contracts::{AuditContract, AUDIT_CONTRACTS, PHASE_EIGHT_AUDIT_SCOPES};
use finding_inventory::{FINDING_GUARANTEES, REQUIRED_FINDINGS};

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

const PHASE_EIGHT_LEDGER: &str =
    "_docs/worth-store/physical-reconstruction-c8-phase-8-closure-ledger.md";

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
    let mut parsed = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        parsed.push(parse_audit_row(line)?);
    }
    let phase_one = parsed
        .iter()
        .filter(|audit| audit.0.starts_with("/root/c8_phase1_"))
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected = expected_audit_records();
    if phase_one != expected
        || parsed
            .iter()
            .filter(|audit| audit.0.starts_with("/root/c8_phase1_"))
            .count()
            != expected.len()
    {
        return Err("C.8 structured QA audit history is incomplete or altered".into());
    }
    let phase_eight = parsed
        .into_iter()
        .filter(|audit| !audit.0.starts_with("/root/c8_phase1_"))
        .collect::<Vec<_>>();
    validate_phase_eight_audits(&phase_eight)?;
    validate_source_manifests(&phase_one)?;
    validate_audit_summary(document, &expected)
}

fn validate_phase_eight_audits(audits: &[AuditRecord]) -> Result<(), String> {
    if audits.len() != PHASE_EIGHT_AUDIT_SCOPES.len() {
        return Err("C.8 Phase 8 audit certification set is incomplete or duplicated".into());
    }
    let ledger = read_repository_document(PHASE_EIGHT_LEDGER)?;
    let snapshot = ledger
        .lines()
        .find_map(|line| line.strip_prefix("Source closure SHA-256: "))
        .map(str::to_owned)
        .filter(|value| value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit()))
        .ok_or_else(|| "C.8 Phase 8 ledger has no valid source closure digest".to_owned())?;
    let mut reviewers = BTreeSet::new();
    let mut scopes = BTreeSet::new();
    for audit in audits {
        let (scope, model, reviewer_prefix) = PHASE_EIGHT_AUDIT_SCOPES
            .iter()
            .find(|(candidate, _, _)| audit.4 == *candidate)
            .ok_or_else(|| format!("unknown C.8 Phase 8 audit scope `{}`", audit.4))?;
        if audit.1 != *model
            || !audit.0.starts_with(reviewer_prefix)
            || audit.0.len() == reviewer_prefix.len()
            || audit.3 != snapshot
            || audit.5 != BTreeSet::from(["none".to_owned()])
            || audit.6 != "clean current-tree certification"
            || audit.7.is_empty()
        {
            return Err(format!(
                "C.8 Phase 8 audit `{scope}` is not source-bound clean evidence"
            ));
        }
        if !reviewers.insert(audit.0.clone()) || !scopes.insert(audit.4.clone()) {
            return Err("C.8 Phase 8 audits must use distinct reviewers and scopes".into());
        }
    }
    Ok(())
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
