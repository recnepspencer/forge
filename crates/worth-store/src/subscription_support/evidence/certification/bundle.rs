use super::super::counter_snapshot::SubscriptionSupportCounterSnapshot;
use super::matrix::SubscriptionSupportCertificationMatrix;
use super::outcome::SubscriptionSupportCertificationLaneOutcome;
use crate::failure::StoreError;
use crate::subscription_support::digest::stable_digest;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportCatalog,
    SubscriptionSupportClassificationReport,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationBundle {
    catalog_family_count: usize,
    counter_snapshot: SubscriptionSupportCounterSnapshot,
    classification_digest: String,
    matrix: Option<SubscriptionSupportCertificationMatrix>,
    truth_digest: String,
    artifact_digest: String,
    subscription_support_digest: String,
    replay_digest: String,
    diagnostics_digest: String,
    failure_digest: String,
    counter_digest: String,
}

impl SubscriptionSupportCertificationBundle {
    pub fn new(
        catalog: &SubscriptionSupportCatalog,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
        reports: &[SubscriptionSupportClassificationReport],
    ) -> Result<Self, StoreError> {
        let classification_digest = stable_digest(&reports)?;
        let counter_digest = stable_digest(&counter_snapshot)?;
        let failure_reports = reports
            .iter()
            .filter(|report| report.classification() != SubscriptionResumeClassification::Exact)
            .cloned()
            .collect::<Vec<_>>();
        Ok(Self {
            catalog_family_count: catalog.family_count(),
            counter_snapshot,
            classification_digest: classification_digest.clone(),
            matrix: None,
            truth_digest: classification_digest.clone(),
            artifact_digest: classification_digest.clone(),
            subscription_support_digest: classification_digest.clone(),
            replay_digest: classification_digest.clone(),
            diagnostics_digest: classification_digest,
            failure_digest: stable_digest(&failure_reports)?,
            counter_digest,
        })
    }

    pub fn from_lane_outcomes(
        catalog: &SubscriptionSupportCatalog,
        counter_snapshot: SubscriptionSupportCounterSnapshot,
        reports: &[SubscriptionSupportClassificationReport],
        lane_outcomes: Vec<SubscriptionSupportCertificationLaneOutcome>,
    ) -> Result<Self, StoreError> {
        let matrix = SubscriptionSupportCertificationMatrix::from_lane_outcomes(lane_outcomes)?;
        Ok(Self {
            catalog_family_count: catalog.family_count(),
            counter_snapshot,
            classification_digest: stable_digest(&reports)?,
            truth_digest: stable_digest(&matrix.truth_digests())?,
            artifact_digest: stable_digest(&matrix.artifact_digests())?,
            subscription_support_digest: stable_digest(&matrix.subscription_support_digests())?,
            replay_digest: stable_digest(&matrix.replay_digests())?,
            diagnostics_digest: stable_digest(&matrix.diagnostics_digests())?,
            failure_digest: stable_digest(&matrix.failure_digests())?,
            counter_digest: stable_digest(&matrix.counter_digests())?,
            matrix: Some(matrix),
        })
    }

    pub fn catalog_family_count(&self) -> usize {
        self.catalog_family_count
    }

    pub fn classification_digest(&self) -> &str {
        &self.classification_digest
    }

    pub fn matrix(&self) -> Option<&SubscriptionSupportCertificationMatrix> {
        self.matrix.as_ref()
    }

    pub fn counter_snapshot(&self) -> &SubscriptionSupportCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn subscription_support_digest(&self) -> &str {
        &self.subscription_support_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}
