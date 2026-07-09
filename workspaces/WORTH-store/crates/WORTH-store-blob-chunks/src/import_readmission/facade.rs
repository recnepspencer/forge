use worth_proof::TransitionOutcome;
use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_operations::{ImportPlacementDisposition, ImportPlacementPlan};
use worth_store_security::{
    accept_s5_1_admitted_security_scope_readiness, admit_store_security_scope,
    readmit_trust_boundary_security_scope_declaration, S51SecurityScopeReadinessReservation,
    StoreKeyVersionPosture, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope, StoreTrustBoundaryReadmissionTrigger,
};

use crate::{
    BlobChunkByteWindow, BlobChunkProofLeaf, BlobChunkSecurityMetadataWitness, StoredChunkDigest,
};

use super::chunk_evidence::BlobImportedChunkEvidence;
use super::classification::classify_import_declaration;
use super::counters::BlobImportReadmissionCounters;
use super::declaration::BoundaryBridgedCanonicalExportArtifact;
use super::denial::BlobImportReadmissionDenial;
use super::parsing::reject_json_import_declaration;
use super::placement::plan_placement_admission;
use super::witness::{BlobImportReadmissionReceipt, ImportedBlobWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportReadmissionAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

#[derive(Debug, Clone)]
struct ImportedBlobWitnessBasis {
    reachable_chunks: Vec<crate::BlobChunkIdentity>,
    stored_digest: StoredChunkDigest,
    chunk_security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobImportReadmissionAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub fn collect_current_chunk_evidence<'a>(
        &self,
        leaf: &BlobChunkProofLeaf,
        bytes: BlobChunkByteWindow<'a>,
    ) -> Result<BlobImportedChunkEvidence<'a>, BlobImportReadmissionDenial> {
        let _physical = self.current_authority.current_physical_authority();
        BlobImportedChunkEvidence::collect_from_leaf(
            leaf,
            bytes,
            BlobImportReadmissionCounters::start(),
        )
    }

    pub fn readmit_import_declaration_after_boundary<'a>(
        &self,
        artifact: &'a BoundaryBridgedCanonicalExportArtifact,
        trigger: StoreTrustBoundaryReadmissionTrigger,
        current_chunks: &[BlobImportedChunkEvidence<'a>],
    ) -> Result<ReadmittedBlobImport<'a>, BlobImportReadmissionDenial> {
        let counters = BlobImportReadmissionCounters::start().record_imported_declaration();
        let classified = classify_import_declaration(artifact.declaration(), counters)?;
        let _export_custody_scope = classified.export_custody_scope();
        let expectation = StoreSecurityScopeAdmissionExpectation::new(
            worth_store_security::StoreKeyScope::BlobChunkEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            worth_store_security::StoreAuthenticityRequirement::required(
                worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            worth_store_security::StoreCustodyPosture::Readmitted,
        );
        let readmitted = readmit_trust_boundary_security_scope_declaration(
            &self.current_authority,
            artifact.declaration().chunk_scope(),
            StoreKeyVersionPosture::Current,
            expectation,
            trigger.clone(),
        )
        .map_err(|source| map_triggered_security_denial(&trigger, source, counters))?;
        let admitted = match admit_store_security_scope(
            StoreSecurityScopeAdmissionRequest::from_raw_declaration(
                &self.current_authority,
                readmitted,
                expectation,
            ),
        ) {
            TransitionOutcome::Success(admitted) => admitted,
            TransitionOutcome::Denied(source) => {
                return Err(map_triggered_security_denial(&trigger, source, counters))
            }
            TransitionOutcome::Stale(_) | TransitionOutcome::RebindRequired(_) => {
                return Err(BlobImportReadmissionDenial::StaleKeyGeneration {
                    counters: counters.record_stale_scope_denial(),
                })
            }
            TransitionOutcome::Deferred(_) | TransitionOutcome::Failed(_) => {
                return Err(BlobImportReadmissionDenial::WrongTenantAuthority {
                    counters: counters.record_stale_scope_denial(),
                })
            }
        };
        let readiness = accept_s5_1_admitted_security_scope_readiness(
            S51SecurityScopeReadinessReservation::blob_chunk(),
            admitted,
        );
        let security_metadata = BlobChunkSecurityMetadataWitness::from_s5_1_readiness(readiness)
            .map_err(|_| BlobImportReadmissionDenial::WrongTenantAuthority {
                counters: counters.record_stale_scope_denial(),
            })?;
        let verified = verify_declared_chunks(
            classified.chunk_rows(),
            current_chunks,
            security_metadata,
            counters,
        )?;
        Ok(ReadmittedBlobImport {
            artifact,
            declared_chunks: classified.chunk_rows().len() as u64,
            local_chunks: verified.local_chunks,
            witness_basis: verified.witness_basis,
            receipt: BlobImportReadmissionReceipt::new(
                security_metadata,
                counters.with_readmitted_chunks(verified.local_chunks),
            ),
        })
    }
}

pub fn parse_import_declaration_json(
    raw: &str,
) -> Result<BoundaryBridgedCanonicalExportArtifact, BlobImportReadmissionDenial> {
    let _ = raw;
    Err(reject_json_import_declaration(raw))
}

