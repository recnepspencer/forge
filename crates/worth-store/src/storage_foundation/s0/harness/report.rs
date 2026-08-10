use super::super::artifacts::S0ArtifactEnvelopeMetadata;
use super::maturity::EvidenceBundleReadiness;
use super::row::HarnessMaturityRow;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HarnessMaturityReport {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) rows: Vec<HarnessMaturityRow>,
    pub(super) evidence_bundle_readiness: EvidenceBundleReadiness,
}
