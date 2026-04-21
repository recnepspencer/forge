use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    AdmittedBridgeSubscription, BridgeRetainedSubscriptionBundle,
    BridgeSubscriptionCounters, BridgeSubscriptionLifecycleIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionLifecycleStateKind {
    ActivationReady,
    Deactivated,
}

impl BridgeSubscriptionLifecycleStateKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivationReady => "activation_ready",
            Self::Deactivated => "deactivated",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionLifecycleRecord {
    lifecycle_identity: BridgeSubscriptionLifecycleIdentity,
    admitted_subscription_identity: super::BridgeAdmittedSubscriptionIdentity,
    state_kind: BridgeSubscriptionLifecycleStateKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionLifecycleRecord {
    pub(crate) fn new(
        admitted: &AdmittedBridgeSubscription,
        state_kind: BridgeSubscriptionLifecycleStateKind,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-lifecycle|admitted={}|state={}",
            admitted.admitted_subscription_identity().as_str(),
            state_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            lifecycle_identity: BridgeSubscriptionLifecycleIdentity::new(format!(
                "bridge-subscription-lifecycle-id:sha256:{digest:x}"
            )),
            admitted_subscription_identity: admitted.admitted_subscription_identity().clone(),
            state_kind,
            counters: BridgeSubscriptionCounters::from_lifecycle_record(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-lifecycle:sha256:{digest:x}")),
        }
    }

    pub fn lifecycle_identity(&self) -> &BridgeSubscriptionLifecycleIdentity {
        &self.lifecycle_identity
    }

    pub fn admitted_subscription_identity(&self) -> &super::BridgeAdmittedSubscriptionIdentity {
        &self.admitted_subscription_identity
    }

    pub fn state_kind(&self) -> BridgeSubscriptionLifecycleStateKind {
        self.state_kind
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
pub struct BridgeSubscriptionActivationReady {
    admitted: AdmittedBridgeSubscription,
    lifecycle_record: BridgeSubscriptionLifecycleRecord,
    retained_bundle: BridgeRetainedSubscriptionBundle,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionActivationReady {
    pub(crate) fn prepare(
        registry_identity: &super::BridgeSubscriptionFamilyRegistryIdentity,
        admitted: &AdmittedBridgeSubscription,
    ) -> Self {
        let lifecycle_record = BridgeSubscriptionLifecycleRecord::new(
            admitted,
            BridgeSubscriptionLifecycleStateKind::ActivationReady,
        );
        let retained_bundle =
            BridgeRetainedSubscriptionBundle::new(registry_identity.clone(), admitted, lifecycle_record.clone());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-activation-ready|admitted={}|lifecycle={}|bundle={}",
            admitted.admitted_subscription_identity().as_str(),
            lifecycle_record.lifecycle_identity().as_str(),
            retained_bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            admitted: admitted.clone(),
            lifecycle_record,
            retained_bundle,
            counters: BridgeSubscriptionCounters::from_lifecycle_record(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-activation-ready:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn deactivate(self) -> BridgeSubscriptionDeactivated {
        BridgeSubscriptionDeactivated::from_activation_ready(self)
    }

    pub fn admitted(&self) -> &AdmittedBridgeSubscription {
        &self.admitted
    }

    pub fn lifecycle_record(&self) -> &BridgeSubscriptionLifecycleRecord {
        &self.lifecycle_record
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionDeactivated {
    admitted: AdmittedBridgeSubscription,
    lifecycle_record: BridgeSubscriptionLifecycleRecord,
    retained_bundle: BridgeRetainedSubscriptionBundle,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionDeactivated {
    fn from_activation_ready(ready: BridgeSubscriptionActivationReady) -> Self {
        let lifecycle_record = BridgeSubscriptionLifecycleRecord::new(
            &ready.admitted,
            BridgeSubscriptionLifecycleStateKind::Deactivated,
        );
        let retained_bundle = BridgeRetainedSubscriptionBundle::new(
            ready.retained_bundle.registry_identity().clone(),
            &ready.admitted,
            lifecycle_record.clone(),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-deactivated|admitted={}|lifecycle={}|bundle={}",
            ready.admitted.admitted_subscription_identity().as_str(),
            lifecycle_record.lifecycle_identity().as_str(),
            retained_bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            admitted: ready.admitted,
            lifecycle_record,
            retained_bundle,
            counters: BridgeSubscriptionCounters::from_lifecycle_record(),
            canonical_basis,
            digest: Arc::from(format!("bridge-subscription-deactivated:sha256:{digest:x}")),
        }
    }

    pub fn admitted(&self) -> &AdmittedBridgeSubscription {
        &self.admitted
    }

    pub fn lifecycle_record(&self) -> &BridgeSubscriptionLifecycleRecord {
        &self.lifecycle_record
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
