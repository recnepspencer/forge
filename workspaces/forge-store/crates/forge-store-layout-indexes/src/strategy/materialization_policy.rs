use super::{
    S8AdmittedLayoutStrategy, S8StrategyAmplificationProfile,
    S8StrategyCorruptionIsolationBehavior, S8StrategyLocalityProfile,
    S8StrategyMaterializationPosture, S8StrategyRebuildSourceRequirement,
};
use crate::catalog::{DurableArtifactMigrationPosture, DurableArtifactRebuildPosture};
use crate::materialization::S8MaterializationStateClass;

impl S8AdmittedLayoutStrategy {
    pub const fn locality_profile(&self) -> S8StrategyLocalityProfile {
        self.declaration.locality()
    }
    pub const fn amplification_profile(&self) -> S8StrategyAmplificationProfile {
        self.declaration.amplification()
    }
    pub const fn materialization_posture(&self) -> S8StrategyMaterializationPosture {
        self.declaration.materialization()
    }
    pub const fn rebuild_source_requirement(&self) -> S8StrategyRebuildSourceRequirement {
        self.declaration.rebuild_source()
    }
    pub const fn corruption_isolation_behavior(&self) -> S8StrategyCorruptionIsolationBehavior {
        self.declaration.corruption_isolation()
    }
    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.declaration.rebuild_posture()
    }
    pub const fn migration_posture(&self) -> DurableArtifactMigrationPosture {
        self.declaration.migration_posture()
    }
    pub const fn supports_materialization_state(&self, state: S8MaterializationStateClass) -> bool {
        self.declaration.materialization().supports_state(state)
    }
}
