use forge_query::facade::consumer_kit::{
    project_workspace_support_snapshot, query_test_backend_residue_audit, EvidenceReport,
    EvidenceReportDeclaration, EvidenceReportError, EvidenceReportScope,
    ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditReport, ForgeQuerySupportPinReport,
    ForgeQuerySupportPinningError, ForgeQueryTestBackendResidueReport,
};
use forge_query::facade::ForgeQueryWorkspace;

use super::residue_assertions::remaining_worth_domain_hygiene_audit_labels;
use crate::construction::query_support_pins::primitive_construction_query_support_pins;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TestBackendAdoptionPosture {
    NotApplicableNoHandAssemblyResidue,
}

impl TestBackendAdoptionPosture {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableNoHandAssemblyResidue => {
                "not-applicable-no-hand-assembled-query-backend-residue"
            }
        }
    }
}

pub(crate) fn test_backend_adoption_posture() -> TestBackendAdoptionPosture {
    TestBackendAdoptionPosture::NotApplicableNoHandAssemblyResidue
}

pub(crate) fn evaluate_test_backend_residue_audit(
) -> Result<ForgeQueryTestBackendResidueReport, ForgeQueryBoundaryAuditError> {
    query_test_backend_residue_audit("worth-kernel")
        .required_root(format!("{}/src/construction", env!("CARGO_MANIFEST_DIR")))
        .evaluate()
}

pub(crate) fn evaluate_reference_support_pins(
    workspace: &ForgeQueryWorkspace,
) -> Result<ForgeQuerySupportPinReport, ForgeQuerySupportPinningError> {
    let snapshot = project_workspace_support_snapshot(workspace);
    primitive_construction_query_support_pins()?.evaluate_snapshot(&snapshot)
}

pub(crate) fn worth_domain_hygiene_classification_report(
) -> Result<EvidenceReport, EvidenceReportError> {
    let hygiene_audit_labels = remaining_worth_domain_hygiene_audit_labels();

    EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-kernel.construction.phase-eight")?,
        "worth-domain-hygiene-audit-classification",
    )?
    .shape_participating(
        "classification-authority",
        "worth-domain-topology-or-legacy-deletion-hygiene",
    )?
    .usize_participating("query-prohibition-row-count", 0)?
    .usize_participating("hygiene-audit-count", hygiene_audit_labels.len())?
    .value_sequence_participating("worth-domain-hygiene-audits", hygiene_audit_labels)?
    .seal()
}

pub(crate) fn reference_consumer_enforcement_adoption_report(
    audit_report: &ForgeQueryBoundaryAuditReport,
    support_pin_report: &ForgeQuerySupportPinReport,
    backend_residue_report: &ForgeQueryTestBackendResidueReport,
) -> Result<EvidenceReport, EvidenceReportError> {
    let hygiene_classification_report = worth_domain_hygiene_classification_report()?;

    EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-kernel.construction.phase-eight")?,
        "reference-consumer-enforcement-adoption",
    )?
    .shape_participating(
        "query-prohibition-enforcement",
        "query-owned-boundary-audit",
    )?
    .usize_participating(
        "query-boundary-source-count",
        audit_report.source_labels().len(),
    )?
    .usize_participating(
        "query-boundary-finding-count",
        audit_report.findings().len(),
    )?
    .identity_participating(
        "query-boundary-audit-report",
        audit_report.report_identity(),
    )?
    .shape_participating("support-pinning", "query-owned-support-pin-contract")?
    .usize_participating(
        "support-pin-requirement-count",
        support_pin_report.requirement_count(),
    )?
    .usize_participating(
        "support-pin-finding-count",
        support_pin_report.finding_count(),
    )?
    .value_participating(
        "support-pin-report-digest",
        support_pin_report.report_digest(),
    )?
    .shape_participating(
        "test-backend-adoption",
        test_backend_adoption_posture().as_str(),
    )?
    .value_participating(
        "test-backend-residue-report-digest",
        backend_residue_report
            .report_identity()
            .terminal_projection_for_reporting(),
    )?
    .usize_participating(
        "test-backend-residue-finding-count",
        backend_residue_report.finding_count(),
    )?
    .identity_participating(
        "worth-domain-hygiene-classification",
        hygiene_classification_report.report_identity(),
    )?
    .value_sequence_participating(
        "worth-domain-hygiene-audits",
        remaining_worth_domain_hygiene_audit_labels(),
    )?
    .seal()
}
