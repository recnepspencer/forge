use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    AdmittedBridgeSubscription, BridgeSubscriptionCounters, BridgeSubscriptionLifecycleRecord,
    BridgeSubscriptionReplayIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRetainedSubscriptionBundle {
    registry_identity: super::BridgeSubscriptionFamilyRegistryIdentity,
    admitted: AdmittedBridgeSubscription,
    lifecycle_record: BridgeSubscriptionLifecycleRecord,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeRetainedSubscriptionBundle {
    pub(crate) fn new(
        registry_identity: super::BridgeSubscriptionFamilyRegistryIdentity,
        admitted: &AdmittedBridgeSubscription,
        lifecycle_record: BridgeSubscriptionLifecycleRecord,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-retained-subscription-bundle|registry={}|declaration={}|admitted={}|lifecycle={}",
            registry_identity.as_str(),
            admitted.declaration().digest(),
            admitted.digest(),
            lifecycle_record.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            registry_identity,
            admitted: admitted.clone(),
            lifecycle_record,
            counters: BridgeSubscriptionCounters::from_lifecycle_record(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-retained-subscription-bundle:sha256:{digest:x}"
            )),
        }
    }

    pub fn registry_identity(&self) -> &super::BridgeSubscriptionFamilyRegistryIdentity {
        &self.registry_identity
    }

    pub fn declaration(&self) -> &super::BridgeSubscriptionDeclaration {
        self.admitted.declaration()
    }

    pub fn admitted(&self) -> &AdmittedBridgeSubscription {
        &self.admitted
    }

    pub fn lifecycle_record(&self) -> &BridgeSubscriptionLifecycleRecord {
        &self.lifecycle_record
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReplayMismatchKind {
    RegistryIdentityMismatch,
    LifecycleAdmittedMismatch,
}

impl BridgeSubscriptionReplayMismatchKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryIdentityMismatch => "registry_identity_mismatch",
            Self::LifecycleAdmittedMismatch => "lifecycle_admitted_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReplayMismatch {
    mismatch_kind: BridgeSubscriptionReplayMismatchKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReplayMismatch {
    fn new(
        mismatch_kind: BridgeSubscriptionReplayMismatchKind,
        bundle: &BridgeRetainedSubscriptionBundle,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-replay-mismatch|kind={}|bundle={}",
            mismatch_kind.as_str(),
            bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            mismatch_kind,
            counters: BridgeSubscriptionCounters::from_replay_mismatch(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-replay-mismatch:sha256:{digest:x}"
            )),
        }
    }

    pub fn mismatch_kind(&self) -> BridgeSubscriptionReplayMismatchKind {
        self.mismatch_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReplaySummary {
    replay_identity: BridgeSubscriptionReplayIdentity,
    retained_bundle: BridgeRetainedSubscriptionBundle,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReplaySummary {
    pub(crate) fn replay(
        current_registry_identity: &super::BridgeSubscriptionFamilyRegistryIdentity,
        bundle: &BridgeRetainedSubscriptionBundle,
    ) -> Result<Self, BridgeSubscriptionReplayMismatch> {
        if bundle.registry_identity() != current_registry_identity {
            return Err(BridgeSubscriptionReplayMismatch::new(
                BridgeSubscriptionReplayMismatchKind::RegistryIdentityMismatch,
                bundle,
            ));
        }
        if bundle.lifecycle_record().admitted_subscription_identity()
            != bundle.admitted().admitted_subscription_identity()
        {
            return Err(BridgeSubscriptionReplayMismatch::new(
                BridgeSubscriptionReplayMismatchKind::LifecycleAdmittedMismatch,
                bundle,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-replay-summary|registry={}|bundle={}",
            current_registry_identity.as_str(),
            bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            replay_identity: BridgeSubscriptionReplayIdentity::admit_bridge_owned(format!(
                "bridge-subscription-replay-id:sha256:{digest:x}"
            )),
            retained_bundle: bundle.clone(),
            counters: BridgeSubscriptionCounters::from_replay_reconstruction(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-replay-summary:sha256:{digest:x}"
            )),
        })
    }

    pub fn replay_identity(&self) -> &BridgeSubscriptionReplayIdentity {
        &self.replay_identity
    }

    pub fn retained_bundle(&self) -> &BridgeRetainedSubscriptionBundle {
        &self.retained_bundle
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
#[path = "replay_tests.rs"]
mod replay_tests;
