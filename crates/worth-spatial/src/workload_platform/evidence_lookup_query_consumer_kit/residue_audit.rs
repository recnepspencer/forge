use std::path::PathBuf;

use forge_query::facade::consumer_kit::{
    query_consumer_residue_audit, ForgeQueryConsumerResidueReport,
};

use super::error::{EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind};
use super::row::EvidenceLookupQueryConsumerResidueRow;

pub(crate) fn audit_evidence_lookup_query_consumer_residue_for_roots(
    roots: impl IntoIterator<Item = PathBuf>,
) -> Result<
    ForgeQueryConsumerResidueReport,
    forge_query::facade::consumer_kit::ForgeQueryBoundaryAuditError,
> {
    let mut audit = query_consumer_residue_audit("worth-spatial.evidence-lookup");
    for root in roots {
        audit = audit.required_root(root);
    }
    audit.evaluate()
}

pub(crate) fn residue_rows_from_report(
    report: &ForgeQueryConsumerResidueReport,
) -> Result<Vec<EvidenceLookupQueryConsumerResidueRow>, EvidenceLookupQueryConsumerKitError> {
    let report_identity = report.report_identity().terminal_projection_for_reporting();
    Ok(report
        .findings()
        .iter()
        .zip(report.finding_identities().iter())
        .map(|(finding, identity)| {
            EvidenceLookupQueryConsumerResidueRow::new(
                finding.source_path(),
                finding.line(),
                finding.column(),
                finding.residue_class(),
                identity.terminal_projection_for_reporting(),
                report_identity,
                report.source_inventory_digest(),
            )
        })
        .collect())
}

pub(crate) fn assert_clean_consumer_residue(
    report: &ForgeQueryConsumerResidueReport,
) -> Result<(), EvidenceLookupQueryConsumerKitError> {
    if report.finding_count() == 0 {
        return Ok(());
    }
    Err(EvidenceLookupQueryConsumerKitError::new(
        EvidenceLookupQueryConsumerKitErrorKind::ResidueAudit,
        format!(
            "lookup query consumer residue audit found {} forbidden Query folklore rows",
            report.finding_count()
        ),
    ))
}
