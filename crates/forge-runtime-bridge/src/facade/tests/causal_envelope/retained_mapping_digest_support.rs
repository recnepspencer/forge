use crate::diagnostics::causal_envelope::retained_mapping::digest_basis::{
    compose_retained_causal_mapping_evidence_identity, retained_mapping_external_authority_part,
    retained_mapping_shape_part, RetainedCausalMappingDigestArtifact,
    RetainedCausalMappingIdentityPart,
};

#[derive(Clone, Copy)]
pub(super) enum ExpectedRetainedCausalDigestArtifact {
    ContinuityRecord,
    HistoricalEvaluationRecord,
    MergeRecord,
    PreviewDiscardRecord,
    PreviewExecutionRecord,
    PreviewPromotionRecord,
    RouteRecord,
    SourceMaterializationRecord,
    StreamReplayRecord,
    StructuralBranchComparisonRecord,
    StructuralRemapRecord,
}

impl From<ExpectedRetainedCausalDigestArtifact> for RetainedCausalMappingDigestArtifact {
    fn from(value: ExpectedRetainedCausalDigestArtifact) -> Self {
        match value {
            ExpectedRetainedCausalDigestArtifact::ContinuityRecord => Self::ContinuityRecord,
            ExpectedRetainedCausalDigestArtifact::HistoricalEvaluationRecord => {
                Self::HistoricalEvaluationRecord
            }
            ExpectedRetainedCausalDigestArtifact::MergeRecord => Self::MergeRecord,
            ExpectedRetainedCausalDigestArtifact::PreviewDiscardRecord => {
                Self::PreviewDiscardRecord
            }
            ExpectedRetainedCausalDigestArtifact::PreviewExecutionRecord => {
                Self::PreviewExecutionRecord
            }
            ExpectedRetainedCausalDigestArtifact::PreviewPromotionRecord => {
                Self::PreviewPromotionRecord
            }
            ExpectedRetainedCausalDigestArtifact::RouteRecord => Self::RouteRecord,
            ExpectedRetainedCausalDigestArtifact::SourceMaterializationRecord => {
                Self::SourceMaterializationRecord
            }
            ExpectedRetainedCausalDigestArtifact::StreamReplayRecord => Self::StreamReplayRecord,
            ExpectedRetainedCausalDigestArtifact::StructuralBranchComparisonRecord => {
                Self::StructuralBranchComparisonRecord
            }
            ExpectedRetainedCausalDigestArtifact::StructuralRemapRecord => {
                Self::StructuralRemapRecord
            }
        }
    }
}

pub(super) fn expected_retained_causal_digest(
    artifact: ExpectedRetainedCausalDigestArtifact,
    parts: &[&str],
) -> String {
    let typed_parts = parts
        .iter()
        .enumerate()
        .map(|(index, part)| expected_part_kind(artifact, index, part))
        .collect::<Vec<_>>();
    compose_retained_causal_mapping_evidence_identity(artifact.into(), &typed_parts)
        .as_str()
        .to_string()
}

fn expected_part_kind<'a>(
    artifact: ExpectedRetainedCausalDigestArtifact,
    index: usize,
    part: &'a str,
) -> RetainedCausalMappingIdentityPart {
    match artifact {
        ExpectedRetainedCausalDigestArtifact::ContinuityRecord
        | ExpectedRetainedCausalDigestArtifact::MergeRecord
        | ExpectedRetainedCausalDigestArtifact::StructuralBranchComparisonRecord
        | ExpectedRetainedCausalDigestArtifact::StructuralRemapRecord
            if index == 1 =>
        {
            retained_mapping_shape_part(part)
        }
        ExpectedRetainedCausalDigestArtifact::StreamReplayRecord if index == 5 => {
            retained_mapping_shape_part(part)
        }
        _ => retained_mapping_external_authority_part(part),
    }
}
