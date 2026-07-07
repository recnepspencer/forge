use forge_store_budgets::CounterEvidenceStrength;
use forge_store_physical_format::PhysicalGeneration;

use crate::lifecycle::generation_registry_test_support::{
    current_authority, digest, lifecycle_receipt_for_publication,
    lifecycle_receipt_for_publication_with_bytes, registry_admission, registry_authority,
    root_publication, root_publication_with_bytes,
};
use crate::{
    reject_chunk_tree_equality_as_blob_identity, reject_copied_lifecycle_receipt_as_blob_identity,
    reject_digest_equality_as_blob_identity, reject_physical_generation_as_blob_generation,
    reject_raw_generation_number_as_blob_identity, reject_semantic_reference_id_as_blob_identity,
    BlobAuthorityClassification, BlobGenerationRegistry, BlobGenerationRegistryAdmission,
    BlobGenerationRegistryDenial, BlobObjectClassificationAdmission, ChunkTreeRoot,
    DerivedBlobRebuildAuthority,
};

#[test]
fn repeated_registry_observations_resolve_to_the_same_published_generation() {
    let mut registry = BlobGenerationRegistry::new();
    let admission = registry_admission(
        "phase5-equivalence",
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let published = registry
        .publish(admission, registry_authority("phase5-equivalence"))
        .expect("registry publication should admit");

    let first = published.observe();
    let second = published.observe();

    assert_eq!(first.object_id(), second.object_id());
    assert_eq!(first.generation(), second.generation());
    assert_eq!(first.chunk_tree_root(), second.chunk_tree_root());
    assert_eq!(
        first.logical_content_digest(),
        second.logical_content_digest()
    );
    assert_eq!(first.classification(), second.classification());
    assert_eq!(first.lifecycle_receipt(), second.lifecycle_receipt());
    assert_eq!(first.counters().strength(), CounterEvidenceStrength::Exact);
    assert_eq!(first.counters().observations(), 1);
}

#[test]
fn weak_representations_have_typed_identity_denials() {
    let mut registry = BlobGenerationRegistry::new();
    let admission = registry_admission(
        "phase5-denials",
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let observation = registry
        .publish(admission, registry_authority("phase5-denials"))
        .expect("registry publication should admit")
        .observe();
    let physical_generation = PhysicalGeneration::from_raw(9).expect("physical generation");

    assert_eq!(
        reject_digest_equality_as_blob_identity(observation.logical_content_digest()),
        BlobGenerationRegistryDenial::DigestEqualityRejected
    );
    assert_eq!(
        reject_chunk_tree_equality_as_blob_identity(observation.chunk_tree_root()),
        BlobGenerationRegistryDenial::ChunkTreeEqualityRejected
    );
    assert_eq!(
        reject_copied_lifecycle_receipt_as_blob_identity(observation.lifecycle_receipt()),
        BlobGenerationRegistryDenial::CopiedLifecycleReceiptRejected
    );
    assert_eq!(
        reject_semantic_reference_id_as_blob_identity(&"semantic-ref-123"),
        BlobGenerationRegistryDenial::SemanticReferenceIdRejected
    );
    assert_eq!(
        reject_raw_generation_number_as_blob_identity(observation.generation().sequence()),
        BlobGenerationRegistryDenial::RawGenerationNumberRejected
    );
    assert_eq!(
        reject_physical_generation_as_blob_generation(&physical_generation),
        BlobGenerationRegistryDenial::PhysicalGenerationRejected
    );
}

#[test]
fn registry_denies_root_or_digest_drift_before_identity_publication() {
    let (publication, stored_digest) = root_publication("phase5-drift");
    let receipt = lifecycle_receipt_for_publication(
        "phase5-drift",
        ChunkTreeRoot::from_declared_digest(digest("sha256:wrong-root")),
        publication.logical_content_digest().clone(),
        stored_digest,
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let authority = registry_authority("phase5-drift");
    let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
    let mut registry = BlobGenerationRegistry::new();

    let denied = BlobGenerationRegistryAdmission::from_executed_lifecycle(
        publication,
        receipt,
        classification,
    )
    .publish(&mut registry, authority);

    assert!(matches!(
        denied,
        Err(BlobGenerationRegistryDenial::RootPublicationLifecycleRootMismatch { .. })
    ));
}

#[test]
fn derived_corruption_requires_admitted_rebuild_authority() {
    let mut registry = BlobGenerationRegistry::new();
    let admission = registry_admission(
        "phase5-derived",
        BlobAuthorityClassification::StoreOwnedDerivedBlob,
    );
    let published = registry
        .publish(admission, registry_authority("phase5-derived"))
        .expect("registry publication should admit");
    let corruption = published.classify_blob_corruption();
    let denied = corruption.deny_rebuild_without_authority();

    assert!(matches!(
        denied,
        BlobGenerationRegistryDenial::DerivedRebuildAuthorityRequired { .. }
    ));

    let rebuild = corruption
        .admit_rebuild(DerivedBlobRebuildAuthority::from_current_store_authority(
            current_authority("phase5-derived.rebuild", "rebuild"),
        ))
        .expect("derived rebuild admits with authority");
    assert_eq!(rebuild.counters().rebuild_admissions(), 1);
}

#[test]
fn authoritative_corruption_cannot_downgrade_into_derived_rebuild() {
    let mut registry = BlobGenerationRegistry::new();
    let admission = registry_admission(
        "phase5-authoritative",
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let published = registry
        .publish(admission, registry_authority("phase5-authoritative"))
        .expect("registry publication should admit");
    let denied = published.classify_blob_corruption().admit_rebuild(
        DerivedBlobRebuildAuthority::from_current_store_authority(current_authority(
            "phase5-authoritative.rebuild",
            "rebuild",
        )),
    );

    assert!(matches!(
        denied,
        Err(BlobGenerationRegistryDenial::AuthoritativeBlobRequiresAuthoritativeRepair { .. })
    ));
}

#[test]
fn registry_denies_conflicting_duplicate_generation_publication() {
    let mut registry = BlobGenerationRegistry::new();
    let first = registry_admission(
        "phase5-duplicate",
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    registry
        .publish(first, registry_authority("phase5-duplicate.first"))
        .expect("first publication should admit");

    let (publication, stored_digest) =
        root_publication_with_bytes("phase5-duplicate.second", b"ddddiiiiffff");
    let receipt = lifecycle_receipt_for_publication_with_bytes(
        "phase5-duplicate",
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        stored_digest,
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
        b"ddddiiiiffff",
    );
    let classification = BlobObjectClassificationAdmission::from_executed_lifecycle(&receipt);
    let denied = registry.publish(
        BlobGenerationRegistryAdmission::from_executed_lifecycle(
            publication,
            receipt,
            classification,
        ),
        registry_authority("phase5-duplicate.second"),
    );

    assert!(matches!(
        denied,
        Err(BlobGenerationRegistryDenial::BlobGenerationAlreadyBoundDifferently { .. })
    ));
}

#[test]
fn registry_denies_lifecycle_classification_downgrade() {
    let (publication, stored_digest) = root_publication("phase5-classification");
    let physical_receipt = lifecycle_receipt_for_publication(
        "phase5-classification",
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        stored_digest.clone(),
        BlobAuthorityClassification::StoreOwnedPhysicalBlob,
    );
    let derived_receipt = lifecycle_receipt_for_publication(
        "phase5-classification",
        publication.chunk_tree_root().clone(),
        publication.logical_content_digest().clone(),
        stored_digest,
        BlobAuthorityClassification::StoreOwnedDerivedBlob,
    );
    let derived_classification =
        BlobObjectClassificationAdmission::from_executed_lifecycle(&derived_receipt);
    let mut registry = BlobGenerationRegistry::new();
    let denied = BlobGenerationRegistryAdmission::from_executed_lifecycle(
        publication,
        physical_receipt,
        derived_classification,
    )
    .publish(&mut registry, registry_authority("phase5-classification"));

    assert!(matches!(
        denied,
        Err(BlobGenerationRegistryDenial::ClassificationLifecycleBindingMismatch { .. })
    ));
}
