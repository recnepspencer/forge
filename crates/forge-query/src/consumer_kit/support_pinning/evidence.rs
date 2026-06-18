use crate::evidence_identity::forge_query_evidence_identity;
use crate::runtime::ForgeQueryRuntimeFacadeFamily;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::evaluation::ForgeQuerySupportPinFinding;
use super::observed_row::ForgeQueryObservedSupportPin;
use super::requirement::ForgeQuerySupportPinRequirement;

pub(crate) fn derive_support_pin_requirement_identity(
    requirement: &ForgeQuerySupportPinRequirement,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportPinRequirement)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            requirement.family().as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("surface"), requirement.surface())
        .field_shape(
            ForgeQueryEvidenceTag::new("required_status"),
            requirement.required_status().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("required_teaching_posture"),
            requirement.required_teaching_posture().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pinned_live_row_digest"),
            requirement.pinned_live_row_digest(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pinned_snapshot_row_digest"),
            requirement.pinned_snapshot_row_digest(),
        )
        .seal()
}

pub(crate) fn derive_support_pin_observed_row_identity(
    observed: &ForgeQueryObservedSupportPin,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportPinObservedRow)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            observed.family().as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("surface"), observed.surface())
        .field_shape(
            ForgeQueryEvidenceTag::new("observed_status"),
            observed.observed_status(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("observed_teaching_posture"),
            observed.observed_teaching_posture(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("observed_live_row_digest"),
            observed.observed_live_row_digest(),
        )
        .seal()
}

pub(crate) fn derive_support_pin_contract_identity(
    consumer_name: &str,
    contract_schema_identity: &str,
    pinned_vocabulary_identity: &str,
    support_snapshot_schema_identity: &str,
    source_matrix_digest: &str,
    requirement_identities: &[ForgeQueryEvidenceIdentity],
    observed_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportPinContract)
        .field_shape(ForgeQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value(
            ForgeQueryEvidenceTag::new("contract_schema_identity"),
            contract_schema_identity,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("pinned_vocabulary_identity"),
            pinned_vocabulary_identity,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("support_snapshot_schema_identity"),
            support_snapshot_schema_identity,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("source_matrix_digest"),
            source_matrix_digest,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("requirement_count"),
            requirement_identities.len(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("requirement_identity"),
            requirement_identities.iter(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("observed_count"),
            observed_identities.len(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("observed_identity"),
            observed_identities.iter(),
        )
        .seal()
}

pub(crate) fn derive_support_pin_contract_document_identity(
    contract_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportPinContractDocument)
        .field_value(
            ForgeQueryEvidenceTag::new("contract_digest"),
            contract_digest,
        )
        .seal()
}

pub(crate) fn derive_support_pin_finding_identity(
    finding: &ForgeQuerySupportPinFinding,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportPinFinding)
        .field_shape(ForgeQueryEvidenceTag::new("kind"), finding.kind().as_str())
        .optional_shape(
            ForgeQueryEvidenceTag::new("family"),
            finding.family().map(ForgeQueryRuntimeFacadeFamily::as_str),
        )
        .field_shape(ForgeQueryEvidenceTag::new("surface"), finding.surface())
        .optional_value(ForgeQueryEvidenceTag::new("expected"), finding.expected())
        .optional_value(ForgeQueryEvidenceTag::new("found"), finding.found())
        .field_bool(ForgeQueryEvidenceTag::new("blocking"), finding.blocking())
        .seal()
}

pub(crate) fn derive_support_pin_report_identity(
    consumer_name: &str,
    contract_digest: &str,
    observed_schema_identity: &str,
    observed_source_matrix_digest: &str,
    observed_snapshot_digest: &str,
    requirement_count: usize,
    observed_count: usize,
    matched_required_count: usize,
    snapshot_row_count: usize,
    finding_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerSupportPinReport)
        .field_shape(ForgeQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value(
            ForgeQueryEvidenceTag::new("contract_digest"),
            contract_digest,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("observed_schema_identity"),
            observed_schema_identity,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("observed_source_matrix_digest"),
            observed_source_matrix_digest,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("observed_snapshot_digest"),
            observed_snapshot_digest,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("requirement_count"),
            requirement_count,
        )
        .field_usize(ForgeQueryEvidenceTag::new("observed_count"), observed_count)
        .field_usize(
            ForgeQueryEvidenceTag::new("matched_required_count"),
            matched_required_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("snapshot_row_count"),
            snapshot_row_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("finding_count"),
            finding_identities.len(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}
