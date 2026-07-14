use super::super::{
    WorthQueryIntentAdmissionAuthorityLaneEligibility, WorthQueryIntentAdmissionBasisEligibility,
    WorthQueryIntentAdmissionCapabilityEligibility, WorthQueryIntentAdmissionInvariantEligibility,
    WorthQueryIntentAdmissionPolicyEligibility,
    WorthQueryIntentAdmissionProjectionSourceEligibility,
    WorthQueryIntentAdmissionRoutingSupportEligibility,
    WorthQueryIntentAdmissionSourceLaneEligibility, WorthQueryIntentAdmissionSupportEligibility,
};
use crate::runtime::WorthQueryEffectWriteAdjacentTrigger;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentEligibilityTraceEvidence {
    support_posture: WorthQueryIntentAdmissionSupportEligibility,
    capability_posture: WorthQueryIntentAdmissionCapabilityEligibility,
    policy_posture: WorthQueryIntentAdmissionPolicyEligibility,
    basis_posture: WorthQueryIntentAdmissionBasisEligibility,
    invariant_posture: WorthQueryIntentAdmissionInvariantEligibility,
    projection_source_posture: WorthQueryIntentAdmissionProjectionSourceEligibility,
    routing_support_posture: WorthQueryIntentAdmissionRoutingSupportEligibility,
    source_lane_posture: WorthQueryIntentAdmissionSourceLaneEligibility,
    authority_lane_posture: WorthQueryIntentAdmissionAuthorityLaneEligibility,
    write_adjacent_trigger: Option<WorthQueryEffectWriteAdjacentTrigger>,
    eligibility_digest: String,
}

impl WorthQueryIntentEligibilityTraceEvidence {
    pub(crate) fn new(
        support_posture: WorthQueryIntentAdmissionSupportEligibility,
        capability_posture: WorthQueryIntentAdmissionCapabilityEligibility,
        policy_posture: WorthQueryIntentAdmissionPolicyEligibility,
        basis_posture: WorthQueryIntentAdmissionBasisEligibility,
        invariant_posture: WorthQueryIntentAdmissionInvariantEligibility,
        projection_source_posture: WorthQueryIntentAdmissionProjectionSourceEligibility,
        routing_support_posture: WorthQueryIntentAdmissionRoutingSupportEligibility,
        source_lane_posture: WorthQueryIntentAdmissionSourceLaneEligibility,
        authority_lane_posture: WorthQueryIntentAdmissionAuthorityLaneEligibility,
        write_adjacent_trigger: Option<WorthQueryEffectWriteAdjacentTrigger>,
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
            write_adjacent_trigger,
            eligibility_digest,
        }
    }

    pub fn support_posture(&self) -> WorthQueryIntentAdmissionSupportEligibility {
        self.support_posture
    }

    pub fn capability_posture(&self) -> WorthQueryIntentAdmissionCapabilityEligibility {
        self.capability_posture
    }

    pub fn policy_posture(&self) -> WorthQueryIntentAdmissionPolicyEligibility {
        self.policy_posture
    }

    pub fn basis_posture(&self) -> WorthQueryIntentAdmissionBasisEligibility {
        self.basis_posture
    }

    pub fn invariant_posture(&self) -> WorthQueryIntentAdmissionInvariantEligibility {
        self.invariant_posture
    }

    pub fn projection_source_posture(
        &self,
    ) -> WorthQueryIntentAdmissionProjectionSourceEligibility {
        self.projection_source_posture
    }

    pub fn routing_support_posture(&self) -> WorthQueryIntentAdmissionRoutingSupportEligibility {
        self.routing_support_posture
    }

    pub fn source_lane_posture(&self) -> WorthQueryIntentAdmissionSourceLaneEligibility {
        self.source_lane_posture
    }

    pub fn authority_lane_posture(&self) -> WorthQueryIntentAdmissionAuthorityLaneEligibility {
        self.authority_lane_posture
    }

    pub fn write_adjacent_trigger(&self) -> Option<&WorthQueryEffectWriteAdjacentTrigger> {
        self.write_adjacent_trigger.as_ref()
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }
}
