use super::{
    WorthUiCandidateInspectionReceipt, WorthUiLoweredApplicationReplacement,
    WorthUiPreparedApplicationReplacement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReplacementCandidateSummary {
    active_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    candidate_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    affected_handle_count: usize,
    affected_source_module_count: usize,
    replacement_classification_count: usize,
    reconciliation_receipt_count: usize,
    query_rebind_entry_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReplacementPlannedCostEnvelope {
    affected_handle_count: usize,
    admission_checks: usize,
    artifact_comparisons: usize,
    impact_metadata_reads: usize,
    identity_node_visits: usize,
    reconciliation_node_visits: usize,
    query_bindings_planned: usize,
}

impl WorthUiPreparedApplicationReplacement {
    pub fn candidate_graph(&self) -> crate::graph::UiGraphAuthority<'_> {
        self.next_app.graph()
    }

    pub fn candidate_declaration_artifacts(&self) -> &[crate::declaration::UiDeclarationArtifact] {
        self.next_app.declaration_artifacts()
    }

    pub fn admit_candidate_query_projection(
        &mut self,
        outcome: worth_ui_query_binding::WorthUiQuerySnapshotProjectionOutcome,
    ) -> Result<(), worth_ui_query_binding::WorthUiQueryMeasurementFactSettlementDenial> {
        self.candidate_query_binding.admit(outcome).map(drop)
    }

    pub fn admit_candidate_live_query_projection(
        &mut self,
        resource: worth_ui_query_binding::WorthUiQueryLiveResource,
        outcome: worth_ui_query_binding::WorthUiQueryLiveProjectionOutcome,
    ) -> Result<(), worth_ui_query_binding::WorthUiQueryLiveAdmissionStop> {
        self.candidate_query_binding
            .admit_live(resource, outcome)
            .map(drop)
    }

    pub fn commit_candidate_mounted_layout_admissions(
        &mut self,
        transitions: Vec<crate::graph::UiGraphMountedReceiptTransition>,
    ) -> Result<(), crate::graph::UiGraphMountedLayoutAdmissionDenial> {
        self.candidate_graph_changed_nodes.extend(
            transitions
                .iter()
                .map(|transition| transition.authority_record().graph_node_identity()),
        );
        let committed = self
            .candidate_graph()
            .commit_mounted_layout_admissions(transitions)?;
        self.next_app.advance_prepared_graph(committed);
        self.basis.rebind_graph(&self.next_app);
        Ok(())
    }

    pub fn candidate_admission(&self) -> crate::admission::UiAdmissionBoundary<'_> {
        self.next_app.admission()
    }

    pub fn try_candidate_query_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        self.next_app.try_query_touch_for_node(graph_node_identity)
    }

    pub fn admit_candidate_allocation_catalog(
        &self,
        entries: Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
    ) -> Result<
        crate::graph::UiAdmittedAllocationCatalogBasisSet,
        crate::graph::UiAllocationCatalogBasisAdmissionDenial,
    > {
        self.next_app
            .graph_snapshot()
            .admit_allocation_catalog_basis_set(entries)
    }

    pub fn admit_candidate_allocation_neighborhood(
        &self,
        basis: &crate::evidence::UiMeasurementBasis,
        selected: &crate::obligations::selection::UiSelectedObligationSet,
    ) -> Result<
        crate::evidence::UiAllocationNeighborhood,
        crate::graph::UiAllocationNeighborhoodDenial,
    > {
        basis.admit_allocation_neighborhood(self.next_app.graph_snapshot(), selected)
    }

    pub fn admit_candidate_allocation_catalog_delta(
        &self,
        changed: Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
        removed_roots: Vec<crate::graph::UiGraphNodeIdentity>,
    ) -> Result<
        crate::graph::UiAdmittedAllocationCatalogDelta,
        crate::graph::UiAllocationCatalogDeltaAdmissionDenial,
    > {
        self.next_app
            .graph_snapshot()
            .admit_allocation_catalog_delta(changed, removed_roots)
    }

    pub fn inspect_candidate(
        &self,
        query: worth_ui_inspection::UiInspectionQuery,
    ) -> WorthUiCandidateInspectionReceipt {
        WorthUiCandidateInspectionReceipt {
            generation_identity: self.next_app.generation_identity().clone(),
            candidate_basis: self.admitted.candidate().basis(),
            receipt: self.next_app.inspect(query),
        }
    }

    pub fn expand_candidate_evidence_ref(
        &self,
        evidence_ref: crate::evidence::UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> crate::evidence::UiEvidenceExpansion {
        self.next_app
            .expand_evidence_ref(evidence_ref, requested_richness)
    }
}

