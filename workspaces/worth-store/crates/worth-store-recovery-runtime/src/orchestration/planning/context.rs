use worth_store_recovery_physics::{
    PhysicalRedoPlanningDenial, PhysicalSourceSelection, RecoveryPlanCostDenial,
    RecoveryPlanningCounters,
};

use crate::entry::{
    AdmittedPlatformAuthority, PhysicalRecoveryBlockKind, PhysicalRecoveryLimitDeclaration,
    PhysicalRecoveryLimitFailure, PhysicalRecoveryOutcome, PhysicalRecoveryPlanningDenial,
};
use crate::progression::{PhysicalRecoveryDiscoveryCounters, SelectedPhysicalRecovery};

use super::super::RecoveryCoordination;
use super::denial::{
    block, block_with_planning_attempt_denial, cost_denial_block, redo_block, redo_denial_block,
};

pub(super) struct PlanningContext {
    pub(super) authority: AdmittedPlatformAuthority,
    pub(super) coordination: RecoveryCoordination,
    pub(super) selection: PhysicalSourceSelection,
    pub(super) counters: PhysicalRecoveryDiscoveryCounters,
    pub(super) limits: PhysicalRecoveryLimitDeclaration,
    pub(super) effects_before: u64,
}

impl PlanningContext {
    pub(super) fn from_selected(selected: SelectedPhysicalRecovery) -> Self {
        let (authority, coordination, selection, counters) = selected.into_parts();
        let limits = authority.limits.declaration();
        let effects_before = authority.media.recovery_effect_count();
        Self {
            authority,
            coordination,
            selection,
            counters,
            limits,
            effects_before,
        }
    }

    pub(super) fn block(
        self,
        kind: PhysicalRecoveryBlockKind,
        artifact: &str,
        limit: Option<PhysicalRecoveryLimitFailure>,
    ) -> PhysicalRecoveryOutcome {
        block(
            self.authority,
            self.coordination,
            kind,
            self.counters,
            artifact,
            limit,
        )
    }

    pub(super) fn block_with_planning_attempt_denial(
        self,
        kind: PhysicalRecoveryBlockKind,
        planning_counters: RecoveryPlanningCounters,
        artifact: &str,
        limit: Option<PhysicalRecoveryLimitFailure>,
        denial: PhysicalRecoveryPlanningDenial,
    ) -> PhysicalRecoveryOutcome {
        block_with_planning_attempt_denial(
            self.authority,
            self.coordination,
            kind,
            self.counters,
            planning_counters,
            artifact,
            limit,
            denial,
        )
    }

    pub(super) fn redo_block(
        self,
        planning_counters: RecoveryPlanningCounters,
        limit: Option<PhysicalRecoveryLimitFailure>,
    ) -> PhysicalRecoveryOutcome {
        redo_block(
            self.authority,
            self.coordination,
            self.counters,
            planning_counters,
            limit,
        )
    }

    pub(super) fn redo_denial_block(
        self,
        planning_counters: RecoveryPlanningCounters,
        limit: Option<PhysicalRecoveryLimitFailure>,
        denial: PhysicalRedoPlanningDenial,
    ) -> PhysicalRecoveryOutcome {
        redo_denial_block(
            self.authority,
            self.coordination,
            self.counters,
            planning_counters,
            limit,
            denial,
        )
    }

    pub(super) fn cost_denial_block(
        self,
        planning_counters: RecoveryPlanningCounters,
        denial: RecoveryPlanCostDenial,
        limit: PhysicalRecoveryLimitFailure,
    ) -> PhysicalRecoveryOutcome {
        cost_denial_block(
            self.authority,
            self.coordination,
            self.counters,
            planning_counters,
            denial,
            limit,
        )
    }
}
