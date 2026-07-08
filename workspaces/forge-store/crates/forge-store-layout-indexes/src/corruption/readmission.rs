#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutReadmissionWitness {
    family: crate::PhysicalArtifactFamily,
    source: super::classification::S8LayoutReadmissionSource,
}

impl S8LayoutReadmissionWitness {
    pub(crate) const fn quarantine_recovery(family: crate::PhysicalArtifactFamily) -> Self {
        Self {
            family,
            source: super::classification::S8LayoutReadmissionSource::QuarantineRecovery,
        }
    }

    pub(crate) const fn offline_evidence(family: crate::PhysicalArtifactFamily) -> Self {
        Self {
            family,
            source: super::classification::S8LayoutReadmissionSource::OfflineRecoveryEvidence,
        }
    }

    pub(crate) const fn terminal_import(family: crate::PhysicalArtifactFamily) -> Self {
        Self {
            family,
            source: super::classification::S8LayoutReadmissionSource::TerminalImport,
        }
    }

    pub const fn family(self) -> crate::PhysicalArtifactFamily {
        self.family
    }

    pub const fn source(self) -> super::classification::S8LayoutReadmissionSource {
        self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8NativeReadmissionInput {
    RecoveryWitness {
        witness: forge_store_recovery_physics::RecoveryLayoutReadmissionWitness,
    },
}
