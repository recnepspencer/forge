#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioDriverKind {
    AdversarialByteDevice,
    PersistedFileDevice,
    CrashInterposer,
    LegacyBackendProbe,
    MemoryPressureDriver,
    PlatformBackendCandidate,
    VerifierOnlyReader,
    ByteFlipInjection,
    TornFrameInjection,
    StaleGenerationProbe,
    ManifestDamageInjection,
    IndexPageDamageInjection,
    WalFrameDamageInjection,
    ExtentDamageInjection,
    ChunkDamageInjection,
    IntegrityBoundaryDenialProbe,
    SyntheticShortcutAttempt,
    RecoveryIntegrityHandoffProbe,
    IntegrityCompositionDiscovery,
}

impl PhysicalScenarioDriverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdversarialByteDevice => "adversarial_byte_device",
            Self::PersistedFileDevice => "persisted_file_device",
            Self::CrashInterposer => "crash_interposer",
            Self::LegacyBackendProbe => "legacy_backend_probe",
            Self::MemoryPressureDriver => "memory_pressure_driver",
            Self::PlatformBackendCandidate => "platform_backend_candidate",
            Self::VerifierOnlyReader => "verifier_only_reader",
            Self::ByteFlipInjection => "s3_byte_flip_injection",
            Self::TornFrameInjection => "s3_torn_frame_injection",
            Self::StaleGenerationProbe => "s3_stale_generation_probe",
            Self::ManifestDamageInjection => "s3_manifest_damage_injection",
            Self::IndexPageDamageInjection => "s3_index_page_damage_injection",
            Self::WalFrameDamageInjection => "s3_wal_frame_damage_injection",
            Self::ExtentDamageInjection => "s3_extent_damage_injection",
            Self::ChunkDamageInjection => "s3_chunk_damage_injection",
            Self::IntegrityBoundaryDenialProbe => "s3_boundary_denial_probe",
            Self::SyntheticShortcutAttempt => "s3_synthetic_shortcut_attempt",
            Self::RecoveryIntegrityHandoffProbe => "s3_recovery_handoff_probe",
            Self::IntegrityCompositionDiscovery => "s3_line_cap_discovery",
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
