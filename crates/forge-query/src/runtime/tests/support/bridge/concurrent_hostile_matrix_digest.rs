use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::projection_consumption::ProjectionFactConsumptionAttempt;
use crate::runtime::tests::support::*;
use crate::runtime::{
    ForgeQueryConcurrentHostileMatrixCounterSnapshot, ForgeQueryConcurrentHostileMatrixTopology,
};

pub(super) fn consume_artifact_title(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
) -> String {
    match hostile_consume_title_attempt(artifact) {
        ForgeQueryPublishedProjectionConsumption::Current(
            ProjectionFactConsumptionAttempt::Admitted(completed),
        ) => completed
            .facts()
            .display_fields()
            .first()
            .and_then(|fact| fact.value().as_str())
            .unwrap_or("none")
            .to_string(),
        ForgeQueryPublishedProjectionConsumption::ResultState(state) => {
            state.result_state_for_reporting().to_string()
        }
        other => panic!("unexpected projection consumption posture: {other:?}"),
    }
}

pub(super) fn raw_matrix_digest(
    topology: ForgeQueryConcurrentHostileMatrixTopology,
    receipt_digests: &[String],
    reader_result_digests: &[String],
    published_artifact_digests: &[String],
    preview_closeout_digests: &[String],
    branch_basis_digests: &[String],
    replay_digest: &str,
    counters: &ForgeQueryConcurrentHostileMatrixCounterSnapshot,
) -> String {
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("reader_thread_count"),
        topology.reader_thread_count(),
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("submitter_thread_count"),
        topology.submitter_thread_count(),
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("submission_round_count"),
        topology.submission_round_count(),
    )
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("receipt_digest"),
        receipt_digests.iter().map(String::as_str),
    )
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("reader_result_digest"),
        reader_result_digests.iter().map(String::as_str),
    )
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("published_artifact_digest"),
        published_artifact_digests.iter().map(String::as_str),
    )
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("preview_closeout_digest"),
        preview_closeout_digests.iter().map(String::as_str),
    )
    .field_value_sequence(
        ForgeQueryEvidenceTag::new("branch_basis_digest"),
        branch_basis_digests.iter().map(String::as_str),
    )
    .field_value(ForgeQueryEvidenceTag::new("replay_digest"), replay_digest)
    .field_usize(
        ForgeQueryEvidenceTag::new("counter_residue_count"),
        counters.exact_zero_residue_count(),
    )
    .field_usize(
        ForgeQueryEvidenceTag::new("registry_lease_count"),
        counters.published_artifact_registry_lease_count(),
    )
    .seal()
    .terminal_projection_for_reporting()
    .to_string()
}
