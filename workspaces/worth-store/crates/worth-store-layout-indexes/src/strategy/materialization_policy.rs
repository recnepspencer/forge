use super::{
    AdmittedLayoutStrategy, StrategyAmplificationProfile, StrategyCorruptionIsolationBehavior,
    StrategyLocalityProfile, StrategyMaterializationPosture, StrategyRebuildSourceRequirement,
};
use crate::catalog::{DurableArtifactMigrationPosture, DurableArtifactRebuildPosture};
use crate::materialization::MaterializationStateClass;

impl AdmittedLayoutStrategy {
    pub const fn locality_profile(&self) -> StrategyLocalityProfile {
        self.declaration.locality()
    }
    pub const fn amplification_profile(&self) -> StrategyAmplificationProfile {
        self.declaration.amplification()
    }
    pub const fn materialization_posture(&self) -> StrategyMaterializationPosture {
        self.declaration.materialization()
    }
    pub const fn rebuild_source_requirement(&self) -> StrategyRebuildSourceRequirement {
        self.declaration.rebuild_source()
    }
    pub const fn corruption_isolation_behavior(&self) -> StrategyCorruptionIsolationBehavior {
        self.declaration.corruption_isolation()
    }
    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.declaration.rebuild_posture()
    }
    pub const fn migration_posture(&self) -> DurableArtifactMigrationPosture {
        self.declaration.migration_posture()
    }
    pub const fn supports_materialization_state(&self, state: MaterializationStateClass) -> bool {
        self.declaration.materialization().supports_state(state)
    }
}
