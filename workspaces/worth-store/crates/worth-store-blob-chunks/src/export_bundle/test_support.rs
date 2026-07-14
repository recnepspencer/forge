use worth_proof::TransitionOutcome;
use worth_store_offline_verifier::{OfflineExportChunkDeclaration, OfflineExportDigestEvidence};
use worth_store_operations_vocabulary::{
    BackupExportCustodyDeclaration, BackupExportCustodyMode, BackupExportCustodyReadiness,
};
use worth_store_security::{StoreKeyVersionPosture, StoreTenantScope};

use crate::placement::admission::test_support::admit_inline_placement;
use crate::reachability::BlobChunkReachabilityRegistry;
use crate::test_support::{admitted_multichunk_sequence_for_scope, blob_scope, current_authority};
use crate::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobChunkByteWindow, BlobChunkProofLeaf,
    BlobChunkRootPublication, BlobGeneration, BlobGenerationObservation, BlobGenerationRegistry,
    BlobGenerationRegistryAdmission, BlobLifecycleAdmission, BlobLifecycleDeclaration,
    BlobLifecycleReadinessAuthority, BlobLifecycleReplayInput, BlobLifecycleStoreAuthority,
    BlobObjectId,
};

use super::{BlobExportAuthority, BlobExportOfflineChunkDeclaration, BlobExportPublishedBundle};

pub(super) fn export_readiness(case: &str) -> BackupExportCustodyReadiness {
    let authority = current_authority(case, "export");
    let admission = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Export,
        StoreKeyVersionPosture::Current,
    )
    .expect("custody declaration should admit")
    .admit_with_current_authority(&authority)
    .expect("custody admission should succeed");
    BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("export readiness should build")
}

pub(crate) fn export_lane(
    authority: &BlobExportAuthority,
    case: &str,
    bytes: &'static [u8],
    chunk_size: u64,
) -> ExportLane<'static> {
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    let sequence = admitted_multichunk_sequence_for_scope(scope, bytes, chunk_size);
    let publication = BlobChunkRootPublication::publish(sequence.clone()).expect("publication");
    let ordered_leaves = sequence.proof_frontier().ordered_leaves().to_vec();
    let declaration = BlobLifecycleDeclaration::new(
        BlobObjectId::from_declared_digest(
            crate::lifecycle::generation_registry_test_support::digest(&format!(
                "sha256:{case}.object"
            )),
        ),
        BlobGeneration::published(1),
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        ordered_leaves[0].security_metadata(),
        ordered_leaves[0].stored_digest().clone(),
        AuthenticatedFrameDigest::from_declared_digest(
            crate::lifecycle::generation_registry_test_support::digest(&format!(
                "sha256:{case}.frame"
            )),
        ),
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let object_id = declaration.object_id().clone();
    let generation = declaration.generation();
    let mut reachability_registry = BlobChunkReachabilityRegistry::new_store_owned();
    let mut exported = Vec::new();
    for leaf in &ordered_leaves {
        let start = leaf.byte_range().start() as usize;
        let end = leaf.byte_range().end() as usize;
        let chunk_bytes = &bytes[start..end];
        let proof = crate::test_support::integrity_proof_for_scope(
            blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
            chunk_bytes,
        );
        reachability_registry
            .admit_lifecycle_primary_reference(
                &declaration,
                crate::ScopedBlobChunk::from_integrity_proof(proof),
            )
            .expect("reachability should admit");
        exported.push(
            authority
                .collect_exported_chunk_bytes(
                    leaf,
                    BlobChunkByteWindow::borrowed(leaf.byte_range().start(), chunk_bytes)
                        .expect("window"),
                )
                .expect("export input should admit"),
        );
    }
    let reachability = reachability_registry
        .prove_reachable_chunks()
        .expect("proof");
    let placement = admit_inline_placement(&reachability);
    let store_authority = BlobLifecycleStoreAuthority::from_current_store_authority(
        crate::lifecycle::generation_registry_test_support::current_authority(case, "lifecycle"),
    );
    let lowering = store_authority.lowering_capability();
    let readiness = BlobLifecycleReadinessAuthority::from_admitted_placement(placement.clone());
    let lifecycle = BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(reachability.clone())
        .success("reachability should admit")
        .admit_placement(placement.clone())
        .success("placement should admit")
        .ready_for_execution(readiness)
        .success("readiness should admit")
        .execute_lifecycle_replay(BlobLifecycleReplayInput::from_stored_chunk_digest(
            reachability.stored_digest().clone(),
        ))
        .success("lifecycle should execute")
        .into_lifecycle_receipt();
    let mut registry = BlobGenerationRegistry::new();
    let classification =
        crate::BlobObjectClassificationAdmission::from_executed_lifecycle(&lifecycle);
    BlobGenerationRegistryAdmission::from_executed_lifecycle(
        publication.clone(),
        lifecycle,
        classification,
    )
    .publish(
        &mut registry,
        crate::lifecycle::generation_registry_test_support::registry_authority(case),
    )
    .expect("registry publication should admit");
    ExportLane {
        registry,
        object_id,
        generation,
        publication,
        reachability,
        placement,
        exported,
        ordered_leaves,
    }
}

