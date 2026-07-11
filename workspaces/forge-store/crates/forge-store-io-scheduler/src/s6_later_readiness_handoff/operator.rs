use forge_store_contracts::{S11OperatorReadinessNonClaim, S6LaterMilestoneDestination};
use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};
use forge_store_security::StoreSecurityScopeIdentity;

use crate::{
    IoSchedulerBackendCapabilityRequirement, IoSchedulerBackgroundMaintenanceAssumption,
    IoSchedulerForegroundInterferenceSurface, IoSchedulerIsolationCounterSnapshot,
    IoSchedulerIsolationAdmission, IoSchedulerSecurityScopeAdmission, SecureIoOperation,
    SecureIoPosture, SecureIoPreservationCounterSnapshot, SecureIoPreservationReceipt,
    SecureIoScopeBasis,
};

use super::{
    core::S6LaterReadinessEvidenceCore, S6LaterReadinessHandoffDenial,
    S6LaterReadinessReadmissionState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S11OperatorIoReadinessHandoff {
    core: S6LaterReadinessEvidenceCore,
    security_scope: SecureIoScopeBasis,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    secure_io_posture: SecureIoPosture,
    secure_io_counters: SecureIoPreservationCounterSnapshot,
    non_claims: [S11OperatorReadinessNonClaim; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S11OperatorIoReadinessSeed {
    handoff: S11OperatorIoReadinessHandoff,
}

pub fn publish_s11_operator_io_readiness_handoff(
    readiness: &IoSchedulerIsolationAdmission,
    security: &IoSchedulerSecurityScopeAdmission,
    secure_io: SecureIoPreservationReceipt,
) -> Result<S11OperatorIoReadinessHandoff, S6LaterReadinessHandoffDenial> {
    require_matching_security_scope(security, secure_io)?;
    Ok(S11OperatorIoReadinessHandoff {
        core: S6LaterReadinessEvidenceCore::from_current_readiness(readiness),
        security_scope: secure_io.basis(),
        backend_requirement: secure_io.backend_requirement(),
        backend_profile: secure_io.backend_profile(),
        backend_evidence_class: secure_io.backend_evidence_class(),
        secure_io_posture: secure_io.posture(),
        secure_io_counters: secure_io.counters(),
        non_claims: S11OperatorReadinessNonClaim::required(),
    })
}

pub fn admit_s11_operator_io_readiness_seed(
    handoff: S11OperatorIoReadinessHandoff,
) -> S11OperatorIoReadinessSeed {
    S11OperatorIoReadinessSeed { handoff }
}

pub const fn readmit_s11_operator_io_readiness_after_publication(
    handoff: S11OperatorIoReadinessHandoff,
) -> S11OperatorIoReadinessHandoff {
    handoff.with_readmission(S6LaterReadinessReadmissionState::ReadmittedAfterPublication)
}

impl S11OperatorIoReadinessHandoff {
    pub const fn destination(&self) -> S6LaterMilestoneDestination {
        S6LaterMilestoneDestination::S11OperatorReadiness
    }

    pub const fn counters(&self) -> IoSchedulerIsolationCounterSnapshot {
        self.core.counters()
    }

    pub const fn foreground_interference(&self) -> IoSchedulerForegroundInterferenceSurface {
        self.core.foreground_interference()
    }

    pub const fn background_maintenance(&self) -> IoSchedulerBackgroundMaintenanceAssumption {
        self.core.background_maintenance()
    }

    pub const fn security_scope_identity(&self) -> StoreSecurityScopeIdentity {
        self.security_scope.identity()
    }

    pub const fn backend_requirement(&self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn secure_io_posture(&self) -> SecureIoPosture {
        self.secure_io_posture
    }

    pub const fn secure_io_counters(&self) -> SecureIoPreservationCounterSnapshot {
        self.secure_io_counters
    }

    pub const fn non_claims(&self) -> &[S11OperatorReadinessNonClaim; 4] {
        &self.non_claims
    }

    pub const fn readmission_state(&self) -> S6LaterReadinessReadmissionState {
        self.core.readmission()
    }

    const fn with_readmission(mut self, readmission: S6LaterReadinessReadmissionState) -> Self {
        self.core = self.core.with_readmission(readmission);
        self
    }
}

impl S11OperatorIoReadinessSeed {
    pub const fn handoff(&self) -> &S11OperatorIoReadinessHandoff {
        &self.handoff
    }

    pub const fn non_claims(&self) -> &[S11OperatorReadinessNonClaim; 4] {
        self.handoff.non_claims()
    }

    pub const fn carries_encryption_algorithm_claim(&self) -> bool {
        false
    }

    pub const fn carries_key_rotation_claim(&self) -> bool {
        false
    }

    pub const fn carries_operator_authorization_claim(&self) -> bool {
        false
    }
}

fn require_matching_security_scope(
    security: &IoSchedulerSecurityScopeAdmission,
    secure_io: SecureIoPreservationReceipt,
) -> Result<(), S6LaterReadinessHandoffDenial> {
    if security.receipt().identity() != secure_io.identity() {
        return Err(S6LaterReadinessHandoffDenial::SecurityScopeMismatch {
            destination: S6LaterMilestoneDestination::S11OperatorReadiness,
        });
    }
    match secure_io.operation() {
        SecureIoOperation::RepairScan
        | SecureIoOperation::VerificationPressure
        | SecureIoOperation::BackendExecution => {}
        _ => {
            return Err(
                S6LaterReadinessHandoffDenial::SecureIoOperationNotFoundation {
                    destination: S6LaterMilestoneDestination::S11OperatorReadiness,
                },
            );
        }
    }
    match secure_io.posture() {
        SecureIoPosture::ScopePreserving | SecureIoPosture::SecureFrameCompatible => Ok(()),
    }
}
