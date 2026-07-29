use super::WorthUiApplicationSessionState;
use crate::runtime::observation::{
    lower_authored_differences, UiAuthoredSourceClassification, UiAuthoredSourceSuccession,
    UiChangeClassificationDenial, UiChangeClassificationOutcome, UiChangeClassificationRequest,
    UiChangeClassifier,
};
use crate::runtime::{WorthUiCandidateAdmission, WorthUiRuntimeArtifactComparisonOutcome};

impl WorthUiApplicationSessionState {
    pub(crate) fn classify_observations(
        &self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        set: crate::runtime::observation::UiAdmittedObservationSet,
    ) -> Result<UiChangeClassificationOutcome, UiChangeClassificationDenial> {
        let source_basis = self.app.capabilities().digest().as_u64();
        let budget = self
            .app
            .prepared_authority()
            .change_profile()
            .rebind()
            .budget();
        UiChangeClassifier::classify(UiChangeClassificationRequest {
            set,
            expected_session: session,
            expected_source_basis: source_basis,
            predecessor_generation: self.app.generation_identity().clone(),
            fact_limit: budget.changed_facts,
            classify_source: |submission| {
                self.classify_authored_source(
                    submission,
                    budget.changed_facts,
                    budget.comparison_structural_entries,
                )
            },
        })
    }

    pub(crate) fn resolve_affected_scope(
        &self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        change: crate::runtime::observation::UiClassifiedChange,
    ) -> Result<
        crate::runtime::rebind::UiResolvedAffectedScope,
        crate::runtime::rebind::UiAffectedScopeDenial,
    > {
        crate::runtime::rebind::UiAffectedScopeResolver::resolve(
            change,
            session,
            self.app.prepared_authority(),
        )
    }

    fn classify_authored_source(
        &self,
        submission: crate::runtime::WorthUiWatchedCandidateSubmission,
        fact_limit: usize,
        structural_entry_limit: usize,
    ) -> Result<UiAuthoredSourceClassification, UiChangeClassificationDenial> {
        let (successor_authority, candidate) =
            crate::facade::lifecycle::prepare_successor_application_authority(
                self.app.prepared_authority(),
                submission,
            )
            .map_err(|denial| UiChangeClassificationDenial::SourcePreparation(Box::new(denial)))?;
        let evidence_changed = self.app.prepared_authority().authored_source_basis()
            != successor_authority.authored_source_basis();
        let admitted_candidate =
            WorthUiCandidateAdmission::for_active_basis(self.runtime.replacement_admission_basis())
                .admit(candidate)
                .map_err(|report| {
                    UiChangeClassificationDenial::CandidateAdmission(Box::new(report))
                })?;
        let comparison = self
            .runtime
            .compare_admitted_replacement_bounded(&admitted_candidate, structural_entry_limit)
            .map_err(UiChangeClassificationDenial::ArtifactComparison)?;

        match comparison.outcome() {
            WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp if !evidence_changed => {
                Ok(UiAuthoredSourceClassification::ObservedNoChange)
            }
            WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp => {
                Ok(UiAuthoredSourceClassification::EvidenceOnly(
                    UiAuthoredSourceSuccession::EvidenceOnly {
                        successor_authority,
                        admitted_candidate,
                        comparison,
                    },
                ))
            }
            WorthUiRuntimeArtifactComparisonOutcome::MeaningfullyDifferent => {
                let facts = lower_authored_differences(
                    &comparison,
                    self.app.prepared_authority(),
                    &successor_authority,
                    fact_limit,
                )?;
                let candidate_query_binding = successor_authority
                    .query_binding_plan()
                    .prepare_downstream_state();
                let replacement = self
                    .runtime
                    .prepare_replacement_node_plan_from_comparison(
                        admitted_candidate,
                        comparison.clone(),
                        successor_authority.query_binding_plan(),
                        &candidate_query_binding,
                    )
                    .map_err(|denial| {
                        UiChangeClassificationDenial::ReplacementPlanning(Box::new(denial))
                    })?;
                let identity_lifecycle_index =
                    crate::runtime::rebind::UiSourceIdentityLifecycleIndex::build(
                        self.app.prepared_authority(),
                        &successor_authority,
                        &replacement.node_plan,
                    )
                    .map_err(|denial| {
                        UiChangeClassificationDenial::IdentityLifecycle(Box::new(denial))
                    })?;
                Ok(UiAuthoredSourceClassification::Changed {
                    facts,
                    succession: UiAuthoredSourceSuccession::Changed {
                        successor_authority,
                        comparison,
                        replacement,
                        identity_lifecycle_index,
                    },
                })
            }
        }
    }
}
