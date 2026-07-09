#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlobChunkScopeCounterSnapshot {
    readiness_inputs: u64,
    admitted_scope_consumed: u64,
    denials: u64,
    key_scope_preservations: u64,
    key_version_preservations: u64,
    tenant_scope_preservations: u64,
    authenticity_preservations: u64,
    custody_preservations: u64,
    metadata_witnesses_issued: u64,
    hostile_metadata_denials: u64,
}

impl BlobChunkScopeCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            readiness_inputs: 1,
            admitted_scope_consumed: 0,
            denials: 0,
            key_scope_preservations: 0,
            key_version_preservations: 0,
            tenant_scope_preservations: 0,
            authenticity_preservations: 0,
            custody_preservations: 0,
            metadata_witnesses_issued: 0,
            hostile_metadata_denials: 0,
        }
    }

    pub(crate) const fn admitted(self) -> Self {
        Self {
            admitted_scope_consumed: self.admitted_scope_consumed + 1,
            ..self
        }
    }

    pub(crate) const fn denied(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub(crate) const fn denied_hostile_metadata(self) -> Self {
        Self {
            denials: self.denials + 1,
            hostile_metadata_denials: self.hostile_metadata_denials + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_key_scope(self) -> Self {
        Self {
            key_scope_preservations: self.key_scope_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_key_version(self) -> Self {
        Self {
            key_version_preservations: self.key_version_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_tenant_scope(self) -> Self {
        Self {
            tenant_scope_preservations: self.tenant_scope_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_authenticity(self) -> Self {
        Self {
            authenticity_preservations: self.authenticity_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_custody(self) -> Self {
        Self {
            custody_preservations: self.custody_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn issue_metadata_witness(self) -> Self {
        Self {
            metadata_witnesses_issued: self.metadata_witnesses_issued + 1,
            ..self
        }
    }

    pub const fn readiness_inputs(self) -> u64 {
        self.readiness_inputs
    }

    pub const fn admitted_scope_consumed(self) -> u64 {
        self.admitted_scope_consumed
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }

    pub const fn key_scope_preservations(self) -> u64 {
        self.key_scope_preservations
    }

    pub const fn key_version_preservations(self) -> u64 {
        self.key_version_preservations
    }

    pub const fn tenant_scope_preservations(self) -> u64 {
        self.tenant_scope_preservations
    }

    pub const fn authenticity_preservations(self) -> u64 {
        self.authenticity_preservations
    }

    pub const fn custody_preservations(self) -> u64 {
        self.custody_preservations
    }

    pub const fn metadata_witnesses_issued(self) -> u64 {
        self.metadata_witnesses_issued
    }

    pub const fn hostile_metadata_denials(self) -> u64 {
        self.hostile_metadata_denials
    }
}
