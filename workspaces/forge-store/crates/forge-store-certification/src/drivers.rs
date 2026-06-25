#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioDriverKind {
    AdversarialByteDevice,
    PersistedFileDevice,
    CrashInterposer,
    LegacyBackendProbe,
    PlatformBackendCandidate,
    VerifierOnlyReader,
}

impl PhysicalScenarioDriverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdversarialByteDevice => "adversarial_byte_device",
            Self::PersistedFileDevice => "persisted_file_device",
            Self::CrashInterposer => "crash_interposer",
            Self::LegacyBackendProbe => "legacy_backend_probe",
            Self::PlatformBackendCandidate => "platform_backend_candidate",
            Self::VerifierOnlyReader => "verifier_only_reader",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalScenarioDriverRequirement {
    kind: PhysicalScenarioDriverKind,
}

impl PhysicalScenarioDriverRequirement {
    pub const fn new(kind: PhysicalScenarioDriverKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> PhysicalScenarioDriverKind {
        self.kind
    }
}
