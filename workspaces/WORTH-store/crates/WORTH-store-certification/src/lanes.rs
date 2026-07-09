use crate::{PhysicalProofOracleKind, PhysicalScenarioDriverKind, PhysicalScenarioObserverKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoadmapLaneFamily {
    PhysicalSubstrate,
    BufferPool,
    Integrity,
    WalRecovery,
    PhysicalIsolation,
    IoQos,
    BlobChunks,
    LayoutIndexes,
    FormalModelAlignment,
    OperationsRepair,
    SecurityTenant,
    PhysicalCertification,
}

impl RoadmapLaneFamily {
    pub const fn reserved_follow_on() -> [Self; 11] {
        [
            Self::BufferPool,
            Self::Integrity,
            Self::WalRecovery,
            Self::PhysicalIsolation,
            Self::IoQos,
            Self::BlobChunks,
            Self::LayoutIndexes,
            Self::FormalModelAlignment,
            Self::OperationsRepair,
            Self::SecurityTenant,
            Self::PhysicalCertification,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalSubstrate => "physical_substrate",
            Self::BufferPool => "buffer_pool",
            Self::Integrity => "integrity",
            Self::WalRecovery => "wal_recovery",
            Self::PhysicalIsolation => "physical_isolation",
            Self::IoQos => "io_qos",
            Self::BlobChunks => "blob_chunks",
            Self::LayoutIndexes => "layout_indexes",
            Self::FormalModelAlignment => "formal_model_alignment",
            Self::OperationsRepair => "operations_repair",
            Self::SecurityTenant => "security_tenant",
            Self::PhysicalCertification => "physical_certification",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalSubstrateLane {
    HappyAuthority,
    HostileReference,
    HostileFormat,
    LegacyOverclaim,
    OfflineVerifier,
    ScaleLocality,
    FoundationalExport,
    S2Handoff,
}

impl PhysicalSubstrateLane {
    pub const fn family(self) -> RoadmapLaneFamily {
        RoadmapLaneFamily::PhysicalSubstrate
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HappyAuthority => "happy_authority",
            Self::HostileReference => "hostile_reference",
            Self::HostileFormat => "hostile_format",
            Self::LegacyOverclaim => "legacy_overclaim",
            Self::OfflineVerifier => "offline_verifier",
            Self::ScaleLocality => "scale_locality",
            Self::FoundationalExport => "foundational_export",
            Self::S2Handoff => "s2_handoff",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaneFamilyExtension {
    family: RoadmapLaneFamily,
    driver: PhysicalScenarioDriverKind,
    observer: PhysicalScenarioObserverKind,
    oracle: PhysicalProofOracleKind,
}

impl LaneFamilyExtension {
    pub const fn new(
        family: RoadmapLaneFamily,
        driver: PhysicalScenarioDriverKind,
        oracle: PhysicalProofOracleKind,
    ) -> Self {
        Self {
            family,
            driver,
            observer: PhysicalScenarioObserverKind::CounterBundle,
            oracle,
        }
    }

    pub const fn with_observer(mut self, observer: PhysicalScenarioObserverKind) -> Self {
        self.observer = observer;
        self
    }

    pub const fn family(&self) -> RoadmapLaneFamily {
        self.family
    }

    pub const fn driver(&self) -> PhysicalScenarioDriverKind {
        self.driver
    }

    pub const fn observer(&self) -> PhysicalScenarioObserverKind {
        self.observer
    }

    pub const fn oracle(&self) -> PhysicalProofOracleKind {
        self.oracle
    }
}