#[derive(Debug)]
pub struct ReadmittedBlobImport<'a> {
    artifact: &'a BoundaryBridgedCanonicalExportArtifact,
    declared_chunks: u64,
    local_chunks: u64,
    witness_basis: Option<ImportedBlobWitnessBasis>,
    receipt: BlobImportReadmissionReceipt,
}

impl<'a> ReadmittedBlobImport<'a> {
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
            ImportPlacementDisposition::RequiresFetch => {
                return Err(BlobImportReadmissionDenial::MissingChunk {
                    counters: self.receipt.counters().record_missing_chunk_denial(),
                })
            }
            ImportPlacementDisposition::ScopeDenied => {
                return Err(BlobImportReadmissionDenial::PlacementOnlyEvidenceRejected {
                    counters: self.receipt.counters().record_placement_only_denial(),
                })
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
}

fn verify_declared_chunks(
    rows: &[super::BlobImportChunkDeclaration],
    current_chunks: &[BlobImportedChunkEvidence<'_>],
    readmitted_security_metadata: BlobChunkSecurityMetadataWitness,
    counters: BlobImportReadmissionCounters,
) -> Result<VerifiedChunkLocality, BlobImportReadmissionDenial> {
    let mut reachable_chunks = Vec::new();
    let mut stored_digest = None;
    let mut chunk_security_metadata = None;
    let mut local_chunks = 0_u64;
    for row in rows {
        let Some(chunk) = current_chunks.iter().find(|chunk| {
            let leaf = chunk.leaf();
            leaf.ordinal().get() == row.ordinal()
                && leaf.identity().chunk_digest().as_str() == row.chunk_identity()
                && leaf.stored_digest().digest().as_str() == row.stored_digest()
                && leaf.checksum_digest().as_str() == row.checksum_digest()
                && chunk.bytes().range().len() == row.bytes()
        }) else {
            continue;
        };
        let leaf_security = chunk.leaf().security_metadata();
        let scope_matches = leaf_security.key_scope() == readmitted_security_metadata.key_scope()
            && leaf_security.key_version_posture()
                == readmitted_security_metadata.key_version_posture()
            && leaf_security.tenant_scope() == readmitted_security_metadata.tenant_scope()
            && leaf_security.authenticity_requirement()
                == readmitted_security_metadata.authenticity_requirement();
        if !scope_matches {
            return Err(BlobImportReadmissionDenial::ChunkEvidenceMismatch { counters });
        }
        match chunk_security_metadata {
            Some(existing) if existing != leaf_security => {
                return Err(BlobImportReadmissionDenial::ChunkEvidenceMismatch { counters })
            }
            None => chunk_security_metadata = Some(leaf_security),
            _ => {}
        }
        stored_digest.get_or_insert_with(|| chunk.leaf().stored_digest().clone());
        reachable_chunks.push(chunk.leaf().identity().clone());
        local_chunks += 1;
    }
    let witness_basis = if local_chunks == rows.len() as u64 {
        Some(ImportedBlobWitnessBasis {
            reachable_chunks,
            stored_digest: stored_digest.expect("fully-local import implies stored digest"),
            chunk_security_metadata: chunk_security_metadata
                .expect("fully-local import implies security metadata"),
        })
    } else {
        None
    };
    Ok(VerifiedChunkLocality {
        local_chunks,
        witness_basis,
    })
}

fn map_triggered_security_denial(
    trigger: &StoreTrustBoundaryReadmissionTrigger,
    denial: worth_store_security::StoreSecurityScopeAdmissionDenial,
    counters: BlobImportReadmissionCounters,
) -> BlobImportReadmissionDenial {
    match trigger.crossing() {
        worth_store_security::StoreTrustBoundaryCrossing::KeyScopeGenerationChanged
        | worth_store_security::StoreTrustBoundaryCrossing::BackupRestoreAfterKeyRotation => {
            return BlobImportReadmissionDenial::StaleKeyGeneration {
                counters: counters.record_stale_scope_denial(),
            }
        }
        worth_store_security::StoreTrustBoundaryCrossing::TenantScopeAuthorityChanged => {
            return BlobImportReadmissionDenial::WrongTenantAuthority {
                counters: counters.record_stale_scope_denial(),
            }
        }
        worth_store_security::StoreTrustBoundaryCrossing::CustodyDomainChanged => {
            return BlobImportReadmissionDenial::CustodyDomainMismatch {
                counters: counters.record_stale_scope_denial(),
            }
        }
        _ => {}
    }
    match denial {
        worth_store_security::StoreSecurityScopeAdmissionDenial::DeniedKeyVersionPosture => {
            BlobImportReadmissionDenial::StaleKeyGeneration {
                counters: counters.record_stale_scope_denial(),
            }
        }
        worth_store_security::StoreSecurityScopeAdmissionDenial::WrongTenantScope => {
            BlobImportReadmissionDenial::WrongTenantAuthority {
                counters: counters.record_stale_scope_denial(),
            }
        }
        worth_store_security::StoreSecurityScopeAdmissionDenial::WrongCustodyPosture => {
            BlobImportReadmissionDenial::CustodyDomainMismatch {
                counters: counters.record_stale_scope_denial(),
            }
        }
        _ => BlobImportReadmissionDenial::WrongTenantAuthority {
            counters: counters.record_stale_scope_denial(),
        },
    }
}

#[derive(Debug, Clone)]
struct VerifiedChunkLocality {
    local_chunks: u64,
    witness_basis: Option<ImportedBlobWitnessBasis>,
}
