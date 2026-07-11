use core::convert::Infallible;

use forge_proof::TransitionOutcome;

use super::{
    LayoutEvolutionDenial, LayoutMigrationPlan, LayoutRollbackPlan, S8LayoutRebindRequired,
    S8LayoutStaleBinding,
};
use crate::production_transition::define_owner_outcome;

macro_rules! define_planning_outcome {
    ($outcome:ident, $view:ident, $case:ident, $plan:ty, $operation:ident) => {
        define_owner_outcome!(
            pub $outcome,
            pub $view,
            $case,
            MigrationRollbackPlanning,
            $operation,
            [
                ready => Ready($plan): Declared => Ready => Ready,
                declaration_denied => DeclarationDenied(LayoutEvolutionDenial): Declared => Deny => Denied,
                lowering_rebind_required => LoweringRebindRequired(S8LayoutRebindRequired): Declared => RequireRebind => RebindRequired,
                stale => Stale(S8LayoutStaleBinding): Declared => Ready => Stale,
                readiness_rebind_required => ReadinessRebindRequired(S8LayoutRebindRequired): Declared => RequireRebind => RebindRequired
            ]
        );

        impl $outcome {
            pub fn into_transition_outcome(
                self,
            ) -> TransitionOutcome<
                $plan,
                LayoutEvolutionDenial,
                Infallible,
                S8LayoutStaleBinding,
                S8LayoutRebindRequired,
            > {
                match self.into_owner_payload() {
                    $case::Ready(value) => TransitionOutcome::success(value),
                    $case::DeclarationDenied(denial) => TransitionOutcome::denied(denial),
                    $case::LoweringRebindRequired(rebind)
                    | $case::ReadinessRebindRequired(rebind) => {
                        TransitionOutcome::rebind_required(rebind)
                    }
                    $case::Stale(stale) => TransitionOutcome::stale(stale),
                }
            }
        }
    };
}

define_planning_outcome!(
    S8MigrationPlanningOutcome,
    S8MigrationPlanningView,
    S8MigrationPlanningCase,
    LayoutMigrationPlan,
    PlanMigration
);

define_planning_outcome!(
    S8RollbackPlanningOutcome,
    S8RollbackPlanningView,
    S8RollbackPlanningCase,
    LayoutRollbackPlan,
    PlanRollback
);
