use worth_ui_inspection::{
    UiInspectionQuery, UiInspectionRelevanceOutcome, UiInspectionScope,
    UiInspectionSupportPosture, UiInspectionSupportReport, UiInspectionTarget,
};

use crate::admission::UiAdmissionBoundary;
use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationAuthoredEvidenceIndex, UiDeclarationCloseoutReport,
};
use crate::evidence::{
    UiEvidenceAuthorityGeneration, UiEvidenceExpansion, UiEvidenceMaterializedDetail,
    UiEvidenceRef, UiInspectionObligationEvidenceReceipt,
};
use crate::facade::{
    inspection::expand_evidence_ref as expand_inspection_evidence_ref,
    retained_obligation_registry::WorthUiRetainedObligationRegistry,
    runtime_bridge::{WorthUiCapabilityRegistrationFreezeCore, WorthUiFacadeLifecycleBootstrap},
    CapabilitySnapshot, UiInspectionClosureReport, UiInspectionFacadeObservation,
    UiInspectionReceipt, WorthUiRuntimeHost, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial,
    WorthUiRuntimeSupportInventory,
};
use crate::graph::{
    UiGraphAspectEvidenceIndexes, UiGraphAuthority, UiGraphCloseoutReport, UiGraphNodeEvidenceIndex,
    UiGraphSnapshot,
};
use crate::obligations::closeout::UiObligationCloseoutReport;

/// Runtime facade entrypoint for building Worth UI applications.
pub struct WorthUi {
    _sealed: (),
}

impl WorthUi {
    /// Start a Worth UI application definition.
    pub fn app() -> crate::facade::WorthUiBuilder {
        crate::facade::WorthUiBuilder::new()
    }
}

/// Worth UI application after capability registration has frozen.
pub struct WorthUiApp {
    capability_snapshot: CapabilitySnapshot,
    declaration_artifacts: Vec<UiDeclarationArtifact>,
    graph_snapshot: UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
    authored_evidence_index: UiDeclarationAuthoredEvidenceIndex,
    graph_node_evidence_index: UiGraphNodeEvidenceIndex,
    graph_aspect_evidence_indexes: UiGraphAspectEvidenceIndexes,
    retained_obligations: WorthUiRetainedObligationRegistry,
}

impl WorthUiApp {
    pub(crate) fn from_freeze_core(core: WorthUiCapabilityRegistrationFreezeCore) -> Self {
        let (capability_snapshot, declaration_artifacts, graph_snapshot, lifecycle) =
            core.into_parts();
        Self::from_authority_parts(
            capability_snapshot,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
        )
    }

    pub(crate) fn from_authority_parts(
        capability_snapshot: CapabilitySnapshot,
        declaration_artifacts: Vec<UiDeclarationArtifact>,
        graph_snapshot: UiGraphSnapshot,
        lifecycle: WorthUiFacadeLifecycleBootstrap,
    ) -> Self {
        let authored_evidence_index =
            UiDeclarationAuthoredEvidenceIndex::rebuild(&declaration_artifacts, &graph_snapshot);
        let graph_node_evidence_index = Self::build_graph_node_evidence_index(
            &declaration_artifacts,
            &graph_snapshot,
            &lifecycle,
        );
        let graph_aspect_evidence_indexes =
            Self::build_graph_aspect_evidence_indexes(
                &graph_snapshot,
                &graph_node_evidence_index,
                &lifecycle,
            );

        Self {
            capability_snapshot,
            declaration_artifacts,
            authored_evidence_index,
            graph_node_evidence_index,
            graph_aspect_evidence_indexes,
            graph_snapshot,
            lifecycle,
            retained_obligations: WorthUiRetainedObligationRegistry::default(),
        }
    }

