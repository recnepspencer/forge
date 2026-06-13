use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::activation::SubscriptionActivationInput;
use super::certification::{
    QuerySubscriptionCertificationDenialKind, QuerySubscriptionCertificationError,
};
use super::counters::QuerySubscriptionDeclarationCounters;
use super::evidence_identities::scale_counter_snapshot_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionScaleFixtureSize {
    Small,
    Medium,
    Large,
}

impl QuerySubscriptionScaleFixtureSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionScaleCounterSnapshot {
    fixture_size: QuerySubscriptionScaleFixtureSize,
    fixture_row_count: u64,
    activation_for_reporting: String,
    admission_for_reporting: String,
    counter_digest: String,
    counters: QuerySubscriptionDeclarationCounters,
    snapshot_digest: String,
}

impl QuerySubscriptionScaleCounterSnapshot {
    pub fn from_activation(
        fixture_size: QuerySubscriptionScaleFixtureSize,
        fixture_row_count: u64,
        activation: &SubscriptionActivationInput,
    ) -> Self {
        let activation_for_reporting = activation.activation_for_reporting().to_string();
        let admission_for_reporting = activation.admission_for_reporting().to_string();
        let counters = activation.counters().clone();
        let counter_digest = counters.digest();
        let snapshot_identity = scale_counter_snapshot_identity(
            fixture_size.as_str(),
            fixture_row_count,
            activation.evidence_identity(),
            activation.admission_identity(),
            &counters.evidence_identity(),
        );
        Self {
            fixture_size,
            fixture_row_count,
            activation_for_reporting,
            admission_for_reporting,
            counter_digest,
            counters,
            snapshot_digest: snapshot_identity.as_str().to_string(),
        }
    }

    pub fn fixture_size(&self) -> &QuerySubscriptionScaleFixtureSize {
        &self.fixture_size
    }

    pub fn fixture_row_count(&self) -> u64 {
        self.fixture_row_count
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_for_reporting
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_for_reporting
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }

