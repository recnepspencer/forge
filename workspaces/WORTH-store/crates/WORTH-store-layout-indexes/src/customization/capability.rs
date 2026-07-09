use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy::S8LayoutStrategyFamily;
use crate::strategy_registry::S8LayoutRequestedCapability;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8FutureLayoutCapabilityRequest {
    PointLookup {
        key_domain: PhysicalKeyDomainWitness,
    },
    OrderedRange {
        key_domain: PhysicalKeyDomainWitness,
    },
    PrefixTraversal {
        key_domain: PhysicalKeyDomainWitness,
    },
    BlobStreaming {
        key_domain: PhysicalKeyDomainWitness,
    },
    VerifierDeclaredScan {
        key_domain: PhysicalKeyDomainWitness,
    },
    RebuildableProjection {
        key_domain: PhysicalKeyDomainWitness,
    },
}

impl S8FutureLayoutCapabilityRequest {
    pub const fn point_lookup(key_domain: PhysicalKeyDomainWitness) -> Self {
        Self::PointLookup { key_domain }
    }

    pub const fn ordered_range(key_domain: PhysicalKeyDomainWitness) -> Self {
        Self::OrderedRange { key_domain }
    }

    pub const fn prefix_traversal(key_domain: PhysicalKeyDomainWitness) -> Self {
        Self::PrefixTraversal { key_domain }
    }

    pub const fn blob_streaming(key_domain: PhysicalKeyDomainWitness) -> Self {
        Self::BlobStreaming { key_domain }
    }

    pub const fn verifier_declared_scan(key_domain: PhysicalKeyDomainWitness) -> Self {
        Self::VerifierDeclaredScan { key_domain }
    }

    pub const fn rebuildable_projection(key_domain: PhysicalKeyDomainWitness) -> Self {
        Self::RebuildableProjection { key_domain }
    }

    pub const fn key_domain(self) -> PhysicalKeyDomainWitness {
        match self {
            Self::PointLookup { key_domain }
            | Self::OrderedRange { key_domain }
            | Self::PrefixTraversal { key_domain }
            | Self::BlobStreaming { key_domain }
            | Self::VerifierDeclaredScan { key_domain }
            | Self::RebuildableProjection { key_domain } => key_domain,
        }
    }

    pub const fn phase_eight_capability(self) -> Option<S8LayoutRequestedCapability> {
        match self {
            Self::PointLookup { .. } => Some(S8LayoutRequestedCapability::PointLookup),
            Self::OrderedRange { .. } => Some(S8LayoutRequestedCapability::OrderedRange),
            Self::PrefixTraversal { .. } => Some(S8LayoutRequestedCapability::PrefixTraversal),
            Self::BlobStreaming { .. } => Some(S8LayoutRequestedCapability::BlobStreaming),
            Self::VerifierDeclaredScan { .. } => Some(S8LayoutRequestedCapability::ExactScan),
            Self::RebuildableProjection { .. } => None,
        }
    }

    pub(crate) const fn admitted_strategy_family(self) -> Option<S8LayoutStrategyFamily> {
        match self {
            Self::PointLookup { key_domain }
            | Self::OrderedRange { key_domain }
            | Self::PrefixTraversal { key_domain } => match key_domain.domain() {
                crate::PhysicalKeyDomain::PageAddressKey
                | crate::PhysicalKeyDomain::SegmentAddressKey
                | crate::PhysicalKeyDomain::ExtentAddressKey
                | crate::PhysicalKeyDomain::PhysicalReferenceKey => {
                    Some(S8LayoutStrategyFamily::BaselineBTreeRange)
                }
                crate::PhysicalKeyDomain::WalRecordKey
                | crate::PhysicalKeyDomain::BlobIdentityKey => {
                    Some(S8LayoutStrategyFamily::BaselineLsmWriteOptimized)
                }
                crate::PhysicalKeyDomain::RootManifestKey => None,
            },
            Self::BlobStreaming { key_domain } | Self::VerifierDeclaredScan { key_domain } => {
                match key_domain.domain() {
                    crate::PhysicalKeyDomain::WalRecordKey
                    | crate::PhysicalKeyDomain::BlobIdentityKey => {
                        Some(S8LayoutStrategyFamily::BaselineLsmWriteOptimized)
                    }
                    crate::PhysicalKeyDomain::RootManifestKey
                    | crate::PhysicalKeyDomain::PageAddressKey
                    | crate::PhysicalKeyDomain::SegmentAddressKey
                    | crate::PhysicalKeyDomain::ExtentAddressKey
                    | crate::PhysicalKeyDomain::PhysicalReferenceKey => None,
                }
            }
            Self::RebuildableProjection { .. } => None,
        }
    }
}
