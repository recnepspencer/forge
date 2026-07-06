use forge_proof::TransitionOutcome;
use forge_store_security::StoreTenantScope;

use crate::lifecycle::generation_registry_test_support::current_authority;
use crate::publication::test_support::{durable_wal_publication, replayable_wal_classification};
use crate::test_support::{blob_scope, candidate_for_bytes_and_scope, canonical_equivalence};
use crate::{
    BlobChunkDedupeAdmission, BlobChunkOrdinal, BlobChunkRegisteredDedupeReference,
    BlobGenerationPublished, BlobVisibleGeneration,
};

pub(crate) fn registered_share_reference_for_first_chunk(
    scope_case: &str,
    chunk: &[u8],
) -> BlobChunkRegisteredDedupeReference {
    let existing = candidate_for_bytes_and_scope(
        chunk,
        blob_scope(scope_case, StoreTenantScope::TenantPhysicalBoundary),
    );
    let candidate = candidate_for_bytes_and_scope(
        chunk,
        blob_scope(scope_case, StoreTenantScope::TenantPhysicalBoundary),
    );
    let equivalence = canonical_equivalence(&existing, &candidate);
    let share_claim = match BlobChunkDedupeAdmission::compare_candidates(existing, candidate)
        .with_foundational_canonical_equivalence(equivalence)
        .admit()
    {
        TransitionOutcome::Success(claim) => claim,
        other => panic!("expected admitted share claim: {other:?}"),
    };
    let mut registry = crate::BlobChunkDedupeReferenceRegistry::new_store_owned();
    share_claim
        .admit_into_reference_registry(&mut registry)
        .expect("registered dedupe reference should admit")
}

pub(crate) fn publish_shared_scope_generation(
    scope_case: &str,
    object_case: &str,
    generation_sequence: u64,
    bytes: &[u8],
    chunk_size: u64,
) -> (BlobGenerationPublished, BlobVisibleGeneration) {
    let (root, stored_digest) = crate::lifecycle::generation_registry_test_support::
        root_publication_with_bytes_and_chunk_size(scope_case, bytes, chunk_size);
    let receipt = crate::lifecycle::generation_registry_test_support::
        lifecycle_receipt_for_publication_with_identity(
            scope_case,
            object_case,
            generation_sequence,
            root.chunk_tree_root().clone(),
            root.logical_content_digest().clone(),
            stored_digest,
            crate::BlobAuthorityClassification::StoreOwnedPhysicalBlob,
            bytes,
        );
    let object_id = receipt.declaration().object_id().clone();
    let generation = receipt.declaration().generation();
    let classification = crate::BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
    let mut registry = crate::BlobGenerationRegistry::new();
    crate::BlobGenerationRegistryAdmission::from_executed_lifecycle(
        root.clone(),
        receipt,
        classification,
    )
    .publish(
        &mut registry,
        crate::lifecycle::generation_registry_test_support::registry_authority(scope_case),
    )
    .expect("registry publication should admit");
    let observation = registry
        .observe_registered_generation(&object_id, generation)
        .expect("registered generation should observe");
    let reachability = observation.lifecycle_receipt().reachability().clone();
    let resumability = observation.lifecycle_receipt().resumability_receipt();
    let candidate = crate::BlobRootCandidateForPublication::from_registry_observation(
        observation,
        root,
    )
    .expect("root candidate should bind registry observation");
    let staged = crate::BlobReachabilityStaging::stage(candidate, reachability)
        .expect("reachability should stage");
    let payload = crate::BlobPublicationWalPayload::from_staged_reachability(&staged);
    let wal_commit = crate::BlobPublicationWalCommit::from_replayable_wal_record(
        staged,
        payload.clone(),
        durable_wal_publication(payload.frame_digest()),
        &replayable_wal_classification(payload.frame_digest()),
    )
    .expect("wal publication commit should admit");
    let wal_record = crate::BlobPublicationWalRecord::append(wal_commit);
    let closeout = crate::BlobPublicationSessionCloseout::close(wal_record, resumability)
        .expect("session closeout should admit");
    let published = BlobGenerationPublished::commit_visible(
        closeout,
        crate::BlobPublicationAuthority::from_current_store_authority(current_authority(
            &format!("{scope_case}.{object_case}.publication"),
            "publication",
        )),
    );
    let visible = BlobVisibleGeneration::from_published(&published);
    (published, visible)
}