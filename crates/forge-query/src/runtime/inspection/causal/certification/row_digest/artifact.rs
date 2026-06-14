use super::super::super::materialization::QueryCausalInspectionArtifact;
use super::super::matrix_kind::CausalInspectionRepresentativeKind;

pub(super) fn inspection_digest(artifact: &QueryCausalInspectionArtifact) -> &str {
    match artifact {
        QueryCausalInspectionArtifact::Admitted(artifact) => artifact.query_admission_for_reporting(),
        QueryCausalInspectionArtifact::Advisory(artifact) => artifact.query_advisory_for_reporting(),
        QueryCausalInspectionArtifact::Denied(artifact) => artifact.query_denial_for_reporting(),
    }
}

pub(super) fn artifact_receipt_digest(artifact: &QueryCausalInspectionArtifact) -> &str {
    match artifact {
        QueryCausalInspectionArtifact::Admitted(artifact) => artifact.receipt().receipt_for_reporting(),
        QueryCausalInspectionArtifact::Advisory(artifact) => artifact.receipt().receipt_for_reporting(),
        QueryCausalInspectionArtifact::Denied(artifact) => artifact.receipt().receipt_for_reporting(),
    }
}

pub(super) fn artifact_policy_digest(artifact: &QueryCausalInspectionArtifact) -> &str {
    match artifact {
        QueryCausalInspectionArtifact::Admitted(artifact) => artifact.receipt().policy_for_reporting(),
        QueryCausalInspectionArtifact::Advisory(artifact) => artifact.receipt().policy_for_reporting(),
        QueryCausalInspectionArtifact::Denied(artifact) => artifact.receipt().policy_for_reporting(),
    }
}

pub(super) fn evidence_reference_collection_digest(
    artifact: &QueryCausalInspectionArtifact,
    kind: CausalInspectionRepresentativeKind,
) -> String {
    let references = match artifact {
        QueryCausalInspectionArtifact::Admitted(artifact) => artifact
            .evidence_references()
            .iter()
            .map(|reference| reference.reference_for_reporting())
            .collect::<Vec<_>>(),
        QueryCausalInspectionArtifact::Advisory(artifact) => artifact
            .evidence_references()
            .iter()
            .map(|reference| reference.reference_for_reporting())
            .collect::<Vec<_>>(),
        QueryCausalInspectionArtifact::Denied(_) => Vec::new(),
    }
    .join("|");
    crate::identity::hash_parts(&[
        "causal_evidence_reference_collection_proof_v1".to_string(),
        format!("kind:{}", kind.as_str()),
        format!("references:{references}"),
    ])
}
