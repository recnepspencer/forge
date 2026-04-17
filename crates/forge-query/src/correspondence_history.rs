use crate::correspondence::{
    CorrespondenceAmbiguityEnvelope, CorrespondenceCostPosture, CorrespondenceCounterSnapshot,
    CorrespondenceDenied, CorrespondenceDisagreementEnvelope, CorrespondenceEvidenceResolved,
};
use crate::execution::{ExecutionCounters, ExecutionResultEnvelope};
use crate::historical::{
    HistoricalCounterSnapshot, HistoricalEvaluationAdmission, HistoricalEvaluationError,
    HistoricalEvaluationRequest, HistoricalMaterializationPathMetadata, HistoricalPathCompatibilityOutcome,
    HistoricalPathCostPosture, HistoricalPathResolved,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataPreservingHistoricalResultView<'a> {
    payload: &'a [String],
    execution_counters: &'a ExecutionCounters,
    correspondence_family_name: &'static str,
    correspondence_cost_posture: &'a CorrespondenceCostPosture,
    correspondence_counters: &'a CorrespondenceCounterSnapshot,
    materialization_metadata: &'a HistoricalMaterializationPathMetadata,
    historical_cost_posture: &'a HistoricalPathCostPosture,
    historical_counters: &'a HistoricalCounterSnapshot,
}

impl<'a> MetadataPreservingHistoricalResultView<'a> {
    pub fn payload(&self) -> &[String] {
        self.payload
    }

    pub fn execution_counters(&self) -> &ExecutionCounters {
        self.execution_counters
    }

