use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::AdmittedTemporalBridgeSubscription;
use crate::subscription::{
    BridgeSubscriptionActivationReady, BridgeSubscriptionCounters,
    BridgeSubscriptionFamilyRegistryIdentity, BridgeSubscriptionTemporalActivationReadyIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalSubscriptionActivationReady {
    temporal_activation_ready_identity: BridgeSubscriptionTemporalActivationReadyIdentity,
    ordinary_activation_ready: BridgeSubscriptionActivationReady,
    temporal_admission: AdmittedTemporalBridgeSubscription,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalSubscriptionActivationReady {
    pub(crate) fn prepare(
        registry_identity: &BridgeSubscriptionFamilyRegistryIdentity,
        temporal_admission: &AdmittedTemporalBridgeSubscription,
    ) -> Self {
        let ordinary_activation_ready = BridgeSubscriptionActivationReady::prepare(
            registry_identity,
            temporal_admission.admitted(),
        );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-temporal-subscription-activation-ready|ordinary={}|temporal-admission={}|temporal-basis={}|family={}",
            ordinary_activation_ready.digest(),
            temporal_admission.temporal_admission_identity().as_str(),
            temporal_admission.temporal_basis().identity().as_str(),
            temporal_admission.family().kind().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            temporal_activation_ready_identity:
                BridgeSubscriptionTemporalActivationReadyIdentity::admit_bridge_owned(format!(
                    "bridge-temporal-subscription-activation-ready-id:sha256:{digest:x}"
                )),
            ordinary_activation_ready,
            temporal_admission: temporal_admission.clone(),
            counters: BridgeSubscriptionCounters::from_temporal_activation_ready(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-temporal-subscription-activation-ready:sha256:{digest:x}"
            )),
        }
    }

    pub fn temporal_activation_ready_identity(
        &self,
    ) -> &BridgeSubscriptionTemporalActivationReadyIdentity {
        &self.temporal_activation_ready_identity
    }

    pub fn ordinary_activation_ready(&self) -> &BridgeSubscriptionActivationReady {
        &self.ordinary_activation_ready
    }

    pub fn temporal_admission(&self) -> &AdmittedTemporalBridgeSubscription {
        &self.temporal_admission
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
