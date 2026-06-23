use forge_query::facade::consumer_kit::{
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportScope,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

pub(crate) mod adoption_sources;

pub(crate) fn sealed_report(
    scope: &'static str,
    report_name: &'static str,
    declare: impl FnOnce(
        EvidenceReportDeclaration,
    ) -> Result<EvidenceReportDeclaration, EvidenceReportError>,
) -> Result<EvidenceReport, EvidenceReportError> {
    declare(EvidenceReportDeclaration::new(
        EvidenceReportScope::new(scope)?,
        report_name,
    )?)?
    .seal()
}

pub(crate) fn sealed_report_identity(
    scope: &'static str,
    report_name: &'static str,
    declare: impl FnOnce(
        EvidenceReportDeclaration,
    ) -> Result<EvidenceReportDeclaration, EvidenceReportError>,
) -> String {
    report_identity(
        &sealed_report(scope, report_name, declare)
            .expect("worth-kernel evidence report declaration should be static-valid"),
    )
}

pub(crate) fn sealed_report_evidence_identity(
    scope: &'static str,
    report_name: &'static str,
    declare: impl FnOnce(
        EvidenceReportDeclaration,
    ) -> Result<EvidenceReportDeclaration, EvidenceReportError>,
) -> Result<ForgeQueryEvidenceIdentity, EvidenceReportError> {
    Ok(sealed_report(scope, report_name, declare)?
        .report_identity()
        .clone())
}

pub(crate) fn report_identity(report: &EvidenceReport) -> String {
    report
        .report_identity()
        .terminal_projection_for_reporting()
        .to_string()
}
