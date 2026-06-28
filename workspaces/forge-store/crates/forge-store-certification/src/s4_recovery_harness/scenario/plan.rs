use super::{
    RecoveryPhysicsCounterKind, RecoveryPhysicsCrashLane, RecoveryPhysicsScenarioDefinition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsScenarioPlan {
    definition: RecoveryPhysicsScenarioDefinition,
}

impl RecoveryPhysicsScenarioPlan {
    pub fn lower(
        definition: RecoveryPhysicsScenarioDefinition,
    ) -> Result<Self, RecoveryPhysicsScenarioPlanDenial> {
        if !definition.drivers().fresh_runtime().is_fresh_runtime() {
            return Err(RecoveryPhysicsScenarioPlanDenial::LiveRuntimeReuse);
        }
        if definition.drivers().storage_interposer().backend_profile()
            != definition.backend_profile()
        {
            return Err(RecoveryPhysicsScenarioPlanDenial::BackendProfileMismatch);
        }
        if definition.boundary_event().backend_profile() != definition.backend_profile() {
            return Err(RecoveryPhysicsScenarioPlanDenial::BoundaryEventProfileMismatch);
        }
        if definition.boundary_event().seam() != definition.lane().crash_seam() {
            return Err(RecoveryPhysicsScenarioPlanDenial::BoundaryEventSeamMismatch);
        }
        for kind in RecoveryPhysicsCounterKind::REQUIRED_SCENARIO_COUNTERS {
            if !definition
                .counter_expectations()
                .iter()
                .any(|expectation| expectation.kind() == kind)
            {
                return Err(RecoveryPhysicsScenarioPlanDenial::MissingCounter(kind));
            }
        }
        Ok(Self { definition })
    }

    pub const fn lane(&self) -> RecoveryPhysicsCrashLane {
        self.definition.lane()
    }

    pub const fn seed(&self) -> u64 {
        self.definition.seed()
    }

    pub const fn backend_profile(&self) -> &'static str {
        self.definition.backend_profile()
    }

    pub const fn definition(&self) -> &RecoveryPhysicsScenarioDefinition {
        &self.definition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPhysicsScenarioPlanDenial {
    MissingCounter(RecoveryPhysicsCounterKind),
    LiveRuntimeReuse,
    BackendProfileMismatch,
    BoundaryEventProfileMismatch,
    BoundaryEventSeamMismatch,
}
