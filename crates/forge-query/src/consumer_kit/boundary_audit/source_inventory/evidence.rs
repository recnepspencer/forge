use crate::evidence_identity::forge_query_evidence_identity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::inventory::ForgeQueryBoundaryAuditSourceInventoryFile;

pub(super) fn derive_source_inventory_identity(
    crate_name: &str,
    required_roots: &[String],
    files: &[ForgeQueryBoundaryAuditSourceInventoryFile],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ConsumerBoundaryAuditSourceInventory)
        .field_shape(ForgeQueryEvidenceTag::new("crate_name"), crate_name)
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("required_root"),
            required_roots.iter().map(String::as_str),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("source_path"),
            files
                .iter()
                .map(ForgeQueryBoundaryAuditSourceInventoryFile::source_path),
        )
        .field_usize(ForgeQueryEvidenceTag::new("source_count"), files.len())
        .seal()
}
