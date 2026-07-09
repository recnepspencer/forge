use crate::evidence_identity::worth_query_evidence_identity;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::report::WorthQueryTestBackendResidueFinding;

pub(super) fn derive_test_backend_residue_finding_identity(
    finding: &WorthQueryTestBackendResidueFinding,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerTestBackendResidueFinding)
        .field_shape(
            WorthQueryEvidenceTag::new("source_path"),
            finding.source_path(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("residue_class"),
            finding.residue_class(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("matched_pattern"),
            finding.matched_pattern(),
        )
        .seal()
}

pub(super) fn derive_test_backend_residue_report_identity(
    consumer_name: &str,
    audited_roots: &[String],
    scanned_file_count: usize,
    finding_identities: &[WorthQueryEvidenceIdentity],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerTestBackendResidueReport)
        .field_shape(WorthQueryEvidenceTag::new("consumer_name"), consumer_name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("audited_root"),
            audited_roots.iter().map(String::as_str),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("scanned_file_count"),
            scanned_file_count,
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("finding_identity"),
            finding_identities.iter(),
        )
        .seal()
}
