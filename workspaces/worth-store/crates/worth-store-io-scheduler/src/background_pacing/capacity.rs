use worth_foundational::FoundationalPolicyAdmissionReceipt;
use worth_store_security::StoreSecurityScopeIdentity;

use crate::foreground_reservation::{ForegroundIoLaneKind, ForegroundReservationReceipt};
use crate::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityRequirement,
    IoSchedulerIsolationAdmission, SecureIoOperation, SecureIoPreservationDenial,
    SecureIoPreservationReceipt,
};

use super::budget::require_policy_receipt;
use super::{
    BackgroundIoPressureShape, BackgroundPacingAdmissionBasis, BackgroundPacingDenial,
    BackgroundPacingFreshness, BackgroundPacingProgressionEvidence, BackgroundResourceBudget,
};

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundCapacityAdmissionRequest<'a> {
    pressure: BackgroundIoPressureShape,
    foreground: &'a ForegroundReservationReceipt,
    backend: &'a IoSchedulerBackendCapabilityAdmission,
    readiness: &'a IoSchedulerIsolationAdmission,
    security_scope_identity: StoreSecurityScopeIdentity,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
    policy_receipt: FoundationalPolicyAdmissionReceipt,
    freshness: BackgroundPacingFreshness,
    secure_io: Option<SecureIoPreservationReceipt>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundCapacityAdmission {
    pressure: BackgroundIoPressureShape,
    basis: BackgroundPacingAdmissionBasis,
    idle_available: BackgroundResourceBudget,
    policy_admitted: BackgroundResourceBudget,
    debt_limit: BackgroundResourceBudget,
    policy_receipt: FoundationalPolicyAdmissionReceipt,
    freshness: BackgroundPacingFreshness,
    secure_io: Option<SecureIoPreservationReceipt>,
}

pub fn admit_background_capacity(
    request: BackgroundCapacityAdmissionRequest<'_>,
) -> Result<BackgroundCapacityAdmission, BackgroundPacingDenial> {
    require_background_basis(&request)?;
    require_policy_receipt(
        &request.policy_receipt,
        request.pressure.requested_budget(),
        request.policy_admitted,
    )?;
    Ok(BackgroundCapacityAdmission {
        pressure: request.pressure,
        basis: BackgroundPacingAdmissionBasis::new(
            request.pressure.class(),
            request.foreground,
            request.backend,
            request.readiness,
            request.security_scope_identity,
        ),
        idle_available: request.idle_available,
        policy_admitted: request.policy_admitted,
        debt_limit: request.debt_limit,
        policy_receipt: request.policy_receipt,
        freshness: request.freshness,
        secure_io: request.secure_io,
    })
}

impl<'a> BackgroundCapacityAdmissionRequest<'a> {
    pub fn new(
        pressure: BackgroundIoPressureShape,
        foreground: &'a ForegroundReservationReceipt,
        backend: &'a IoSchedulerBackendCapabilityAdmission,
        readiness: &'a IoSchedulerIsolationAdmission,
        policy_receipt: FoundationalPolicyAdmissionReceipt,
    ) -> Self {
        let requested = pressure.requested_budget();
        Self {
            pressure,
            foreground,
            backend,
            readiness,
            security_scope_identity: foreground.security_scope_identity(),
            idle_available: requested,
            policy_admitted: requested,
            debt_limit: BackgroundResourceBudget::new(),
            policy_receipt,
            freshness: BackgroundPacingProgressionEvidence::current(readiness).freshness(),
            secure_io: None,
        }
    }

    pub fn with_idle_available(mut self, idle_available: BackgroundResourceBudget) -> Self {
        self.idle_available = idle_available;
        self
    }
    pub fn with_policy_admitted(mut self, policy_admitted: BackgroundResourceBudget) -> Self {
        self.policy_admitted = policy_admitted;
        self
    }
    pub fn with_debt_limit(mut self, debt_limit: BackgroundResourceBudget) -> Self {
        self.debt_limit = debt_limit;
        self
    }
    pub fn with_policy_receipt(
        mut self,
        policy_receipt: FoundationalPolicyAdmissionReceipt,
    ) -> Self {
        self.policy_receipt = policy_receipt;
        self
    }
    pub fn with_progression_evidence(
        mut self,
        evidence: BackgroundPacingProgressionEvidence,
    ) -> Self {
        self.freshness = evidence.freshness();
        self
    }

    pub const fn with_secure_io_scope(mut self, secure_io: SecureIoPreservationReceipt) -> Self {
        self.secure_io = Some(secure_io);
        self
    }
}

