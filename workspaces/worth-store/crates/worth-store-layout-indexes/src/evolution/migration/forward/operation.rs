use worth_store_authority::StoreCurrentAuthorityWitness;

use super::{
    plan::{MigrationLowering, MigrationResolution},
    LayoutEvolutionDenial, LayoutMigrationPlan, LayoutMigrationRequest, LayoutRebindRequired,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMigrationOperation;

pub const fn layout_migration_operation() -> LayoutMigrationOperation {
    LayoutMigrationOperation
}

#[derive(Debug, PartialEq, Eq)]
enum MigrationPlanningCase {
    Ready(Box<LayoutMigrationPlan>),
    DeclarationDenied(LayoutEvolutionDenial),
    LoweringRebindRequired(LayoutRebindRequired),
}

#[derive(Debug, PartialEq, Eq)]
pub struct MigrationPlanningOutcome {
    case: MigrationPlanningCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationPlanningView<'a> {
    Ready(&'a LayoutMigrationPlan),
    DeclarationDenied(&'a LayoutEvolutionDenial),
    LoweringRebindRequired(&'a LayoutRebindRequired),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MigrationPlanningCaseId(&'static str);

impl MigrationPlanningCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn migration_planning_cases() -> impl Iterator<Item = MigrationPlanningCaseId> {
    [
        MigrationPlanningCaseId("layout.migration.planning.ready"),
        MigrationPlanningCaseId("layout.migration.planning.declaration_denied"),
        MigrationPlanningCaseId("layout.migration.planning.lowering_rebind_required"),
    ]
    .into_iter()
}

impl MigrationPlanningOutcome {
    fn issue(case: MigrationPlanningCase) -> Self {
        Self { case }
    }

    pub const fn view(&self) -> MigrationPlanningView<'_> {
        match &self.case {
            MigrationPlanningCase::Ready(value) => MigrationPlanningView::Ready(value),
            MigrationPlanningCase::DeclarationDenied(value) => {
                MigrationPlanningView::DeclarationDenied(value)
            }
            MigrationPlanningCase::LoweringRebindRequired(value) => {
                MigrationPlanningView::LoweringRebindRequired(value)
            }
        }
    }

    pub const fn case_id(&self) -> MigrationPlanningCaseId {
        match self.case {
            MigrationPlanningCase::Ready(_) => {
                MigrationPlanningCaseId("layout.migration.planning.ready")
            }
            MigrationPlanningCase::DeclarationDenied(_) => {
                MigrationPlanningCaseId("layout.migration.planning.declaration_denied")
            }
            MigrationPlanningCase::LoweringRebindRequired(_) => {
                MigrationPlanningCaseId("layout.migration.planning.lowering_rebind_required")
            }
        }
    }

    pub fn into_ready(self) -> Result<LayoutMigrationPlan, Self> {
        match self.case {
            MigrationPlanningCase::Ready(value) => Ok(*value),
            case => Err(Self { case }),
        }
    }
}

impl LayoutMigrationOperation {
    pub fn plan(
        self,
        request: LayoutMigrationRequest,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> MigrationPlanningOutcome {
        let resolved = match request.resolve() {
            MigrationResolution::Resolved(value) => value,
            MigrationResolution::Denied(denial) => {
                return MigrationPlanningOutcome::issue(MigrationPlanningCase::DeclarationDenied(
                    *denial,
                ));
            }
        };
        let lowered = match (*resolved).lower(current_store_authority) {
            MigrationLowering::Lowered(value) => value,
            MigrationLowering::RebindRequired(rebind) => {
                return MigrationPlanningOutcome::issue(
                    MigrationPlanningCase::LoweringRebindRequired(*rebind),
                );
            }
        };
        MigrationPlanningOutcome::issue(MigrationPlanningCase::Ready(lowered.finish()))
    }
}
