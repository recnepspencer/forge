use super::counters::BlobImportReadmissionCounters;
use super::denial::BlobImportReadmissionDenial;

pub(crate) fn reject_json_import_declaration(_: &str) -> BlobImportReadmissionDenial {
    BlobImportReadmissionDenial::ImportedJsonRejected {
        counters: BlobImportReadmissionCounters::start()
            .record_imported_declaration()
            .record_terminal_projection_denial(),
    }
}
