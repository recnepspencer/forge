use crate::runtime::admission::WorthUiActiveReplacementBasis;
use crate::runtime::equivalence::WorthUiRuntimeArtifactComparator;
use crate::runtime::impact::WorthUiReplacementImpactClassifier;
use crate::runtime::matching::WorthUiIdentityMatchGraphBuilder;
use crate::runtime::narrowing::WorthUiRuntimeImpactNarrower;
use crate::runtime::query_binding::WorthUiQueryBindingComparisonPlanner;
use crate::runtime::query_live_rebind::WorthUiQueryLiveRebindPlanner;
use crate::runtime::reconciliation::WorthUiDurableStateReconciliationPlanner;
use crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassifier;
use crate::runtime::source_ingress::WorthUiSourceWatcher;
use crate::runtime::state_inventory::WorthUiDurableStateInventoryBuilder;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiAmbiguousReplacementDenial,
    WorthUiDurableStateInventory, WorthUiDurableStateReconciliationDenial,
    WorthUiDurableStateReconciliationPlan, WorthUiIdentityMatchDenial, WorthUiIdentityMatchReport,
    WorthUiNodeReplacementPlan, WorthUiPendingExecutionPlanLoweringInput,
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
    pub fn replacement_admission_basis(&self) -> WorthUiActiveReplacementBasis {
        WorthUiActiveReplacementBasis::from_observation(self.inspect_active())
    }

    pub fn replacement_admission_transition(&self) -> WorthUiReplacementAdmissionBasis {
        WorthUiReplacementAdmissionBasis(self.replacement_admission_basis())
    }

    pub fn compare_admitted_replacement(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .compare_admitted(admitted)
    }

    pub fn compare_admitted_from_basis(
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

    pub fn classify_replacement_impact(
        &self,
        comparison: &WorthUiRuntimeArtifactComparison,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiReplacementImpactClassification, WorthUiReplacementImpactDenial> {
        WorthUiReplacementImpactClassifier::classify(comparison, admitted)
    }

    pub fn classify_replacement_impact_from_comparison(
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

    pub fn narrow_replacement_impact(
        &self,
        classification: &WorthUiReplacementImpactClassification,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeImpactNarrowing, WorthUiRuntimeImpactNarrowingDenial> {
        WorthUiRuntimeImpactNarrower::narrow(classification, admitted)
    }

    pub fn narrow_replacement_impact_from_classification(
        &self,
        ready: WorthUiReplacementImpactReady,
    ) -> Result<WorthUiReplacementNarrowingReady, WorthUiRuntimeImpactNarrowingDenial> {
        let WorthUiReplacementImpactReady {
            admitted,
            comparison: _,
            impact,
        } = ready;
        let narrowing = self.narrow_replacement_impact(&impact, &admitted)?;
        Ok(WorthUiReplacementNarrowingReady {
            admitted,
            impact,
            narrowing,
        })
    }

    pub fn build_identity_match_graph(
        &self,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiIdentityMatchReport, WorthUiIdentityMatchDenial> {
        WorthUiIdentityMatchGraphBuilder::build(self.active.active_artifact(), narrowing, admitted)
    }

    pub fn build_identity_match_graph_from_narrowing(
        &self,
        ready: WorthUiReplacementNarrowingReady,
    ) -> Result<WorthUiReplacementIdentityReady, WorthUiIdentityMatchDenial> {
        let WorthUiReplacementNarrowingReady {
            admitted,
            impact,
            narrowing,
        } = ready;
        let identity_report = self.build_identity_match_graph(&narrowing, &admitted)?;
        Ok(WorthUiReplacementIdentityReady {
            admitted,
            impact,
            narrowing,
            identity_report,
        })
    }

    pub fn classify_node_replacements(
        &self,
        impact: &WorthUiReplacementImpactClassification,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        identity_report: &WorthUiIdentityMatchReport,
    ) -> Result<WorthUiNodeReplacementPlan, WorthUiAmbiguousReplacementDenial> {
        WorthUiNodeReplacementClassifier::classify(impact, narrowing, identity_report)
    }

    pub fn classify_node_replacements_from_identity(
        &self,
        ready: WorthUiReplacementIdentityReady,
    ) -> Result<WorthUiReplacementNodePlanReady, WorthUiAmbiguousReplacementDenial> {
        let WorthUiReplacementIdentityReady {
            admitted,
            impact,
            narrowing,
            identity_report,
        } = ready;
        let node_plan = self.classify_node_replacements(&impact, &narrowing, &identity_report)?;
        Ok(WorthUiReplacementNodePlanReady {
            admitted,
            impact,
            narrowing,
            node_plan,
        })
    }

    pub fn durable_state_inventory(&self) -> WorthUiDurableStateInventoryBuilder {
        WorthUiDurableStateInventoryBuilder::new()
    }

    pub fn source_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> WorthUiSourceWatcher {
        WorthUiSourceWatcher::new(provider)
    }

    pub fn reconcile_durable_state(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationDenial>
    {
        WorthUiDurableStateReconciliationPlanner::reconcile(node_plan, inventory)
    }

    pub fn reconcile_durable_state_from_node_plan(
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
        } = ready;
        let reconciliation_plan = self.reconcile_durable_state(&node_plan, inventory)?;
        Ok(WorthUiReplacementReconciliationReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
        })
    }

    pub fn compare_query_bindings(
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

    pub fn compare_query_bindings_from_reconciliation(
        &self,
        ready: WorthUiReplacementReconciliationReady,
    ) -> Result<WorthUiReplacementQueryComparisonReady, WorthUiQueryBindingComparisonDenial> {
        let WorthUiReplacementReconciliationReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
        } = ready;
        let query_comparison = self.compare_query_bindings(&node_plan, &narrowing, &admitted)?;
        Ok(WorthUiReplacementQueryComparisonReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_comparison,
        })
    }

    pub fn plan_query_live_rebinds(
        &self,
        comparison: &WorthUiQueryBindingComparison,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial> {
        WorthUiQueryLiveRebindPlanner::plan(comparison, node_plan, narrowing, admitted)
    }

    pub fn prepare_pending_execution_plan_lowering_input(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        reconciliation_plan: &WorthUiDurableStateReconciliationPlan,
        query_rebind_plan: &WorthUiQueryLiveRebindPlan,
    ) -> WorthUiPendingExecutionPlanLoweringInput {
        WorthUiPendingExecutionPlanLoweringInput::from_staged_plans(
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
        )
    }

    pub fn prepare_replacement_lowering_from_query_comparison(
        &self,
        ready: WorthUiReplacementQueryComparisonReady,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiQueryLiveRebindPlanDenial> {
        let WorthUiReplacementQueryComparisonReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_comparison,
        } = ready;
        let query_rebind_plan =
            self.plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)?;
        let pending_execution_plan_lowering_input = self
            .prepare_pending_execution_plan_lowering_input(
                &node_plan,
                &reconciliation_plan,
                &query_rebind_plan,
            );
        Ok(WorthUiReplacementLoweringReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        })
    }

    pub fn prepare_replacement_lowering(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
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
        let reconciliation = self
            .reconcile_durable_state_from_node_plan(node_plan, inventory)
            .map_err(WorthUiReplacementLoweringDenial::Reconciliation)?;
        let query_comparison = self
            .compare_query_bindings_from_reconciliation(reconciliation)
            .map_err(WorthUiReplacementLoweringDenial::QueryComparison)?;
        self.prepare_replacement_lowering_from_query_comparison(query_comparison)
            .map_err(WorthUiReplacementLoweringDenial::QueryRebind)
    }
}

#[derive(Debug)]
pub enum WorthUiReplacementLoweringDenial {
    Comparison(WorthUiRuntimeArtifactComparisonDenial),
    Impact(WorthUiReplacementImpactDenial),
    Narrowing(WorthUiRuntimeImpactNarrowingDenial),
    Identity(WorthUiIdentityMatchDenial),
    NodePlan(WorthUiAmbiguousReplacementDenial),
    Reconciliation(WorthUiDurableStateReconciliationDenial),
    QueryComparison(WorthUiQueryBindingComparisonDenial),
    QueryRebind(WorthUiQueryLiveRebindPlanDenial),
}