impl WorthUiLoweredApplicationReplacement {
    /// Compact observation of a candidate after expensive replacement
    /// lowering and before the activation transaction.
    pub fn summary(&self) -> WorthUiReplacementCandidateSummary {
        WorthUiReplacementCandidateSummary {
            active_generation: self.active_generation.clone(),
            candidate_generation: self.next_app.generation_identity().clone(),
            affected_handle_count: self.lowering.narrowing().affected_handle_count(),
            affected_source_module_count: self.lowering.narrowing().affected_source_modules().len(),
            replacement_classification_count: self.lowering.node_plan().classifications().len(),
            reconciliation_receipt_count: self.lowering.reconciliation_plan().receipts().len(),
            query_rebind_entry_count: self.lowering.query_rebind_plan().entries().len(),
        }
    }

    /// Work already performed by replacement lowering. Candidate plan
    /// construction and equivalence are intentionally absent until activation
    /// prepares the complete bundle.
    pub fn cost_envelope(&self) -> WorthUiReplacementPlannedCostEnvelope {
        let admission = self.lowering.admitted().report().counters();
        let impact = self.lowering.narrowing().counters();
        let identity = self.lowering.identity_match_counters;
        let reconciliation = self.lowering.reconciliation_plan().counters();
        WorthUiReplacementPlannedCostEnvelope {
            affected_handle_count: self.lowering.narrowing().affected_handle_count(),
            admission_checks: admission.candidate_proof_checks()
                + admission.snapshot_compatibility_checks()
                + admission.runtime_posture_checks()
                + admission.query_support_checks(),
            artifact_comparisons: self
                .lowering
                .artifact_comparison_counters
                .artifact_comparisons(),
            impact_metadata_reads: impact.dependency_metadata_reads()
                + impact.module_impact_lookups()
                + impact.subtree_impact_lookups()
                + impact.runtime_hook_lookups(),
            identity_node_visits: identity.active_nodes_indexed()
                + identity.candidate_nodes_indexed()
                + identity.stable_seed_lookups(),
            reconciliation_node_visits: reconciliation.reconciled_node_count(),
            query_bindings_planned: self
                .lowering
                .query_rebind_plan()
                .counters()
                .bindings_planned(),
        }
    }
}

impl WorthUiReplacementCandidateSummary {
    pub fn active_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.active_generation
    }
    pub fn candidate_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.candidate_generation
    }
    pub fn affected_handle_count(&self) -> usize {
        self.affected_handle_count
    }
    pub fn affected_source_module_count(&self) -> usize {
        self.affected_source_module_count
    }
    pub fn replacement_classification_count(&self) -> usize {
        self.replacement_classification_count
    }
    pub fn reconciliation_receipt_count(&self) -> usize {
        self.reconciliation_receipt_count
    }
    pub fn query_rebind_entry_count(&self) -> usize {
        self.query_rebind_entry_count
    }
}

impl WorthUiReplacementPlannedCostEnvelope {
    pub fn affected_handle_count(self) -> usize {
        self.affected_handle_count
    }
    pub fn admission_checks(self) -> usize {
        self.admission_checks
    }
    pub fn artifact_comparisons(self) -> usize {
        self.artifact_comparisons
    }
    pub fn impact_metadata_reads(self) -> usize {
        self.impact_metadata_reads
    }
    pub fn identity_node_visits(self) -> usize {
        self.identity_node_visits
    }
    pub fn reconciliation_node_visits(self) -> usize {
        self.reconciliation_node_visits
    }
    pub fn query_bindings_planned(self) -> usize {
        self.query_bindings_planned
    }
}
