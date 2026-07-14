use crate::keyspace::PhysicalKeyDomainWitness;
use crate::strategy::registry::{
    LayoutAdmissionDeferred, LayoutAdmissionDenial, LayoutStrategyRegistrySnapshot,
};

use super::{FutureLayoutCapabilityRequest, FutureLayoutWorkloadEnvelope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureLayoutCustomizationAdmission {
    request: super::FutureLayoutCustomizationRequest,
    registry_snapshot: LayoutStrategyRegistrySnapshot,
}

impl FutureLayoutCustomizationAdmission {
    pub(crate) const fn new(
        request: super::FutureLayoutCustomizationRequest,
        registry_snapshot: LayoutStrategyRegistrySnapshot,
    ) -> Self {
        Self {
            request,
            registry_snapshot,
        }
    }

    pub const fn request(&self) -> super::FutureLayoutCustomizationRequest {
        self.request
    }

    pub const fn registry_snapshot(&self) -> &LayoutStrategyRegistrySnapshot {
        &self.registry_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutureLayoutCustomizationDenial {
    AuthoritySourceDoesNotMatchKeyDomain,
    NoStrategySupportsRequestedCapability {
        capability: FutureLayoutCapabilityRequest,
        key_domain: PhysicalKeyDomainWitness,
    },
    WorkloadEnvelopeDoesNotSupportCapability {
        capability: FutureLayoutCapabilityRequest,
        envelope: FutureLayoutWorkloadEnvelope,
    },
    RebuildableProjectionNotYetSupported {
        key_domain: PhysicalKeyDomainWitness,
    },
    StoreAdmissionDenied(LayoutAdmissionDenialProjection),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutAdmissionDenialProjection {
    denial: LayoutAdmissionDenial,
}

impl LayoutAdmissionDenialProjection {
    pub(crate) const fn new(denial: LayoutAdmissionDenial) -> Self {
        Self { denial }
    }

    pub const fn denial(&self) -> &LayoutAdmissionDenial {
        &self.denial
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutureLayoutCustomizationDeferred {
    StoreAdmissionDeferred(LayoutAdmissionDeferred),
}
