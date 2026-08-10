use super::super::contracts::CorrespondenceComplexityContract;
use super::super::cost::CorrespondenceCostPosture;
use super::super::counters::CorrespondenceCounterSnapshot;
#[cfg(test)]
use super::super::error::CorrespondenceEvaluationError;
use super::super::outcome::{CorrespondenceOutcome, LineageContinuity};
use super::super::request::{CorrespondenceEvaluationRequest, LineageEvidenceInput};
use super::CorrespondenceEvidenceResolved;
use crate::identity::LineageDigest;

pub(super) fn resolve_lineage_only(
    request: &CorrespondenceEvaluationRequest,
    lineage: &LineageEvidenceInput,
    lineage_digest: LineageDigest,
) -> CorrespondenceEvidenceResolved {
    match lineage {
        LineageEvidenceInput::AuthoritativeContinuity {
            canonical_subject,
            authoritative_counterpart,
        } => CorrespondenceEvidenceResolved::new(
            lineage_digest,
            CorrespondenceOutcome::lineage_continuity(LineageContinuity::new(
                canonical_subject.clone(),
                authoritative_counterpart.clone(),
            )),
            request.discovery_plan().clone(),
            request.budget().clone(),
            CorrespondenceCostPosture::LineageDirect,
            CorrespondenceComplexityContract::lineage_direct(),
            CorrespondenceCounterSnapshot::lineage_direct(),
        ),
        #[cfg(test)]
        LineageEvidenceInput::UnsupportedTopology { topology } => super::denied_resolution(
            request,
            lineage_digest,
            CorrespondenceEvaluationError::UnsupportedTopology { topology },
            0,
            0,
            0,
        ),
    }
}

pub(super) fn lineage_digest_for(lineage: Option<&LineageEvidenceInput>) -> LineageDigest {
    match lineage {
        Some(LineageEvidenceInput::AuthoritativeContinuity {
            canonical_subject,
            authoritative_counterpart,
        }) => LineageDigest::from_parts(&[
            "lineage:authoritative".to_string(),
            format!("subject:{canonical_subject}"),
            format!("counterpart:{authoritative_counterpart}"),
        ]),
        #[cfg(test)]
        Some(LineageEvidenceInput::UnsupportedTopology { topology }) => {
            LineageDigest::from_parts(&[
                "lineage:unsupported_topology".to_string(),
                format!("topology:{topology}"),
            ])
        }
        None => LineageDigest::from_parts(&["lineage:absent".to_string()]),
    }
}
