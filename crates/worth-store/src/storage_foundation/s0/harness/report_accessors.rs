use super::super::artifacts::S0ArtifactEnvelopeMetadata;
use super::maturity::EvidenceBundleReadiness;
use super::report::HarnessMaturityReport;
use super::row::HarnessMaturityRow;

impl HarnessMaturityReport {
    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[HarnessMaturityRow] {
        &self.rows
    }

    pub fn evidence_bundle_readiness(&self) -> EvidenceBundleReadiness {
        self.evidence_bundle_readiness
    }
}
