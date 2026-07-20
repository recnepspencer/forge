use crate::runtime::admission::WorthUiActiveReplacementBasis;
use crate::runtime::equivalence::WorthUiRuntimeArtifactComparator;
use crate::runtime::impact::WorthUiReplacementImpactClassifier;
use crate::runtime::matching::WorthUiIdentityMatchGraphBuilder;
use crate::runtime::narrowing::WorthUiRuntimeImpactNarrower;
use crate::runtime::query_binding::WorthUiQueryBindingComparisonPlanner;
use crate::runtime::query_live_rebind::WorthUiQueryLiveRebindPlanner;
use crate::runtime::reconciliation::WorthUiDurableStateReconciliationPlanner;
use crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassifier;
use crate::runtime::source_ingress::WorthUiSourceEventIngress;
use crate::runtime::state_inventory::WorthUiDurableStateInventoryBuilder;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiAmbiguousReplacementDenial,
    WorthUiDurableStateInventory, WorthUiDurableStateInventoryDenial,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationPlan,
    WorthUiIdentityMatchDenial, WorthUiIdentityMatchReport, WorthUiNodeReplacementPlan,
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial, WorthUiQueryLiveRebindPlan,
    WorthUiQueryLiveRebindPlanDenial, WorthUiReplacementImpactClassification,
    WorthUiReplacementImpactDenial, WorthUiRuntimeArtifactComparison,
    WorthUiRuntimeArtifactComparisonDenial, WorthUiRuntimeImpactNarrowing,
    WorthUiRuntimeImpactNarrowingDenial,
};

use super::transitions::{
    WorthUiReplacementAdmissionBasis, WorthUiReplacementComparisonReady,
    WorthUiReplacementIdentityReady, WorthUiReplacementImpactReady,
    WorthUiReplacementLoweringReady, WorthUiReplacementNarrowingReady,
    WorthUiReplacementNodePlanReady, WorthUiReplacementQueryComparisonReady,
    WorthUiReplacementReconciliationReady,
};
use crate::runtime::launch::runtime_instance::WorthUiRuntime;

impl WorthUiRuntime {
    pub(crate) fn replacement_admission_basis(&self) -> WorthUiActiveReplacementBasis {
        WorthUiActiveReplacementBasis::from_observation(self.inspect_active())
    }

    pub(crate) fn replacement_admission_transition(&self) -> WorthUiReplacementAdmissionBasis {
        WorthUiReplacementAdmissionBasis(self.replacement_admission_basis())
    }

    pub(crate) fn compare_admitted_replacement(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .compare_admitted(admitted)
    }

