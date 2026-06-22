use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryVerificationReadSetBreadth, ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionAssumptionSummary {
    assumption_snapshot_digests: Vec<ForgeQueryEvidenceIdentity>,
    verified_precondition_digests: Vec<ForgeQueryEvidenceIdentity>,
    verified_step_count: usize,
    verification_read_set_breadth: ForgeQueryVerificationReadSetBreadth,
    counter_snapshot: String,
    aggregate_assumption_snapshot_digest: ForgeQueryEvidenceIdentity,
    aggregate_verified_precondition_digest: ForgeQueryEvidenceIdentity,
    assumption_summary_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphCompositionAssumptionSummary {
    pub(in crate::runtime) fn derive(write_receipts: &[ForgeQueryWriteReceipt]) -> Option<Self> {
        let assumption_sets = write_receipts
            .iter()
            .filter_map(|receipt| {
                receipt
                    .existing_truth_assertion_evidence()
                    .and_then(|evidence| evidence.verified_assumption_set())
            })
            .collect::<Vec<_>>();
        if assumption_sets.is_empty() {
            return None;
        }

        let assumption_snapshot_digests = assumption_sets
            .iter()
            .map(|set| set.assumption_snapshot_evidence_digest().clone())
            .collect::<Vec<_>>();
        let verified_precondition_digests = assumption_sets
            .iter()
            .map(|set| set.verified_precondition_evidence_digest().clone())
            .collect::<Vec<_>>();

        let target_binding_count = assumption_sets
            .iter()
            .map(|set| set.verification_read_set_breadth().target_binding_count())
            .sum();
        let asserted_aspect_count = assumption_sets
            .iter()
            .map(|set| set.verification_read_set_breadth().asserted_aspect_count())
            .sum();
        let asserted_aspects = assumption_sets
            .iter()
            .flat_map(|set| set.asserted_aspects().iter().cloned())
            .collect::<Vec<_>>();
        let asserted_aspect_touch_digests =
            native_asserted_aspect_touch_digest_parts(&asserted_aspects);
        let cleared_assertion_count = assumption_sets
            .iter()
            .map(|set| {
                set.verification_read_set_breadth()
                    .cleared_assertion_count()
            })
            .sum();
        let verification_read_set_breadth = ForgeQueryVerificationReadSetBreadth::new(
            target_binding_count,
            asserted_aspect_count,
            &asserted_aspects,
            cleared_assertion_count,
        );
        let verified_step_count = assumption_sets.len();
        let counter_snapshot = diagnostic_counter_snapshot_with_tail(
            &[("verified_steps", verified_step_count)],
            verification_read_set_breadth.counter_snapshot(),
        );
        let aggregate_assumption_snapshot_digest = aggregate_digest(
            "forge_query_graph_composition_assumption_snapshot_digest_v1",
            &assumption_snapshot_digests,
        );
        let aggregate_verified_precondition_digest = aggregate_digest(
            "forge_query_graph_composition_verified_precondition_digest_v1",
            &verified_precondition_digests,
        );
        let assumption_summary_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-assumption-summary",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("assumption_snapshots"),
                    &aggregate_assumption_snapshot_digest,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("verified_preconditions"),
                    &aggregate_verified_precondition_digest,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("verified_step_count"),
                    verified_step_count,
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
                    ForgeQueryEvidenceTag::new("cleared_assertion_count"),
                    verification_read_set_breadth.cleared_assertion_count(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("asserted_aspect_touch"),
                    asserted_aspect_touch_digests.iter().map(String::as_str),
                )
                .seal();

        Some(Self {
            assumption_snapshot_digests,
            verified_precondition_digests,
            verified_step_count,
            verification_read_set_breadth,
            counter_snapshot,
            aggregate_assumption_snapshot_digest,
            aggregate_verified_precondition_digest,
            assumption_summary_digest,
        })
    }

    pub fn assumption_snapshot_digests(&self) -> Vec<&str> {
        self.assumption_snapshot_digests
            .iter()
            .map(ForgeQueryEvidenceIdentity::as_str)
            .collect()
    }

    pub fn assumption_snapshot_evidence_digests(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.assumption_snapshot_digests
    }

    pub fn verified_precondition_digests(&self) -> Vec<&str> {
        self.verified_precondition_digests
            .iter()
            .map(ForgeQueryEvidenceIdentity::as_str)
            .collect()
    }

    pub fn verified_precondition_evidence_digests(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.verified_precondition_digests
    }

    pub fn verified_step_count(&self) -> usize {
        self.verified_step_count
    }

    pub fn verification_read_set_breadth(&self) -> &ForgeQueryVerificationReadSetBreadth {
        &self.verification_read_set_breadth
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }

    pub fn aggregate_assumption_snapshot_digest(&self) -> &str {
        self.aggregate_assumption_snapshot_digest.as_str()
    }

    pub fn aggregate_verified_precondition_digest(&self) -> &str {
        self.aggregate_verified_precondition_digest.as_str()
    }

    pub fn assumption_summary_digest(&self) -> &str {
        self.assumption_summary_digest.as_str()
    }

    pub fn assumption_summary_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.assumption_summary_digest
    }
}

fn diagnostic_counter_snapshot_with_tail(fields: &[(&str, usize)], tail: &str) -> String {
    let mut snapshot = diagnostic_counter_snapshot(fields);
    if !snapshot.is_empty() && !tail.is_empty() {
        snapshot.push(';');
    }
    snapshot.push_str(tail);
    snapshot
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

fn aggregate_digest(
    label: &str,
    digests: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(ForgeQueryEvidenceTag::new("role"), label)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("digest"), digests.iter())
        .seal()
}

fn native_asserted_aspect_touch_digest_parts(
    asserted_aspects: &[ForgeQueryAspectTouch],
) -> Vec<String> {
    asserted_aspects
        .iter()
        .map(|touch| touch.admitted_touch_digest_part().to_string())
        .collect()
}
