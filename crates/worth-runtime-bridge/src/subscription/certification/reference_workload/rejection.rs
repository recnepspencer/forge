use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::subscription::certification::BridgeSubscriptionReferenceWorkloadLaneKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReferenceWorkloadRejectionKind {
    InsufficientLaneSet,
    MissingAuthoritativeControlLane,
    LaneNotDeclaredByManifest,
    CostProfileRejected,
    BundleAssemblyRejected,
    ComparisonPlanRejected,
    OfflineAuditRejected,
    CoverageProofRejected,
}

impl BridgeSubscriptionReferenceWorkloadRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InsufficientLaneSet => "insufficient_lane_set",
            Self::MissingAuthoritativeControlLane => "missing_authoritative_control_lane",
            Self::LaneNotDeclaredByManifest => "lane_not_declared_by_manifest",
            Self::CostProfileRejected => "cost_profile_rejected",
            Self::BundleAssemblyRejected => "bundle_assembly_rejected",
            Self::ComparisonPlanRejected => "comparison_plan_rejected",
            Self::OfflineAuditRejected => "offline_audit_rejected",
            Self::CoverageProofRejected => "coverage_proof_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReferenceWorkloadRejection {
    rejection_kind: BridgeSubscriptionReferenceWorkloadRejectionKind,
    lane_kind: Option<BridgeSubscriptionReferenceWorkloadLaneKind>,
    detail: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReferenceWorkloadRejection {
    pub(crate) fn new(
        rejection_kind: BridgeSubscriptionReferenceWorkloadRejectionKind,
        lane_kind: Option<BridgeSubscriptionReferenceWorkloadLaneKind>,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let detail = detail.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-reference-workload-rejection|kind={}|lane={}|detail={detail}",
            rejection_kind.as_str(),
            lane_kind
                .map(BridgeSubscriptionReferenceWorkloadLaneKind::as_str)
                .unwrap_or("none"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            lane_kind,
            detail,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-reference-workload-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionReferenceWorkloadRejectionKind {
        self.rejection_kind
    }

    pub fn lane_kind(&self) -> Option<BridgeSubscriptionReferenceWorkloadLaneKind> {
        self.lane_kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
