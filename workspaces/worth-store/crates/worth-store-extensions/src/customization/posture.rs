use super::FutureLayoutTarget;
use worth_store_layout_indexes::customization::{
    FutureLayoutCapabilityRequest, FutureLayoutWorkloadEnvelope,
};
use worth_store_layout_indexes::AdmittedPhysicalKeyDomain;

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
    declared_domain: AdmittedPhysicalKeyDomain,
}

impl FutureLayoutTargetDeclaration {
    pub(crate) const fn new(
        target: FutureLayoutTarget,
        posture: ExtensionFamilyPosture,
        declared_domain: AdmittedPhysicalKeyDomain,
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

    pub const fn declared_domain(self) -> AdmittedPhysicalKeyDomain {
        self.declared_domain
    }

    pub const fn capability_request(self) -> FutureLayoutCapabilityRequest {
        match self.target {
            FutureLayoutTarget::StableBasisRead => {
                FutureLayoutCapabilityRequest::point_lookup(self.declared_domain)
            }
            FutureLayoutTarget::AspectProjection => {
                FutureLayoutCapabilityRequest::rebuildable_projection(self.declared_domain)
            }
            FutureLayoutTarget::SubscriptionSupport => {
                FutureLayoutCapabilityRequest::ordered_range(self.declared_domain)
            }
            FutureLayoutTarget::SupportTrust => {
                FutureLayoutCapabilityRequest::verifier_declared_scan(self.declared_domain)
            }
        }
    }

    pub const fn workload_envelope(self) -> FutureLayoutWorkloadEnvelope {
        match self.target {
            FutureLayoutTarget::StableBasisRead => {
                FutureLayoutWorkloadEnvelope::foreground_low_fanout()
            }
            FutureLayoutTarget::AspectProjection => {
                FutureLayoutWorkloadEnvelope::background_rebuild_projection()
            }
            FutureLayoutTarget::SubscriptionSupport => {
                FutureLayoutWorkloadEnvelope::foreground_bounded_traversal()
            }
            FutureLayoutTarget::SupportTrust => {
                FutureLayoutWorkloadEnvelope::verifier_corpus_inspection()
            }
        }
    }
}
