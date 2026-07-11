use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy_registry::{
    S8LayoutAdmissionDeferred, S8LayoutAdmissionDenial, S8LayoutStrategyRegistrySnapshot,
};

use super::{S8FutureLayoutCapabilityRequest, S8FutureLayoutWorkloadEnvelope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8FutureLayoutCustomizationAdmission {
    request: super::S8FutureLayoutCustomizationRequest,
    registry_snapshot: S8LayoutStrategyRegistrySnapshot,
    layout_admission_transition: crate::production_transition::S8LayoutProductionTransition,
}

impl S8FutureLayoutCustomizationAdmission {
    pub(crate) const fn new(
        request: super::S8FutureLayoutCustomizationRequest,
        registry_snapshot: S8LayoutStrategyRegistrySnapshot,
        layout_admission_transition: crate::production_transition::S8LayoutProductionTransition,
    ) -> Self {
        Self {
            request,
            registry_snapshot,
            layout_admission_transition,
        }
    }

    pub const fn request(self) -> super::S8FutureLayoutCustomizationRequest {
        self.request
    }

    pub const fn registry_snapshot(self) -> S8LayoutStrategyRegistrySnapshot {
        self.registry_snapshot
    }

    pub const fn layout_admission_transition(
        self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        self.layout_admission_transition
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
    StoreAdmissionDenied(S8LayoutAdmissionDenialProjection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutAdmissionDenialProjection {
    denial: S8LayoutAdmissionDenial,
    transition: crate::production_transition::S8LayoutProductionTransition,
}

impl S8LayoutAdmissionDenialProjection {
    pub(crate) const fn new(
        denial: S8LayoutAdmissionDenial,
        transition: crate::production_transition::S8LayoutProductionTransition,
    ) -> Self {
        Self { denial, transition }
    }

    pub const fn denial(self) -> S8LayoutAdmissionDenial {
        self.denial
    }
    pub const fn transition(self) -> crate::production_transition::S8LayoutProductionTransition {
        self.transition
    }
}

impl S8FutureLayoutCustomizationDenial {
    /// Preserves the lower owner fact when customization reached Store layout
    /// admission. Earlier customization denials correctly have no such fact.
    pub const fn layout_admission_transition(
        self,
    ) -> Option<crate::production_transition::S8LayoutProductionTransition> {
        match self {
            Self::StoreAdmissionDenied(projection) => Some(projection.transition()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FutureLayoutCustomizationDeferred {
    StoreAdmissionDeferred(S8LayoutAdmissionDeferred),
}