pub(super) fn ordered_multichunk_exported_chunks(
    authority: &BlobExportAuthority,
    case: &str,
    bytes: &'static [u8],
    chunk_size: u64,
) -> Vec<super::BlobExportedChunkBytes<'static>> {
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    let sequence = admitted_multichunk_sequence_for_scope(scope, bytes, chunk_size);
    sequence
        .proof_frontier()
        .ordered_leaves()
        .iter()
        .map(|leaf| {
            let start = leaf.byte_range().start() as usize;
            let end = leaf.byte_range().end() as usize;
            authority
                .collect_exported_chunk_bytes(
                    leaf,
                    BlobChunkByteWindow::borrowed(leaf.byte_range().start(), &bytes[start..end])
                        .expect("window"),
                )
                .expect("export input should admit")
        })
        .collect()
}

pub(crate) struct ExportLane<'a> {
    pub(crate) registry: BlobGenerationRegistry,
    pub(crate) object_id: BlobObjectId,
    pub(crate) generation: BlobGeneration,
    pub(crate) publication: BlobChunkRootPublication,
    pub(crate) reachability: crate::BlobChunkReachabilityProofSet,
    pub(crate) placement: crate::AdmittedBlobPlacement,
    pub(crate) exported: Vec<super::BlobExportedChunkBytes<'a>>,
    pub(crate) ordered_leaves: Vec<BlobChunkProofLeaf>,
}

impl ExportLane<'_> {
    pub(crate) fn observe(&self) -> BlobGenerationObservation<'_> {
        self.registry
            .observe_registered_generation(&self.object_id, self.generation)
            .expect("registered generation should observe")
    }
}

pub(super) fn offline_declarations(
    declarations: &[BlobExportOfflineChunkDeclaration],
) -> Vec<OfflineExportChunkDeclaration> {
    declarations
        .iter()
        .map(|chunk| OfflineExportChunkDeclaration {
            ordinal: chunk.ordinal(),
            chunk_identity: chunk.chunk_identity().to_owned(),
            stored_digest: chunk.stored_digest().to_owned(),
            checksum_digest: chunk.checksum_digest().to_owned(),
            bytes: chunk.bytes(),
        })
        .collect()
}

pub(super) fn offline_digest_evidence(
    bundle: &BlobExportPublishedBundle,
) -> OfflineExportDigestEvidence {
    OfflineExportDigestEvidence {
        logical_content_digest: bundle
            .digest_evidence()
            .logical_content_digest()
            .digest()
            .as_str()
            .to_owned(),
        export_bundle_digest: bundle
            .digest_evidence()
            .export_bundle_digest()
            .value()
            .bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
        declaration_digest: bundle
            .digest_evidence()
            .declaration_digest()
            .as_str()
            .to_owned(),
        declared_chunk_count: bundle.digest_evidence().declared_chunk_count(),
        declared_total_bytes: bundle.digest_evidence().declared_total_bytes(),
    }
}

pub(super) fn offline_declaration_digest(declarations: &[OfflineExportChunkDeclaration]) -> String {
    let mut hash = stable_hash_bytes(0xcbf2_9ce4_8422_2325, b"phase19.export.declarations");
    let total_bytes: u64 = declarations.iter().map(|chunk| chunk.bytes).sum();
    for declaration in declarations {
        hash = stable_hash_u64(hash, declaration.ordinal);
        hash = stable_hash_bytes(hash, declaration.chunk_identity.as_bytes());
        hash = stable_hash_bytes(hash, declaration.stored_digest.as_bytes());
        hash = stable_hash_bytes(hash, declaration.checksum_digest.as_bytes());
        hash = stable_hash_u64(hash, declaration.bytes);
    }
    hash = stable_hash_u64(hash, total_bytes);
    hash = stable_hash_u64(hash, declarations.len() as u64);
    format!("s7:export-declarations:{hash:016x}")
}

fn stable_hash_u64(hash: u64, value: u64) -> u64 {
    stable_hash_bytes(hash, &value.to_le_bytes())
}

fn stable_hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

trait TestTransitionSuccess<S> {
    fn success(self, message: &str) -> S;
}

impl<S, D, De, St, R, F> TestTransitionSuccess<S> for TransitionOutcome<S, D, De, St, R, F>
where
    S: core::fmt::Debug,
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    fn success(self, message: &str) -> S {
        match self {
            TransitionOutcome::Success(value) => value,
            outcome => panic!("{message}: {outcome:?}"),
        }
    }
}
