use super::super::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentEligibilityTraceEvidence {
    support_posture: ForgeQueryIntentAdmissionSupportEligibility,
    capability_posture: ForgeQueryIntentAdmissionCapabilityEligibility,
    policy_posture: ForgeQueryIntentAdmissionPolicyEligibility,
    basis_posture: ForgeQueryIntentAdmissionBasisEligibility,
    invariant_posture: ForgeQueryIntentAdmissionInvariantEligibility,
    projection_source_posture: ForgeQueryIntentAdmissionProjectionSourceEligibility,
    routing_support_posture: ForgeQueryIntentAdmissionRoutingSupportEligibility,
    source_lane_posture: ForgeQueryIntentAdmissionSourceLaneEligibility,
    authority_lane_posture: ForgeQueryIntentAdmissionAuthorityLaneEligibility,
    eligibility_digest: String,
}

impl ForgeQueryIntentEligibilityTraceEvidence {
    pub(crate) fn new(
        support_posture: ForgeQueryIntentAdmissionSupportEligibility,
        capability_posture: ForgeQueryIntentAdmissionCapabilityEligibility,
        policy_posture: ForgeQueryIntentAdmissionPolicyEligibility,
        basis_posture: ForgeQueryIntentAdmissionBasisEligibility,
        invariant_posture: ForgeQueryIntentAdmissionInvariantEligibility,
        projection_source_posture: ForgeQueryIntentAdmissionProjectionSourceEligibility,
        routing_support_posture: ForgeQueryIntentAdmissionRoutingSupportEligibility,
        source_lane_posture: ForgeQueryIntentAdmissionSourceLaneEligibility,
        authority_lane_posture: ForgeQueryIntentAdmissionAuthorityLaneEligibility,
        eligibility_digest: String,
    ) -> Self {
        Self {
            support_posture,
            capability_posture,
            policy_posture,
            basis_posture,
            invariant_posture,
            projection_source_posture,
            routing_support_posture,
            source_lane_posture,
            authority_lane_posture,
            eligibility_digest,
        }
    }

    pub fn support_posture(&self) -> ForgeQueryIntentAdmissionSupportEligibility {
        self.support_posture
    }

    pub fn capability_posture(&self) -> ForgeQueryIntentAdmissionCapabilityEligibility {
        self.capability_posture
    }

    pub fn policy_posture(&self) -> ForgeQueryIntentAdmissionPolicyEligibility {
        self.policy_posture
    }

    pub fn basis_posture(&self) -> ForgeQueryIntentAdmissionBasisEligibility {
        self.basis_posture
    }

    pub fn invariant_posture(&self) -> ForgeQueryIntentAdmissionInvariantEligibility {
        self.invariant_posture
    }

    pub fn projection_source_posture(
        &self,
    ) -> ForgeQueryIntentAdmissionProjectionSourceEligibility {
        self.projection_source_posture
    }

    pub fn routing_support_posture(&self) -> ForgeQueryIntentAdmissionRoutingSupportEligibility {
        self.routing_support_posture
    }

    pub fn source_lane_posture(&self) -> ForgeQueryIntentAdmissionSourceLaneEligibility {
        self.source_lane_posture
    }

    pub fn authority_lane_posture(&self) -> ForgeQueryIntentAdmissionAuthorityLaneEligibility {
        self.authority_lane_posture
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }
}
