use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryVerificationReadSetBreadth {
    target_binding_count: usize,
    asserted_aspect_count: usize,
    distinct_asserted_aspect_path_count: usize,
    cleared_assertion_count: usize,
    counter_snapshot: String,
}

impl ForgeQueryVerificationReadSetBreadth {
    pub(in crate::runtime) fn new(
        target_binding_count: usize,
        asserted_aspect_count: usize,
        asserted_aspect_paths: &[String],
        cleared_assertion_count: usize,
    ) -> Self {
        let distinct_asserted_aspect_path_count =
            asserted_aspect_paths.iter().collect::<BTreeSet<_>>().len();
        let counter_snapshot = diagnostic_counter_snapshot(&[
            ("target_bindings", target_binding_count),
            ("asserted_aspects", asserted_aspect_count),
            (
                "distinct_asserted_aspect_paths",
                distinct_asserted_aspect_path_count,
            ),
            ("cleared_assertions", cleared_assertion_count),
        ]);
        Self {
            target_binding_count,
            asserted_aspect_count,
            distinct_asserted_aspect_path_count,
            cleared_assertion_count,
            counter_snapshot,
        }
    }

    pub fn target_binding_count(&self) -> usize {
        self.target_binding_count
    }

    pub fn asserted_aspect_count(&self) -> usize {
        self.asserted_aspect_count
    }

    pub fn distinct_asserted_aspect_path_count(&self) -> usize {
        self.distinct_asserted_aspect_path_count
    }

    pub fn cleared_assertion_count(&self) -> usize {
        self.cleared_assertion_count
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

fn diagnostic_counter_snapshot(fields: &[(&str, usize)]) -> String {
    let mut snapshot = String::new();
    for (index, (label, value)) in fields.iter().enumerate() {
        if index > 0 {
            snapshot.push(';');
        }
        snapshot.push_str(label);
        snapshot.push('=');
        snapshot.push_str(&value.to_string());
    }
    snapshot
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryVerifiedAssumptionSet {
    binding_digest: ForgeQueryEvidenceIdentity,
    asserted_aspect_paths: Vec<String>,
    assumption_snapshot_identity: ForgeQuerySnapshotIdentity,
    assumption_snapshot_evidence_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
    assumption_snapshot_digest: ForgeQueryEvidenceIdentity,
    verified_precondition_digest: ForgeQueryEvidenceIdentity,
    verification_read_set_breadth: ForgeQueryVerificationReadSetBreadth,
    verified_assumption_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryVerifiedAssumptionSet {
    pub(in crate::runtime) fn new(
        binding_digest: ForgeQueryEvidenceIdentity,
        asserted_aspect_paths: Vec<String>,
        assumed_value_fragments: Vec<ForgeQueryEvidenceIdentity>,
        cleared_assertion_count: usize,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        let assumption_snapshot_evidence_identity = snapshot_identity.evidence_identity();
        let verification_read_set_breadth = ForgeQueryVerificationReadSetBreadth::new(
            1,
            asserted_aspect_paths.len(),
            &asserted_aspect_paths,
            cleared_assertion_count,
        );
        let assumption_snapshot_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-assumption-snapshot",
                )
                .field_evidence_identity(ForgeQueryEvidenceTag::new("binding"), &binding_digest)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("snapshot"),
                    &assumption_snapshot_evidence_identity,
                )
                .seal();
        let verified_precondition_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-verified-precondition",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("assumption_snapshot"),
                    &assumption_snapshot_digest,
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("assumed_value"),
                    assumed_value_fragments.iter(),
                )
                .seal();
        let verified_assumption_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-verified-assumption-set",
                )
                .field_evidence_identity(ForgeQueryEvidenceTag::new("binding"), &binding_digest)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("assumption_snapshot"),
                    &assumption_snapshot_digest,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("precondition"),
                    &verified_precondition_digest,
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("asserted_aspect_path"),
                    asserted_aspect_paths.iter().map(String::as_str),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("target_binding_count"),
                    verification_read_set_breadth.target_binding_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("asserted_aspect_count"),
                    verification_read_set_breadth.asserted_aspect_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("distinct_asserted_aspect_path_count"),
                    verification_read_set_breadth.distinct_asserted_aspect_path_count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("cleared_assertion_count"),
                    verification_read_set_breadth.cleared_assertion_count(),
                )
                .seal();
        Self {
            binding_digest,
            asserted_aspect_paths,
            assumption_snapshot_identity: snapshot_identity,
            assumption_snapshot_evidence_identity,
            assumption_snapshot_digest,
            verified_precondition_digest,
            verification_read_set_breadth,
            verified_assumption_digest,
        }
    }

    pub fn binding_digest(&self) -> &str {
        self.binding_digest.as_str()
    }

    pub fn binding_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.binding_digest
    }

    pub fn asserted_aspect_paths(&self) -> &[String] {
        &self.asserted_aspect_paths
    }

    pub fn assumption_snapshot_token(&self) -> &str {
        self.assumption_snapshot_evidence_identity.reporting_projection()
    }

    pub fn assumption_snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.assumption_snapshot_identity
    }

    pub fn assumption_snapshot_digest(&self) -> &str {
        self.assumption_snapshot_digest.as_str()
    }

    pub fn assumption_snapshot_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.assumption_snapshot_digest
    }

    pub fn verified_precondition_digest(&self) -> &str {
        self.verified_precondition_digest.as_str()
    }

    pub fn verified_precondition_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.verified_precondition_digest
    }

    pub fn verification_read_set_breadth(&self) -> &ForgeQueryVerificationReadSetBreadth {
        &self.verification_read_set_breadth
    }

    pub fn verified_assumption_digest(&self) -> &str {
        self.verified_assumption_digest.as_str()
    }

    pub fn verified_assumption_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.verified_assumption_digest
    }
}
