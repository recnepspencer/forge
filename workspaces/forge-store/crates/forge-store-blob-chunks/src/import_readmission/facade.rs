use forge_store_security::StoreTrustBoundaryReadmissionTrigger;

use super::admitted_import::ReadmittedBlobImport;
use super::authority::BlobImportReadmissionAuthority;
use super::chunk_evidence::BlobImportedChunkEvidence;
use super::classification::classify_import_declaration;
use super::counters::BlobImportReadmissionCounters;
use super::declaration::BoundaryBridgedCanonicalExportArtifact;
use super::denial::BlobImportReadmissionDenial;
use super::locality_verification::verify_declared_chunks;
use super::security_readmission::readmit_security_scope;
use super::witness::BlobImportReadmissionReceipt;

impl BlobImportReadmissionAuthority {
    pub fn readmit_import_declaration_after_boundary<'a>(
        &self,
        artifact: &'a BoundaryBridgedCanonicalExportArtifact,
        trigger: StoreTrustBoundaryReadmissionTrigger,
        current_chunks: &[BlobImportedChunkEvidence<'a>],
    ) -> Result<ReadmittedBlobImport<'a>, BlobImportReadmissionDenial> {
        let counters = BlobImportReadmissionCounters::start().record_imported_declaration();
        let classified = classify_import_declaration(artifact.declaration(), counters)?;
        let _export_custody_scope = classified.export_custody_scope();
        let security_metadata =
            readmit_security_scope(self, artifact.declaration(), &trigger, counters)?;
        let verified = verify_declared_chunks(
            classified.chunk_rows(),
            current_chunks,
            security_metadata,
            counters,
        )?;
        Ok(ReadmittedBlobImport::admitted(
            artifact,
            classified.chunk_rows().len() as u64,
            verified.local_chunks,
            verified.witness_basis,
            BlobImportReadmissionReceipt::new(
                security_metadata,
                self.current_authority().authority_identity(),
                counters.with_readmitted_chunks(verified.local_chunks),
            ),
        ))
    }
}
