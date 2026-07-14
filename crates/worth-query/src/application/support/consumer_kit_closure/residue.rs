use crate::WorthQueryEvidenceIdentity;

use super::evidence::{
    consumer_kit_reference_residue_identity, consumer_kit_residue_breakdown_identity,
};

const QUERY_OWNED_BACKEND_APPLICABILITY: &str =
    "worth-query consumer-kit closure no longer certifies downstream consumer residue inside Query authority";
const QUERY_OWNED_RESIDUE_SOURCE_DIGEST: &str = "query-owned-consumer-kit-residue:none";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitReferenceResidue {
    query_owned_residue_count: usize,
    defended_residue_count: usize,
    breakdown: WorthQueryConsumerKitResidueBreakdown,
    backend_applicability: &'static str,
    backend_applicability_certified: bool,
    residue_source_digest: String,
    residue_identity: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitResidueBreakdown {
    report_digest_residue_count: usize,
    prohibition_audit_residue_count: usize,
    support_pinning_residue_count: usize,
    test_backend_residue_count: usize,
    defended_worth_domain_residue_count: usize,
    breakdown_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryConsumerKitResidueBreakdown {
    fn current() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }

    #[cfg(test)]
    pub(crate) fn new(
        report_digest_residue_count: usize,
        prohibition_audit_residue_count: usize,
        support_pinning_residue_count: usize,
        test_backend_residue_count: usize,
        defended_worth_domain_residue_count: usize,
    ) -> Self {
        let breakdown_identity = consumer_kit_residue_breakdown_identity(
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
        );
        Self {
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
            breakdown_identity,
        }
    }

    #[cfg(not(test))]
    fn new(
        report_digest_residue_count: usize,
        prohibition_audit_residue_count: usize,
        support_pinning_residue_count: usize,
        test_backend_residue_count: usize,
        defended_worth_domain_residue_count: usize,
    ) -> Self {
        let breakdown_identity = consumer_kit_residue_breakdown_identity(
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
        );
        Self {
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
            breakdown_identity,
        }
    }

    pub fn report_digest_residue_count(&self) -> usize {
        self.report_digest_residue_count
    }

    pub fn prohibition_audit_residue_count(&self) -> usize {
        self.prohibition_audit_residue_count
    }

    pub fn support_pinning_residue_count(&self) -> usize {
        self.support_pinning_residue_count
    }

    pub fn test_backend_residue_count(&self) -> usize {
        self.test_backend_residue_count
    }

    pub fn defended_worth_domain_residue_count(&self) -> usize {
        self.defended_worth_domain_residue_count
    }

    pub fn query_owned_residue_count(&self) -> usize {
        self.report_digest_residue_count
            + self.prohibition_audit_residue_count
            + self.support_pinning_residue_count
            + self.test_backend_residue_count
    }

    pub fn breakdown_digest(&self) -> &str {
        self.breakdown_identity.as_str()
    }
}

impl WorthQueryConsumerKitReferenceResidue {
    pub(crate) fn current() -> Self {
        Self::new_with_certification(
            0,
            0,
            WorthQueryConsumerKitResidueBreakdown::current(),
            QUERY_OWNED_BACKEND_APPLICABILITY,
            true,
            QUERY_OWNED_RESIDUE_SOURCE_DIGEST.to_owned(),
        )
    }

    #[cfg(test)]
    pub(crate) fn new(
        query_owned_residue_count: usize,
        defended_residue_count: usize,
        backend_applicability: &'static str,
    ) -> Self {
        Self::new_with_certification(
            query_owned_residue_count,
            defended_residue_count,
            WorthQueryConsumerKitResidueBreakdown::new(
                query_owned_residue_count,
                0,
                0,
                0,
                defended_residue_count,
            ),
            backend_applicability,
            query_owned_residue_count == 0,
            "manual-reference-residue-sabotage".to_owned(),
        )
    }

    fn new_with_certification(
        query_owned_residue_count: usize,
        defended_residue_count: usize,
        breakdown: WorthQueryConsumerKitResidueBreakdown,
        backend_applicability: &'static str,
        backend_applicability_certified: bool,
        residue_source_digest: String,
    ) -> Self {
        let residue_identity = consumer_kit_reference_residue_identity(
            query_owned_residue_count,
            defended_residue_count,
            &breakdown,
            backend_applicability,
            backend_applicability_certified,
            &residue_source_digest,
        );
        Self {
            query_owned_residue_count,
            defended_residue_count,
            breakdown,
            backend_applicability,
            backend_applicability_certified,
            residue_source_digest,
            residue_identity,
        }
    }

    pub fn query_owned_residue_count(&self) -> usize {
        self.query_owned_residue_count
    }

    pub fn defended_residue_count(&self) -> usize {
        self.defended_residue_count
    }

    pub fn breakdown(&self) -> &WorthQueryConsumerKitResidueBreakdown {
        &self.breakdown
    }

    pub fn backend_applicability(&self) -> &'static str {
        self.backend_applicability
    }

    pub fn backend_applicability_certified(&self) -> bool {
        self.backend_applicability_certified
    }

    pub fn residue_source_digest(&self) -> &str {
        &self.residue_source_digest
    }

    pub fn is_query_owned_clean(&self) -> bool {
        self.query_owned_residue_count == 0 && self.backend_applicability_certified
    }

    pub fn residue_digest(&self) -> &str {
        self.residue_identity.as_str()
    }

    pub fn residue_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.residue_identity
    }
}
