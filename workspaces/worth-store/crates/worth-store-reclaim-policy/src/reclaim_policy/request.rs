use worth_store_physical_format::PhysicalReclaimRegion;

use super::{
    ReclaimLaterHandoffPolicy, ReclaimPermit, ReclaimPolicyPosture, ReclaimPolicyReachabilityProof,
    ReclaimPolicySecurityScope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimPolicyRequest {
    region: Option<PhysicalReclaimRegion>,
    posture: Option<ReclaimPolicyPosture>,
    reachability: Option<ReclaimPolicyReachabilityProof>,
    security_scope: Option<ReclaimPolicySecurityScope>,
    permit: Option<ReclaimPermit>,
    handoff_policy: ReclaimLaterHandoffPolicy,
}

impl ReclaimPolicyRequest {
    pub const fn new() -> Self {
        Self {
            region: None,
            posture: None,
            reachability: None,
            security_scope: None,
            permit: None,
            handoff_policy: ReclaimLaterHandoffPolicy::non_claim(),
        }
    }

    pub const fn for_region(mut self, region: PhysicalReclaimRegion) -> Self {
        self.region = Some(region);
        self
    }

    pub const fn with_posture(mut self, posture: ReclaimPolicyPosture) -> Self {
        self.posture = Some(posture);
        self
    }

    pub const fn with_reachability(mut self, proof: ReclaimPolicyReachabilityProof) -> Self {
        self.reachability = Some(proof);
        self
    }

    pub const fn with_security_scope(mut self, scope: ReclaimPolicySecurityScope) -> Self {
        self.security_scope = Some(scope);
        self
    }

    pub const fn with_reclaim_permit(mut self, permit: ReclaimPermit) -> Self {
        self.permit = Some(permit);
        self
    }

    pub const fn with_later_handoff_policy(mut self, policy: ReclaimLaterHandoffPolicy) -> Self {
        self.handoff_policy = policy;
        self
    }

    pub const fn region(&self) -> Option<PhysicalReclaimRegion> {
        self.region
    }

    pub const fn posture(&self) -> Option<ReclaimPolicyPosture> {
        self.posture
    }

    pub const fn reachability(&self) -> Option<&ReclaimPolicyReachabilityProof> {
        self.reachability.as_ref()
    }

    pub const fn security_scope(&self) -> Option<ReclaimPolicySecurityScope> {
        self.security_scope
    }

    pub const fn permit(&self) -> Option<ReclaimPermit> {
        self.permit
    }

    pub const fn handoff_policy(&self) -> ReclaimLaterHandoffPolicy {
        self.handoff_policy
    }
}

impl Default for ReclaimPolicyRequest {
    fn default() -> Self {
        Self::new()
    }
}
