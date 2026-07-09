use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeSubscriptionBundleField, BridgeSubscriptionBundleFieldState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationCompletenessReport {
    required_field_count: usize,
    present_field_count: usize,
    not_exercised_field_count: usize,
    rejected_before_produced_field_count: usize,
    unavailable_prior_artifact_field_count: usize,
    unavailable_schema_divergent_field_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationCompletenessReport {
    pub(crate) fn from_fields(
        required_field_count: usize,
        fields: &[BridgeSubscriptionBundleField],
    ) -> Self {
        let mut present_field_count = 0;
        let mut not_exercised_field_count = 0;
        let mut rejected_before_produced_field_count = 0;
        let mut unavailable_prior_artifact_field_count = 0;
        let mut unavailable_schema_divergent_field_count = 0;
        for field in fields {
            match field.field_state() {
                BridgeSubscriptionBundleFieldState::Present => present_field_count += 1,
                BridgeSubscriptionBundleFieldState::NotExercised => not_exercised_field_count += 1,
                BridgeSubscriptionBundleFieldState::RejectedBeforeProduced => {
                    rejected_before_produced_field_count += 1;
                }
                BridgeSubscriptionBundleFieldState::UnavailableBecausePriorArtifactMissing => {
                    unavailable_prior_artifact_field_count += 1;
                }
                BridgeSubscriptionBundleFieldState::UnavailableBecauseSchemaDivergent => {
                    unavailable_schema_divergent_field_count += 1;
                }
            }
        }
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-subscription-certification-completeness|required={}|",
                "present={}|not-exercised={}|rejected-before-produced={}|",
                "unavailable-prior-artifact={}|unavailable-schema-divergent={}"
            ),
            required_field_count,
            present_field_count,
            not_exercised_field_count,
            rejected_before_produced_field_count,
            unavailable_prior_artifact_field_count,
            unavailable_schema_divergent_field_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            required_field_count,
            present_field_count,
            not_exercised_field_count,
            rejected_before_produced_field_count,
            unavailable_prior_artifact_field_count,
            unavailable_schema_divergent_field_count,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-completeness:sha256:{digest:x}"
            )),
        }
    }

    pub fn required_field_count(&self) -> usize {
        self.required_field_count
    }

    pub fn present_field_count(&self) -> usize {
        self.present_field_count
    }

    pub fn not_exercised_field_count(&self) -> usize {
        self.not_exercised_field_count
    }

    pub fn rejected_before_produced_field_count(&self) -> usize {
        self.rejected_before_produced_field_count
    }

    pub fn unavailable_prior_artifact_field_count(&self) -> usize {
        self.unavailable_prior_artifact_field_count
    }

    pub fn unavailable_schema_divergent_field_count(&self) -> usize {
        self.unavailable_schema_divergent_field_count
    }

    pub fn accounted_field_count(&self) -> usize {
        self.present_field_count
            + self.not_exercised_field_count
            + self.rejected_before_produced_field_count
            + self.unavailable_prior_artifact_field_count
            + self.unavailable_schema_divergent_field_count
    }

    pub fn is_sufficient(&self) -> bool {
        self.accounted_field_count() >= self.required_field_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
