use crate::evidence_identity::forge_query_evidence_identity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::report::ForgeQueryTestBackendResidueFinding;

pub(super) fn derive_test_backend_residue_finding_identity(
    finding: &ForgeQueryTestBackendResidueFinding,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerTestBackendResidueFinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("source_path"),
            finding.source_path(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("residue_class"),
            finding.residue_class(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("matched_pattern"),
            finding.matched_pattern(),
        )
        .seal()
}

pub(super) fn derive_test_backend_residue_report_identity(
    consumer_name: &str,
    audited_roots: &[String],
    scanned_file_count: usize,
    finding_identities: &[ForgeQueryEvidenceIdentity],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerTestBackendResidueReport)
        .field_shape(ForgeQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("audited_root"),
            audited_roots.iter().map(String::as_str),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("scanned_file_count"),
            scanned_file_count,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}
