use crate::runtime::WorthUiPlanSwapReceipt;

pub(crate) struct WorthUiQueryAwarePlanSwap {
    plan_swap: WorthUiPlanSwapReceipt,
    query_retirement:
        worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirement,
    plan_decision: crate::runtime::WorthUiExecutablePlanDecision,
    catalog_successor_receipt: Option<crate::runtime::UiAllocationCatalogSuccessorReceipt>,
}

pub(crate) enum WorthUiQueryAwarePlanOutcome {
    SemanticNoOp(Box<crate::runtime::WorthUiSemanticNoOpReceipt>),
    Activated(Box<WorthUiQueryAwarePlanSwap>),
}

impl WorthUiQueryAwarePlanOutcome {
    #[cfg(any(test, feature = "certification-support"))]
    pub(super) fn into_plan_swap_after_asserting_no_query_retirement(
        self,
    ) -> WorthUiPlanSwapReceipt {
        match self {
            Self::Activated(publication) => {
                publication.into_plan_swap_after_asserting_no_query_retirement()
            }
            Self::SemanticNoOp(_) => {
                panic!("lower-level publication helper received a semantic no-op")
            }
        }
    }
}

impl WorthUiQueryAwarePlanSwap {
    pub(super) fn new(
        plan_swap: WorthUiPlanSwapReceipt,
        query_retirement:
            worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirement,
        plan_decision: crate::runtime::WorthUiExecutablePlanDecision,
        catalog_successor_receipt: Option<crate::runtime::UiAllocationCatalogSuccessorReceipt>,
    ) -> Self {
        Self {
            plan_swap,
            query_retirement,
            plan_decision,
            catalog_successor_receipt,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiPlanSwapReceipt,
        worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirement,
        crate::runtime::WorthUiExecutablePlanDecision,
        Option<crate::runtime::UiAllocationCatalogSuccessorReceipt>,
    ) {
        (
            self.plan_swap,
            self.query_retirement,
            self.plan_decision,
            self.catalog_successor_receipt,
        )
    }

    #[cfg(any(test, feature = "certification-support"))]
    fn into_plan_swap_after_asserting_no_query_retirement(self) -> WorthUiPlanSwapReceipt {
        let (plan_swap, query_retirement, plan_decision, catalog_successor_receipt) =
            self.into_parts();
        assert!(
            query_retirement.is_empty(),
            "lower-level activation helpers cannot erase Query retirement; use the public Query-aware cutover"
        );
        debug_assert_ne!(
            plan_decision.kind(),
            crate::runtime::WorthUiExecutablePlanDecisionKind::Denied
        );
        debug_assert!(catalog_successor_receipt.is_none());
        plan_swap
    }
}
