use super::{
    WorthUiApplicationCutoverReceipt, WorthUiApplicationReplacementOutcome,
    WorthUiCandidateInspectionReceipt,
};
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

impl WorthUiCandidateInspectionReceipt {
    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn candidate_basis(&self) -> crate::runtime::WorthUiReplacementCandidateBasis {
        self.candidate_basis
    }

    pub fn receipt(&self) -> &crate::facade::inspection_bridge::UiInspectionReceipt {
        &self.receipt
    }
}

impl WorthUiApplicationCutoverReceipt {
    pub fn plan_decision(&self) -> crate::runtime::WorthUiExecutablePlanDecision {
        self.committed_parts().1
    }

    pub fn prior_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.transition.identity.prior_generation
    }

    pub fn active_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.transition.identity.active_generation
    }

    pub fn plan_swap(&self) -> &crate::runtime::WorthUiPlanSwapReceipt {
        self.committed_parts().0
    }

    pub fn structural_reuse(&self) -> &crate::runtime::WorthUiPlanRegionalEvidence {
        self.plan_swap().structural_reuse()
    }

    pub fn allocation_catalog_successor(
        &self,
    ) -> &crate::runtime::UiAllocationCatalogSuccessorReceipt {
        self.committed_parts().3
    }

    pub fn publication(&self) -> &super::WorthUiApplicationPublicationObservation {
        &self.transition.publication
    }

    pub fn operation_live_retirement(
        &self,
    ) -> &worth_ui_query_binding::WorthUiOperationLiveRetirement {
        self.committed_parts().2
    }

    pub fn into_operation_live_retirement(
        self,
    ) -> worth_ui_query_binding::WorthUiOperationLiveRetirement {
        let transition = self
            .transition
            .transition
            .expect("published application transition is present");
        match transition {
            super::WorthUiApplicationCutoverTransition::Committed {
                query_retirement, ..
            } => query_retirement,
            super::WorthUiApplicationCutoverTransition::Prepared(_) => {
                unreachable!("published application receipt cannot be prepared")
            }
        }
    }

    pub fn reload_cost(
        &self,
    ) -> Result<
        &crate::runtime::WorthUiReloadLoweringCounterReceipt,
        &crate::runtime::WorthUiReloadCounterBoundaryDenial,
    > {
        self.transition.reload_cost.as_ref()
    }

    fn committed_parts(
        &self,
    ) -> (
        &crate::runtime::WorthUiPlanSwapReceipt,
        crate::runtime::WorthUiExecutablePlanDecision,
        &worth_ui_query_binding::WorthUiOperationLiveRetirement,
        &crate::runtime::UiAllocationCatalogSuccessorReceipt,
    ) {
        match self
            .transition
            .transition
            .as_ref()
            .expect("published application transition is present")
        {
            super::WorthUiApplicationCutoverTransition::Committed {
                plan_swap,
                plan_decision,
                query_retirement,
                allocation_catalog_successor,
            } => (
                plan_swap,
                *plan_decision,
                query_retirement,
                allocation_catalog_successor,
            ),
            super::WorthUiApplicationCutoverTransition::Prepared(_) => {
                unreachable!("published application receipt cannot be prepared")
            }
        }
    }
}

impl super::WorthUiApplicationSemanticNoOpReceipt {
    pub fn receipt(&self) -> &crate::runtime::WorthUiSemanticNoOpReceipt {
        &self.receipt
    }

    pub fn reload_cost(
        &self,
    ) -> Result<
        &crate::runtime::WorthUiReloadLoweringCounterReceipt,
        &crate::runtime::WorthUiReloadCounterBoundaryDenial,
    > {
        self.reload_cost.as_ref()
    }
}

impl WorthUiApplicationReplacementOutcome {
    pub fn semantic_no_op(&self) -> Option<&crate::runtime::WorthUiSemanticNoOpReceipt> {
        match self {
            Self::SemanticNoOp(receipt) => Some(receipt.receipt()),
            Self::Activated(_) => None,
        }
    }

    pub fn activation(&self) -> Option<&WorthUiApplicationCutoverReceipt> {
        match self {
            Self::SemanticNoOp(_) => None,
            Self::Activated(receipt) => Some(receipt.as_ref()),
        }
    }

    pub fn into_activation(self) -> Option<WorthUiApplicationCutoverReceipt> {
        match self {
            Self::SemanticNoOp(_) => None,
            Self::Activated(receipt) => Some(*receipt),
        }
    }
}
