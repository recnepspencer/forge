use forge_store_operations_vocabulary::{ImportPlacementDisposition, ImportPlacementPlan};

use super::declaration::BoundaryBridgedCanonicalExportArtifact;
use super::denial::BlobImportReadmissionDenial;
use super::locality_verification::ImportedBlobWitnessBasis;
use super::placement::plan_placement_admission;
use super::witness::{BlobImportReadmissionReceipt, ImportedBlobWitness};

#[derive(Debug)]
pub struct ReadmittedBlobImport<'a> {
    artifact: &'a BoundaryBridgedCanonicalExportArtifact,
    declared_chunks: u64,
    local_chunks: u64,
    witness_basis: Option<ImportedBlobWitnessBasis>,
    receipt: BlobImportReadmissionReceipt,
}

impl<'a> ReadmittedBlobImport<'a> {
    pub(super) fn admitted(
        artifact: &'a BoundaryBridgedCanonicalExportArtifact,
        declared_chunks: u64,
        local_chunks: u64,
        witness_basis: Option<ImportedBlobWitnessBasis>,
        receipt: BlobImportReadmissionReceipt,
    ) -> Self {
        Self {
            artifact,
            declared_chunks,
            local_chunks,
            witness_basis,
            receipt,
        }
    }

    pub fn plan_placement_admission(
        &self,
    ) -> Result<ImportPlacementPlan, BlobImportReadmissionDenial> {
        plan_placement_admission(
            self.artifact.declaration().placement_source(),
            self.declared_chunks,
            self.local_chunks,
            self.receipt.counters(),
        )
    }

    pub fn admit_imported_blob(
        &self,
        placement_plan: &ImportPlacementPlan,
    ) -> Result<ImportedBlobWitness, BlobImportReadmissionDenial> {
        match placement_plan.disposition() {
            ImportPlacementDisposition::AlreadyPresentLocally
            | ImportPlacementDisposition::DedupedLocally => {}
            ImportPlacementDisposition::RequiresFetch => return Err(self.missing_chunk_denial()),
            ImportPlacementDisposition::ScopeDenied => {
                return Err(self.placement_only_denial());
            }
        }
        let basis = self
            .witness_basis
            .as_ref()
            .expect("already-local placement implies complete witness basis");
        Ok(ImportedBlobWitness::new(
            self.artifact.declaration().object_id().clone(),
            self.artifact.declaration().generation(),
            self.artifact.declaration().chunk_tree_root().clone(),
            self.artifact.declaration().logical_content_digest().clone(),
            basis.chunk_security_metadata,
            basis.reachable_chunks.clone(),
            basis.stored_digest.clone(),
            *placement_plan,
            self.receipt.counters().record_witness_construction(),
        ))
    }

    pub const fn receipt(&self) -> &BlobImportReadmissionReceipt {
        &self.receipt
    }

    pub const fn declared_chunks(&self) -> u64 {
        self.declared_chunks
    }

    pub const fn local_chunks(&self) -> u64 {
        self.local_chunks
    }

    fn missing_chunk_denial(&self) -> BlobImportReadmissionDenial {
        BlobImportReadmissionDenial::MissingChunk {
            counters: self.receipt.counters().record_missing_chunk_denial(),
        }
    }

    fn placement_only_denial(&self) -> BlobImportReadmissionDenial {
        BlobImportReadmissionDenial::PlacementOnlyEvidenceRejected {
            counters: self.receipt.counters().record_placement_only_denial(),
        }
    }
}
