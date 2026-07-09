use crate::{BackendCapabilityClaimWitness, BackendTargetProfile, CapabilityEvidenceClass};

use super::{
    AccessPolicyCounterSnapshot, AccessPolicyExecutionObservation, AccessPolicyRequest,
    AccessPolicySecurityScope, AccessPolicyViolation, AccessPolicyViolationKind,
    MixedAccessCoherenceBasis, MmapFaultPosture, StoreAccessMode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmittedAccessPolicy {
    request: AccessPolicyRequest,
    capability: BackendCapabilityClaimWitness,
    counters: AccessPolicyCounterSnapshot,
}

impl AdmittedAccessPolicy {
    pub(crate) const fn new(
        request: AccessPolicyRequest,
        capability: BackendCapabilityClaimWitness,
        counters: AccessPolicyCounterSnapshot,
    ) -> Self {
        Self {
            request,
            capability,
            counters,
        }
    }

    pub const fn mode(self) -> StoreAccessMode {
        self.request.mode()
    }
    pub const fn request(self) -> AccessPolicyRequest {
        self.request
    }
    pub const fn security_scope(self) -> Option<AccessPolicySecurityScope> {
        self.request.security_scope()
    }
    pub const fn coherence_basis(self) -> Option<MixedAccessCoherenceBasis> {
        self.request.coherence_basis()
    }
    pub const fn mmap_fault_posture(self) -> MmapFaultPosture {
        self.request.mmap_fault_posture()
    }
    pub const fn profile(self) -> BackendTargetProfile {
        self.capability.profile()
    }
    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.capability.evidence_class()
    }
    pub const fn counters(self) -> AccessPolicyCounterSnapshot {
        self.counters
    }

    pub(crate) fn complete_execution_with_store_authority(
        self,
        observation: AccessPolicyExecutionObservation,
    ) -> Result<AccessPolicyExecutionReceipt, AccessPolicyViolation> {
        let mut counters = self.counters;
        match observation.violation() {
            AccessPolicyViolationKind::None => {
                counters = self.require_success_observations(observation, counters)?;
                Ok(AccessPolicyExecutionReceipt {
                    policy: self,
                    counters,
                })
            }
            AccessPolicyViolationKind::MmapLazyFault => {
                counters = counters.with_mmap_fault_observation().with_violation();
                Err(AccessPolicyViolation::new(
                    observation.violation(),
                    counters,
                ))
            }
            AccessPolicyViolationKind::MixedModeInvalidationMissed => {
                counters = counters.with_mixed_mode_invalidation().with_violation();
                Err(AccessPolicyViolation::new(
                    observation.violation(),
                    counters,
                ))
            }
            _ => Err(AccessPolicyViolation::new(
                observation.violation(),
                counters.with_violation(),
            )),
        }
    }

    fn require_success_observations(
        self,
        observation: AccessPolicyExecutionObservation,
        mut counters: AccessPolicyCounterSnapshot,
    ) -> Result<AccessPolicyCounterSnapshot, AccessPolicyViolation> {
        if !observation.security_scope_preserved() {
            return Err(AccessPolicyViolation::new(
                AccessPolicyViolationKind::BackendContradictedWitness,
                counters.with_violation(),
            ));
        }
        if !observation.page_cache_visibility_observed() {
            return Err(AccessPolicyViolation::new(
                AccessPolicyViolationKind::PageCacheVisibilityLost,
                counters.with_page_cache_visibility_check().with_violation(),
            ));
        }
        counters = counters.with_page_cache_visibility_check();
        if self.requires_direct_io_observation() {
            if !observation.direct_io_alignment_observed() {
                return Err(AccessPolicyViolation::new(
                    AccessPolicyViolationKind::DirectIoAlignmentContradicted,
                    counters.with_direct_io_alignment_check().with_violation(),
                ));
            }
            counters = counters.with_direct_io_alignment_check();
        }
        if self.mode() == StoreAccessMode::Mixed {
            if !observation.mixed_mode_invalidation_observed() {
                return Err(AccessPolicyViolation::new(
                    AccessPolicyViolationKind::MixedModeInvalidationMissed,
                    counters.with_mixed_mode_invalidation().with_violation(),
                ));
            }
            counters = counters.with_mixed_mode_invalidation();
        }
        Ok(counters)
    }

    fn requires_direct_io_observation(self) -> bool {
        self.mode() == StoreAccessMode::DirectIo
            || self
                .request
                .mixed_transition()
                .is_some_and(|transition| transition.involves(StoreAccessMode::DirectIo))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyExecutionReceipt {
    policy: AdmittedAccessPolicy,
    counters: AccessPolicyCounterSnapshot,
}

impl AccessPolicyExecutionReceipt {
    pub const fn policy(self) -> AdmittedAccessPolicy {
        self.policy
    }
    pub const fn counters(self) -> AccessPolicyCounterSnapshot {
        self.counters
    }
}
