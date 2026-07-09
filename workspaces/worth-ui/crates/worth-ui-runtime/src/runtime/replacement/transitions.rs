use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiDurableStateReconciliationPlan,
    WorthUiIdentityMatchReport, WorthUiNodeReplacementPlan,
    WorthUiPendingExecutionPlanLoweringInput, WorthUiQueryBindingComparison,
    WorthUiQueryLiveRebindPlan, WorthUiReplacementImpactClassification,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeImpactNarrowing,
};

/// Active-runtime observation basis for replacement admission.
#[derive(Debug, Clone)]
pub struct WorthUiReplacementAdmissionBasis(
    pub(crate) crate::runtime::WorthUiActiveReplacementBasis,
);

impl WorthUiReplacementAdmissionBasis {
    pub fn into_active_basis(self) -> crate::runtime::WorthUiActiveReplacementBasis {
        self.0
    }

    pub fn active_basis(&self) -> &crate::runtime::WorthUiActiveReplacementBasis {
        &self.0
    }
}

/// Artifact comparison completed for an admitted replacement candidate.
#[derive(Debug)]
pub struct WorthUiReplacementComparisonReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) comparison: WorthUiRuntimeArtifactComparison,
}

/// Impact classification completed for a compared replacement.
#[derive(Debug)]
pub struct WorthUiReplacementImpactReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) comparison: WorthUiRuntimeArtifactComparison,
    pub(crate) impact: WorthUiReplacementImpactClassification,
}

/// Impact narrowing completed for a classified replacement.
#[derive(Debug)]
pub struct WorthUiReplacementNarrowingReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
}

/// Identity match graph completed for a narrowed replacement.
#[derive(Debug)]
pub struct WorthUiReplacementIdentityReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) identity_report: WorthUiIdentityMatchReport,
}

/// Node replacement plan completed for an identity-ready replacement.
#[derive(Debug)]
pub struct WorthUiReplacementNodePlanReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
}

/// Durable-state reconciliation completed for a node-plan-ready replacement.
#[derive(Debug)]
pub struct WorthUiReplacementReconciliationReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
    pub(crate) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
}

/// Query binding comparison completed for a reconciliation-ready replacement.
#[derive(Debug)]
pub struct WorthUiReplacementQueryComparisonReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
    pub(crate) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub(crate) query_comparison: WorthUiQueryBindingComparison,
}

/// Lowering input proof for activation staging; only minted by the replacement lane orchestrator.
#[derive(Debug)]
pub struct WorthUiReplacementLoweringReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
    pub(crate) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub(crate) query_rebind_plan: WorthUiQueryLiveRebindPlan,
    pub(crate) pending_execution_plan_lowering_input: WorthUiPendingExecutionPlanLoweringInput,
}

impl WorthUiReplacementComparisonReady {
    pub fn admitted(&self) -> &WorthUiAdmittedReplacementCandidate {
        &self.admitted
    }

    pub fn comparison(&self) -> &WorthUiRuntimeArtifactComparison {
        &self.comparison
    }
}

impl WorthUiReplacementImpactReady {
    pub fn admitted(&self) -> &WorthUiAdmittedReplacementCandidate {
        &self.admitted
    }

    pub fn comparison(&self) -> &WorthUiRuntimeArtifactComparison {
        &self.comparison
    }

    pub fn impact(&self) -> &WorthUiReplacementImpactClassification {
        &self.impact
    }
}

impl WorthUiReplacementLoweringReady {
    pub fn admitted(&self) -> &WorthUiAdmittedReplacementCandidate {
        &self.admitted
    }

    pub fn impact(&self) -> &WorthUiReplacementImpactClassification {
        &self.impact
    }

    pub fn narrowing(&self) -> &WorthUiRuntimeImpactNarrowing {
        &self.narrowing
    }

    pub fn node_plan(&self) -> &WorthUiNodeReplacementPlan {
        &self.node_plan
    }

    pub fn reconciliation_plan(&self) -> &WorthUiDurableStateReconciliationPlan {
        &self.reconciliation_plan
    }

    pub fn query_rebind_plan(&self) -> &WorthUiQueryLiveRebindPlan {
        &self.query_rebind_plan
    }

    pub fn pending_execution_plan_lowering_input(
        &self,
    ) -> &WorthUiPendingExecutionPlanLoweringInput {
        &self.pending_execution_plan_lowering_input
    }
}
