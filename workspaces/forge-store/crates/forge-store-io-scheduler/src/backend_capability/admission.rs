use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityClaimWitness, BackendTargetProfile,
    CapabilityEvidenceClass,
};

use crate::s6_readiness::IoSchedulerS6SecurityScopeAdmission;

use super::{IoSchedulerBackendCapabilityDenial, IoSchedulerBackendCapabilityRequirement};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IoSchedulerBackendCapabilityAdmission {
    requirement: IoSchedulerBackendCapabilityRequirement,
    claim: BackendCapabilityClaimWitness,
    security_scope_bound: bool,
}

impl IoSchedulerBackendCapabilityAdmission {
    pub const fn requirement(&self) -> IoSchedulerBackendCapabilityRequirement {
        self.requirement
    }

    pub const fn profile(&self) -> BackendTargetProfile {
        self.claim.profile()
    }

    pub const fn evidence_class(&self) -> CapabilityEvidenceClass {
        self.claim.evidence_class()
    }

    pub const fn security_scope_bound(&self) -> bool {
        self.security_scope_bound
    }
}

pub fn admit_backend_capability_for_scheduler_claim(
    witness: &AdmittedBackendCapabilityWitness,
    requirement: IoSchedulerBackendCapabilityRequirement,
) -> Result<IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial> {
    if requirement == IoSchedulerBackendCapabilityRequirement::SecureFrameIo {
        return Err(IoSchedulerBackendCapabilityDenial::SecureFrameRequiresSecurityScope);
    }
    let claim = witness
        .require(
            requirement.capability_kind(),
            requirement.required_evidence(),
        )
        .map_err(IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied)?;
    Ok(IoSchedulerBackendCapabilityAdmission {
        requirement,
        claim,
        security_scope_bound: false,
    })
}

pub fn admit_secure_frame_backend_capability_for_scheduler_claim(
    witness: &AdmittedBackendCapabilityWitness,
    security_scope: &IoSchedulerS6SecurityScopeAdmission,
) -> Result<IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial> {
    let _receipt = security_scope.receipt();
    let requirement = IoSchedulerBackendCapabilityRequirement::SecureFrameIo;
    let claim = witness
        .require(
            requirement.capability_kind(),
            requirement.required_evidence(),
        )
        .map_err(IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied)?;
    Ok(IoSchedulerBackendCapabilityAdmission {
        requirement,
        claim,
        security_scope_bound: true,
    })
}
