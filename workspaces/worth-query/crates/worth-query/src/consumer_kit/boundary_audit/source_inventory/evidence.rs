use crate::evidence_identity::worth_query_evidence_identity;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::inventory::WorthQueryBoundaryAuditSourceInventoryFile;

pub(super) fn derive_source_inventory_identity(
    crate_name: &str,
    required_roots: &[String],
    files: &[WorthQueryBoundaryAuditSourceInventoryFile],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ConsumerBoundaryAuditSourceInventory)
        .field_shape(WorthQueryEvidenceTag::new("crate_name"), crate_name)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("required_root"),
            required_roots.iter().map(String::as_str),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("source_path"),
            files
                .iter()
                .map(WorthQueryBoundaryAuditSourceInventoryFile::source_path),
        )
        .field_usize(WorthQueryEvidenceTag::new("source_count"), files.len())
        .seal()
}
