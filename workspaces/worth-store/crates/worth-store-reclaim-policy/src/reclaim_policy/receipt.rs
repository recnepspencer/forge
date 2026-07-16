use crate::{AdmittedBackendCapabilityWitness, BackendTargetProfile, CapabilityEvidenceClass};
use worth_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};

use super::{
    ReclaimLaterHandoffPolicy, ReclaimPermit, ReclaimPolicyCounterSnapshot,
    ReclaimPolicyExecutionObservation, ReclaimPolicyPosture, ReclaimPolicyReachabilityProof,
    ReclaimPolicySecurityScope, ReclaimPolicyViolation, ReclaimPolicyViolationKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedReclaimPolicy {
    backend: AdmittedBackendCapabilityWitness,
    region: PhysicalReclaimRegion,
    posture: ReclaimPolicyPosture,
    reachability: ReclaimPolicyReachabilityProof,
    security_scope: ReclaimPolicySecurityScope,
    permit: ReclaimPermit,
    handoff_policy: ReclaimLaterHandoffPolicy,
    counters: ReclaimPolicyCounterSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReclaimPolicyExecutionReceipt {
    policy: AdmittedReclaimPolicy,
    observed_interpretation: ReclaimedByteInterpretation,
    counters: ReclaimPolicyCounterSnapshot,
}

pub(crate) struct AdmittedReclaimPolicyBasis {
    pub(super) backend: AdmittedBackendCapabilityWitness,
    pub(super) region: PhysicalReclaimRegion,
    pub(super) posture: ReclaimPolicyPosture,
    pub(super) reachability: ReclaimPolicyReachabilityProof,
    pub(super) security_scope: ReclaimPolicySecurityScope,
    pub(super) permit: ReclaimPermit,
    pub(super) handoff_policy: ReclaimLaterHandoffPolicy,
}

impl AdmittedReclaimPolicy {
    pub(crate) const fn new(
        basis: AdmittedReclaimPolicyBasis,
        counters: ReclaimPolicyCounterSnapshot,
    ) -> Self {
        Self {
            backend: basis.backend,
            region: basis.region,
            posture: basis.posture,
            reachability: basis.reachability,
            security_scope: basis.security_scope,
            permit: basis.permit,
            handoff_policy: basis.handoff_policy,
            counters,
        }
    }

    pub const fn region(&self) -> PhysicalReclaimRegion {
        self.region
    }
    pub const fn posture(&self) -> ReclaimPolicyPosture {
        self.posture
    }
    pub const fn security_scope(&self) -> ReclaimPolicySecurityScope {
        self.security_scope
    }
    pub const fn reachability(&self) -> &ReclaimPolicyReachabilityProof {
        &self.reachability
    }
    pub const fn permit(&self) -> ReclaimPermit {
        self.permit
    }
    pub const fn handoff_policy(&self) -> ReclaimLaterHandoffPolicy {
        self.handoff_policy
    }
    pub const fn profile(&self) -> BackendTargetProfile {
        self.backend.profile()
    }
    pub const fn evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend.evidence_class()
    }
    pub const fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.counters
    }

    pub(crate) fn complete_execution_with_store_authority(
        self,
        observation: ReclaimPolicyExecutionObservation,
    ) -> Result<ReclaimPolicyExecutionReceipt, ReclaimPolicyViolation> {
        let counters = self.counters.with_byte_interpretation_observation();
        if observation.observed_region() != self.region
            || !self
                .reachability
                .covers_region(observation.observed_region())
        {
            return Err(ReclaimPolicyViolation::new(
                ReclaimPolicyViolationKind::ProtectedReachabilityLost,
                counters.with_violation(),
            ));
        }
        if observation.observed_security_scope() != self.security_scope {
            return Err(ReclaimPolicyViolation::new(
                ReclaimPolicyViolationKind::SecurityScopeLost,
                counters.with_violation(),
            ));
        }
        if !observation.non_claim_handoff_preserved() {
            return Err(ReclaimPolicyViolation::new(
                ReclaimPolicyViolationKind::LaterHandoffStrengthened,
                counters.with_violation(),
            ));
        }
        if observation.observed_interpretation() != self.posture.interpretation() {
            return Err(ReclaimPolicyViolation::new(
                ReclaimPolicyViolationKind::ByteInterpretationContradicted {
                    admitted: self.posture.interpretation(),
                    observed: observation.observed_interpretation(),
                },
                counters.with_violation(),
            ));
        }
        Ok(ReclaimPolicyExecutionReceipt {
            policy: self,
            observed_interpretation: observation.observed_interpretation(),
            counters: counters.with_executed(),
        })
    }
}

impl ReclaimPolicyExecutionReceipt {
    pub const fn policy(&self) -> &AdmittedReclaimPolicy {
        &self.policy
    }
    pub const fn observed_interpretation(&self) -> ReclaimedByteInterpretation {
        self.observed_interpretation
    }
    pub const fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.counters
    }
}
