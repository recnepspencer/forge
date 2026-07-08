use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy_registry::{
    S8LayoutAdmissionDeferred, S8LayoutAdmissionDenial, S8LayoutStrategyRegistrySnapshot,
};

use super::{S8FutureLayoutCapabilityRequest, S8FutureLayoutWorkloadEnvelope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8FutureLayoutCustomizationAdmission {
    request: super::S8FutureLayoutCustomizationRequest,
    registry_snapshot: S8LayoutStrategyRegistrySnapshot,
}

impl S8FutureLayoutCustomizationAdmission {
    pub(crate) const fn new(
        request: super::S8FutureLayoutCustomizationRequest,
        registry_snapshot: S8LayoutStrategyRegistrySnapshot,
    ) -> Self {
        Self {
            request,
            registry_snapshot,
        }
    }

    pub const fn request(self) -> super::S8FutureLayoutCustomizationRequest {
        self.request
    }

    pub const fn registry_snapshot(self) -> S8LayoutStrategyRegistrySnapshot {
        self.registry_snapshot
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FutureLayoutCustomizationDenial {
    AuthoritySourceDoesNotMatchKeyDomain,
    NoStrategySupportsRequestedCapability {
        capability: S8FutureLayoutCapabilityRequest,
        key_domain: PhysicalKeyDomainWitness,
    },
    WorkloadEnvelopeDoesNotSupportCapability {
        capability: S8FutureLayoutCapabilityRequest,
        envelope: S8FutureLayoutWorkloadEnvelope,
    },
    RebuildableProjectionNotYetSupported {
        key_domain: PhysicalKeyDomainWitness,
    },
    StoreAdmissionDenied(S8LayoutAdmissionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FutureLayoutCustomizationDeferred {
    StoreAdmissionDeferred(S8LayoutAdmissionDeferred),
}
