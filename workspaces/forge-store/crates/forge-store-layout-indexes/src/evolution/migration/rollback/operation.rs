use forge_store_authority::StoreCurrentAuthorityWitness;

use super::{
    plan::{RollbackLowering, RollbackResolution},
    LayoutEvolutionDenial, LayoutRebindRequired, LayoutRollbackPlan, LayoutRollbackRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRollbackOperation;

pub const fn layout_rollback_operation() -> LayoutRollbackOperation {
    LayoutRollbackOperation
}

#[derive(Debug, PartialEq, Eq)]
enum RollbackPlanningCase {
    Ready(Box<LayoutRollbackPlan>),
    DeclarationDenied(LayoutEvolutionDenial),
    LoweringRebindRequired(LayoutRebindRequired),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RollbackPlanningOutcome {
    case: RollbackPlanningCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackPlanningView<'a> {
    Ready(&'a LayoutRollbackPlan),
    DeclarationDenied(&'a LayoutEvolutionDenial),
    LoweringRebindRequired(&'a LayoutRebindRequired),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RollbackPlanningCaseId(&'static str);

impl RollbackPlanningCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn rollback_planning_cases() -> impl Iterator<Item = RollbackPlanningCaseId> {
    [
        RollbackPlanningCaseId("layout.rollback.planning.ready"),
        RollbackPlanningCaseId("layout.rollback.planning.declaration_denied"),
        RollbackPlanningCaseId("layout.rollback.planning.lowering_rebind_required"),
    ]
    .into_iter()
}

impl RollbackPlanningOutcome {
    fn issue(case: RollbackPlanningCase) -> Self {
        Self { case }
    }

    pub const fn view(&self) -> RollbackPlanningView<'_> {
        match &self.case {
            RollbackPlanningCase::Ready(value) => RollbackPlanningView::Ready(value),
            RollbackPlanningCase::DeclarationDenied(value) => {
                RollbackPlanningView::DeclarationDenied(value)
            }
            RollbackPlanningCase::LoweringRebindRequired(value) => {
                RollbackPlanningView::LoweringRebindRequired(value)
            }
        }
    }

    pub const fn case_id(&self) -> RollbackPlanningCaseId {
        match self.case {
            RollbackPlanningCase::Ready(_) => {
                RollbackPlanningCaseId("layout.rollback.planning.ready")
            }
            RollbackPlanningCase::DeclarationDenied(_) => {
                RollbackPlanningCaseId("layout.rollback.planning.declaration_denied")
            }
            RollbackPlanningCase::LoweringRebindRequired(_) => {
                RollbackPlanningCaseId("layout.rollback.planning.lowering_rebind_required")
            }
        }
    }

    pub fn into_ready(self) -> Result<LayoutRollbackPlan, Self> {
        match self.case {
            RollbackPlanningCase::Ready(value) => Ok(*value),
            case => Err(Self { case }),
        }
    }
}

impl LayoutRollbackOperation {
    pub fn plan(
        self,
        request: LayoutRollbackRequest,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> RollbackPlanningOutcome {
        let resolved = match request.resolve() {
            RollbackResolution::Resolved(value) => value,
            RollbackResolution::Denied(denial) => {
                return RollbackPlanningOutcome::issue(RollbackPlanningCase::DeclarationDenied(
                    *denial,
                ));
            }
        };
        let lowered = match (*resolved).lower(current_store_authority) {
            RollbackLowering::Lowered(value) => value,
            RollbackLowering::RebindRequired(rebind) => {
                return RollbackPlanningOutcome::issue(
                    RollbackPlanningCase::LoweringRebindRequired(*rebind),
                );
            }
        };
        RollbackPlanningOutcome::issue(RollbackPlanningCase::Ready(lowered.finish()))
    }
}
