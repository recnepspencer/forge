use super::FutureLayoutTarget;
use worth_store_layout_indexes::PhysicalKeyDomainWitness;
use worth_store_layout_indexes::{S8FutureLayoutCapabilityRequest, S8FutureLayoutWorkloadEnvelope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionFamilyPosture {
    Registered,
    RebuildRequired,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureLayoutTargetDeclaration {
    target: FutureLayoutTarget,
    posture: ExtensionFamilyPosture,
    declared_domain: PhysicalKeyDomainWitness,
}

impl FutureLayoutTargetDeclaration {
    pub(crate) const fn new(
        target: FutureLayoutTarget,
        posture: ExtensionFamilyPosture,
        declared_domain: PhysicalKeyDomainWitness,
    ) -> Self {
        Self {
            target,
            posture,
            declared_domain,
        }
    }

    pub const fn target(self) -> FutureLayoutTarget {
        self.target
    }

    pub const fn posture(self) -> ExtensionFamilyPosture {
        self.posture
    }

    pub const fn declared_domain(self) -> PhysicalKeyDomainWitness {
        self.declared_domain
    }

    pub const fn capability_request(self) -> S8FutureLayoutCapabilityRequest {
        match self.target {
            FutureLayoutTarget::StableBasisRead => {
                S8FutureLayoutCapabilityRequest::point_lookup(self.declared_domain)
            }
            FutureLayoutTarget::AspectProjection => {
                S8FutureLayoutCapabilityRequest::rebuildable_projection(self.declared_domain)
            }
            FutureLayoutTarget::SubscriptionSupport => {
                S8FutureLayoutCapabilityRequest::ordered_range(self.declared_domain)
            }
            FutureLayoutTarget::SupportTrust => {
                S8FutureLayoutCapabilityRequest::verifier_declared_scan(self.declared_domain)
            }
        }
    }

    pub const fn workload_envelope(self) -> S8FutureLayoutWorkloadEnvelope {
        match self.target {
            FutureLayoutTarget::StableBasisRead => {
                S8FutureLayoutWorkloadEnvelope::foreground_low_fanout()
            }
            FutureLayoutTarget::AspectProjection => {
                S8FutureLayoutWorkloadEnvelope::background_rebuild_projection()
            }
            FutureLayoutTarget::SubscriptionSupport => {
                S8FutureLayoutWorkloadEnvelope::foreground_bounded_traversal()
            }
            FutureLayoutTarget::SupportTrust => {
                S8FutureLayoutWorkloadEnvelope::verifier_corpus_inspection()
            }
        }
    }
}
