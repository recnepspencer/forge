use super::{S8LayoutAdmissionRequest, S8LayoutStrategyCapability};
use crate::keyspace::{CompositeKeyOrderingLaw, HashCollisionLaw};
use crate::strategy::S8AdmittedLayoutStrategy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutStrategyRegistrySnapshot {
    admitted_strategy: S8AdmittedLayoutStrategy,
    request: S8LayoutAdmissionRequest,
    granted_capability: S8LayoutStrategyCapability,
    hash_equality_law: Option<HashCollisionLaw>,
    composite_ordering_law: Option<CompositeKeyOrderingLaw>,
}

impl S8LayoutStrategyRegistrySnapshot {
    pub(crate) const fn new(
        admitted_strategy: S8AdmittedLayoutStrategy,
        request: S8LayoutAdmissionRequest,
        granted_capability: S8LayoutStrategyCapability,
        hash_equality_law: Option<HashCollisionLaw>,
        composite_ordering_law: Option<CompositeKeyOrderingLaw>,
    ) -> Self {
        Self {
            admitted_strategy,
            request,
            granted_capability,
            hash_equality_law,
            composite_ordering_law,
        }
    }

    pub const fn admitted_strategy(self) -> S8AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn request(self) -> S8LayoutAdmissionRequest {
        self.request
    }

    pub const fn granted_capability(self) -> S8LayoutStrategyCapability {
        self.granted_capability
    }

    pub const fn hash_equality_law(self) -> Option<HashCollisionLaw> {
        self.hash_equality_law
    }

    pub const fn composite_ordering_law(self) -> Option<CompositeKeyOrderingLaw> {
        self.composite_ordering_law
    }
}