    pub(crate) fn compare_admitted_from_basis(
        &self,
        basis: &WorthUiReplacementAdmissionBasis,
        admitted: WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiReplacementComparisonReady, WorthUiRuntimeArtifactComparisonDenial> {
        let _ = basis;
        let comparison = self.compare_admitted_replacement(&admitted)?;
        Ok(WorthUiReplacementComparisonReady {
            admitted,
            comparison,
        })
    }

    pub(crate) fn classify_replacement_impact(
        &self,
        comparison: &WorthUiRuntimeArtifactComparison,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiReplacementImpactClassification, WorthUiReplacementImpactDenial> {
        WorthUiReplacementImpactClassifier::classify(
            self.active.active_artifact().artifact(),
            comparison,
            admitted,
        )
    }

    pub(crate) fn classify_replacement_impact_from_comparison(
        &self,
        ready: WorthUiReplacementComparisonReady,
    ) -> Result<WorthUiReplacementImpactReady, WorthUiReplacementImpactDenial> {
        let WorthUiReplacementComparisonReady {
            admitted,
            comparison,
        } = ready;
        let impact = self.classify_replacement_impact(&comparison, &admitted)?;
        Ok(WorthUiReplacementImpactReady {
            admitted,
            comparison,
            impact,
        })
    }

    pub(crate) fn narrow_replacement_impact(
        &self,
        classification: &WorthUiReplacementImpactClassification,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeImpactNarrowing, WorthUiRuntimeImpactNarrowingDenial> {
        WorthUiRuntimeImpactNarrower::narrow(classification, admitted)
    }

    pub(crate) fn narrow_replacement_impact_from_classification(
        &self,
        ready: WorthUiReplacementImpactReady,
    ) -> Result<WorthUiReplacementNarrowingReady, WorthUiRuntimeImpactNarrowingDenial> {
        let WorthUiReplacementImpactReady {
            admitted,
            comparison,
            impact,
        } = ready;
        let artifact_comparison_counters = comparison.counters();
        let narrowing = self.narrow_replacement_impact(&impact, &admitted)?;
        Ok(WorthUiReplacementNarrowingReady {
            admitted,
            impact,
            narrowing,
            artifact_comparison_counters,
        })
    }

    pub(crate) fn build_identity_match_graph(
        &self,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiIdentityMatchReport, WorthUiIdentityMatchDenial> {
        WorthUiIdentityMatchGraphBuilder::build(self.active.active_artifact(), narrowing, admitted)
    }

    pub(crate) fn build_identity_match_graph_from_narrowing(
        &self,
        ready: WorthUiReplacementNarrowingReady,
    ) -> Result<WorthUiReplacementIdentityReady, WorthUiIdentityMatchDenial> {
        let WorthUiReplacementNarrowingReady {
            admitted,
            impact,
            narrowing,
            artifact_comparison_counters,
        } = ready;
        let identity_report = self.build_identity_match_graph(&narrowing, &admitted)?;
        Ok(WorthUiReplacementIdentityReady {
            admitted,
            impact,
            narrowing,
            identity_report,
            artifact_comparison_counters,
        })
    }

    pub(crate) fn classify_node_replacements(
        &self,
        impact: &WorthUiReplacementImpactClassification,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        identity_report: &WorthUiIdentityMatchReport,
    ) -> Result<WorthUiNodeReplacementPlan, WorthUiAmbiguousReplacementDenial> {
        WorthUiNodeReplacementClassifier::classify(impact, narrowing, identity_report)
    }

    pub(crate) fn classify_node_replacements_from_identity(
        &self,
        ready: WorthUiReplacementIdentityReady,
    ) -> Result<WorthUiReplacementNodePlanReady, WorthUiAmbiguousReplacementDenial> {
        let WorthUiReplacementIdentityReady {
            admitted,
            impact,
            narrowing,
            identity_report,
            artifact_comparison_counters,
        } = ready;
        let identity_match_counters = identity_report.counters();
        let node_plan = self.classify_node_replacements(&impact, &narrowing, &identity_report)?;
        Ok(WorthUiReplacementNodePlanReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            artifact_comparison_counters,
            identity_match_counters,
        })
    }

    pub(crate) fn durable_state_inventory(&self) -> WorthUiDurableStateInventoryBuilder {
        WorthUiDurableStateInventoryBuilder::new()
    }

    pub(crate) fn source_event_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> WorthUiSourceEventIngress {
        WorthUiSourceEventIngress::new(provider)
    }

    pub(crate) fn reconcile_durable_state(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationDenial>
    {
        WorthUiDurableStateReconciliationPlanner::reconcile(node_plan, inventory)
    }

    pub(crate) fn reconcile_durable_state_from_node_plan(
        &self,
        ready: WorthUiReplacementNodePlanReady,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiReplacementReconciliationReady, WorthUiDurableStateReconciliationDenial>
    {
        let WorthUiReplacementNodePlanReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            artifact_comparison_counters,
            identity_match_counters,
        } = ready;
        let reconciliation_plan = self.reconcile_durable_state(&node_plan, inventory)?;
        Ok(WorthUiReplacementReconciliationReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            artifact_comparison_counters,
            identity_match_counters,
        })
    }

    pub(crate) fn compare_query_bindings(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        WorthUiQueryBindingComparisonPlanner::compare(
            self.active.active_artifact().artifact(),
            node_plan,
            narrowing,
            admitted,
        )
    }

    pub(crate) fn compare_query_bindings_from_reconciliation(
        &self,
        ready: WorthUiReplacementReconciliationReady,
    ) -> Result<WorthUiReplacementQueryComparisonReady, WorthUiQueryBindingComparisonDenial> {
        let WorthUiReplacementReconciliationReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            artifact_comparison_counters,
            identity_match_counters,
        } = ready;
        let query_comparison = self.compare_query_bindings(&node_plan, &narrowing, &admitted)?;
        Ok(WorthUiReplacementQueryComparisonReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        })
    }

    pub(crate) fn plan_query_live_rebinds(
        &self,
        comparison: &WorthUiQueryBindingComparison,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial> {
        WorthUiQueryLiveRebindPlanner::plan(comparison, node_plan, narrowing, admitted)
    }

    pub(crate) fn prepare_replacement_lowering_from_query_comparison(
        &self,
        ready: WorthUiReplacementQueryComparisonReady,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiQueryLiveRebindPlanDenial> {
        let WorthUiReplacementQueryComparisonReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        } = ready;
        let query_rebind_plan =
            self.plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)?;
        Ok(WorthUiReplacementLoweringReady {
            candidate_application_authority,
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            artifact_comparison_counters,
            identity_match_counters,
        })
    }

    pub(crate) fn prepare_application_replacement_lowering(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        configure: impl FnOnce(
            WorthUiDurableStateInventoryBuilder,
        ) -> WorthUiDurableStateInventoryBuilder,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
        if !candidate_application_authority.admits_candidate(&admitted) {
            return Err(WorthUiReplacementLoweringDenial::CandidateApplicationAuthorityMismatch);
        }
        let node_plan = self.prepare_replacement_node_plan(admitted)?;
        let inventory = configure(self.platform_durable_state_inventory())
            .build_for_replacement(&node_plan.node_plan)
            .map_err(WorthUiReplacementLoweringDenial::Inventory)?;
        self.finish_replacement_lowering(node_plan, &inventory, candidate_application_authority)
    }

    #[cfg(test)]
    pub(crate) fn prepare_replacement_lowering(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
        let candidate_application_authority = self
            .active_application_lowering_authority
            .synthetic_successor_for_certification(&admitted);
        let node_plan = self.prepare_replacement_node_plan(admitted)?;
        self.finish_replacement_lowering(node_plan, inventory, candidate_application_authority)
    }

    fn prepare_replacement_node_plan(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiReplacementNodePlanReady, WorthUiReplacementLoweringDenial> {
        let basis = self.replacement_admission_transition();
        let comparison = self
            .compare_admitted_from_basis(&basis, admitted)
            .map_err(WorthUiReplacementLoweringDenial::Comparison)?;
        let impact = self
            .classify_replacement_impact_from_comparison(comparison)
            .map_err(WorthUiReplacementLoweringDenial::Impact)?;
        let narrowing = self
            .narrow_replacement_impact_from_classification(impact)
            .map_err(WorthUiReplacementLoweringDenial::Narrowing)?;
        let identity = self
            .build_identity_match_graph_from_narrowing(narrowing)
            .map_err(WorthUiReplacementLoweringDenial::Identity)?;
        let node_plan = self
            .classify_node_replacements_from_identity(identity)
            .map_err(WorthUiReplacementLoweringDenial::NodePlan)?;
        Ok(node_plan)
    }

    fn finish_replacement_lowering(
        &self,
        node_plan: WorthUiReplacementNodePlanReady,
        inventory: &WorthUiDurableStateInventory,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
        let reconciliation = self
            .reconcile_durable_state_from_node_plan(node_plan, inventory)
            .map_err(WorthUiReplacementLoweringDenial::Reconciliation)?;
        let query_comparison = self
            .compare_query_bindings_from_reconciliation(reconciliation)
            .map_err(WorthUiReplacementLoweringDenial::QueryComparison)?;
        self.prepare_replacement_lowering_from_query_comparison(
            query_comparison,
            candidate_application_authority,
        )
        .map_err(WorthUiReplacementLoweringDenial::QueryRebind)
    }
}

#[derive(Debug)]
pub enum WorthUiReplacementLoweringDenial {
    CandidateApplicationAuthorityMismatch,
    Inventory(WorthUiDurableStateInventoryDenial),
    Comparison(WorthUiRuntimeArtifactComparisonDenial),
    Impact(WorthUiReplacementImpactDenial),
    Narrowing(WorthUiRuntimeImpactNarrowingDenial),
    Identity(WorthUiIdentityMatchDenial),
    NodePlan(WorthUiAmbiguousReplacementDenial),
    Reconciliation(WorthUiDurableStateReconciliationDenial),
    QueryComparison(WorthUiQueryBindingComparisonDenial),
    QueryRebind(WorthUiQueryLiveRebindPlanDenial),
}
