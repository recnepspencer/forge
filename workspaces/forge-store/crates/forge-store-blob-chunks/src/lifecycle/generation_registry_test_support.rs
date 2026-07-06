use forge_proof::TransitionOutcome;
use forge_store_contracts::StableDigest;
use forge_store_security::StoreTenantScope;

use crate::test_support::{admitted_multichunk_sequence_for_scope, blob_scope};
use crate::placement::admission::test_support::admit_inline_placement;
use crate::{
    AuthenticatedFrameDigest, BlobAuthorityClassification, BlobChunkReachabilityRegistry,
    BlobChunkRootPublication, BlobGeneration, BlobGenerationRegistryAdmission,
    BlobGenerationRegistryAuthority, BlobLifecycleAdmission, BlobLifecycleDeclaration,
    BlobLifecycleReadinessAuthority, BlobLifecycleStoreAuthority,
    BlobObjectClassificationAdmission, BlobObjectId, ChunkTreeRoot, LifecycleReceipt,
    LogicalContentDigest, ScopedBlobChunk, StoredChunkDigest,
};

pub(crate) fn registry_admission(
    case: &str,
    authority_classification: BlobAuthorityClassification,
) -> BlobGenerationRegistryAdmission {
    let (publication, stored_digest) = root_publication(case);
    let receipt = lifecycle_receipt_for_publication(
        case,
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        stored_digest,
        authority_classification,
    );
    let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
    BlobGenerationRegistryAdmission::from_executed_lifecycle(publication, receipt, classification)
}

pub(crate) fn root_publication(case: &str) -> (BlobChunkRootPublication, StoredChunkDigest) {
    root_publication_with_bytes(case, b"aaaabbbbcccc")
}

pub(crate) fn root_publication_with_bytes(
    case: &str,
    bytes: &[u8],
) -> (BlobChunkRootPublication, StoredChunkDigest) {
    root_publication_with_bytes_and_chunk_size(case, bytes, 12)
}

pub(crate) fn root_publication_with_bytes_and_chunk_size(
    case: &str,
    bytes: &[u8],
    chunk_size: u64,
) -> (BlobChunkRootPublication, StoredChunkDigest) {
    let sequence = admitted_multichunk_sequence_for_scope(
        blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
        bytes,
        chunk_size,
    );
    let stored_digest = scoped_chunk_with_bytes(case, bytes).stored_digest().clone();
    (
        BlobChunkRootPublication::publish(sequence).expect("root publication should admit"),
        stored_digest,
    )
}

pub(crate) fn lifecycle_receipt_for_publication(
    case: &str,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    authority_classification: BlobAuthorityClassification,
) -> LifecycleReceipt {
    lifecycle_receipt_for_publication_with_bytes(
        case,
        chunk_tree_root,
        logical_content_digest,
        stored_digest,
        authority_classification,
        b"aaaabbbbcccc",
    )
}

pub(crate) fn lifecycle_receipt_for_publication_with_bytes(
    case: &str,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    authority_classification: BlobAuthorityClassification,
    bytes: &[u8],
) -> LifecycleReceipt {
    lifecycle_receipt_for_publication_with_identity(
        case,
        case,
        1,
        chunk_tree_root,
        logical_content_digest,
        stored_digest,
        authority_classification,
        bytes,
    )
}

pub(crate) fn lifecycle_receipt_for_publication_with_identity(
    case: &str,
    object_case: &str,
    generation_sequence: u64,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    stored_digest: StoredChunkDigest,
    authority_classification: BlobAuthorityClassification,
    bytes: &[u8],
) -> LifecycleReceipt {
    let store_authority = BlobLifecycleStoreAuthority::from_current_store_authority(
        current_authority(case, "lifecycle"),
    );
    let lowering = store_authority.lowering_capability();
    let scoped_chunk = scoped_chunk_with_bytes(case, bytes);
    let declaration = declaration_with_identity(
        case,
        object_case,
        generation_sequence,
        chunk_tree_root,
        logical_content_digest,
        scoped_chunk.security_metadata(),
        stored_digest,
        authority_classification,
    );
    let reachability = reachability_proof_for_declaration(&declaration, scoped_chunk);
    let placement = admit_inline_placement(&reachability);
    let readiness = BlobLifecycleReadinessAuthority::from_admitted_placement(placement.clone());
    let ready = BlobLifecycleAdmission::start(declaration)
        .resolve_store_authority(store_authority)
        .lower_lifecycle_plan(lowering)
        .admit_reachability(reachability)
        .success("reachability should admit")
        .admit_placement(placement)
        .success("placement should admit")
        .ready_for_execution(readiness)
        .success("readiness should admit");
    let replay = ready.admitted_replay_input();
    ready
        .execute_lifecycle_replay(replay)
        .success("lifecycle should execute")
        .into_lifecycle_receipt()
}

fn reachability_proof_for_declaration(
    declaration: &BlobLifecycleDeclaration,
    scoped_chunk: ScopedBlobChunk,
) -> crate::BlobChunkReachabilityProofSet {
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry
        .admit_lifecycle_primary_reference(declaration, scoped_chunk)
        .expect("lifecycle reachability proof should admit")
}

pub(crate) fn registry_authority(case: &str) -> BlobGenerationRegistryAuthority {
    BlobGenerationRegistryAuthority::from_current_store_authority(current_authority(
        &format!("{case}.registry"),
        "registry",
    ))
}

pub(crate) fn digest(raw: &str) -> StableDigest {
    StableDigest::new(raw).expect("stable digest")
}

pub(crate) fn current_authority(
    identity_key: &str,
    value: &str,
) -> forge_store_authority::StoreCurrentAuthorityWitness {
    use forge_foundational::{
        aspects, AspectContract, AspectValue, InternedString, ScalarAspectType,
    };
    use forge_store_aspect_native::{
        StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
        StorePhysicalBoundaryWitness,
    };
    use forge_store_authority::require_current_store_authority;
    use forge_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };

    let key = aspects()
        .vocabulary()
        .key(identity_key)
        .expect("aspect key");
    let contract: AspectContract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let value = aspects()
        .validate()
        .against(&contract)
        .value(AspectValue::String(InternedString::from(value)))
        .success("aspect value should validate");
    let admitted_state = aspects()
        .authoritative_state()
        .admit([value])
        .success("aspect state should admit");
    let physical = StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary");
    let boundary = StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical),
    )
    .expect("boundary fact");
    require_current_store_authority(boundary)
}

fn declaration_with_identity(
    case: &str,
    object_case: &str,
    generation_sequence: u64,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    security_metadata: crate::BlobChunkSecurityMetadataWitness,
    stored_digest: StoredChunkDigest,
    authority_classification: BlobAuthorityClassification,
) -> BlobLifecycleDeclaration {
    BlobLifecycleDeclaration::new(
        BlobObjectId::from_declared_digest(digest(&format!("sha256:{object_case}.object"))),
        BlobGeneration::published(generation_sequence),
        chunk_tree_root,
        logical_content_digest,
        security_metadata,
        stored_digest,
        AuthenticatedFrameDigest::from_declared_digest(digest(&format!("sha256:{case}.frame"))),
        authority_classification,
    )
}

fn scoped_chunk_with_bytes(case: &str, bytes: &[u8]) -> ScopedBlobChunk {
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    ScopedBlobChunk::from_integrity_proof(
        crate::test_support::integrity_proof_for_scope(scope, bytes),
    )
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
