use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationCostProfile,
    BridgeSubscriptionCertificationCostProfileRejection,
    BridgeSubscriptionCertificationCostProfileRejectionKind,
    BridgeSubscriptionCertificationCounterSnapshot, BridgeSubscriptionCertificationDensityPosture,
    BridgeSubscriptionCertificationScratch,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationCostPostureReport {
    sparse_cost_profile_digest: Arc<str>,
    dense_cost_profile_digest: Arc<str>,
    over_budget_rejection_digest: Arc<str>,
    first_scratch_digest: Arc<str>,
    repeated_scratch_digest: Arc<str>,
    dense_selected_before_assembly: bool,
    over_budget_rejected_before_assembly: bool,
    scratch_lifecycle_reuse_visible: bool,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationCostPostureReport {
    pub(crate) fn certify() -> Self {
        let sparse_cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
            BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            16,
            16,
            32,
            false,
        )
        .expect("sparse certification posture is admitted by construction");
        let dense_cost_profile = BridgeSubscriptionCertificationCostProfile::admit(
            BridgeSubscriptionCertificationDensityPosture::DenseCertificationRebuild,
            128,
            128,
            256,
            false,
        )
        .expect("dense certification posture is admitted by construction");
        let over_budget_rejection = BridgeSubscriptionCertificationCostProfile::admit(
            BridgeSubscriptionCertificationDensityPosture::RejectedOverBudgetCertification,
            16,
            16,
            32,
            false,
        )
        .expect_err("over-budget certification posture must reject before assembly");
        let first_scratch = BridgeSubscriptionCertificationScratch::prepare(&sparse_cost_profile);
        let repeated_scratch = BridgeSubscriptionCertificationScratch::reuse_from(&first_scratch);
        Self::from_certified_parts(
            &sparse_cost_profile,
            &dense_cost_profile,
            &over_budget_rejection,
            &first_scratch,
            &repeated_scratch,
        )
    }

    fn from_certified_parts(
        sparse_cost_profile: &BridgeSubscriptionCertificationCostProfile,
        dense_cost_profile: &BridgeSubscriptionCertificationCostProfile,
        over_budget_rejection: &BridgeSubscriptionCertificationCostProfileRejection,
        first_scratch: &BridgeSubscriptionCertificationScratch,
        repeated_scratch: &BridgeSubscriptionCertificationScratch,
    ) -> Self {
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *sparse_cost_profile.counters(),
            *dense_cost_profile.counters(),
            *over_budget_rejection.counters(),
            *first_scratch.counters(),
            *repeated_scratch.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_cost_posture_report(),
        ]);
        let dense_selected_before_assembly = dense_cost_profile.density_posture()
            == BridgeSubscriptionCertificationDensityPosture::DenseCertificationRebuild
            && dense_cost_profile.counters().dense_rebuild_count() == 1
            && dense_cost_profile.counters().certification_bundle_count() == 0;
        let over_budget_rejected_before_assembly = over_budget_rejection.rejection_kind()
            == BridgeSubscriptionCertificationCostProfileRejectionKind::OverBudgetPostureRejected
            && over_budget_rejection
                .counters()
                .over_budget_rejection_count()
                == 1
            && over_budget_rejection
                .counters()
                .certification_bundle_count()
                == 0;
        let scratch_lifecycle_reuse_visible = first_scratch.digest() == repeated_scratch.digest()
            && first_scratch.scratch_capacity() == repeated_scratch.scratch_capacity()
            && counters.scratch_allocation_count() == 1
            && counters.scratch_reuse_count() == 2;
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-cost-posture-report|sparse={}|dense={}|over-budget={}|first-scratch={}|repeated-scratch={}|dense-before-assembly={dense_selected_before_assembly}|over-budget-before-assembly={over_budget_rejected_before_assembly}|scratch-reuse-visible={scratch_lifecycle_reuse_visible}|counters={}",
            sparse_cost_profile.digest(),
            dense_cost_profile.digest(),
            over_budget_rejection.digest(),
            first_scratch.digest(),
            repeated_scratch.digest(),
            counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            sparse_cost_profile_digest: Arc::from(sparse_cost_profile.digest()),
            dense_cost_profile_digest: Arc::from(dense_cost_profile.digest()),
            over_budget_rejection_digest: Arc::from(over_budget_rejection.digest()),
            first_scratch_digest: Arc::from(first_scratch.digest()),
            repeated_scratch_digest: Arc::from(repeated_scratch.digest()),
            dense_selected_before_assembly,
            over_budget_rejected_before_assembly,
            scratch_lifecycle_reuse_visible,
            counters,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-cost-posture-report:sha256:{digest:x}"
            )),
        }
    }

    pub fn sparse_cost_profile_digest(&self) -> &str {
        self.sparse_cost_profile_digest.as_ref()
    }

    pub fn dense_cost_profile_digest(&self) -> &str {
        self.dense_cost_profile_digest.as_ref()
    }

    pub fn over_budget_rejection_digest(&self) -> &str {
        self.over_budget_rejection_digest.as_ref()
    }

    pub fn first_scratch_digest(&self) -> &str {
        self.first_scratch_digest.as_ref()
    }

    pub fn repeated_scratch_digest(&self) -> &str {
        self.repeated_scratch_digest.as_ref()
    }

    pub fn dense_selected_before_assembly(&self) -> bool {
        self.dense_selected_before_assembly
    }

    pub fn over_budget_rejected_before_assembly(&self) -> bool {
        self.over_budget_rejected_before_assembly
    }

    pub fn scratch_lifecycle_reuse_visible(&self) -> bool {
        self.scratch_lifecycle_reuse_visible
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
