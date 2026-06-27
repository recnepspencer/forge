use std::collections::BTreeSet;
#[cfg(test)]
use std::path::Path;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::counters::EvidenceLookupSourceFirewallCounters;
use super::covered_root::EvidenceLookupSourceFirewallCoveredRoot;
use super::error::{EvidenceLookupSourceFirewallError, EvidenceLookupSourceFirewallErrorKind};
use super::exception::named_exception_for_path;
use super::exception_summary::EvidenceLookupSourceFirewallExceptionSummary;
use super::row::{
    EvidenceLookupSourceFirewallExceptionKind, EvidenceLookupSourceFirewallRow,
    EvidenceLookupSourceFirewallRowPosture,
};
use super::scan_roots::{
    current_source_firewall_snapshot, SourceFirewallRecord, SourceFirewallSnapshot,
};
use super::semantic_shape::matched_semantic_shapes;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSourceFirewallOutcome {
    Clean,
    ExceptionsOnly,
    ForbiddenAuthorityPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSourceFirewallReport {
    covered_root_inventory: Vec<EvidenceLookupSourceFirewallCoveredRoot>,
    covered_roots: Vec<String>,
    exception_summaries: Vec<EvidenceLookupSourceFirewallExceptionSummary>,
    rows: Vec<EvidenceLookupSourceFirewallRow>,
    counters: EvidenceLookupSourceFirewallCounters,
    outcome: EvidenceLookupSourceFirewallOutcome,
    firewall_digest: String,
}

impl EvidenceLookupSourceFirewallReport {
    pub fn covered_root_inventory(&self) -> &[EvidenceLookupSourceFirewallCoveredRoot] {
        &self.covered_root_inventory
    }

    pub fn covered_roots(&self) -> &[String] {
        &self.covered_roots
    }

    pub fn exception_summaries(&self) -> &[EvidenceLookupSourceFirewallExceptionSummary] {
        &self.exception_summaries
    }

    pub fn rows(&self) -> &[EvidenceLookupSourceFirewallRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &EvidenceLookupSourceFirewallCounters {
        &self.counters
    }

    pub const fn outcome(&self) -> EvidenceLookupSourceFirewallOutcome {
        self.outcome
    }

    pub fn firewall_digest(&self) -> &str {
        &self.firewall_digest
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }

    pub fn forbidden_rows(&self) -> Vec<&EvidenceLookupSourceFirewallRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.posture()
                    == EvidenceLookupSourceFirewallRowPosture::ForbiddenProductionAuthority
            })
            .collect()
    }

    pub fn allowed_exception_rows(&self) -> Vec<&EvidenceLookupSourceFirewallRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.posture() == EvidenceLookupSourceFirewallRowPosture::AllowedNamedException
            })
            .collect()
    }
}

pub fn current_evidence_lookup_source_firewall_report(
) -> Result<EvidenceLookupSourceFirewallReport, EvidenceLookupSourceFirewallError> {
    report_from_snapshot(&current_source_firewall_snapshot()?)
}

#[cfg(test)]
pub(crate) fn source_firewall_report_for_snapshot_root(
    workspace_root: &Path,
) -> Result<EvidenceLookupSourceFirewallReport, EvidenceLookupSourceFirewallError> {
    report_from_snapshot(
        &super::scan_roots::source_firewall_snapshot_for_workspace_root(workspace_root)?,
    )
}

fn report_from_snapshot(
    snapshot: &SourceFirewallSnapshot,
) -> Result<EvidenceLookupSourceFirewallReport, EvidenceLookupSourceFirewallError> {
    let mut row_identities = BTreeSet::new();
    let mut rows = Vec::new();
    for record in snapshot.records() {
        for matched_shape in matched_semantic_shapes(&record.source_path, &record.source) {
            let identity = format!(
                "{}:{}:{}",
                record.source_path,
                matched_shape.kind.as_digest_label(),
                matched_shape.matched_surface
            );
            if !row_identities.insert(identity) {
                return Err(EvidenceLookupSourceFirewallError::new(
                    EvidenceLookupSourceFirewallErrorKind::DuplicateFirewallRow,
                    record.source_path.clone(),
                ));
            }
            rows.push(build_row(record, matched_shape));
        }
    }
    let counters = EvidenceLookupSourceFirewallCounters::from_rows(
        snapshot.scanned_root_count(),
        snapshot.scanned_file_count(),
        &rows,
    );
    let outcome = outcome_for_rows(&rows);
    let covered_root_inventory = snapshot.covered_root_inventory().to_vec();
    let covered_roots = snapshot.covered_roots();
    let exception_summaries = exception_summaries(&counters);
    let firewall_digest = firewall_digest(
        &covered_root_inventory,
        &rows,
        &exception_summaries,
        &counters,
        outcome,
    );
    Ok(EvidenceLookupSourceFirewallReport {
        covered_root_inventory,
        covered_roots,
        exception_summaries,
        rows,
        counters,
        outcome,
        firewall_digest,
    })
}