    pub fn correspondence_family_name(&self) -> &'static str {
        self.correspondence_family_name
    }

    pub fn correspondence_cost_posture(&self) -> &CorrespondenceCostPosture {
        self.correspondence_cost_posture
    }

    pub fn correspondence_counters(&self) -> &CorrespondenceCounterSnapshot {
        self.correspondence_counters
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        self.materialization_metadata
    }

    pub fn historical_cost_posture(&self) -> &HistoricalPathCostPosture {
        self.historical_cost_posture
    }

    pub fn historical_counters(&self) -> &HistoricalCounterSnapshot {
        self.historical_counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalSuccessEnvelope {
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    historical: HistoricalPathResolved,
    materialization_metadata: HistoricalMaterializationPathMetadata,
}

impl CorrespondenceHistoricalSuccessEnvelope {
    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn historical(&self) -> &HistoricalPathResolved {
        &self.historical
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        &self.materialization_metadata
    }

    pub fn result_view(&self) -> MetadataPreservingHistoricalResultView<'_> {
        MetadataPreservingHistoricalResultView {
            payload: self.execution.payload(),
            execution_counters: self.execution.counters(),
            correspondence_family_name: self.correspondence.outcome().family_name(),
            correspondence_cost_posture: self.correspondence.cost_posture(),
            correspondence_counters: self.correspondence.counters(),
            materialization_metadata: &self.materialization_metadata,
            historical_cost_posture: self.historical.cost_posture(),
            historical_counters: self.historical.counters(),
        }
    }

    fn new(
        execution: ExecutionResultEnvelope,
        correspondence: CorrespondenceEvidenceResolved,
        historical: HistoricalPathResolved,
    ) -> Self {
        let materialization_metadata =
            HistoricalMaterializationPathMetadata::from_resolved(historical.clone());
        Self {
            execution,
            correspondence,
            historical,
            materialization_metadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalAmbiguityEnvelope {
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    ambiguity: CorrespondenceAmbiguityEnvelope,
    historical: HistoricalPathResolved,
    materialization_metadata: HistoricalMaterializationPathMetadata,
}

impl CorrespondenceHistoricalAmbiguityEnvelope {
    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn ambiguity(&self) -> &CorrespondenceAmbiguityEnvelope {
        &self.ambiguity
    }

    pub fn historical(&self) -> &HistoricalPathResolved {
        &self.historical
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        &self.materialization_metadata
    }

    pub fn result_view(&self) -> MetadataPreservingHistoricalResultView<'_> {
        MetadataPreservingHistoricalResultView {
            payload: self.execution.payload(),
            execution_counters: self.execution.counters(),
            correspondence_family_name: self.correspondence.outcome().family_name(),
            correspondence_cost_posture: self.correspondence.cost_posture(),
            correspondence_counters: self.correspondence.counters(),
            materialization_metadata: &self.materialization_metadata,
            historical_cost_posture: self.historical.cost_posture(),
            historical_counters: self.historical.counters(),
        }
    }

    fn new(
        execution: ExecutionResultEnvelope,
        correspondence: CorrespondenceEvidenceResolved,
        ambiguity: CorrespondenceAmbiguityEnvelope,
        historical: HistoricalPathResolved,
    ) -> Self {
        let materialization_metadata =
            HistoricalMaterializationPathMetadata::from_resolved(historical.clone());
        Self {
            execution,
            correspondence,
            ambiguity,
            historical,
            materialization_metadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalDisagreementEnvelope {
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    disagreement: CorrespondenceDisagreementEnvelope,
    historical: HistoricalPathResolved,
    materialization_metadata: HistoricalMaterializationPathMetadata,
}

impl CorrespondenceHistoricalDisagreementEnvelope {
    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn disagreement(&self) -> &CorrespondenceDisagreementEnvelope {
        &self.disagreement
    }

    pub fn historical(&self) -> &HistoricalPathResolved {
        &self.historical
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        &self.materialization_metadata
    }

    pub fn result_view(&self) -> MetadataPreservingHistoricalResultView<'_> {
        MetadataPreservingHistoricalResultView {
            payload: self.execution.payload(),
            execution_counters: self.execution.counters(),
            correspondence_family_name: self.correspondence.outcome().family_name(),
            correspondence_cost_posture: self.correspondence.cost_posture(),
            correspondence_counters: self.correspondence.counters(),
            materialization_metadata: &self.materialization_metadata,
            historical_cost_posture: self.historical.cost_posture(),
            historical_counters: self.historical.counters(),
        }
    }

    fn new(
        execution: ExecutionResultEnvelope,
        correspondence: CorrespondenceEvidenceResolved,
        disagreement: CorrespondenceDisagreementEnvelope,
        historical: HistoricalPathResolved,
    ) -> Self {
        let materialization_metadata =
            HistoricalMaterializationPathMetadata::from_resolved(historical.clone());
        Self {
            execution,
            correspondence,
            disagreement,
            historical,
            materialization_metadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalDeniedEnvelope {
    correspondence: CorrespondenceEvidenceResolved,
    denied: CorrespondenceDenied,
}

impl CorrespondenceHistoricalDeniedEnvelope {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn denied(&self) -> &CorrespondenceDenied {
        &self.denied
    }

    fn new(correspondence: CorrespondenceEvidenceResolved, denied: CorrespondenceDenied) -> Self {
        Self {
            correspondence,
            denied,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathDeniedEnvelope {
    correspondence: CorrespondenceEvidenceResolved,
    admission: HistoricalEvaluationAdmission,
    error: HistoricalEvaluationError,
    denial_cost_posture: HistoricalPathCostPosture,
    counters: HistoricalCounterSnapshot,
    compatibility_outcome: HistoricalPathCompatibilityOutcome,
}

impl HistoricalPathDeniedEnvelope {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn admission(&self) -> &HistoricalEvaluationAdmission {
        &self.admission
    }

    pub fn error(&self) -> &HistoricalEvaluationError {
        &self.error
    }

    pub fn denial_cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.denial_cost_posture
    }

    pub fn counters(&self) -> &HistoricalCounterSnapshot {
        &self.counters
    }

    pub fn compatibility_outcome(&self) -> &HistoricalPathCompatibilityOutcome {
        &self.compatibility_outcome
    }

    fn new(
        correspondence: CorrespondenceEvidenceResolved,
        admission: HistoricalEvaluationAdmission,
        error: HistoricalEvaluationError,
    ) -> Self {
        let denial_cost_posture = error.denial_cost_posture();
        let compatibility_outcome = match error.failure_class() {
            crate::historical::HistoricalEvaluationFailureClass::HiddenPathSubstitutionDenied => {
                HistoricalPathCompatibilityOutcome::SubstitutionDenied
            }
            _ => HistoricalPathCompatibilityOutcome::Denied,
        };
        let counters = match compatibility_outcome {
            HistoricalPathCompatibilityOutcome::SubstitutionDenied => {
                admission.counters().clone().with_hidden_path_substitution_denial()
            }
            HistoricalPathCompatibilityOutcome::Denied => {
                admission.counters().clone().with_path_denial()
            }
            HistoricalPathCompatibilityOutcome::Admitted => admission.counters().clone(),
        };
        Self {
            correspondence,
            admission,
            error,
            denial_cost_posture,
            counters,
            compatibility_outcome,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathAdmissionDeniedEnvelope {
    correspondence: CorrespondenceEvidenceResolved,
    request: HistoricalEvaluationRequest,
    error: HistoricalEvaluationError,
    denial_cost_posture: HistoricalPathCostPosture,
    counters: HistoricalCounterSnapshot,
}

impl HistoricalPathAdmissionDeniedEnvelope {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn request(&self) -> &HistoricalEvaluationRequest {
        &self.request
    }

    pub fn error(&self) -> &HistoricalEvaluationError {
        &self.error
    }

    pub fn denial_cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.denial_cost_posture
    }

    pub fn counters(&self) -> &HistoricalCounterSnapshot {
        &self.counters
    }

    pub fn compatibility_outcome(&self) -> HistoricalPathCompatibilityOutcome {
        HistoricalPathCompatibilityOutcome::Denied
    }

    fn new(
        correspondence: CorrespondenceEvidenceResolved,
        request: HistoricalEvaluationRequest,
        error: HistoricalEvaluationError,
    ) -> Self {
        let denial_cost_posture = error.denial_cost_posture();
        let counters = HistoricalCounterSnapshot::denied(
            request.replay_budget().max_replay_events(),
            request.reconstruction_budget().max_reconstruction_scope(),
        );
        Self {
            correspondence,
            request,
            error,
            denial_cost_posture,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoricalEnvelope {
    Success(CorrespondenceHistoricalSuccessEnvelope),
    Ambiguity(CorrespondenceHistoricalAmbiguityEnvelope),
    Disagreement(CorrespondenceHistoricalDisagreementEnvelope),
    CorrespondenceDenied(CorrespondenceHistoricalDeniedEnvelope),
    HistoricalPathDenied(HistoricalPathDeniedEnvelope),
    HistoricalPathAdmissionDenied(HistoricalPathAdmissionDeniedEnvelope),
}

impl CorrespondenceHistoricalEnvelope {
    pub fn result_view(&self) -> Option<MetadataPreservingHistoricalResultView<'_>> {
        match self {
            Self::Success(envelope) => Some(envelope.result_view()),
            Self::Ambiguity(envelope) => Some(envelope.result_view()),
            Self::Disagreement(envelope) => Some(envelope.result_view()),
            Self::CorrespondenceDenied(_)
            | Self::HistoricalPathDenied(_)
            | Self::HistoricalPathAdmissionDenied(_) => None,
        }
    }
}

pub fn compose_correspondence_historical_envelope(
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    historical: HistoricalPathResolved,
) -> CorrespondenceHistoricalEnvelope {
    let ambiguous = correspondence
        .outcome()
        .as_advisory_structural_ambiguous()
        .cloned();
    let disagreement = correspondence
        .outcome()
        .as_lineage_structural_disagreement()
        .cloned();
    let denied = correspondence.outcome().as_denied().cloned();

    if let Some(ambiguous) = ambiguous {
        return CorrespondenceHistoricalEnvelope::Ambiguity(
            CorrespondenceHistoricalAmbiguityEnvelope::new(
                execution,
                correspondence,
                CorrespondenceAmbiguityEnvelope::new(
                    ambiguous,
                    "structural correspondence remained advisory and ambiguous",
                ),
                historical,
            ),
        );
    }

    if let Some(disagreement) = disagreement {
        return CorrespondenceHistoricalEnvelope::Disagreement(
            CorrespondenceHistoricalDisagreementEnvelope::new(
                execution,
                correspondence,
                CorrespondenceDisagreementEnvelope::new(
                    disagreement,
                    "lineage and structural correspondence disagree",
                ),
                historical,
            ),
        );
    }

    if let Some(denied) = denied {
        return CorrespondenceHistoricalEnvelope::CorrespondenceDenied(
            CorrespondenceHistoricalDeniedEnvelope::new(correspondence, denied),
        );
    }

    CorrespondenceHistoricalEnvelope::Success(CorrespondenceHistoricalSuccessEnvelope::new(
        execution,
        correspondence,
        historical,
    ))
}

pub fn compose_historical_path_denied_envelope(
    correspondence: CorrespondenceEvidenceResolved,
    admission: HistoricalEvaluationAdmission,
    error: HistoricalEvaluationError,
) -> CorrespondenceHistoricalEnvelope {
    CorrespondenceHistoricalEnvelope::HistoricalPathDenied(HistoricalPathDeniedEnvelope::new(
        correspondence,
        admission,
        error,
    ))
}

pub fn compose_historical_admission_denied_envelope(
    correspondence: CorrespondenceEvidenceResolved,
    request: HistoricalEvaluationRequest,
    error: HistoricalEvaluationError,
) -> CorrespondenceHistoricalEnvelope {
    CorrespondenceHistoricalEnvelope::HistoricalPathAdmissionDenied(
        HistoricalPathAdmissionDeniedEnvelope::new(correspondence, request, error),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        compose_correspondence_historical_envelope, compose_historical_admission_denied_envelope,
        CorrespondenceHistoricalEnvelope,
    };
    use crate::correspondence::{
        resolve_correspondence_evidence, CorrespondenceEvaluationRequest,
        StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract,
    };
    use crate::execution::execute_preflight_bundle;
    use crate::historical::{
        admit_historical_evaluation_path, resolve_historical_materialization_path,
        HistoricalCapabilityDescriptor, HistoricalEvaluationRequest,
        HistoricalMaterializationDescriptor, HistoricalPathReuseDescriptor,
        ResolvedHistoricalPathClass,
    };

    #[test]
    fn success_envelope_preserves_payload_and_metadata_together() {
        let execution =
            execute_preflight_bundle(&detail_preflight_bundle()).expect("execution should succeed");
        let correspondence =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
                "subject:a",
                "record:a",
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                1,
            ))
            .expect("correspondence should resolve");
        let request = HistoricalEvaluationRequest::retained_snapshot(
            "basis:a",
            1,
            1,
            HistoricalPathReuseDescriptor::retained_reuse(),
        );
        let capability = HistoricalCapabilityDescriptor::new(
            "basis:a",
            Some(crate::historical::AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
            false,
            false,
            true,
            false,
            HistoricalPathReuseDescriptor::retained_reuse(),
        );
        let admission = admit_historical_evaluation_path(request, capability)
            .expect("admission should succeed");
        let resolved = resolve_historical_materialization_path(
            admission,
            HistoricalMaterializationDescriptor::new(
                "basis:a",
                ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
            ),
        )
        .expect("resolution should succeed");

        let envelope =
            compose_correspondence_historical_envelope(execution.clone(), correspondence, resolved);
        let view = envelope
            .result_view()
            .expect("success envelope should expose metadata-preserving view");

        assert_eq!(view.payload(), execution.payload());
        assert_eq!(view.correspondence_family_name(), "lineage_continuity");
        assert_eq!(
            view.materialization_metadata()
                .resolved_path_class()
                .as_str(),
            "resolved_retained_snapshot_path"
        );
    }

    #[test]
    fn ambiguity_envelope_still_requires_metadata_preserving_view() {
        let execution = execute_preflight_bundle(&collection_preflight_bundle())
            .expect("collection execution should succeed");
        let correspondence =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
                vec!["record:a".into(), "record:b".into()],
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                4,
                StructuralCandidateOrderingContract::StableFingerprintOrder,
            ))
            .expect("correspondence should resolve");
        let request = HistoricalEvaluationRequest::delta_replay(
            "basis:replay",
            4,
            8,
            HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
        );
        let capability = HistoricalCapabilityDescriptor::new(
            "basis:replay",
            Some(crate::historical::AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
            true,
            false,
            false,
            true,
            HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
        );
        let admission = admit_historical_evaluation_path(request, capability)
            .expect("admission should succeed");
        let resolved = resolve_historical_materialization_path(
            admission,
            HistoricalMaterializationDescriptor::new(
                "basis:replay",
                ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
            ),
        )
        .expect("resolution should succeed");

        let envelope =
            compose_correspondence_historical_envelope(execution, correspondence, resolved);

        match envelope {
            CorrespondenceHistoricalEnvelope::Ambiguity(ref ambiguity) => {
                assert_eq!(
                    ambiguity.result_view().correspondence_family_name(),
                    "advisory_structural_ambiguous"
                );
            }
            _ => panic!("expected ambiguity envelope"),
        }
    }

    #[test]
    fn path_denied_envelope_carries_typed_denial_without_payload() {
        let correspondence =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
                "subject:a",
                "record:a",
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                1,
            ))
            .expect("correspondence should resolve");
        let denied_request = HistoricalEvaluationRequest::delta_replay(
            "basis:replay",
            2,
            2,
            HistoricalPathReuseDescriptor::no_reuse(),
        );
        let capability = HistoricalCapabilityDescriptor::new(
            "basis:replay",
            None,
            false,
            false,
            true,
            false,
            HistoricalPathReuseDescriptor::no_reuse(),
        );
        let error = admit_historical_evaluation_path(denied_request.clone(), capability)
            .expect_err("admission should fail for denied replay");
        let envelope =
            compose_historical_admission_denied_envelope(correspondence, denied_request, error);
        assert!(envelope.result_view().is_none());
        match envelope {
            CorrespondenceHistoricalEnvelope::HistoricalPathAdmissionDenied(ref denied) => {
                assert_eq!(
                    denied.error().failure_class(),
                    crate::historical::HistoricalEvaluationFailureClass::ReplayNotPermitted
                );
                assert_eq!(
                    denied.compatibility_outcome().as_str(),
                    "denied"
                );
            }
            _ => panic!("expected historical path denied envelope"),
        }
    }

    fn detail_preflight_bundle() -> crate::basis::ExecutionPreflightBundle {
        let validated = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
        let request =
            crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
        let basis =
            crate::harness::fixtures::resolved_bases::runtime_basis(&validated, "snapshot-1");
        let plan = crate::facade::plan_validated_bundle(&validated, request)
            .expect("detail validated bundle should plan");
        crate::facade::preflight_execution_basis(plan, basis).expect("detail plan should preflight")
    }

    fn collection_preflight_bundle() -> crate::basis::ExecutionPreflightBundle {
        let validated = crate::harness::fixtures::validated_bundles::ordered_collection_bundle();
        let request =
            crate::harness::fixtures::planning_requests::direct_runtime_request(&validated);
        let basis =
            crate::harness::fixtures::resolved_bases::runtime_basis(&validated, "snapshot-1");
        let plan = crate::facade::plan_validated_bundle(&validated, request)
            .expect("collection validated bundle should plan");
        crate::facade::preflight_execution_basis(plan, basis)
            .expect("collection plan should preflight")
    }
}
