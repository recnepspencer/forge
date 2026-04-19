use crate::correspondence_history::{
    CorrespondenceHistoricalAmbiguityEnvelope, CorrespondenceHistoricalDeniedEnvelope,
    CorrespondenceHistoricalDisagreementEnvelope, CorrespondenceHistoricalEnvelope,
    CorrespondenceHistoricalSuccessEnvelope, HistoricalPathAdmissionDeniedEnvelope,
    HistoricalPathDeniedEnvelope,
};
use crate::historical::HistoricalPathCompatibilityOutcome;
use crate::identity::{BasisDigest, FailureDigest, ValidatedQueryDigest};

use super::bundle::{
    CorrespondenceHistoricalParityBundle, CorrespondenceHistoricalParityBundleError,
    CorrespondenceHistoricalParityVariant,
};
use super::digests::{
    digest_admitted_path, digest_correspondence_cost_posture, digest_correspondence_outcome,
    digest_counter_snapshot, digest_historical_cost_posture, digest_historical_failure,
    digest_requested_path, digest_resolved_path, infer_prediction_drift,
};

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
    ambiguity: &CorrespondenceHistoricalAmbiguityEnvelope,
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
