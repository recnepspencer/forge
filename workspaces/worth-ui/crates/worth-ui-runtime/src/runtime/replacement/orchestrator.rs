use crate::runtime::replacement::admission::WorthUiActiveReplacementBasis;
use crate::runtime::replacement::equivalence::WorthUiRuntimeArtifactComparator;
use crate::runtime::replacement::impact::WorthUiReplacementImpactClassifier;
use crate::runtime::replacement::matching::WorthUiIdentityMatchGraphBuilder;
use crate::runtime::replacement::narrowing::WorthUiRuntimeImpactNarrower;
use crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassifier;
use crate::runtime::replacement::reconciliation::WorthUiDurableStateReconciliationPlanner;
use crate::runtime::source_ingress::WorthUiSourceEventIngress;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiAmbiguousReplacementDenial,
    WorthUiDurableStateInventory, WorthUiDurableStateInventoryDenial,
    WorthUiDurableStateReconciliationDenial, WorthUiDurableStateReconciliationPlan,
    WorthUiIdentityMatchDenial, WorthUiIdentityMatchReport, WorthUiNodeReplacementPlan,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryLiveRebindPlanDenial,
    WorthUiReplacementImpactClassification, WorthUiReplacementImpactDenial,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeImpactNarrowing, WorthUiRuntimeImpactNarrowingDenial,
};

use super::transitions::{
    WorthUiReplacementComparisonReady, WorthUiReplacementIdentityReady,
    WorthUiReplacementImpactReady, WorthUiReplacementLoweringReady,
    WorthUiReplacementNarrowingReady, WorthUiReplacementNodePlanReady,
    WorthUiReplacementQueryComparisonReady, WorthUiReplacementQueryImpactReady,
    WorthUiReplacementReconciliationReady,
};
use crate::runtime::launch::runtime_instance::WorthUiRuntime;

impl WorthUiRuntime {
    pub(crate) fn replacement_admission_basis(&self) -> WorthUiActiveReplacementBasis {
        WorthUiActiveReplacementBasis::from_observation(self.inspect_active())
    }

    pub(crate) fn compare_admitted_replacement(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .compare_admitted(admitted)
    }

