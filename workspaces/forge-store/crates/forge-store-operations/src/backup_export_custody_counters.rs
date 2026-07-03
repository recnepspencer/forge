use crate::BackupExportCustodyMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BackupExportCustodyCounterSnapshot {
    declaration_inputs: u64,
    readiness_inputs: u64,
    key_version_checks: u64,
    custody_checks: u64,
    custody_admitted: u64,
    custody_denied: u64,
    key_version_stale: u64,
    unsupported_secure_posture: u64,
    unavailable_custody_evidence: u64,
    emissions_prepared: u64,
    terminal_projections_prepared: u64,
    trust_boundary_crossings: u64,
    readmissions: u64,
    readmission_required: u64,
    denials: u64,
}

impl BackupExportCustodyCounterSnapshot {
    pub(crate) const fn for_declaration(_: BackupExportCustodyMode) -> Self {
        Self {
            declaration_inputs: 1,
            readiness_inputs: 0,
            key_version_checks: 1,
            custody_checks: 1,
            custody_admitted: 0,
            custody_denied: 0,
            key_version_stale: 0,
            unsupported_secure_posture: 0,
            unavailable_custody_evidence: 0,
            emissions_prepared: 0,
            terminal_projections_prepared: 0,
            trust_boundary_crossings: 0,
            readmissions: 0,
            readmission_required: 0,
            denials: 0,
        }
    }

    pub(crate) const fn from_readiness() -> Self {
        Self {
            declaration_inputs: 0,
            readiness_inputs: 1,
            key_version_checks: 1,
            custody_checks: 1,
            custody_admitted: 0,
            custody_denied: 0,
            key_version_stale: 0,
            unsupported_secure_posture: 0,
            unavailable_custody_evidence: 0,
            emissions_prepared: 0,
            terminal_projections_prepared: 0,
            trust_boundary_crossings: 0,
            readmissions: 0,
            readmission_required: 0,
            denials: 0,
        }
    }

    pub(crate) const fn denied(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_custody_admitted(self) -> Self {
        Self {
            custody_admitted: self.custody_admitted + 1,
            ..self
        }
    }

    pub(crate) const fn record_custody_denied(self) -> Self {
        Self {
            custody_denied: self.custody_denied + 1,
            ..self
        }
    }

    pub(crate) const fn record_stale_key_version(self) -> Self {
        Self {
            key_version_stale: self.key_version_stale + 1,
            ..self
        }
    }

    pub(crate) const fn record_unsupported_secure_posture(self) -> Self {
        Self {
            unsupported_secure_posture: self.unsupported_secure_posture + 1,
            ..self
        }
    }

    pub(crate) const fn record_unavailable_custody_evidence(self) -> Self {
        Self {
            unavailable_custody_evidence: self.unavailable_custody_evidence + 1,
            ..self
        }
    }

    pub(crate) const fn prepared_emission(self) -> Self {
        Self {
            emissions_prepared: self.emissions_prepared + 1,
            ..self
        }
    }

    pub(crate) const fn prepared_terminal_projection(self) -> Self {
        Self {
            terminal_projections_prepared: self.terminal_projections_prepared + 1,
            ..self
        }
    }

    pub(crate) const fn crossed_trust_boundary(self) -> Self {
        Self {
            trust_boundary_crossings: self.trust_boundary_crossings + 1,
            ..self
        }
    }

    pub(crate) const fn readmitted(self) -> Self {
        Self {
            readmissions: self.readmissions + 1,
            ..self
        }
    }

    pub(crate) const fn record_readmission_required(self) -> Self {
        Self {
            readmission_required: self.readmission_required + 1,
            ..self
        }
    }

    pub const fn declaration_inputs(self) -> u64 {
        self.declaration_inputs
    }

    pub const fn readiness_inputs(self) -> u64 {
        self.readiness_inputs
    }

    pub const fn key_version_checks(self) -> u64 {
        self.key_version_checks
    }

    pub const fn custody_checks(self) -> u64 {
        self.custody_checks
    }

    pub const fn custody_admitted(self) -> u64 {
        self.custody_admitted
    }

    pub const fn custody_denied(self) -> u64 {
        self.custody_denied
    }

    pub const fn key_version_stale(self) -> u64 {
        self.key_version_stale
    }

    pub const fn unsupported_secure_posture(self) -> u64 {
        self.unsupported_secure_posture
    }

    pub const fn unavailable_custody_evidence(self) -> u64 {
        self.unavailable_custody_evidence
    }

    pub const fn emissions_prepared(self) -> u64 {
        self.emissions_prepared
    }

    pub const fn terminal_projections_prepared(self) -> u64 {
        self.terminal_projections_prepared
    }

    pub const fn trust_boundary_crossings(self) -> u64 {
        self.trust_boundary_crossings
    }

    pub const fn readmissions(self) -> u64 {
        self.readmissions
    }

    pub const fn readmission_required(self) -> u64 {
        self.readmission_required
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
