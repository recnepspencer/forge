use forge_query::facade::consumer_kit::{
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportScope,
};

use super::adoption_inventory::test_backend_adoption_posture;
use super::residue_assertions::{
    query_enforcement_folklore_violation_count, remaining_worth_domain_hygiene_audit_labels,
};

#[derive(Clone, Debug)]
pub(crate) struct ReferenceConsumerAdoptionResidueReport {
    evidence_report: EvidenceReport,
    report_digest_residue_count: usize,
    prohibition_audit_residue_count: usize,
    support_pinning_residue_count: usize,
    test_backend_residue_count: usize,
    defended_worth_domain_residue_count: usize,
}

impl ReferenceConsumerAdoptionResidueReport {
    pub(crate) fn current() -> Result<Self, EvidenceReportError> {
        let report_digest_residue_count = 0;
        let query_enforcement_residue_count = query_enforcement_folklore_violation_count();
        let support_pinning_residue_count = query_enforcement_residue_count;
        let prohibition_audit_residue_count = 0;
        let test_backend_residue_count = usize::from(
            test_backend_adoption_posture().as_str()
                != "not-applicable-no-hand-assembled-query-backend-residue",
        );
        let defended_worth_domain_residue_count =
            remaining_worth_domain_hygiene_audit_labels().len();
        let evidence_report = EvidenceReportDeclaration::new(
            EvidenceReportScope::new("worth-kernel.construction.phase-nine")?,
            "reference-consumer-adoption-residue",
        )?
        .usize_participating("report-digest-residue-count", report_digest_residue_count)?
        .usize_participating(
            "prohibition-audit-residue-count",
            prohibition_audit_residue_count,
        )?
        .usize_participating(
            "support-pinning-residue-count",
            support_pinning_residue_count,
        )?
        .usize_participating("test-backend-residue-count", test_backend_residue_count)?
        .usize_participating(
            "defended-worth-domain-residue-count",
            defended_worth_domain_residue_count,
        )?
        .value_sequence_participating(
            "defended-worth-domain-residue-labels",
            remaining_worth_domain_hygiene_audit_labels(),
        )?
        .seal()?;
        Ok(Self {
            evidence_report,
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
        })
    }

    pub(crate) fn report_digest_residue_count(&self) -> usize {
        self.report_digest_residue_count
    }

    pub(crate) fn prohibition_audit_residue_count(&self) -> usize {
        self.prohibition_audit_residue_count
    }

    pub(crate) fn support_pinning_residue_count(&self) -> usize {
        self.support_pinning_residue_count
    }

    pub(crate) fn test_backend_residue_count(&self) -> usize {
        self.test_backend_residue_count
    }

    pub(crate) fn defended_worth_domain_residue_count(&self) -> usize {
        self.defended_worth_domain_residue_count
    }

    pub(crate) fn query_owned_residue_count(&self) -> usize {
        self.report_digest_residue_count
            + self.prohibition_audit_residue_count
            + self.support_pinning_residue_count
            + self.test_backend_residue_count
    }

    pub(crate) fn evidence_report(&self) -> &EvidenceReport {
        &self.evidence_report
    }
}