    pub fn counters(&self) -> &QuerySubscriptionDeclarationCounters {
        &self.counters
    }

    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    #[cfg(test)]
    pub(super) fn with_bridge_slice_count_for_test(
        mut self,
        activation: &SubscriptionActivationInput,
        bridge_slice_count: u64,
    ) -> Self {
        self.counters.bridge_slice_count = bridge_slice_count;
        self.counter_digest = self.counters.digest();
        let snapshot_identity = scale_counter_snapshot_identity(
            self.fixture_size.as_str(),
            self.fixture_row_count,
            activation.evidence_identity(),
            activation.admission_identity(),
            &self.counters.evidence_identity(),
        );
        self.snapshot_digest = snapshot_identity.as_str().to_string();
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionScaleSlopeReport {
    digest: String,
    activation_digest: String,
    admission_digest: String,
    small_snapshot_digest: String,
    medium_snapshot_digest: String,
    large_snapshot_digest: String,
    small_row_count: u64,
    medium_row_count: u64,
    large_row_count: u64,
    structural_counter_digest: String,
}

impl QuerySubscriptionScaleSlopeReport {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn small_snapshot_digest(&self) -> &str {
        &self.small_snapshot_digest
    }

    pub fn medium_snapshot_digest(&self) -> &str {
        &self.medium_snapshot_digest
    }

    pub fn large_snapshot_digest(&self) -> &str {
        &self.large_snapshot_digest
    }

    pub fn small_row_count(&self) -> u64 {
        self.small_row_count
    }

    pub fn medium_row_count(&self) -> u64 {
        self.medium_row_count
    }

    pub fn large_row_count(&self) -> u64 {
        self.large_row_count
    }

    pub fn structural_counter_digest(&self) -> &str {
        &self.structural_counter_digest
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_scale_slope_report_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("activation"),
            self.activation_digest.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("admission"),
            self.admission_digest.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("small_snapshot"),
            self.small_snapshot_digest.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("medium_snapshot"),
            self.medium_snapshot_digest.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("large_snapshot"),
            self.large_snapshot_digest.as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("small_row_count"),
            self.small_row_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("medium_row_count"),
            self.medium_row_count as usize,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("large_row_count"),
            self.large_row_count as usize,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("structural_counter"),
            self.structural_counter_digest.as_str(),
        )
        .seal()
    }
}

pub fn certify_query_subscription_scale_slope(
    small: QuerySubscriptionScaleCounterSnapshot,
    medium: QuerySubscriptionScaleCounterSnapshot,
    large: QuerySubscriptionScaleCounterSnapshot,
) -> Result<QuerySubscriptionScaleSlopeReport, QuerySubscriptionCertificationError> {
    if small.fixture_size != QuerySubscriptionScaleFixtureSize::Small
        || medium.fixture_size != QuerySubscriptionScaleFixtureSize::Medium
        || large.fixture_size != QuerySubscriptionScaleFixtureSize::Large
        || small.fixture_row_count == 0
        || !(small.fixture_row_count < medium.fixture_row_count
            && medium.fixture_row_count < large.fixture_row_count)
        || small.activation_digest() != medium.activation_digest()
        || medium.activation_digest() != large.activation_digest()
        || small.admission_digest() != medium.admission_digest()
        || medium.admission_digest() != large.admission_digest()
        || small.counters != medium.counters
        || medium.counters != large.counters
    {
        return Err(QuerySubscriptionCertificationError::new(
            QuerySubscriptionCertificationDenialKind::ScaleSlopeDrift,
            "subscription structural counters must remain stable across row-count-only fixture scale",
            &[
                format!("small:{}", small.snapshot_digest()),
                format!("medium:{}", medium.snapshot_digest()),
                format!("large:{}", large.snapshot_digest()),
                format!("small_row_count:{}", small.fixture_row_count()),
                format!("medium_row_count:{}", medium.fixture_row_count()),
                format!("large_row_count:{}", large.fixture_row_count()),
                format!("small_activation:{}", small.activation_digest()),
                format!("medium_activation:{}", medium.activation_digest()),
                format!("large_activation:{}", large.activation_digest()),
                format!("small_admission:{}", small.admission_digest()),
                format!("medium_admission:{}", medium.admission_digest()),
                format!("large_admission:{}", large.admission_digest()),
                format!("small_counters:{}", small.counter_digest()),
                format!("medium_counters:{}", medium.counter_digest()),
                format!("large_counters:{}", large.counter_digest()),
            ],
        ));
    }

    let structural_counter_digest = small.counter_digest.clone();
    let activation_digest = small.activation_for_reporting.clone();
    let admission_digest = small.admission_for_reporting.clone();
    let digest = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "query_subscription_scale_slope_report_v1",
    )
    .field_shape(ForgeQueryEvidenceTag::new("activation"), activation_digest.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("admission"), admission_digest.as_str())
    .field_shape(ForgeQueryEvidenceTag::new("small_snapshot"), small.snapshot_digest.as_str())
    .field_shape(
        ForgeQueryEvidenceTag::new("medium_snapshot"),
        medium.snapshot_digest.as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("large_snapshot"), large.snapshot_digest.as_str())
    .field_usize(
        ForgeQueryEvidenceTag::new("small_row_count"),
        small.fixture_row_count as usize,
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("medium_row_count"),
        medium.fixture_row_count as usize,
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("large_row_count"),
        large.fixture_row_count as usize,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("structural_counter"),
        structural_counter_digest.as_str(),
    )
    .seal()
    .as_str()
    .to_string();

    Ok(QuerySubscriptionScaleSlopeReport {
        digest,
        activation_digest,
        admission_digest,
        small_snapshot_digest: small.snapshot_digest,
        medium_snapshot_digest: medium.snapshot_digest,
        large_snapshot_digest: large.snapshot_digest,
        small_row_count: small.fixture_row_count,
        medium_row_count: medium.fixture_row_count,
        large_row_count: large.fixture_row_count,
        structural_counter_digest,
    })
}
