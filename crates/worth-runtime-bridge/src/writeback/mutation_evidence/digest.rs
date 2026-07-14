use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MutationEvidenceAggregateArtifact {
    BatchContinuityMutation,
    BatchExistingTruthBinding,
    BatchMutationCausality,
    BatchMutationProvenance,
    BatchNamingMutation,
    BatchSymbolicTargetReference,
    ExistingTruthBinding,
}

impl MutationEvidenceAggregateArtifact {
    fn digest_domain(self) -> &'static str {
        match self {
            Self::BatchContinuityMutation => "bridge-batch-continuity-mutation",
            Self::BatchExistingTruthBinding => "bridge-batch-existing-truth-binding",
            Self::BatchMutationCausality => "bridge-batch-mutation-causality",
            Self::BatchMutationProvenance => "bridge-batch-mutation-provenance",
            Self::BatchNamingMutation => "bridge-batch-naming-mutation",
            Self::BatchSymbolicTargetReference => "bridge-batch-symbolic-target-reference",
            Self::ExistingTruthBinding => "bridge-existing-truth-binding",
        }
    }
}

pub(super) fn existing_truth_binding_digest(entries: impl IntoIterator<Item = String>) -> Arc<str> {
    derive_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::ExistingTruthBinding,
        entries,
    )
}

pub(super) fn batch_existing_truth_binding_digest(
    entries: impl IntoIterator<Item = String>,
) -> Option<Arc<str>> {
    derive_optional_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::BatchExistingTruthBinding,
        entries,
    )
}

pub(super) fn batch_symbolic_target_reference_digest(
    entries: impl IntoIterator<Item = String>,
) -> Option<Arc<str>> {
    derive_optional_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::BatchSymbolicTargetReference,
        entries,
    )
}

pub(super) fn batch_naming_mutation_digest(
    entries: impl IntoIterator<Item = String>,
) -> Option<Arc<str>> {
    derive_optional_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::BatchNamingMutation,
        entries,
    )
}

pub(super) fn batch_continuity_mutation_digest(
    entries: impl IntoIterator<Item = String>,
) -> Option<Arc<str>> {
    derive_optional_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::BatchContinuityMutation,
        entries,
    )
}

pub(super) fn batch_mutation_causality_digest(
    entries: impl IntoIterator<Item = String>,
) -> Arc<str> {
    derive_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::BatchMutationCausality,
        entries,
    )
}

pub(super) fn batch_mutation_provenance_digest(
    entries: impl IntoIterator<Item = String>,
) -> Arc<str> {
    derive_mutation_evidence_digest(
        MutationEvidenceAggregateArtifact::BatchMutationProvenance,
        entries,
    )
}

fn derive_mutation_evidence_digest(
    artifact: MutationEvidenceAggregateArtifact,
    entries: impl IntoIterator<Item = String>,
) -> Arc<str> {
    use sha2::{Digest, Sha256};

    let digest_domain = artifact.digest_domain();
    let mut hasher = Sha256::new();
    hasher.update(digest_domain.as_bytes());
    for entry in entries {
        hasher.update(entry.as_bytes());
    }
    let digest = hasher.finalize();
    Arc::from(format!("{digest_domain}:sha256:{digest:x}"))
}

fn derive_optional_mutation_evidence_digest(
    artifact: MutationEvidenceAggregateArtifact,
    entries: impl IntoIterator<Item = String>,
) -> Option<Arc<str>> {
    let entries = entries.into_iter().collect::<Vec<_>>();
    if entries.is_empty() {
        return None;
    }
    Some(derive_mutation_evidence_digest(artifact, entries))
}
