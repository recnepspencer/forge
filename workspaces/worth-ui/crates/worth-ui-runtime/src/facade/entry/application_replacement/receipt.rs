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
        self.plan_decision
    }

    pub fn prior_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.prior_generation
    }

    pub fn active_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.active_generation
    }

    pub fn plan_swap(&self) -> &crate::runtime::WorthUiPlanSwapReceipt {
        &self.plan_swap
    }

    pub fn structural_reuse(&self) -> &crate::runtime::WorthUiPlanRegionalEvidence {
        self.plan_swap.structural_reuse()
    }

    pub fn allocation_catalog_successor(
        &self,
    ) -> &crate::runtime::UiAllocationCatalogSuccessorReceipt {
        &self.allocation_catalog_successor
    }

    pub fn publication(&self) -> &super::WorthUiApplicationPublicationObservation {
        &self.publication
    }

    pub fn managed_live_compatibility_retirement(
        &self,
    ) -> &worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirement {
        &self.query_retirement
    }

    pub fn into_managed_live_compatibility_retirement(
        self,
    ) -> worth_ui_query_binding::compatibility::managed_live::WorthUiQueryLiveRetirement {
        self.query_retirement
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
