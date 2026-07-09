use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionCertificationCostProfile, BridgeSubscriptionCertificationCounterSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationScratch {
    scratch_capacity: usize,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationScratch {
    pub(crate) fn prepare(cost_profile: &BridgeSubscriptionCertificationCostProfile) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-scratch|cost-profile={}|capacity={}",
            cost_profile.digest(),
            cost_profile.scratch_capacity(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            scratch_capacity: cost_profile.scratch_capacity(),
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_scratch(1),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-scratch:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn reuse_from(existing: &Self) -> Self {
        Self {
            scratch_capacity: existing.scratch_capacity,
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_scratch_reuse(),
            canonical_basis: Arc::clone(&existing.canonical_basis),
            digest: Arc::clone(&existing.digest),
        }
    }

    pub fn scratch_capacity(&self) -> usize {
        self.scratch_capacity
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
