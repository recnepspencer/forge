#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioDriverKind {
    AdversarialByteDevice,
    PersistedFileDevice,
    CrashInterposer,
    LegacyBackendProbe,
    MemoryPressureDriver,
    PlatformBackendCandidate,
    VerifierOnlyReader,
    S3ByteFlipInjection,
    S3TornFrameInjection,
    S3StaleGenerationProbe,
    S3ManifestDamageInjection,
    S3IndexPageDamageInjection,
    S3WalFrameDamageInjection,
    S3ExtentDamageInjection,
    S3ChunkDamageInjection,
    S3BoundaryDenialProbe,
    S3SyntheticShortcutAttempt,
    S3RecoveryHandoffProbe,
    S3LineCapDiscovery,
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
            Self::S3ByteFlipInjection => "s3_byte_flip_injection",
            Self::S3TornFrameInjection => "s3_torn_frame_injection",
            Self::S3StaleGenerationProbe => "s3_stale_generation_probe",
            Self::S3ManifestDamageInjection => "s3_manifest_damage_injection",
            Self::S3IndexPageDamageInjection => "s3_index_page_damage_injection",
            Self::S3WalFrameDamageInjection => "s3_wal_frame_damage_injection",
            Self::S3ExtentDamageInjection => "s3_extent_damage_injection",
            Self::S3ChunkDamageInjection => "s3_chunk_damage_injection",
            Self::S3BoundaryDenialProbe => "s3_boundary_denial_probe",
            Self::S3SyntheticShortcutAttempt => "s3_synthetic_shortcut_attempt",
            Self::S3RecoveryHandoffProbe => "s3_recovery_handoff_probe",
            Self::S3LineCapDiscovery => "s3_line_cap_discovery",
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
