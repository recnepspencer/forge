use crate::{
    LaneFamilyExtension, PhysicalProofOracleKind, PhysicalScenarioDriverKind,
    PhysicalScenarioHarnessDenial, PhysicalScenarioObserverKind, PhysicalScenarioQualityHarness,
    RoadmapLaneFamily,
};

impl PhysicalScenarioQualityHarness {
    pub fn with_buffer_pool_large_store_pressure_lanes(
        self,
    ) -> Result<Self, PhysicalScenarioHarnessDenial> {
        self.with_lane_family_extension(pressure_lane(
            PhysicalProofOracleKind::LargeStorePressureBounded,
            PhysicalScenarioObserverKind::ResidentBudget,
        ))?
        .with_lane_family_extension(pressure_lane(
            PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization,
            PhysicalScenarioObserverKind::AllocationEnvelope,
        ))?
        .with_lane_family_extension(pressure_lane(
            PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization,
            PhysicalScenarioObserverKind::Materialization,
        ))?
        .with_lane_family_extension(pressure_lane(
            PhysicalProofOracleKind::PressureTranscriptReplayStable,
            PhysicalScenarioObserverKind::CounterBundle,
        ))?
        .with_lane_family_extension(pressure_lane(
            PhysicalProofOracleKind::ShortcutCertificationRejected,
            PhysicalScenarioObserverKind::MaterializationShortcut,
        ))
    }
}

const fn pressure_lane(
    oracle: PhysicalProofOracleKind,
    observer: PhysicalScenarioObserverKind,
) -> LaneFamilyExtension {
    LaneFamilyExtension::new(
        RoadmapLaneFamily::BufferPool,
        PhysicalScenarioDriverKind::MemoryPressureDriver,
        oracle,
    )
    .with_observer(observer)
}
