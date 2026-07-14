use super::counters::BlobImportReadmissionCounters;
use super::declaration::BoundaryBridgedCanonicalExportArtifact;
use super::denial::BlobImportReadmissionDenial;

pub fn parse_import_declaration_json(
    raw: &str,
) -> Result<BoundaryBridgedCanonicalExportArtifact, BlobImportReadmissionDenial> {
    Err(reject_json_import_declaration(raw))
}

pub(crate) fn reject_json_import_declaration(_: &str) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::ImportedJsonRejected {
        counters: BlobImportReadmissionCounters::start()
            .record_imported_declaration()
            .record_terminal_projection_denial(),
    }
}
