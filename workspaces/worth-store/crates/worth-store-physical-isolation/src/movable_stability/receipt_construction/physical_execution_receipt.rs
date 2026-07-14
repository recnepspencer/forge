use super::super::{ChunkMigrationReadInterlockPlan, TierMovementStabilityCounterSnapshot};

#[cfg(any(test, feature = "certification-authority"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhysicalPlacementMovementExecutionAuthority {
    _private: (),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalPlacementMovementExecutionReceipt<ExecutionIntent> {
    intent: ExecutionIntent,
    movement_interlock: ChunkMigrationReadInterlockPlan,
    counters: TierMovementStabilityCounterSnapshot,
}

#[cfg_attr(not(any(test, feature = "certification-authority")), allow(dead_code))]
pub(crate) fn construct_physical_execution_receipt<ExecutionIntent>(
    intent: ExecutionIntent,
    movement_interlock: ChunkMigrationReadInterlockPlan,
) -> PhysicalPlacementMovementExecutionReceipt<ExecutionIntent> {
    PhysicalPlacementMovementExecutionReceipt {
        intent,
        movement_interlock,
        counters: TierMovementStabilityCounterSnapshot::default()
            .with_stability_admission()
            .with_chunk_placeholder(),
    }
}

#[cfg(any(test, feature = "certification-authority"))]
impl PhysicalPlacementMovementExecutionAuthority {
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub(crate) fn execute_admitted_chunk_migration<ExecutionIntent>(
        self,
        intent: ExecutionIntent,
        movement_interlock: ChunkMigrationReadInterlockPlan,
    ) -> PhysicalPlacementMovementExecutionReceipt<ExecutionIntent> {
        construct_physical_execution_receipt(intent, movement_interlock)
    }
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn physical_placement_movement_execution_for_certification_test<ExecutionIntent>(
    intent: ExecutionIntent,
    movement_interlock: ChunkMigrationReadInterlockPlan,
) -> PhysicalPlacementMovementExecutionReceipt<ExecutionIntent> {
    PhysicalPlacementMovementExecutionAuthority::store_owned()
        .execute_admitted_chunk_migration(intent, movement_interlock)
}

impl<ExecutionIntent> PhysicalPlacementMovementExecutionReceipt<ExecutionIntent> {
    pub const fn intent(&self) -> &ExecutionIntent {
        &self.intent
    }

    pub const fn movement_interlock(&self) -> ChunkMigrationReadInterlockPlan {
        self.movement_interlock
    }

    pub const fn counters(&self) -> TierMovementStabilityCounterSnapshot {
        self.counters
    }
}