    #[cfg(test)]
    pub(crate) fn into_authority_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        Vec<UiDeclarationArtifact>,
        UiGraphSnapshot,
        WorthUiFacadeLifecycleBootstrap,
    ) {
        (
            self.capability_snapshot,
            self.declaration_artifacts,
            self.graph_snapshot,
            self.lifecycle,
        )
    }

    /// Inspect the immutable capability snapshot owned by this app.
    pub fn capabilities(&self) -> &CapabilitySnapshot {
        &self.capability_snapshot
    }

    /// Inspect the canonical declaration artifacts admitted during app freeze.
    pub fn declaration_artifacts(&self) -> &[UiDeclarationArtifact] {
        &self.declaration_artifacts
    }

    /// Inspect the proof-bearing graph authority surface owned by this app.
    pub fn graph(&self) -> UiGraphAuthority<'_> {
        UiGraphAuthority::new(&self.graph_snapshot)
    }

    /// Enter the runtime-owned admission boundary through one formal facade lane.
    pub fn admission(&self) -> UiAdmissionBoundary<'_> {
        UiAdmissionBoundary::new(&self.declaration_artifacts, &self.graph_snapshot)
    }

    pub(crate) fn graph_snapshot(&self) -> &UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub(crate) fn authored_evidence_index(&self) -> &UiDeclarationAuthoredEvidenceIndex {
        &self.authored_evidence_index
    }

    pub(crate) fn graph_node_evidence_index(&self) -> &UiGraphNodeEvidenceIndex {
        &self.graph_node_evidence_index
    }

    pub(crate) fn graph_aspect_evidence_indexes(&self) -> &UiGraphAspectEvidenceIndexes {
        &self.graph_aspect_evidence_indexes
    }

    pub fn graph_closeout_report(&self) -> UiGraphCloseoutReport {
        UiGraphCloseoutReport::milestone33()
    }

    /// Inspect milestone-closeout metadata owned by the declaration boundary.
    pub fn declaration_closeout_report(&self) -> UiDeclarationCloseoutReport {
        UiDeclarationCloseoutReport::milestone32()
    }

    pub fn obligation_closeout_report(&self) -> UiObligationCloseoutReport {
        UiObligationCloseoutReport::milestone34()
    }

    /// Enter the runtime-owned inspection surface through one formal facade lane.
    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        self.lifecycle.record_inspection_query();
        let authority_generation = Some(UiEvidenceAuthorityGeneration::new(
            self.graph_snapshot.generation().as_u64(),
        ));
        match query.target() {
            UiInspectionTarget::ProductRoot | UiInspectionTarget::DeclaredSurface { .. } => {
                let support_report = self.inspection_support_report_for(&query);
                let relevance_admission = query
                    .admit_relevance()
                    .refined_for_support_report(support_report);
                if !matches!(
                    support_report.posture(),
                    UiInspectionSupportPosture::Supported
                ) {
                    self.lifecycle.record_unsupported_inspection_query();
                }
                if !matches!(
                    relevance_admission.outcome(),
                    UiInspectionRelevanceOutcome::Matched
                ) {
                    return UiInspectionReceipt::from_support(
                        query,
                        relevance_admission,
                        support_report,
                        authority_generation,
                    );
                }
                UiInspectionReceipt::from_support(
                    query,
                    relevance_admission,
                    support_report,
                    authority_generation,
                )
            }
            UiInspectionTarget::DeclarationIdentity { .. }
            | UiInspectionTarget::AuthoredSourceProvenance { .. } => {
                let relevance_admission = query.admit_relevance();
                if !matches!(
                    relevance_admission.outcome(),
                    UiInspectionRelevanceOutcome::Matched
                ) {
                    return UiInspectionReceipt::from_relevance_admission(
                        query,
                        relevance_admission,
                        authority_generation,
                    );
                }
                self.lifecycle.record_authored_lookup();
                let fallback_query = query.clone();
                self.authored_inspection_boundary()
                    .inspect(
                        query,
                        authority_generation
                            .expect("graph-backed inspection has one active generation"),
                    )
                    .unwrap_or_else(|| {
                        UiInspectionReceipt::from_relevance_admission(
                            fallback_query.clone(),
                            fallback_query.admit_relevance(),
                            authority_generation,
                        )
                    })
            }
            UiInspectionTarget::GraphNodeIdentity { .. } => {
                let relevance_admission = query.admit_relevance();
                if !matches!(
                    relevance_admission.outcome(),
                    UiInspectionRelevanceOutcome::Matched
                ) {
                    return UiInspectionReceipt::from_relevance_admission(
                        query,
                        relevance_admission,
                        authority_generation,
                    );
                }
                self.graph_inspection_boundary()
                    .inspect(
                        query.clone(),
                        authority_generation
                            .expect("graph-backed inspection has one active generation"),
                    )
                    .unwrap_or_else(|| {
                        UiInspectionReceipt::from_relevance_admission(
                            query.clone(),
                            query.admit_relevance(),
                            authority_generation,
                        )
                    })
            }
            UiInspectionTarget::PublishedAspect { .. } | UiInspectionTarget::ConsumedAspect { .. } => {
                let support_report = self.inspection_support_report_for(&query);
                let relevance_admission = query.admit_relevance();
                let refined_relevance = relevance_admission.refined_for_support_report(support_report);
                if !matches!(
                    support_report.posture(),
                    UiInspectionSupportPosture::Supported
                ) {
                    self.lifecycle.record_unsupported_inspection_query();
                }
                if !matches!(
                    refined_relevance.outcome(),
                    UiInspectionRelevanceOutcome::Matched
                ) {
                    return UiInspectionReceipt::from_support(
                        query,
                        refined_relevance,
                        support_report,
                        authority_generation,
                    );
                }
                self.aspect_inspection_boundary()
                    .inspect(
                        query.clone(),
                        authority_generation
                            .expect("graph-backed inspection has one active generation"),
                    )
                    .unwrap_or_else(|| {
                        UiInspectionReceipt::from_support(
                            query.clone(),
                            query.admit_relevance().refined_for_support_report(support_report),
                            support_report,
                            authority_generation,
                        )
                    })
            }
            UiInspectionTarget::ObligationGraphNode { .. }
            | UiInspectionTarget::ObligationTouch { .. }
            | UiInspectionTarget::ObligationEvidenceHandle { .. } => {
                let relevance_admission = query.admit_relevance();
                if !matches!(
                    relevance_admission.outcome(),
                    UiInspectionRelevanceOutcome::Matched
                ) {
                    return UiInspectionReceipt::from_relevance_admission(
                        query,
                        relevance_admission,
                        authority_generation,
                    );
                }
                if let Some(receipt) = self.inspect_retained_obligation_query(query.clone()) {
                    return receipt;
                }

                UiInspectionReceipt::from_obligation(
                    query,
                    relevance_admission,
                    authority_generation
                        .expect("graph-backed inspection has one active generation"),
                    UiInspectionObligationEvidenceReceipt::new(Box::new([]), Box::new([])),
                )
            }
            _ => UiInspectionReceipt::from_relevance_admission(
                query.clone(),
                query.admit_relevance(),
                authority_generation,
            ),
        }
    }

    pub fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        expand_inspection_evidence_ref(self, evidence_ref, requested_richness)
    }

    pub fn discard_evidence_slice(&self, slice_ref: crate::evidence::UiEvidenceSliceRef) -> bool {
        self.retained_obligations.discard_slice(slice_ref)
    }

    pub fn inspection_support_report(&self, scope: UiInspectionScope) -> UiInspectionSupportReport {
        self.lifecycle.inspection_support_report(scope)
    }

    pub fn inspection_closure_report(&self) -> UiInspectionClosureReport {
        self.lifecycle.inspection_closure_report()
    }

    pub fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        self.lifecycle.runtime_support_inventory()
    }

    pub fn inspection_observation(&self) -> UiInspectionFacadeObservation {
        self.lifecycle.inspection_observation()
    }

    #[cfg(test)]
    pub(crate) fn rebuild_authored_evidence_index_from_authority(&mut self) {
        self.authored_evidence_index = crate::declaration::UiDeclarationAuthoredEvidenceIndex::rebuild(
            &self.declaration_artifacts,
            &self.graph_snapshot,
        );
    }

    #[cfg(test)]
    pub(crate) fn rebuild_graph_node_evidence_index_from_authority(&mut self) {
        self.graph_node_evidence_index = Self::build_graph_node_evidence_index(
            &self.declaration_artifacts,
            &self.graph_snapshot,
            &self.lifecycle,
        );
        self.graph_aspect_evidence_indexes = Self::build_graph_aspect_evidence_indexes(
            &self.graph_snapshot,
            &self.graph_node_evidence_index,
            &self.lifecycle,
        );
    }

    /// Launch a runtime host from canonical artifact truth validated against this app snapshot.
    pub fn launch_runtime(
        &self,
        launch: WorthUiRuntimeLaunch,
    ) -> Result<WorthUiRuntimeHost, WorthUiRuntimeLaunchDenial> {
        WorthUiRuntimeHost::launch(launch, self.capability_snapshot.digest())
    }
    pub(crate) fn retained_obligation_registry(&self) -> &WorthUiRetainedObligationRegistry {
        &self.retained_obligations
    }

    pub(crate) fn expand_retained_obligation_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        if let Some(selected) = self
            .retained_obligations
            .retained_selection(evidence_ref.handle().handle_digest())
        {
            let expansion = selected.expand_evidence_ref(evidence_ref, requested_richness);
            if expansion.outcome().is_available() {
                self.lifecycle.record_rich_artifact_materialization();
            }
            return expansion;
        }

        self.lifecycle.record_rich_artifact_materialization();
        UiEvidenceExpansion::new(
            evidence_ref,
            requested_richness,
            worth_ui_inspection::UiEvidenceExpansionOutcome::Available,
            Some(UiEvidenceMaterializedDetail::Obligation(
                UiInspectionObligationEvidenceReceipt::new(Box::new([evidence_ref]), Box::new([])),
            )),
            Box::new([]),
            None,
        )
    }
}