impl BackgroundCapacityAdmission {
    pub const fn pressure(&self) -> BackgroundIoPressureShape {
        self.pressure
    }
    pub const fn basis(&self) -> BackgroundPacingAdmissionBasis {
        self.basis
    }
    pub const fn idle_available(&self) -> BackgroundResourceBudget {
        self.idle_available
    }
    pub const fn policy_admitted(&self) -> BackgroundResourceBudget {
        self.policy_admitted
    }
    pub const fn debt_limit(&self) -> BackgroundResourceBudget {
        self.debt_limit
    }
    pub const fn policy_receipt(&self) -> &FoundationalPolicyAdmissionReceipt {
        &self.policy_receipt
    }
    pub const fn freshness(&self) -> BackgroundPacingFreshness {
        self.freshness
    }

    pub const fn secure_io(&self) -> Option<SecureIoPreservationReceipt> {
        self.secure_io
    }
}

fn require_background_basis(
    request: &BackgroundCapacityAdmissionRequest<'_>,
) -> Result<(), BackgroundPacingDenial> {
    let pressure = request.pressure;
    if pressure.requested_budget().is_empty() {
        return Err(BackgroundPacingDenial::MissingDeclaredResourceBudget);
    }
    if pressure.backend_requirement() != request.backend.requirement() {
        return Err(BackgroundPacingDenial::BackendRequirementMismatch {
            pressure_required: pressure.backend_requirement(),
            admitted: request.backend.requirement(),
        });
    }
    let blob_ingest_preserves_foreground = blob_ingest_preserves_page_or_wal_foreground(request);
    if !blob_ingest_preserves_foreground
        && request.foreground.backend_requirement() != request.backend.requirement()
    {
        return Err(
            BackgroundPacingDenial::ForegroundReservationBackendMismatch {
                reservation_required: request.foreground.backend_requirement(),
                admitted: request.backend.requirement(),
            },
        );
    }
    if !blob_ingest_preserves_foreground
        && request.foreground.backend_profile() != request.backend.profile()
    {
        return Err(BackgroundPacingDenial::BackendProfileMismatch {
            reservation_profile: request.foreground.backend_profile(),
            admitted_profile: request.backend.profile(),
        });
    }
    if !blob_ingest_preserves_foreground
        && request.foreground.backend_evidence_class() != request.backend.evidence_class()
    {
        return Err(BackgroundPacingDenial::BackendEvidenceClassMismatch {
            reservation_evidence: request.foreground.backend_evidence_class(),
            admitted_evidence: request.backend.evidence_class(),
        });
    }
    if request.foreground.security_scope_identity() != request.security_scope_identity {
        return Err(BackgroundPacingDenial::SecurityScopeMismatch {
            reservation_scope: request.foreground.security_scope_identity(),
            requested_scope: request.security_scope_identity,
        });
    }
    if pressure.backend_requirement() == IoSchedulerBackendCapabilityRequirement::SecureFrameIo
        && !request.backend.security_scope_bound()
    {
        return Err(BackgroundPacingDenial::SecureBackgroundPressureRequiresSecurityBoundBackend);
    }
    require_secure_io_scope(request)?;
    let _readiness = request.readiness.background_maintenance();
    Ok(())
}

fn blob_ingest_preserves_page_or_wal_foreground(
    request: &BackgroundCapacityAdmissionRequest<'_>,
) -> bool {
    request.pressure.class() == super::BackgroundIoPressureClass::IngestPressure
        && matches!(
            request.foreground.lane(),
            ForegroundIoLaneKind::CommitCriticalWalWrite | ForegroundIoLaneKind::OrdinaryPageWrite
        )
}

fn require_secure_io_scope(
    request: &BackgroundCapacityAdmissionRequest<'_>,
) -> Result<(), BackgroundPacingDenial> {
    if !request.pressure.secure_scope_required()
        && request.pressure.backend_requirement()
            != IoSchedulerBackendCapabilityRequirement::SecureFrameIo
    {
        return Ok(());
    }
    let Some(secure_io) = request.secure_io else {
        return Err(BackgroundPacingDenial::MissingSecureIoPreservation);
    };
    if secure_io.identity() != request.security_scope_identity {
        return Err(BackgroundPacingDenial::SecureIoDenied(
            SecureIoPreservationDenial::ScopeMismatch {
                operation: secure_io.operation(),
            },
        ));
    }
    if secure_io.backend_requirement() != request.backend.requirement() {
        return Err(BackgroundPacingDenial::SecureIoDenied(
            SecureIoPreservationDenial::BackendRequirementMismatch {
                required: secure_io.backend_requirement(),
                admitted: request.backend.requirement(),
            },
        ));
    }
    let expected = match request.pressure.class() {
        super::BackgroundIoPressureClass::RepairScan => SecureIoOperation::RepairScan,
        super::BackgroundIoPressureClass::VerificationPressure => {
            SecureIoOperation::VerificationPressure
        }
        _ => SecureIoOperation::BackgroundLease,
    };
    if secure_io.operation() != expected {
        return Err(BackgroundPacingDenial::SecureIoDenied(
            SecureIoPreservationDenial::OperationMismatch {
                expected,
                actual: secure_io.operation(),
            },
        ));
    }
    Ok(())
}
