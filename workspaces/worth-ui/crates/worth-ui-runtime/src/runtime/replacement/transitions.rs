use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiDurableStateReconciliationPlan,
    WorthUiIdentityMatchReport, WorthUiNodeReplacementPlan, WorthUiQueryBindingComparison,
    WorthUiQueryLiveRebindPlan, WorthUiReplacementImpactClassification,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeImpactNarrowing,
};

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
    pub(crate) artifact_comparison_counters:
        crate::runtime::WorthUiRuntimeArtifactComparisonCounters,
}

/// Identity match graph completed for a narrowed replacement.
#[derive(Debug)]
pub struct WorthUiReplacementIdentityReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) identity_report: WorthUiIdentityMatchReport,
    pub(crate) artifact_comparison_counters:
        crate::runtime::WorthUiRuntimeArtifactComparisonCounters,
}

/// Node replacement plan completed for an identity-ready replacement.
#[derive(Debug)]
pub struct WorthUiReplacementNodePlanReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
    pub(crate) artifact_comparison_counters:
        crate::runtime::WorthUiRuntimeArtifactComparisonCounters,
    pub(crate) identity_match_counters: crate::runtime::WorthUiIdentityMatchCounters,
}

/// Durable-state reconciliation completed for a node-plan-ready replacement.
#[derive(Debug)]
pub struct WorthUiReplacementReconciliationReady {
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
    pub(crate) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub(crate) artifact_comparison_counters:
        crate::runtime::WorthUiRuntimeArtifactComparisonCounters,
    pub(crate) identity_match_counters: crate::runtime::WorthUiIdentityMatchCounters,
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
    pub(crate) artifact_comparison_counters:
        crate::runtime::WorthUiRuntimeArtifactComparisonCounters,
    pub(crate) identity_match_counters: crate::runtime::WorthUiIdentityMatchCounters,
}

/// Lowering input proof for activation staging; only minted by the replacement lane orchestrator.
#[derive(Debug)]
pub struct WorthUiReplacementLoweringReady {
    pub(crate) candidate_application_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub(crate) admitted: WorthUiAdmittedReplacementCandidate,
    pub(crate) impact: WorthUiReplacementImpactClassification,
    pub(crate) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(crate) node_plan: WorthUiNodeReplacementPlan,
    pub(crate) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub(crate) query_rebind_plan: WorthUiQueryLiveRebindPlan,
    pub(crate) artifact_comparison_counters:
        crate::runtime::WorthUiRuntimeArtifactComparisonCounters,
    pub(crate) identity_match_counters: crate::runtime::WorthUiIdentityMatchCounters,
}

impl WorthUiReplacementLoweringReady {
    pub fn admitted(&self) -> &WorthUiAdmittedReplacementCandidate {
        &self.admitted
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

    pub(crate) fn reload_cost_seed(&self) -> crate::runtime::WorthUiReloadCostSeed {
        crate::runtime::WorthUiReloadCostSeed::from_lowering(self)
    }
}
