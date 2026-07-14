use crate::correspondence::{
    CorrespondenceAmbiguityEnvelope, CorrespondenceDisagreementEnvelope,
    CorrespondenceEvidenceResolved,
};
use crate::execution::ExecutionResultEnvelope;
use crate::historical::{
    HistoricalEvaluationAdmission, HistoricalEvaluationError, HistoricalEvaluationRequest,
    HistoricalPathResolved,
};

use super::denied::{
    CorrespondenceHistoricalDeniedEnvelope, HistoricalPathAdmissionDeniedEnvelope,
    HistoricalPathDeniedEnvelope,
};
use super::envelope::CorrespondenceHistoricalEnvelope;
use super::success::{
    CorrespondenceHistoricalAmbiguityEnvelope, CorrespondenceHistoricalDisagreementEnvelope,
    CorrespondenceHistoricalSuccessEnvelope,
};

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
