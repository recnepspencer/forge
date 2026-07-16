use super::{admit_blob_import_source_custody, AdmittedBlobImportSourceCustody};

use super::counters::BlobImportReadmissionCounters;
use super::declaration::{BlobImportChunkDeclaration, BlobImportDeclaration};
use super::denial::BlobImportReadmissionDenial;

pub(crate) struct ClassifiedImportDeclaration {
    chunk_rows: Vec<BlobImportChunkDeclaration>,
    export_custody_scope: AdmittedBlobImportSourceCustody,
}

pub(crate) fn classify_import_declaration(
    declaration: &BlobImportDeclaration,
    counters: BlobImportReadmissionCounters,
) -> Result<ClassifiedImportDeclaration, BlobImportReadmissionDenial> {
    let export_custody_scope = admit_blob_import_source_custody(declaration.export_custody_scope())
        .map_err(|_| BlobImportReadmissionDenial::CustodyDomainMismatch {
            counters: counters.record_stale_scope_denial(),
        })?;
    let mut rows = declaration.chunk_rows().to_vec();
    rows.sort_by(|left, right| {
        left.ordinal()
            .cmp(&right.ordinal())
            .then_with(|| left.chunk_identity().cmp(right.chunk_identity()))
    });
    for pair in rows.windows(2) {
        if pair[0].ordinal() == pair[1].ordinal()
            || pair[0].chunk_identity() == pair[1].chunk_identity()
        {
            return Err(BlobImportReadmissionDenial::CopiedExportRowRejected {
                counters: counters.record_copied_row_denial(),
            });
        }
    }
    Ok(ClassifiedImportDeclaration {
        chunk_rows: rows,
        export_custody_scope,
    })
}

impl ClassifiedImportDeclaration {
    pub(crate) fn chunk_rows(&self) -> &[BlobImportChunkDeclaration] {
        &self.chunk_rows
    }

    pub(crate) const fn export_custody_scope(&self) -> AdmittedBlobImportSourceCustody {
        self.export_custody_scope
    }
}
