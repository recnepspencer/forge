use super::{LayoutAdmissionRequest, LayoutStrategyCapability};
use crate::keyspace::{CompositeKeyOrderingLaw, HashCollisionLaw};
use crate::strategy::AdmittedLayoutStrategy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutStrategyRegistrySnapshot {
    admitted_strategy: AdmittedLayoutStrategy,
    request: LayoutAdmissionRequest,
    granted_capability: LayoutStrategyCapability,
    hash_equality_law: Option<HashCollisionLaw>,
    composite_ordering_law: Option<CompositeKeyOrderingLaw>,
}

impl LayoutStrategyRegistrySnapshot {
    pub(crate) const fn new(
        admitted_strategy: AdmittedLayoutStrategy,
        request: LayoutAdmissionRequest,
        granted_capability: LayoutStrategyCapability,
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

    pub const fn admitted_strategy(&self) -> AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn request(&self) -> &LayoutAdmissionRequest {
        &self.request
    }

    pub const fn granted_capability(&self) -> LayoutStrategyCapability {
        self.granted_capability
    }

    pub const fn hash_equality_law(&self) -> Option<HashCollisionLaw> {
        self.hash_equality_law
    }

    pub const fn composite_ordering_law(&self) -> Option<CompositeKeyOrderingLaw> {
        self.composite_ordering_law
    }
}
