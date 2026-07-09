use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    precedence_stage_for_boundary, BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDivergenceAxis,
    BridgeSubscriptionCertificationFailureBoundary,
    BridgeSubscriptionCertificationFailurePrecedenceStage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionCertificationComparisonPlanRejectionKind {
    ExpectedRejectionRequiresBoundary,
    IntentionalDivergenceRequiresAxis,
    CounterContractDoesNotTakeFailureBoundary,
}

impl BridgeSubscriptionCertificationComparisonPlanRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpectedRejectionRequiresBoundary => "expected_rejection_requires_boundary",
            Self::IntentionalDivergenceRequiresAxis => "intentional_divergence_requires_axis",
            Self::CounterContractDoesNotTakeFailureBoundary => {
                "counter_contract_does_not_take_failure_boundary"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationComparisonPlanRejection {
    rejection_kind: BridgeSubscriptionCertificationComparisonPlanRejectionKind,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationComparisonPlanRejection {
    fn new(rejection_kind: BridgeSubscriptionCertificationComparisonPlanRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-comparison-plan-rejection|kind={}",
            rejection_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-comparison-plan-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionCertificationComparisonPlanRejectionKind {
        self.rejection_kind
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationComparisonPlan {
    relationship: BridgeSubscriptionCertificationComparisonRelationship,
    expected_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
    expected_failure_precedence_stage:
        Option<BridgeSubscriptionCertificationFailurePrecedenceStage>,
    divergence_axis: Option<BridgeSubscriptionCertificationDivergenceAxis>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationComparisonPlan {
    pub(crate) fn admit(
        relationship: BridgeSubscriptionCertificationComparisonRelationship,
        expected_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
        divergence_axis: Option<BridgeSubscriptionCertificationDivergenceAxis>,
    ) -> Result<Self, BridgeSubscriptionCertificationComparisonPlanRejection> {
        if relationship == BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection
            && expected_failure_boundary.is_none()
        {
            return Err(BridgeSubscriptionCertificationComparisonPlanRejection::new(
                BridgeSubscriptionCertificationComparisonPlanRejectionKind::ExpectedRejectionRequiresBoundary,
            ));
        }
        if relationship
            == BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence
            && divergence_axis.is_none()
        {
            return Err(BridgeSubscriptionCertificationComparisonPlanRejection::new(
                BridgeSubscriptionCertificationComparisonPlanRejectionKind::IntentionalDivergenceRequiresAxis,
            ));
        }
        if relationship == BridgeSubscriptionCertificationComparisonRelationship::CounterContract
            && expected_failure_boundary.is_some()
        {
            return Err(BridgeSubscriptionCertificationComparisonPlanRejection::new(
                BridgeSubscriptionCertificationComparisonPlanRejectionKind::CounterContractDoesNotTakeFailureBoundary,
            ));
        }
        let expected_failure_precedence_stage =
            expected_failure_boundary.map(precedence_stage_for_boundary);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-comparison-plan|relationship={}|expected-boundary={}|expected-stage={}|axis={}",
            relationship.as_str(),
            expected_failure_boundary
                .map(BridgeSubscriptionCertificationFailureBoundary::as_str)
                .unwrap_or("none"),
            expected_failure_precedence_stage
                .map(BridgeSubscriptionCertificationFailurePrecedenceStage::as_str)
                .unwrap_or("none"),
            divergence_axis
                .map(BridgeSubscriptionCertificationDivergenceAxis::as_str)
                .unwrap_or("none"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            relationship,
            expected_failure_boundary,
            expected_failure_precedence_stage,
            divergence_axis,
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_comparison_plan(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-comparison-plan:sha256:{digest:x}"
            )),
        })
    }

    pub fn relationship(&self) -> BridgeSubscriptionCertificationComparisonRelationship {
        self.relationship
    }

    pub fn expected_failure_boundary(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
        self.expected_failure_boundary
    }

    pub fn expected_failure_precedence_stage(
        &self,
    ) -> Option<BridgeSubscriptionCertificationFailurePrecedenceStage> {
        self.expected_failure_precedence_stage
    }

    pub fn divergence_axis(&self) -> Option<BridgeSubscriptionCertificationDivergenceAxis> {
        self.divergence_axis
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
