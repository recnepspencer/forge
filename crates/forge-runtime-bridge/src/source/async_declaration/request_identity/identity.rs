use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{AsyncInFlightRequestIdentityTag, AsyncRequestIdentityTag, BridgeIdentity};
use forge_signal::facade::{InFlightResourceRequest, ResourceAttemptId, ResourceRequestHandle};

use super::super::LoweredBridgeAsyncSourceDeclaration;
use super::binding::ValidatedBridgeAsyncRequestBasisBinding;
use super::counters::BridgeAsyncRequestIdentityCounters;
use super::subscription_instance::BridgeAsyncRequestSubscriptionInstance;

pub(super) type BridgeAsyncRequestIdentity = BridgeIdentity<AsyncRequestIdentityTag>;
pub(super) type BridgeAsyncInFlightRequestIdentityHandle =
    BridgeIdentity<AsyncInFlightRequestIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeAsyncRequestFamilyAdmission {
    RequestResponse,
    SubscriptionBacked {
        subscription_instance: BridgeAsyncRequestSubscriptionInstance,
    },
}

impl BridgeAsyncRequestFamilyAdmission {
    pub fn subscription_instance(&self) -> Option<&BridgeAsyncRequestSubscriptionInstance> {
        match self {
            Self::RequestResponse => None,
            Self::SubscriptionBacked {
                subscription_instance,
            } => Some(subscription_instance),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncRequestAdmissionRequest {
    lowered: LoweredBridgeAsyncSourceDeclaration,
    basis_binding: ValidatedBridgeAsyncRequestBasisBinding,
    family_admission: BridgeAsyncRequestFamilyAdmission,
}

impl BridgeAsyncRequestAdmissionRequest {
    pub(super) fn new(
        lowered: LoweredBridgeAsyncSourceDeclaration,
        basis_binding: ValidatedBridgeAsyncRequestBasisBinding,
        family_admission: BridgeAsyncRequestFamilyAdmission,
    ) -> Self {
        Self {
            lowered,
            basis_binding,
            family_admission,
        }
    }

    pub(crate) fn rebind(
        lowered: &LoweredBridgeAsyncSourceDeclaration,
        basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
        family_admission: &BridgeAsyncRequestFamilyAdmission,
    ) -> Result<Self, super::rejection::BridgeAsyncRequestIdentityRejection> {
        match family_admission {
            BridgeAsyncRequestFamilyAdmission::RequestResponse => {
                Self::request_response(lowered, basis_binding)
            }
            BridgeAsyncRequestFamilyAdmission::SubscriptionBacked {
                subscription_instance,
            } => Self::subscription_backed(lowered, basis_binding, subscription_instance.clone()),
        }
    }

    pub fn lowered(&self) -> &LoweredBridgeAsyncSourceDeclaration {
        &self.lowered
    }

    pub fn basis_binding(&self) -> &ValidatedBridgeAsyncRequestBasisBinding {
        &self.basis_binding
    }

    pub fn family_admission(&self) -> &BridgeAsyncRequestFamilyAdmission {
        &self.family_admission
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncInFlightRequestIdentity {
    in_flight_identity: BridgeAsyncInFlightRequestIdentityHandle,
    request_identity: BridgeAsyncRequestIdentity,
    in_flight: InFlightResourceRequest,
    counters: BridgeAsyncRequestIdentityCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncInFlightRequestIdentity {
    pub(super) fn new(
        request_identity: &BridgeAsyncRequestIdentity,
        in_flight: InFlightResourceRequest,
        counters: BridgeAsyncRequestIdentityCounters,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-in-flight-request|request={}|handle={}#{}|attempt={}|status={:?}|intent={}",
            request_identity.as_str(),
            in_flight.handle().request_id().get(),
            in_flight.handle().generation().get(),
            in_flight.attempt().get(),
            in_flight.status(),
            in_flight.request_intent_digest().as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            in_flight_identity: BridgeAsyncInFlightRequestIdentityHandle::new(format!(
                "bridge-async-in-flight-request-id:sha256:{digest:x}"
            )),
            request_identity: request_identity.clone(),
            in_flight,
            counters,
            canonical_basis,
            digest: Arc::from(format!("bridge-async-in-flight-request:sha256:{digest:x}")),
        }
    }

    pub fn in_flight_identity(&self) -> &BridgeAsyncInFlightRequestIdentityHandle {
        &self.in_flight_identity
    }

    pub fn request_identity(&self) -> &BridgeAsyncRequestIdentity {
        &self.request_identity
    }

    pub fn in_flight(&self) -> &InFlightResourceRequest {
        &self.in_flight
    }

    pub fn counters(&self) -> &BridgeAsyncRequestIdentityCounters {
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
pub struct AdmittedBridgeAsyncRequestIdentity {
    request_identity: BridgeAsyncRequestIdentity,
    lowered: LoweredBridgeAsyncSourceDeclaration,
    basis_binding: ValidatedBridgeAsyncRequestBasisBinding,
    family_admission: BridgeAsyncRequestFamilyAdmission,
    request_handle: ResourceRequestHandle,
    attempt: ResourceAttemptId,
    request_intent_digest: Arc<str>,
    in_flight_identity: BridgeAsyncInFlightRequestIdentity,
    counters: BridgeAsyncRequestIdentityCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl AdmittedBridgeAsyncRequestIdentity {
    pub(super) fn new(
        request_identity: BridgeAsyncRequestIdentity,
        lowered: LoweredBridgeAsyncSourceDeclaration,
        basis_binding: ValidatedBridgeAsyncRequestBasisBinding,
        family_admission: BridgeAsyncRequestFamilyAdmission,
        request_handle: ResourceRequestHandle,
        attempt: ResourceAttemptId,
        request_intent_digest: Arc<str>,
        in_flight_identity: BridgeAsyncInFlightRequestIdentity,
        counters: BridgeAsyncRequestIdentityCounters,
        canonical_basis: Arc<str>,
        digest: Arc<str>,
    ) -> Self {
        Self {
            request_identity,
            lowered,
            basis_binding,
            family_admission,
            request_handle,
            attempt,
            request_intent_digest,
            in_flight_identity,
            counters,
            canonical_basis,
            digest,
        }
    }

    pub fn request_identity(&self) -> &BridgeAsyncRequestIdentity {
        &self.request_identity
    }

    pub fn lowered(&self) -> &LoweredBridgeAsyncSourceDeclaration {
        &self.lowered
    }

    pub fn basis_binding(&self) -> &ValidatedBridgeAsyncRequestBasisBinding {
        &self.basis_binding
    }

    pub fn family_admission(&self) -> &BridgeAsyncRequestFamilyAdmission {
        &self.family_admission
    }

    pub fn subscription_instance(&self) -> Option<&BridgeAsyncRequestSubscriptionInstance> {
        self.family_admission.subscription_instance()
    }

    pub fn request_handle(&self) -> ResourceRequestHandle {
        self.request_handle
    }

    pub fn attempt(&self) -> ResourceAttemptId {
        self.attempt
    }

    pub fn request_intent_digest(&self) -> &str {
        self.request_intent_digest.as_ref()
    }

    pub fn in_flight_identity(&self) -> &BridgeAsyncInFlightRequestIdentity {
        &self.in_flight_identity
    }

    pub fn counters(&self) -> &BridgeAsyncRequestIdentityCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
