use crate::correspondence::CorrespondenceEvidenceResolved;
use crate::correspondence_history::{
    CorrespondenceHistoricalDeniedEnvelope, CorrespondenceHistoricalDisagreementEnvelope,
    CorrespondenceHistoricalEnvelope, CorrespondenceHistoricalSuccessEnvelope,
    HistoricalPathAdmissionDeniedEnvelope, HistoricalPathDeniedEnvelope,
};
use crate::historical::{
    HistoricalCounterSnapshot, HistoricalEvaluationError, HistoricalPathCompatibilityOutcome,
    HistoricalPathCostPosture,
};
use crate::identity::{
    BasisDigest, CorrespondenceCostPostureDigest, CorrespondenceOutcomeDigest,
    CounterSnapshotDigest, FailureDigest, HistoricalCostPostureDigest, HistoricalPathClassDigest,
    LineageDigest, ResultDigest, ValidatedQueryDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoricalParityBundleError {
    MissingDeniedQueryDigest,
    MissingDeniedBasisDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoricalParityVariant {
    Success,
    Ambiguity,
    Disagreement,
    CorrespondenceDenied,
    HistoricalPathDenied,
}

impl CorrespondenceHistoricalParityVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Ambiguity => "ambiguity",
            Self::Disagreement => "disagreement",
            Self::CorrespondenceDenied => "correspondence_denied",
            Self::HistoricalPathDenied => "historical_path_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalParityBundle {
    parity_variant: CorrespondenceHistoricalParityVariant,
    query_digest: ValidatedQueryDigest,
    lineage_digest: LineageDigest,
    basis_digest: BasisDigest,
    result_digest: Option<ResultDigest>,
    failure_digest: Option<FailureDigest>,
    correspondence_outcome_digest: CorrespondenceOutcomeDigest,
    requested_path_digest: Option<HistoricalPathClassDigest>,
    admitted_path_digest: Option<HistoricalPathClassDigest>,
    resolved_path_digest: Option<HistoricalPathClassDigest>,
    historical_compatibility_outcome: Option<HistoricalPathCompatibilityOutcome>,
    correspondence_cost_posture_digest: CorrespondenceCostPostureDigest,
    historical_cost_posture_digest: Option<HistoricalCostPostureDigest>,
    counter_snapshot_digest: CounterSnapshotDigest,
    performance_prediction_drift_outcome: crate::historical::PerformancePredictionDriftOutcome,
}

impl CorrespondenceHistoricalParityBundle {
    pub fn parity_variant(&self) -> &CorrespondenceHistoricalParityVariant {
        &self.parity_variant
    }

    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn result_digest(&self) -> Option<&ResultDigest> {
        self.result_digest.as_ref()
    }

    pub fn failure_digest(&self) -> Option<&FailureDigest> {
        self.failure_digest.as_ref()
    }

    pub fn correspondence_outcome_digest(&self) -> &CorrespondenceOutcomeDigest {
        &self.correspondence_outcome_digest
    }

    pub fn requested_path_digest(&self) -> Option<&HistoricalPathClassDigest> {
        self.requested_path_digest.as_ref()
    }

    pub fn admitted_path_digest(&self) -> Option<&HistoricalPathClassDigest> {
        self.admitted_path_digest.as_ref()
    }

    pub fn resolved_path_digest(&self) -> Option<&HistoricalPathClassDigest> {
        self.resolved_path_digest.as_ref()
    }

    pub fn historical_compatibility_outcome(&self) -> Option<&HistoricalPathCompatibilityOutcome> {
        self.historical_compatibility_outcome.as_ref()
    }

    pub fn correspondence_cost_posture_digest(&self) -> &CorrespondenceCostPostureDigest {
        &self.correspondence_cost_posture_digest
    }

    pub fn historical_cost_posture_digest(&self) -> Option<&HistoricalCostPostureDigest> {
        self.historical_cost_posture_digest.as_ref()
    }

    pub fn counter_snapshot_digest(&self) -> &CounterSnapshotDigest {
        &self.counter_snapshot_digest
    }

    pub fn performance_prediction_drift_outcome(
        &self,
    ) -> &crate::historical::PerformancePredictionDriftOutcome {
        &self.performance_prediction_drift_outcome
    }
}

pub fn build_correspondence_historical_parity_bundle(
    envelope: &CorrespondenceHistoricalEnvelope,
    denied_query_digest: Option<ValidatedQueryDigest>,
    denied_basis_digest: Option<BasisDigest>,
) -> Result<CorrespondenceHistoricalParityBundle, CorrespondenceHistoricalParityBundleError> {
    match envelope {
        CorrespondenceHistoricalEnvelope::Success(success) => Ok(from_success(success)),
        CorrespondenceHistoricalEnvelope::Ambiguity(ambiguity) => Ok(from_ambiguity(ambiguity)),
        CorrespondenceHistoricalEnvelope::Disagreement(disagreement) => {
            Ok(from_disagreement(disagreement))
        }
        CorrespondenceHistoricalEnvelope::CorrespondenceDenied(denied) => {
            let query_digest = denied_query_digest
                .ok_or(CorrespondenceHistoricalParityBundleError::MissingDeniedQueryDigest)?;
            let basis_digest = denied_basis_digest
                .ok_or(CorrespondenceHistoricalParityBundleError::MissingDeniedBasisDigest)?;
            Ok(from_correspondence_denied(
                denied,
                query_digest,
                basis_digest,
            ))
        }
        CorrespondenceHistoricalEnvelope::HistoricalPathDenied(denied) => {
            let query_digest = denied_query_digest
                .ok_or(CorrespondenceHistoricalParityBundleError::MissingDeniedQueryDigest)?;
            let basis_digest = denied_basis_digest
                .ok_or(CorrespondenceHistoricalParityBundleError::MissingDeniedBasisDigest)?;
            Ok(from_historical_path_denied(
                denied,
                query_digest,
                basis_digest,
            ))
        }
        CorrespondenceHistoricalEnvelope::HistoricalPathAdmissionDenied(denied) => {
            let query_digest = denied_query_digest
                .ok_or(CorrespondenceHistoricalParityBundleError::MissingDeniedQueryDigest)?;
            let basis_digest = denied_basis_digest
                .ok_or(CorrespondenceHistoricalParityBundleError::MissingDeniedBasisDigest)?;
            Ok(from_historical_path_admission_denied(
                denied,
                query_digest,
                basis_digest,
            ))
        }
    }
}

fn from_success(
    success: &CorrespondenceHistoricalSuccessEnvelope,
) -> CorrespondenceHistoricalParityBundle {
    let report = success.execution().report();
    CorrespondenceHistoricalParityBundle {
        parity_variant: CorrespondenceHistoricalParityVariant::Success,
        query_digest: report.query_digest().clone(),
        lineage_digest: success.correspondence().lineage_digest().clone(),
        basis_digest: report.basis_digest().clone(),
        result_digest: Some(report.result_digest().clone()),
        failure_digest: None,
        correspondence_outcome_digest: digest_correspondence_outcome(success.correspondence()),
        requested_path_digest: Some(digest_requested_path(
            success.historical().requested_path_class().as_str(),
        )),
        admitted_path_digest: Some(digest_admitted_path(
            success.historical().admitted_path_class().as_str(),
        )),
        resolved_path_digest: Some(digest_resolved_path(
            success.historical().resolved_path_class().as_str(),
        )),
        historical_compatibility_outcome: Some(HistoricalPathCompatibilityOutcome::Admitted),
        correspondence_cost_posture_digest: digest_correspondence_cost_posture(
            success.correspondence().cost_posture().as_str(),
        ),
        historical_cost_posture_digest: Some(digest_historical_cost_posture(
            success.historical().cost_posture(),
        )),
        counter_snapshot_digest: digest_counter_snapshot(
            success.correspondence(),
            Some(success.historical().counters()),
        ),
        performance_prediction_drift_outcome: infer_prediction_drift(
            success.correspondence(),
            Some(success.historical().counters()),
        ),
    }
}

fn from_ambiguity(
    ambiguity: &crate::correspondence_history::CorrespondenceHistoricalAmbiguityEnvelope,
) -> CorrespondenceHistoricalParityBundle {
    let report = ambiguity.execution().report();
    CorrespondenceHistoricalParityBundle {
        parity_variant: CorrespondenceHistoricalParityVariant::Ambiguity,
        query_digest: report.query_digest().clone(),
        lineage_digest: ambiguity.correspondence().lineage_digest().clone(),
        basis_digest: report.basis_digest().clone(),
        result_digest: Some(report.result_digest().clone()),
        failure_digest: None,
        correspondence_outcome_digest: digest_correspondence_outcome(ambiguity.correspondence()),
        requested_path_digest: Some(digest_requested_path(
            ambiguity.historical().requested_path_class().as_str(),
        )),
        admitted_path_digest: Some(digest_admitted_path(
            ambiguity.historical().admitted_path_class().as_str(),
        )),
        resolved_path_digest: Some(digest_resolved_path(
            ambiguity.historical().resolved_path_class().as_str(),
        )),
        historical_compatibility_outcome: Some(HistoricalPathCompatibilityOutcome::Admitted),
        correspondence_cost_posture_digest: digest_correspondence_cost_posture(
            ambiguity.correspondence().cost_posture().as_str(),
        ),
        historical_cost_posture_digest: Some(digest_historical_cost_posture(
            ambiguity.historical().cost_posture(),
        )),
        counter_snapshot_digest: digest_counter_snapshot(
            ambiguity.correspondence(),
            Some(ambiguity.historical().counters()),
        ),
        performance_prediction_drift_outcome: infer_prediction_drift(
            ambiguity.correspondence(),
            Some(ambiguity.historical().counters()),
        ),
    }
}

fn from_disagreement(
    disagreement: &CorrespondenceHistoricalDisagreementEnvelope,
) -> CorrespondenceHistoricalParityBundle {
    let report = disagreement.execution().report();
    CorrespondenceHistoricalParityBundle {
        parity_variant: CorrespondenceHistoricalParityVariant::Disagreement,
        query_digest: report.query_digest().clone(),
        lineage_digest: disagreement.correspondence().lineage_digest().clone(),
        basis_digest: report.basis_digest().clone(),
        result_digest: Some(report.result_digest().clone()),
        failure_digest: None,
        correspondence_outcome_digest: digest_correspondence_outcome(disagreement.correspondence()),
        requested_path_digest: Some(digest_requested_path(
            disagreement.historical().requested_path_class().as_str(),
        )),
        admitted_path_digest: Some(digest_admitted_path(
            disagreement.historical().admitted_path_class().as_str(),
        )),
        resolved_path_digest: Some(digest_resolved_path(
            disagreement.historical().resolved_path_class().as_str(),
        )),
        historical_compatibility_outcome: Some(HistoricalPathCompatibilityOutcome::Admitted),
        correspondence_cost_posture_digest: digest_correspondence_cost_posture(
            disagreement.correspondence().cost_posture().as_str(),
        ),
        historical_cost_posture_digest: Some(digest_historical_cost_posture(
            disagreement.historical().cost_posture(),
        )),
        counter_snapshot_digest: digest_counter_snapshot(
            disagreement.correspondence(),
            Some(disagreement.historical().counters()),
        ),
        performance_prediction_drift_outcome: infer_prediction_drift(
            disagreement.correspondence(),
            Some(disagreement.historical().counters()),
        ),
    }
}

fn from_correspondence_denied(
    denied: &CorrespondenceHistoricalDeniedEnvelope,
    query_digest: ValidatedQueryDigest,
    basis_digest: BasisDigest,
) -> CorrespondenceHistoricalParityBundle {
    CorrespondenceHistoricalParityBundle {
        parity_variant: CorrespondenceHistoricalParityVariant::CorrespondenceDenied,
        query_digest,
        lineage_digest: denied.correspondence().lineage_digest().clone(),
        basis_digest,
        result_digest: None,
        failure_digest: Some(FailureDigest::from_parts(&[
            "failure_class:correspondence_denied".to_string(),
            format!("reason:{}", denied.denied().reason()),
            format!("cost_posture:{}", denied.denied().cost_posture().as_str()),
        ])),
        correspondence_outcome_digest: digest_correspondence_outcome(denied.correspondence()),
        requested_path_digest: None,
        admitted_path_digest: None,
        resolved_path_digest: None,
        historical_compatibility_outcome: None,
        correspondence_cost_posture_digest: digest_correspondence_cost_posture(
            denied.correspondence().cost_posture().as_str(),
        ),
        historical_cost_posture_digest: None,
        counter_snapshot_digest: digest_counter_snapshot(denied.correspondence(), None),
        performance_prediction_drift_outcome: infer_prediction_drift(denied.correspondence(), None),
    }
}

fn from_historical_path_denied(
    denied: &HistoricalPathDeniedEnvelope,
    query_digest: ValidatedQueryDigest,
    basis_digest: BasisDigest,
) -> CorrespondenceHistoricalParityBundle {
    CorrespondenceHistoricalParityBundle {
        parity_variant: CorrespondenceHistoricalParityVariant::HistoricalPathDenied,
        query_digest,
        lineage_digest: denied.correspondence().lineage_digest().clone(),
        basis_digest,
        result_digest: None,
        failure_digest: Some(digest_historical_failure(denied.error())),
        correspondence_outcome_digest: digest_correspondence_outcome(denied.correspondence()),
        requested_path_digest: Some(digest_requested_path(
            denied
                .admission()
                .requested_path()
                .requested_path_class()
                .as_str(),
        )),
        admitted_path_digest: Some(digest_admitted_path(
            denied
                .admission()
                .admitted_path()
                .admitted_path_class()
                .as_str(),
        )),
        resolved_path_digest: denied
            .error()
            .attempted_resolved_path_class()
            .map(|class| digest_resolved_path(class.as_str())),
        historical_compatibility_outcome: Some(denied.compatibility_outcome().clone()),
        correspondence_cost_posture_digest: digest_correspondence_cost_posture(
            denied.correspondence().cost_posture().as_str(),
        ),
        historical_cost_posture_digest: Some(digest_historical_cost_posture(
            denied.denial_cost_posture(),
        )),
        counter_snapshot_digest: digest_counter_snapshot(
            denied.correspondence(),
            Some(denied.counters()),
        ),
        performance_prediction_drift_outcome: infer_prediction_drift(
            denied.correspondence(),
            Some(denied.counters()),
        ),
    }
}

fn from_historical_path_admission_denied(
    denied: &HistoricalPathAdmissionDeniedEnvelope,
    query_digest: ValidatedQueryDigest,
    basis_digest: BasisDigest,
) -> CorrespondenceHistoricalParityBundle {
    CorrespondenceHistoricalParityBundle {
        parity_variant: CorrespondenceHistoricalParityVariant::HistoricalPathDenied,
        query_digest,
        lineage_digest: denied.correspondence().lineage_digest().clone(),
        basis_digest,
        result_digest: None,
        failure_digest: Some(digest_historical_failure(denied.error())),
        correspondence_outcome_digest: digest_correspondence_outcome(denied.correspondence()),
        requested_path_digest: Some(digest_requested_path(
            denied.request().requested_path_class().as_str(),
        )),
        admitted_path_digest: None,
        resolved_path_digest: None,
        historical_compatibility_outcome: Some(denied.compatibility_outcome()),
        correspondence_cost_posture_digest: digest_correspondence_cost_posture(
            denied.correspondence().cost_posture().as_str(),
        ),
        historical_cost_posture_digest: Some(digest_historical_cost_posture(
            denied.denial_cost_posture(),
        )),
        counter_snapshot_digest: digest_counter_snapshot(
            denied.correspondence(),
            Some(denied.counters()),
        ),
        performance_prediction_drift_outcome: infer_prediction_drift(
            denied.correspondence(),
            Some(denied.counters()),
        ),
    }
}

fn digest_correspondence_outcome(
    correspondence: &CorrespondenceEvidenceResolved,
) -> CorrespondenceOutcomeDigest {
    let outcome = correspondence.outcome();
    let mut parts = vec![format!("family:{}", outcome.family_name())];

    if let Some(lineage) = outcome.as_lineage_continuity() {
        parts.push(format!("canonical_subject:{}", lineage.canonical_subject()));
        parts.push(format!(
            "authoritative_counterpart:{}",
            lineage.authoritative_counterpart()
        ));
    }

    if let Some(unique) = outcome.as_advisory_structural_unique() {
        parts.push(format!(
            "advisory_candidate:{}",
            unique.advisory_candidate()
        ));
        parts.push("uniqueness_witness:present".to_string());
    }

    if let Some(ambiguous) = outcome.as_advisory_structural_ambiguous() {
        parts.push(format!(
            "candidate_count:{}",
            ambiguous.candidate_set().len()
        ));
        parts.extend(
            ambiguous
                .candidate_set()
                .candidates()
                .iter()
                .map(|candidate| format!("candidate:{candidate}")),
        );
    }

    if let Some(disagreement) = outcome.as_lineage_structural_disagreement() {
        parts.push(format!(
            "lineage_counterpart:{}",
            disagreement.lineage_counterpart()
        ));
        parts.push(format!(
            "structural_counterpart:{}",
            disagreement.structural_counterpart()
        ));
    }

    if let Some(denied) = outcome.as_denied() {
        parts.push(format!("denied_reason:{}", denied.reason()));
        parts.push(format!(
            "denied_cost_posture:{}",
            denied.cost_posture().as_str()
        ));
    }

    CorrespondenceOutcomeDigest::from_parts(&parts)
}

fn digest_requested_path(value: &str) -> HistoricalPathClassDigest {
    HistoricalPathClassDigest::from_parts(&[format!("requested_path:{value}")])
}

fn digest_admitted_path(value: &str) -> HistoricalPathClassDigest {
    HistoricalPathClassDigest::from_parts(&[format!("admitted_path:{value}")])
}

fn digest_resolved_path(value: &str) -> HistoricalPathClassDigest {
    HistoricalPathClassDigest::from_parts(&[format!("resolved_path:{value}")])
}

fn digest_correspondence_cost_posture(value: &str) -> CorrespondenceCostPostureDigest {
    CorrespondenceCostPostureDigest::from_parts(&[format!("correspondence_cost_posture:{value}")])
}

fn digest_historical_cost_posture(
    value: &HistoricalPathCostPosture,
) -> HistoricalCostPostureDigest {
    HistoricalCostPostureDigest::from_parts(&[format!(
        "historical_cost_posture:{}",
        value.as_str()
    )])
}

fn digest_counter_snapshot(
    correspondence: &CorrespondenceEvidenceResolved,
    historical: Option<&HistoricalCounterSnapshot>,
) -> CounterSnapshotDigest {
    let counters = correspondence.counters();
    let mut parts = vec![
        format!(
            "correspondence.predicted_structural_candidate_count:{}",
            counters.predicted_structural_candidate_count()
        ),
        format!(
            "correspondence.structural_candidate_count:{}",
            counters.structural_candidate_count()
        ),
        format!(
            "correspondence.structural_candidate_rejection_count:{}",
            counters.structural_candidate_rejection_count()
        ),
        format!(
            "correspondence.structural_ambiguity_count:{}",
            counters.structural_ambiguity_count()
        ),
        format!(
            "correspondence.structural_unique_witness_count:{}",
            counters.structural_unique_witness_count()
        ),
        format!(
            "correspondence.lineage_structural_disagreement_count:{}",
            counters.lineage_structural_disagreement_count()
        ),
        format!(
            "correspondence.structural_authority_promotion_denial_count:{}",
            counters.structural_authority_promotion_denial_count()
        ),
        format!(
            "correspondence.predicted_correspondence_resolution_width:{}",
            counters.predicted_correspondence_resolution_width()
        ),
        format!(
            "correspondence.structural_candidate_prediction_drift_count:{}",
            counters.structural_candidate_prediction_drift_count()
        ),
        format!(
            "correspondence.executor_rediscovery_count:{}",
            counters.correspondence_executor_rediscovery_count()
        ),
    ];

    if let Some(historical) = historical {
        parts.extend([
            format!(
                "historical.requested_path_count:{}",
                historical.historical_requested_path_count()
            ),
            format!(
                "historical.admitted_path_count:{}",
                historical.historical_admitted_path_count()
            ),
            format!(
                "historical.resolved_path_count:{}",
                historical.historical_resolved_path_count()
            ),
            format!(
                "historical.compatibility_check_count:{}",
                historical.historical_compatibility_check_count()
            ),
            format!(
                "historical.predicted_replay_span:{}",
                historical.predicted_historical_replay_span()
            ),
            format!(
                "historical.predicted_reconstruction_scope:{}",
                historical.predicted_historical_reconstruction_scope()
            ),
            format!(
                "historical.retained_snapshot_admission_count:{}",
                historical.historical_retained_snapshot_admission_count()
            ),
            format!(
                "historical.delta_replay_admission_count:{}",
                historical.historical_delta_replay_admission_count()
            ),
            format!(
                "historical.full_reconstruction_admission_count:{}",
                historical.historical_full_reconstruction_admission_count()
            ),
            format!(
                "historical.path_denial_count:{}",
                historical.historical_path_denial_count()
            ),
            format!(
                "historical.hidden_path_substitution_denial_count:{}",
                historical.historical_hidden_path_substitution_denial_count()
            ),
            format!(
                "historical.result_path_metadata_count:{}",
                historical.historical_result_path_metadata_count()
            ),
            format!(
                "historical.replay_span_drift_count:{}",
                historical.historical_replay_span_drift_count()
            ),
            format!(
                "historical.reconstruction_scope_drift_count:{}",
                historical.historical_reconstruction_scope_drift_count()
            ),
            format!(
                "historical.executor_rediscovery_count:{}",
                historical.historical_executor_rediscovery_count()
            ),
        ]);
    } else {
        parts.push("historical:absent".to_string());
    }

    CounterSnapshotDigest::from_parts(&parts)
}

fn infer_prediction_drift(
    correspondence: &CorrespondenceEvidenceResolved,
    historical: Option<&HistoricalCounterSnapshot>,
) -> crate::historical::PerformancePredictionDriftOutcome {
    if correspondence
        .counters()
        .structural_candidate_prediction_drift_count()
        > 0
    {
        return crate::historical::PerformancePredictionDriftOutcome::StructuralCandidatePredictionDrift;
    }

    if let Some(historical) = historical {
        if historical.historical_replay_span_drift_count() > 0 {
            return crate::historical::PerformancePredictionDriftOutcome::HistoricalReplaySpanDrift;
        }

        if historical.historical_reconstruction_scope_drift_count() > 0 {
            return crate::historical::PerformancePredictionDriftOutcome::HistoricalReconstructionScopeDrift;
        }
    }

    crate::historical::PerformancePredictionDriftOutcome::WithinBudget
}

fn digest_historical_failure(error: &HistoricalEvaluationError) -> FailureDigest {
    let mut parts = vec![
        format!("failure_class:{:?}", error.failure_class()),
        format!("requested_path:{}", error.requested_path_class().as_str()),
        format!("reason:{}", error.reason()),
    ];

    if let Some(admitted) = error.admitted_path_class() {
        parts.push(format!("admitted_path:{}", admitted.as_str()));
    }

    if let Some(resolved) = error.attempted_resolved_path_class() {
        parts.push(format!("attempted_resolved_path:{}", resolved.as_str()));
    }

    FailureDigest::from_parts(&parts)
}

#[cfg(test)]
mod tests {
    use super::{
        build_correspondence_historical_parity_bundle, CorrespondenceHistoricalParityBundleError,
    };
    use crate::correspondence::{
        resolve_correspondence_evidence, CorrespondenceEvaluationRequest,
        StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract,
    };
    use crate::correspondence_history::{
        compose_correspondence_historical_envelope, compose_historical_admission_denied_envelope,
        CorrespondenceHistoricalEnvelope,
    };
    use crate::execution::execute_preflight_bundle;
    use crate::historical::{
        admit_historical_evaluation_path, resolve_historical_materialization_path,
        AdmittedHistoricalPathClass, HistoricalCapabilityDescriptor, HistoricalEvaluationRequest,
        HistoricalMaterializationDescriptor, HistoricalPathReuseDescriptor,
        RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
    };

    #[test]
    fn ambiguity_and_disagreement_bundle_digests_stay_distinct() {
        let ambiguity =
            build_correspondence_historical_parity_bundle(&ambiguous_envelope(), None, None)
                .expect("ambiguity bundle should build");
        let disagreement =
            build_correspondence_historical_parity_bundle(&disagreement_envelope(), None, None)
                .expect("disagreement bundle should build");

        assert_ne!(
            ambiguity.correspondence_outcome_digest().as_str(),
            disagreement.correspondence_outcome_digest().as_str()
        );
        assert_eq!(
            ambiguity.performance_prediction_drift_outcome().as_str(),
            "within_budget"
        );
    }

    #[test]
    fn retained_and_replay_paths_digest_differ() {
        let retained = build_correspondence_historical_parity_bundle(
            &success_envelope_for(
                RequestedHistoricalPathClass::RequestedRetainedSnapshotPath,
                AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath,
                ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath,
                HistoricalPathReuseDescriptor::retained_reuse(),
            ),
            None,
            None,
        )
        .expect("retained bundle should build");
        let replay = build_correspondence_historical_parity_bundle(
            &success_envelope_for(
                RequestedHistoricalPathClass::RequestedDeltaReplayPath,
                AdmittedHistoricalPathClass::AdmittedDeltaReplayPath,
                ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
                HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
            ),
            None,
            None,
        )
        .expect("replay bundle should build");

        assert_ne!(
            retained
                .requested_path_digest()
                .expect("retained path")
                .as_str(),
            replay
                .requested_path_digest()
                .expect("replay path")
                .as_str()
        );
        assert_ne!(
            retained
                .historical_cost_posture_digest()
                .expect("retained posture")
                .as_str(),
            replay
                .historical_cost_posture_digest()
                .expect("replay posture")
                .as_str()
        );
    }

    #[test]
    fn historical_denial_bundle_reports_failure_digest() {
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
            .expect_err("admission should fail");
        let envelope = compose_historical_admission_denied_envelope(
            correspondence,
            denied_request,
            error,
        );
        let preflight = detail_preflight_bundle();
        let bundle = build_correspondence_historical_parity_bundle(
            &envelope,
            Some(preflight.plan().query().validated_query_digest().clone()),
            Some(preflight.basis().proof().digest().clone()),
        )
        .expect("denied bundle should build");

        assert!(bundle.failure_digest().is_some());
        assert_eq!(
            bundle
                .historical_compatibility_outcome()
                .expect("compatibility outcome")
                .as_str(),
            "denied"
        );
    }

    #[test]
    fn denied_bundle_requires_query_and_basis_digest_overrides() {
        let envelope = correspondence_denied_envelope();
        let error =
            build_correspondence_historical_parity_bundle(&envelope, None, None).unwrap_err();

        assert_eq!(
            error,
            CorrespondenceHistoricalParityBundleError::MissingDeniedQueryDigest
        );
    }

    fn success_envelope_for(
        requested: RequestedHistoricalPathClass,
        admitted: AdmittedHistoricalPathClass,
        resolved: ResolvedHistoricalPathClass,
        reuse: HistoricalPathReuseDescriptor,
    ) -> CorrespondenceHistoricalEnvelope {
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
        let request = match requested {
            RequestedHistoricalPathClass::RequestedRetainedSnapshotPath => {
                HistoricalEvaluationRequest::retained_snapshot("basis:a", 1, 1, reuse)
            }
            RequestedHistoricalPathClass::RequestedDeltaReplayPath => {
                HistoricalEvaluationRequest::delta_replay("basis:a", 4, 8, reuse)
            }
            RequestedHistoricalPathClass::RequestedFullReconstructionPath => {
                HistoricalEvaluationRequest::full_reconstruction("basis:a", 4, 8, reuse)
            }
        };
        let capability = HistoricalCapabilityDescriptor::new(
            "basis:a",
            Some(admitted),
            true,
            false,
            true,
            true,
            request.reuse_descriptor().clone(),
        );
        let admission = admit_historical_evaluation_path(request, capability)
            .expect("admission should succeed");
        let resolved = resolve_historical_materialization_path(
            admission,
            HistoricalMaterializationDescriptor::new("basis:a", resolved),
        )
        .expect("resolution should succeed");

        compose_correspondence_historical_envelope(execution, correspondence, resolved)
    }

    fn ambiguous_envelope() -> CorrespondenceHistoricalEnvelope {
        let execution = execute_preflight_bundle(&collection_preflight_bundle())
            .expect("execution should succeed");
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
            Some(AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
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

        compose_correspondence_historical_envelope(execution, correspondence, resolved)
    }

    fn disagreement_envelope() -> CorrespondenceHistoricalEnvelope {
        let execution =
            execute_preflight_bundle(&detail_preflight_bundle()).expect("execution should succeed");
        let correspondence =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::mixed(
                "subject:a",
                "record:a",
                vec!["record:z".into()],
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                2,
                StructuralCandidateOrderingContract::StableFingerprintThenLineageHintOrder,
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
            Some(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
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

        compose_correspondence_historical_envelope(execution, correspondence, resolved)
    }

    fn correspondence_denied_envelope() -> CorrespondenceHistoricalEnvelope {
        let execution =
            execute_preflight_bundle(&detail_preflight_bundle()).expect("execution should succeed");
        let correspondence =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
                vec!["record:a".into(), "record:b".into(), "record:c".into()],
                StructuralCandidateDiscoveryPlan::RequiresBroadScanDenied,
                2,
                StructuralCandidateOrderingContract::StableFingerprintOrder,
            ))
            .expect("correspondence should resolve into denial");
        let request = HistoricalEvaluationRequest::retained_snapshot(
            "basis:a",
            1,
            1,
            HistoricalPathReuseDescriptor::retained_reuse(),
        );
        let capability = HistoricalCapabilityDescriptor::new(
            "basis:a",
            Some(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
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

        compose_correspondence_historical_envelope(execution, correspondence, resolved)
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
