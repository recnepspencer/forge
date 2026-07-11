use core::convert::Infallible;

use forge_proof::TransitionOutcome;

use super::{
    LayoutEvolutionDenial, LayoutMigrationPlan, LayoutRollbackPlan, S8LayoutRebindRequired,
    S8LayoutStaleBinding,
};
macro_rules! define_planning_outcome {
    ($outcome:ident, $view:ident, $case:ident, $plan:ty) => {
        #[derive(Debug, PartialEq, Eq)]
        enum $case {
            Ready($plan),
            DeclarationDenied(LayoutEvolutionDenial),
            LoweringRebindRequired(S8LayoutRebindRequired),
            Stale(S8LayoutStaleBinding),
            ReadinessRebindRequired(S8LayoutRebindRequired),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub struct $outcome {
            case: $case,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $view<'a> {
            Ready(&'a $plan),
            DeclarationDenied(&'a LayoutEvolutionDenial),
            LoweringRebindRequired(&'a S8LayoutRebindRequired),
            Stale(&'a S8LayoutStaleBinding),
            ReadinessRebindRequired(&'a S8LayoutRebindRequired),
        }

        impl $outcome {
            pub(crate) fn ready(plan: $plan) -> Self {
                Self::from_owner_case($case::Ready(plan))
            }

            pub(crate) fn declaration_denied(denial: LayoutEvolutionDenial) -> Self {
                Self::from_owner_case($case::DeclarationDenied(denial))
            }

            pub(crate) fn lowering_rebind_required(rebind: S8LayoutRebindRequired) -> Self {
                Self::from_owner_case($case::LoweringRebindRequired(rebind))
            }

            pub(crate) fn stale(stale: S8LayoutStaleBinding) -> Self {
                Self::from_owner_case($case::Stale(stale))
            }

            pub(crate) fn readiness_rebind_required(rebind: S8LayoutRebindRequired) -> Self {
                Self::from_owner_case($case::ReadinessRebindRequired(rebind))
            }

            fn from_owner_case(case: $case) -> Self {
                Self { case }
            }

            pub fn view(&self) -> $view<'_> {
                match &self.case {
                    $case::Ready(value) => $view::Ready(value),
                    $case::DeclarationDenied(value) => $view::DeclarationDenied(value),
                    $case::LoweringRebindRequired(value) => $view::LoweringRebindRequired(value),
                    $case::Stale(value) => $view::Stale(value),
                    $case::ReadinessRebindRequired(value) => $view::ReadinessRebindRequired(value),
                }
            }

            pub fn into_transition_outcome(
                self,
            ) -> TransitionOutcome<
                $plan,
                LayoutEvolutionDenial,
                Infallible,
                S8LayoutStaleBinding,
                S8LayoutRebindRequired,
            > {
                match self.case {
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
    LayoutMigrationPlan
);

define_planning_outcome!(
    S8RollbackPlanningOutcome,
    S8RollbackPlanningView,
    S8RollbackPlanningCase,
    LayoutRollbackPlan
);