fn build_row(
    record: &SourceFirewallRecord,
    matched_shape: super::semantic_shape::MatchedSemanticShape,
) -> EvidenceLookupSourceFirewallRow {
    let Some((exception_kind, exception_reason)) =
        named_exception_for_path(&record.source_path, record.test_support)
    else {
        return EvidenceLookupSourceFirewallRow::forbidden(
            record.source_path.clone(),
            matched_shape.matched_surface,
            matched_shape.kind,
            matched_shape.reason,
        );
    };
    EvidenceLookupSourceFirewallRow::allowed_exception(
        record.source_path.clone(),
        matched_shape.matched_surface,
        matched_shape.kind,
        exception_kind,
        exception_reason,
    )
}

fn outcome_for_rows(
    rows: &[EvidenceLookupSourceFirewallRow],
) -> EvidenceLookupSourceFirewallOutcome {
    if rows.is_empty() {
        return EvidenceLookupSourceFirewallOutcome::Clean;
    }
    if rows.iter().any(|row| {
        row.posture() == EvidenceLookupSourceFirewallRowPosture::ForbiddenProductionAuthority
    }) {
        return EvidenceLookupSourceFirewallOutcome::ForbiddenAuthorityPresent;
    }
    EvidenceLookupSourceFirewallOutcome::ExceptionsOnly
}

fn firewall_digest(
    covered_root_inventory: &[EvidenceLookupSourceFirewallCoveredRoot],
    rows: &[EvidenceLookupSourceFirewallRow],
    exception_summaries: &[EvidenceLookupSourceFirewallExceptionSummary],
    counters: &EvidenceLookupSourceFirewallCounters,
    outcome: EvidenceLookupSourceFirewallOutcome,
) -> String {
    let mut parts = vec![
        "evidence-lookup-source-firewall".to_string(),
        format!("roots:{}", counters.scanned_root_count()),
        format!("files:{}", counters.scanned_file_count()),
        format!("rows:{}", counters.total_row_count()),
        format!("forbidden:{}", counters.forbidden_row_count()),
        format!("exceptions:{}", counters.allowed_exception_row_count()),
        format!("outcome:{outcome:?}"),
    ];
    for covered_root in covered_root_inventory {
        parts.push(format!(
            "covered-root:{}:{:?}",
            covered_root.source_path(),
            covered_root.kind()
        ));
    }
    for summary in exception_summaries {
        parts.push(format!(
            "exception-summary:{:?}:{}",
            summary.kind(),
            summary.row_count()
        ));
    }
    for row in rows {
        parts.push(format!(
            "{}:{}:{:?}:{}:{}",
            row.source_path(),
            row.matched_surface(),
            row.posture(),
            row.forbidden_authority_kind().as_digest_label(),
            row.exception_kind()
                .map(EvidenceLookupSourceFirewallExceptionKind::as_digest_label)
                .unwrap_or("none")
        ));
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn exception_summaries(
    counters: &EvidenceLookupSourceFirewallCounters,
) -> Vec<EvidenceLookupSourceFirewallExceptionSummary> {
    [
        (
            EvidenceLookupSourceFirewallExceptionKind::CertificationOnlyCodec,
            counters.certification_only_exception_row_count(),
        ),
        (
            EvidenceLookupSourceFirewallExceptionKind::DocumentationReportCodec,
            counters.documentation_report_exception_row_count(),
        ),
        (
            EvidenceLookupSourceFirewallExceptionKind::TestSupportFixture,
            counters.test_support_exception_row_count(),
        ),
    ]
    .into_iter()
    .filter(|(_, row_count)| *row_count > 0)
    .map(|(kind, row_count)| EvidenceLookupSourceFirewallExceptionSummary::new(kind, row_count))
    .collect()
}
