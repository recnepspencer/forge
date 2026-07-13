use super::{
    LayoutEvolutionDenial, LayoutMigrationPlan, LayoutRebindRequired, LayoutRollbackPlan,
    LayoutStaleBinding,
};
macro_rules! define_planning_outcome {
    ($outcome:ident, $view:ident, $case:ident, $case_id:ident, $cases_fn:ident, $prefix:literal, $plan:ty) => {
        #[derive(Debug, PartialEq, Eq)]
        enum $case {
            Ready($plan),
            DeclarationDenied(LayoutEvolutionDenial),
            LoweringRebindRequired(LayoutRebindRequired),
            Stale(LayoutStaleBinding),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub struct $outcome {
            case: $case,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $view<'a> {
            Ready(&'a $plan),
            DeclarationDenied(&'a LayoutEvolutionDenial),
            LoweringRebindRequired(&'a LayoutRebindRequired),
            Stale(&'a LayoutStaleBinding),
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $case_id(&'static str);

        impl $case_id {
            pub const fn as_str(self) -> &'static str {
                self.0
            }
        }

        pub fn $cases_fn() -> impl Iterator<Item = $case_id> {
            [
                $case_id(concat!($prefix, ".ready")),
                $case_id(concat!($prefix, ".declaration_denied")),
                $case_id(concat!($prefix, ".lowering_rebind_required")),
                $case_id(concat!($prefix, ".stale")),
            ]
            .into_iter()
        }

        impl $outcome {
            pub(crate) fn ready(plan: $plan) -> Self {
                Self::from_owner_case($case::Ready(plan))
            }

            pub(crate) fn declaration_denied(denial: LayoutEvolutionDenial) -> Self {
                Self::from_owner_case($case::DeclarationDenied(denial))
            }

            pub(crate) fn lowering_rebind_required(rebind: LayoutRebindRequired) -> Self {
                Self::from_owner_case($case::LoweringRebindRequired(rebind))
            }

            pub(crate) fn stale(stale: LayoutStaleBinding) -> Self {
                Self::from_owner_case($case::Stale(stale))
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
                }
            }

            pub fn case_id(&self) -> $case_id {
                match &self.case {
                    $case::Ready(_) => $case_id(concat!($prefix, ".ready")),
                    $case::DeclarationDenied(_) => {
                        $case_id(concat!($prefix, ".declaration_denied"))
                    }
                    $case::LoweringRebindRequired(_) => {
                        $case_id(concat!($prefix, ".lowering_rebind_required"))
                    }
                    $case::Stale(_) => $case_id(concat!($prefix, ".stale")),
                }
            }

            pub fn into_ready(self) -> Result<$plan, Self> {
                match self.case {
                    $case::Ready(value) => Ok(value),
                    case => Err(Self { case }),
                }
            }
        }
    };
}

define_planning_outcome!(
    MigrationPlanningOutcome,
    MigrationPlanningView,
    MigrationPlanningCase,
    MigrationPlanningCaseId,
    migration_planning_cases,
    "layout.migration.planning",
    LayoutMigrationPlan
);

define_planning_outcome!(
    RollbackPlanningOutcome,
    RollbackPlanningView,
    RollbackPlanningCase,
    RollbackPlanningCaseId,
    rollback_planning_cases,
    "layout.rollback.planning",
    LayoutRollbackPlan
);