    pub(crate) fn compare_admitted_replacement_bounded(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
        structural_entry_limit: usize,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .compare_admitted_bounded(admitted, structural_entry_limit)
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
        ready: WorthUiReplacementQueryImpactReady,
    ) -> Result<WorthUiReplacementIdentityReady, WorthUiIdentityMatchDenial> {
        let WorthUiReplacementQueryImpactReady {
            admitted,
            impact,
            narrowing,
            query_comparison,
            artifact_comparison_counters,
        } = ready;
        let identity_report = self.build_identity_match_graph(&narrowing, &admitted)?;
        Ok(WorthUiReplacementIdentityReady {
            admitted,
            impact,
            narrowing,
            identity_report,
            query_comparison,
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
            query_comparison,
            artifact_comparison_counters,
        } = ready;
        let identity_match_counters = identity_report.counters();
        let node_plan = self.classify_node_replacements(&impact, &narrowing, &identity_report)?;
        Ok(WorthUiReplacementNodePlanReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        })
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
            query_comparison,
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
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        })
    }

    pub(crate) fn prepare_application_replacement_lowering(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
        candidate_query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
        if !candidate_application_authority.admits_candidate(&admitted) {
            return Err(WorthUiReplacementLoweringDenial::CandidateApplicationAuthorityMismatch);
        }
        let node_plan = self.prepare_replacement_node_plan(
            admitted,
            candidate_application_authority.query_binding_plan(),
            candidate_query_binding,
        )?;
        let inventory = WorthUiDurableStateInventory::assemble_for_replacement(
            &node_plan.node_plan,
            candidate_application_authority.mosaic_state_capabilities(),
        )
        .map_err(WorthUiReplacementLoweringDenial::Inventory)?;
        self.finish_replacement_lowering(node_plan, &inventory, candidate_application_authority)
    }

    pub(crate) fn finish_precomputed_replacement_lowering(
        &self,
        node_plan: WorthUiReplacementNodePlanReady,
        candidate: &crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationAuthority,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
        let candidate_application_authority = candidate.lowering_authority();
        if !candidate_application_authority.admits_candidate(&node_plan.admitted) {
            return Err(WorthUiReplacementLoweringDenial::CandidateApplicationAuthorityMismatch);
        }
        let inventory = WorthUiDurableStateInventory::assemble_for_replacement(
            &node_plan.node_plan,
            candidate_application_authority.mosaic_state_capabilities(),
        )
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
        let candidate_query_binding = candidate_application_authority
            .query_binding_plan()
            .prepare_downstream_state();
        let node_plan = self.prepare_replacement_node_plan(
            admitted,
            candidate_application_authority.query_binding_plan(),
            &candidate_query_binding,
        )?;
        self.finish_replacement_lowering(node_plan, inventory, candidate_application_authority)
    }

    fn prepare_replacement_node_plan(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        candidate_query_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        candidate_query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<WorthUiReplacementNodePlanReady, WorthUiReplacementLoweringDenial> {
        let comparison = self
            .compare_admitted_replacement(&admitted)
            .map_err(WorthUiReplacementLoweringDenial::Comparison)?;
        self.prepare_replacement_node_plan_from_comparison(
            admitted,
            comparison,
            candidate_query_plan,
            candidate_query_binding,
        )
    }

    pub(crate) fn prepare_replacement_node_plan_from_comparison(
        &self,
        admitted: WorthUiAdmittedReplacementCandidate,
        comparison: WorthUiRuntimeArtifactComparison,
        candidate_query_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        candidate_query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<WorthUiReplacementNodePlanReady, WorthUiReplacementLoweringDenial> {
        let comparison = WorthUiReplacementComparisonReady {
            admitted,
            comparison,
        };
        let impact = self
            .classify_replacement_impact_from_comparison(comparison)
            .map_err(WorthUiReplacementLoweringDenial::Impact)?;
        let narrowing = self
            .narrow_replacement_impact_from_classification(impact)
            .map_err(WorthUiReplacementLoweringDenial::Narrowing)?;
        let query_impact = self
            .resolve_query_impact(narrowing, candidate_query_plan, candidate_query_binding)
            .map_err(WorthUiReplacementLoweringDenial::QueryComparison)?;
        let identity = self
            .build_identity_match_graph_from_narrowing(query_impact)
            .map_err(WorthUiReplacementLoweringDenial::Identity)?;
        let node_plan = self
            .classify_node_replacements_from_identity(identity)
            .map_err(WorthUiReplacementLoweringDenial::NodePlan)?;
        Ok(node_plan)
    }

    fn resolve_query_impact(
        &self,
        ready: WorthUiReplacementNarrowingReady,
        candidate_query_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        candidate_query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Result<WorthUiReplacementQueryImpactReady, WorthUiQueryBindingComparisonDenial> {
        let WorthUiReplacementNarrowingReady {
            admitted,
            impact,
            mut narrowing,
            artifact_comparison_counters,
        } = ready;
        let query_comparison = self.compare_query_bindings_for_narrowing_with_candidate_authority(
            &narrowing,
            &admitted,
            candidate_query_plan,
            candidate_query_binding,
        )?;
        narrowing.replace_with_exact_query_invalidations(&query_comparison);
        Ok(WorthUiReplacementQueryImpactReady {
            admitted,
            impact,
            narrowing,
            query_comparison,
            artifact_comparison_counters,
        })
    }

    fn finish_replacement_lowering(
        &self,
        node_plan: WorthUiReplacementNodePlanReady,
        inventory: &WorthUiDurableStateInventory,
        candidate_application_authority: crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    ) -> Result<WorthUiReplacementLoweringReady, WorthUiReplacementLoweringDenial> {
        let inventory_counters = inventory.counters();
        let reconciliation = self
            .reconcile_durable_state_from_node_plan(node_plan, inventory)
            .map_err(WorthUiReplacementLoweringDenial::Reconciliation)?;
        let WorthUiReplacementReconciliationReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        } = reconciliation;
        let query_comparison = WorthUiReplacementQueryComparisonReady {
            admitted,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            inventory_counters,
            query_comparison,
            artifact_comparison_counters,
            identity_match_counters,
        };
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
