#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RepairBlastRadiusCounterSnapshot {
    declaration_inputs: u64,
    admission_attempts: u64,
    repair_admitted: u64,
    repair_denied: u64,
    repair_reads_prepared: u64,
    cross_scope_region_rejections: u64,
    stale_key_version_rejections: u64,
    key_rebind_required_rejections: u64,
    custody_unavailable_rejections: u64,
    missing_authenticity_rejections: u64,
    quarantine_preserved_scope: u64,
}

impl RepairBlastRadiusCounterSnapshot {
    pub(crate) const fn from_declaration() -> Self {
        Self {
            declaration_inputs: 1,
            ..Self::empty()
        }
    }

    pub(crate) const fn empty() -> Self {
        Self {
            declaration_inputs: 0,
            admission_attempts: 0,
            repair_admitted: 0,
            repair_denied: 0,
            repair_reads_prepared: 0,
            cross_scope_region_rejections: 0,
            stale_key_version_rejections: 0,
            key_rebind_required_rejections: 0,
            custody_unavailable_rejections: 0,
            missing_authenticity_rejections: 0,
            quarantine_preserved_scope: 0,
        }
    }

    pub(crate) const fn attempted_admission(self) -> Self {
        Self {
            admission_attempts: self.admission_attempts + 1,
            ..self
        }
    }

    pub(crate) const fn admitted(self) -> Self {
        Self {
            repair_admitted: self.repair_admitted + 1,
            ..self
        }
    }

    pub(crate) const fn denied(self) -> Self {
        Self {
            repair_denied: self.repair_denied + 1,
            ..self
        }
    }

    pub(crate) const fn prepared_repair_read(self) -> Self {
        Self {
            repair_reads_prepared: self.repair_reads_prepared + 1,
            ..self
        }
    }

    pub(crate) const fn rejected_cross_scope_region(self) -> Self {
        Self {
            cross_scope_region_rejections: self.cross_scope_region_rejections + 1,
            ..self
        }
    }

    pub(crate) const fn rejected_stale_key_version(self) -> Self {
        Self {
            stale_key_version_rejections: self.stale_key_version_rejections + 1,
            ..self
        }
    }

    pub(crate) const fn rejected_key_rebind_required(self) -> Self {
        Self {
            key_rebind_required_rejections: self.key_rebind_required_rejections + 1,
            ..self
        }
    }

    pub(crate) const fn rejected_unavailable_custody(self) -> Self {
        Self {
            custody_unavailable_rejections: self.custody_unavailable_rejections + 1,
            ..self
        }
    }

    pub(crate) const fn rejected_missing_authenticity(self) -> Self {
        Self {
            missing_authenticity_rejections: self.missing_authenticity_rejections + 1,
            ..self
        }
    }

    pub(crate) const fn preserved_quarantine_scope(self) -> Self {
        Self {
            quarantine_preserved_scope: self.quarantine_preserved_scope + 1,
            ..self
        }
    }

    pub const fn declaration_inputs(self) -> u64 {
        self.declaration_inputs
    }

    pub const fn admission_attempts(self) -> u64 {
        self.admission_attempts
    }

    pub const fn repair_admitted(self) -> u64 {
        self.repair_admitted
    }

    pub const fn repair_denied(self) -> u64 {
        self.repair_denied
    }

    pub const fn repair_reads_prepared(self) -> u64 {
        self.repair_reads_prepared
    }

    pub const fn cross_scope_region_rejections(self) -> u64 {
        self.cross_scope_region_rejections
    }

    pub const fn stale_key_version_rejections(self) -> u64 {
        self.stale_key_version_rejections
    }

    pub const fn key_rebind_required_rejections(self) -> u64 {
        self.key_rebind_required_rejections
    }

    pub const fn custody_unavailable_rejections(self) -> u64 {
        self.custody_unavailable_rejections
    }

    pub const fn missing_authenticity_rejections(self) -> u64 {
        self.missing_authenticity_rejections
    }

    pub const fn quarantine_preserved_scope(self) -> u64 {
        self.quarantine_preserved_scope
    }
}
