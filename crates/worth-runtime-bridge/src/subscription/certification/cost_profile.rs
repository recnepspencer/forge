use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeSubscriptionCertificationCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionCertificationDensityPosture {
    SparseCertificationWindow,
    BoundedWorkloadWindow,
    DenseCertificationRebuild,
    RejectedOverBudgetCertification,
}

impl BridgeSubscriptionCertificationDensityPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SparseCertificationWindow => "sparse_certification_window",
            Self::BoundedWorkloadWindow => "bounded_workload_window",
            Self::DenseCertificationRebuild => "dense_certification_rebuild",
            Self::RejectedOverBudgetCertification => "rejected_over_budget_certification",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionCertificationCostProfileRejectionKind {
    EmptySourceArtifactBudget,
    EmptyBundleFieldBudget,
    EmptyScratchCapacity,
    OverBudgetPostureRejected,
}

impl BridgeSubscriptionCertificationCostProfileRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySourceArtifactBudget => "empty_source_artifact_budget",
            Self::EmptyBundleFieldBudget => "empty_bundle_field_budget",
            Self::EmptyScratchCapacity => "empty_scratch_capacity",
            Self::OverBudgetPostureRejected => "over_budget_posture_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationCostProfileRejection {
    rejection_kind: BridgeSubscriptionCertificationCostProfileRejectionKind,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationCostProfileRejection {
    fn new(rejection_kind: BridgeSubscriptionCertificationCostProfileRejectionKind) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-cost-profile-rejection|kind={}",
            rejection_kind.as_str()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_cost_profile_rejection(
                rejection_kind
                    == BridgeSubscriptionCertificationCostProfileRejectionKind::OverBudgetPostureRejected,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-cost-profile-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionCertificationCostProfileRejectionKind {
        self.rejection_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationCostProfile {
    density_posture: BridgeSubscriptionCertificationDensityPosture,
    max_source_artifact_entries: usize,
    max_bundle_field_count: usize,
    scratch_capacity: usize,
    rich_diagnostics_admitted: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationCostProfile {
    pub(crate) fn admit(
        density_posture: BridgeSubscriptionCertificationDensityPosture,
        max_source_artifact_entries: usize,
        max_bundle_field_count: usize,
        scratch_capacity: usize,
        rich_diagnostics_admitted: bool,
    ) -> Result<Self, BridgeSubscriptionCertificationCostProfileRejection> {
        if max_source_artifact_entries == 0 {
            return Err(BridgeSubscriptionCertificationCostProfileRejection::new(
                BridgeSubscriptionCertificationCostProfileRejectionKind::EmptySourceArtifactBudget,
            ));
        }
        if max_bundle_field_count == 0 {
            return Err(BridgeSubscriptionCertificationCostProfileRejection::new(
                BridgeSubscriptionCertificationCostProfileRejectionKind::EmptyBundleFieldBudget,
            ));
        }
        if scratch_capacity == 0 {
            return Err(BridgeSubscriptionCertificationCostProfileRejection::new(
                BridgeSubscriptionCertificationCostProfileRejectionKind::EmptyScratchCapacity,
            ));
        }
        if density_posture
            == BridgeSubscriptionCertificationDensityPosture::RejectedOverBudgetCertification
        {
            return Err(BridgeSubscriptionCertificationCostProfileRejection::new(
                BridgeSubscriptionCertificationCostProfileRejectionKind::OverBudgetPostureRejected,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-cost-profile|posture={}|max-source-artifacts={}|max-fields={}|scratch-capacity={}|rich-diagnostics={}",
            density_posture.as_str(),
            max_source_artifact_entries,
            max_bundle_field_count,
            scratch_capacity,
            rich_diagnostics_admitted,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            density_posture,
            max_source_artifact_entries,
            max_bundle_field_count,
            scratch_capacity,
            rich_diagnostics_admitted,
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_cost_profile(
                density_posture
                    == BridgeSubscriptionCertificationDensityPosture::DenseCertificationRebuild,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-cost-profile:sha256:{digest:x}"
            )),
        })
    }

    pub fn density_posture(&self) -> BridgeSubscriptionCertificationDensityPosture {
        self.density_posture
    }

    pub fn max_source_artifact_entries(&self) -> usize {
        self.max_source_artifact_entries
    }

    pub fn max_bundle_field_count(&self) -> usize {
        self.max_bundle_field_count
    }

    pub fn scratch_capacity(&self) -> usize {
        self.scratch_capacity
    }

    pub fn rich_diagnostics_admitted(&self) -> bool {
        self.rich_diagnostics_admitted
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
