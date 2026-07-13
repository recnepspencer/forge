use super::{LayoutPlanBudget, LayoutPlanBudgetDenial, LayoutPlanBudgetScope, LayoutPlanWork};

#[derive(Debug, PartialEq, Eq)]
pub struct AdmittedLayoutPlanBudget {
    work: LayoutPlanWork,
    scope: LayoutPlanBudgetScope,
}

impl AdmittedLayoutPlanBudget {
    pub const fn work(&self) -> LayoutPlanWork {
        self.work
    }
    pub const fn scope(&self) -> LayoutPlanBudgetScope {
        self.scope
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LayoutPlanBudgetOutcome {
    Admitted(AdmittedLayoutPlanBudget),
    Denied(LayoutPlanBudgetDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutPlanBudgetAdmission;

impl LayoutPlanBudgetAdmission {
    pub fn admit(
        self,
        work: LayoutPlanWork,
        scope: LayoutPlanBudgetScope,
        budget: LayoutPlanBudget,
    ) -> LayoutPlanBudgetOutcome {
        let denial = if scope != budget.scope() {
            Some(LayoutPlanBudgetDenial::ScopeMismatch {
                requested: scope,
                admitted: budget.scope(),
            })
        } else if work.page_reads() > budget.max_page_reads() {
            Some(LayoutPlanBudgetDenial::PageReadsExceeded {
                planned: work.page_reads(),
                admitted: budget.max_page_reads(),
            })
        } else if work.byte_reads() > budget.max_byte_reads() {
            Some(LayoutPlanBudgetDenial::ByteReadsExceeded {
                planned: work.byte_reads(),
                admitted: budget.max_byte_reads(),
            })
        } else if work.allocations() > budget.max_allocations() {
            Some(LayoutPlanBudgetDenial::AllocationsExceeded {
                planned: work.allocations(),
                admitted: budget.max_allocations(),
            })
        } else {
            None
        };

        match denial {
            Some(denial) => LayoutPlanBudgetOutcome::Denied(denial),
            None => LayoutPlanBudgetOutcome::Admitted(AdmittedLayoutPlanBudget { work, scope }),
        }
    }
}

pub const fn layout_plan_budget_admission() -> LayoutPlanBudgetAdmission {
    LayoutPlanBudgetAdmission
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admission_is_bound_to_scope_and_exact_work() {
        let outcome = layout_plan_budget_admission().admit(
            LayoutPlanWork::exact(2, 4096, 0),
            LayoutPlanBudgetScope::ForegroundIndexed,
            LayoutPlanBudget::new(LayoutPlanBudgetScope::ForegroundIndexed, 2, 4096, 0),
        );
        assert!(matches!(outcome, LayoutPlanBudgetOutcome::Admitted(_)));
    }
}
