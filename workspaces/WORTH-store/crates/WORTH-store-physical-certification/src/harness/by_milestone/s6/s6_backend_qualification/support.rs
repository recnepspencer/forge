use worth_store_physical_backend::{
    BackendCapabilityKind, BackendCapabilitySupportPosture, BackendRebindTriggers,
    CapabilityEvidenceClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualificationResidualDebtReason {
    None,
    MissingEvidence,
    BackendSpecificDenial,
    DegradedOperation,
    StaleEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QualificationResidualDebt {
    reason: QualificationResidualDebtReason,
    affected_capability: BackendCapabilityKind,
    missing_evidence: CapabilityEvidenceClass,
    rebind_triggers: BackendRebindTriggers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishedQualificationPosture {
    Supported,
    Degraded,
    Unsupported,
    Unavailable,
    Unknown,
    Stale,
    RebindRequired,
}

impl QualificationResidualDebt {
    pub const fn none(
        affected_capability: BackendCapabilityKind,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self {
            reason: QualificationResidualDebtReason::None,
            affected_capability,
            missing_evidence: CapabilityEvidenceClass::CertifiedBackendProfile,
            rebind_triggers,
        }
    }

    pub const fn missing_evidence(
        affected_capability: BackendCapabilityKind,
        missing_evidence: CapabilityEvidenceClass,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self {
            reason: QualificationResidualDebtReason::MissingEvidence,
            affected_capability,
            missing_evidence,
            rebind_triggers,
        }
    }

    pub const fn degraded_operation(
        affected_capability: BackendCapabilityKind,
        missing_evidence: CapabilityEvidenceClass,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self {
            reason: QualificationResidualDebtReason::DegradedOperation,
            affected_capability,
            missing_evidence,
            rebind_triggers,
        }
    }

    pub const fn backend_specific_denial(
        affected_capability: BackendCapabilityKind,
        missing_evidence: CapabilityEvidenceClass,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self {
            reason: QualificationResidualDebtReason::BackendSpecificDenial,
            affected_capability,
            missing_evidence,
            rebind_triggers,
        }
    }

    pub const fn stale_evidence(
        affected_capability: BackendCapabilityKind,
        missing_evidence: CapabilityEvidenceClass,
        rebind_triggers: BackendRebindTriggers,
    ) -> Self {
        Self {
            reason: QualificationResidualDebtReason::StaleEvidence,
            affected_capability,
            missing_evidence,
            rebind_triggers,
        }
    }

    pub const fn reason(self) -> QualificationResidualDebtReason {
        self.reason
    }

    pub const fn affected_capability(self) -> BackendCapabilityKind {
        self.affected_capability
    }

    pub const fn missing_evidence_class(self) -> CapabilityEvidenceClass {
        self.missing_evidence
    }

    pub const fn rebind_triggers(self) -> BackendRebindTriggers {
        self.rebind_triggers
    }

    pub const fn is_clear(self) -> bool {
        matches!(self.reason, QualificationResidualDebtReason::None)
    }
}

pub const fn published_posture(
    support: BackendCapabilitySupportPosture,
    residual_debt: QualificationResidualDebt,
) -> PublishedQualificationPosture {
    match support {
        BackendCapabilitySupportPosture::Supported if residual_debt.is_clear() => {
            PublishedQualificationPosture::Supported
        }
        BackendCapabilitySupportPosture::Supported => PublishedQualificationPosture::Degraded,
        BackendCapabilitySupportPosture::Unsupported => PublishedQualificationPosture::Unsupported,
        BackendCapabilitySupportPosture::Unavailable => PublishedQualificationPosture::Unavailable,
        BackendCapabilitySupportPosture::Unknown => PublishedQualificationPosture::Unknown,
        BackendCapabilitySupportPosture::Stale => PublishedQualificationPosture::Stale,
        BackendCapabilitySupportPosture::RebindRequired => {
            PublishedQualificationPosture::RebindRequired
        }
    }
}
