use super::tests_support::{
    admit_key_domain_scope, admitted_scope, alternate_blob_evidence_bundle,
    alternate_blob_identity, alternate_blob_import_declaration, page_slot_reference_admission,
    published_blob_evidence_bundle, published_blob_identity, published_blob_import_declaration,
};
use crate::layout_families::layout_declarations;
use crate::{ArtifactFamilyDenial, PhysicalKeyDomain};
use forge_store_contracts::{DurableArtifactFamilyId, WalRecordFamily};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use forge_store_wal::StoreWalRecordIdentity;

#[test]
fn wal_blob_and_physical_reference_domains_prove_real_hash_and_replay_behavior() {
    let wal_scope = admit_key_domain_scope(
        DurableArtifactFamilyId::WalRecoveryDecision,
        &admitted_scope(
            StoreKeyScope::WalCheckpointEnvelope,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let wal_domain = layout_declarations()
        .declare_physical_key_domain(wal_scope)
        .unwrap();
    let wal_encoding = layout_declarations().require_canonical_key_encoding(wal_domain);
    let wal_comparator = layout_declarations().declare_comparator_law(wal_encoding);
    let wal_hash = layout_declarations().declare_hash_collision_law(wal_domain);
    let wal_first = layout_declarations()
        .admit_wal_record_key(
            wal_domain,
            WalRecordFamily::RecoveryDecision,
            StoreWalRecordIdentity::new(41),
        )
        .unwrap();
    let wal_second = layout_declarations()
        .admit_wal_record_key(
            wal_domain,
            WalRecordFamily::RecoveryDecision,
            StoreWalRecordIdentity::new(42),
        )
        .unwrap();

    let wal_bytes = layout_declarations()
        .canonical_key_bytes(wal_comparator, wal_second.clone())
        .unwrap();

    assert_eq!(wal_domain.domain(), PhysicalKeyDomain::WalRecordKey);
    assert_eq!(
        wal_bytes.as_bytes(),
        [vec![0x30, 0x01, 0x05, 0x05], 42u64.to_be_bytes().to_vec()].concat()
    );
    assert_eq!(
        wal_hash.behavior(),
        crate::HashCollisionBehavior::ImpossibleByCanonicalIdentity
    );
    assert_eq!(
        layout_declarations()
            .compare_concrete_keys(wal_comparator, wal_first, wal_second.clone())
            .unwrap(),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        layout_declarations().require_exact_hash_identity_claim(wal_hash),
        Ok(wal_hash)
    );
    assert_eq!(
        wal_bytes,
        layout_declarations()
            .canonical_key_bytes(wal_comparator, wal_second)
            .unwrap()
    );

    let blob_scope = admit_key_domain_scope(
        DurableArtifactFamilyId::DedupeIndex,
        &admitted_scope(
            StoreKeyScope::BlobChunkEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let blob_domain = layout_declarations()
        .declare_physical_key_domain(blob_scope)
        .unwrap();
    let blob_encoding = layout_declarations().require_canonical_key_encoding(blob_domain);
    let blob_comparator = layout_declarations().declare_comparator_law(blob_encoding);
    let blob_prefix = layout_declarations()
        .require_prefix_law(blob_encoding)
        .unwrap();
    let blob_hash = layout_declarations().declare_hash_collision_law(blob_domain);
    let replay_bundle = published_blob_evidence_bundle();
    let certification_bundle = alternate_blob_evidence_bundle();
    let replay_import_declaration = published_blob_import_declaration();
    let certification_import_declaration = alternate_blob_import_declaration();
    let blob_identity = published_blob_identity();
    let blob_key = layout_declarations()
        .admit_blob_identity_key(blob_domain, blob_identity.clone())
        .unwrap();
    let blob_same = layout_declarations()
        .admit_blob_identity_key(blob_domain, blob_identity.clone())
        .unwrap();
    let other_blob_identity = alternate_blob_identity();
    let blob_other = layout_declarations()
        .admit_blob_identity_key(blob_domain, other_blob_identity)
        .unwrap();

    let prefix_bytes = layout_declarations()
        .prefix_bytes(blob_prefix, blob_key.clone())
        .unwrap();
    let prefix_successor = layout_declarations().prefix_successor_bytes(&prefix_bytes);

    assert_eq!(blob_domain.domain(), PhysicalKeyDomain::BlobIdentityKey);
    assert_eq!(
        layout_declarations().require_exact_hash_identity_claim(blob_hash),
        Err(ArtifactFamilyDenial::HashIdentityRequiresCollisionVerification)
    );
    assert_eq!(
        prefix_bytes.as_bytes(),
        [
            vec![0x40, 0x02, 0x06],
            blob_identity.object_digest().as_str().as_bytes().to_vec(),
        ]
        .concat()
        .as_slice()
    );
    assert!(prefix_successor.as_bytes() > prefix_bytes.as_bytes());
    layout_declarations()
        .verify_hash_identity(blob_hash, blob_key.clone(), blob_same)
        .unwrap();
    assert_eq!(
        layout_declarations().verify_hash_identity(blob_hash, blob_key, blob_other),
        Err(ArtifactFamilyDenial::HashIdentityRequiresCollisionVerification)
    );

    let replay_lifecycle_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                replay_bundle
                    .lifecycle_declaration()
                    .object_id()
                    .digest()
                    .clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    replay_bundle
                        .lifecycle_declaration()
                        .generation()
                        .sequence(),
                ),
            ),
        )
        .unwrap();
    let replay_import_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                replay_import_declaration.object_id().digest().clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    replay_import_declaration.generation().sequence(),
                ),
            ),
        )
        .unwrap();
    let replay_export_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                replay_bundle.export_object_id().digest().clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    replay_bundle.export_generation().sequence(),
                ),
            ),
        )
        .unwrap();
    let certification_export_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                certification_bundle.export_object_id().digest().clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    certification_bundle.export_generation().sequence(),
                ),
            ),
        )
        .unwrap();
    let certification_import_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                certification_import_declaration
                    .object_id()
                    .digest()
                    .clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    certification_import_declaration.generation().sequence(),
                ),
            ),
        )
        .unwrap();
    let certification_lifecycle_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                certification_bundle
                    .lifecycle_declaration()
                    .object_id()
                    .digest()
                    .clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    certification_bundle
                        .lifecycle_declaration()
                        .generation()
                        .sequence(),
                ),
            ),
        )
        .unwrap();

    let replay_primary_bytes = layout_declarations()
        .canonical_key_bytes(blob_comparator, replay_lifecycle_key.clone())
        .unwrap();
    let replay_import_bytes = layout_declarations()
        .canonical_key_bytes(blob_comparator, replay_import_key.clone())
        .unwrap();
    let replay_export_bytes = layout_declarations()
        .canonical_key_bytes(blob_comparator, replay_export_key.clone())
        .unwrap();
    let certification_lifecycle_bytes = layout_declarations()
        .canonical_key_bytes(blob_comparator, certification_lifecycle_key.clone())
        .unwrap();
    let certification_import_bytes = layout_declarations()
        .canonical_key_bytes(blob_comparator, certification_import_key.clone())
        .unwrap();
    let certification_export_bytes = layout_declarations()
        .canonical_key_bytes(blob_comparator, certification_export_key.clone())
        .unwrap();

    let replay_primary_prefix = layout_declarations()
        .prefix_bytes(blob_prefix, replay_lifecycle_key.clone())
        .unwrap();
    let replay_import_prefix = layout_declarations()
        .prefix_bytes(blob_prefix, replay_import_key.clone())
        .unwrap();
    let replay_export_prefix = layout_declarations()
        .prefix_bytes(blob_prefix, replay_export_key.clone())
        .unwrap();
    let certification_lifecycle_prefix = layout_declarations()
        .prefix_bytes(blob_prefix, certification_lifecycle_key.clone())
        .unwrap();
    let certification_import_prefix = layout_declarations()
        .prefix_bytes(blob_prefix, certification_import_key.clone())
        .unwrap();
    let certification_export_prefix = layout_declarations()
        .prefix_bytes(blob_prefix, certification_export_key.clone())
        .unwrap();

    let replay_primary_hash = layout_declarations()
        .hash_digest_for_key(blob_hash, replay_lifecycle_key.clone())
        .unwrap();
    let replay_import_hash = layout_declarations()
        .hash_digest_for_key(blob_hash, replay_import_key.clone())
        .unwrap();
    let replay_export_hash = layout_declarations()
        .hash_digest_for_key(blob_hash, replay_export_key.clone())
        .unwrap();
    let certification_lifecycle_hash = layout_declarations()
        .hash_digest_for_key(blob_hash, certification_lifecycle_key.clone())
        .unwrap();
    let certification_import_hash = layout_declarations()
        .hash_digest_for_key(blob_hash, certification_import_key.clone())
        .unwrap();
    let certification_export_hash = layout_declarations()
        .hash_digest_for_key(blob_hash, certification_export_key.clone())
        .unwrap();

    let replay_surface_rederived_key = layout_declarations()
        .admit_blob_identity_key(
            blob_domain,
            crate::S8BlobIdentityKeyBasis::new(
                replay_bundle
                    .lifecycle_declaration()
                    .object_id()
                    .digest()
                    .clone(),
                crate::S8BlobGenerationBasis::from_sequence(
                    replay_bundle
                        .lifecycle_declaration()
                        .generation()
                        .sequence(),
                ),
            ),
        )
        .unwrap();
    let replay_lifecycle_order = layout_declarations()
        .compare_concrete_keys(
            blob_comparator,
            replay_lifecycle_key,
            certification_lifecycle_key,
        )
        .unwrap();
    let replay_import_order = layout_declarations()
        .compare_concrete_keys(blob_comparator, replay_import_key, certification_import_key)
        .unwrap();
    let replay_export_order = layout_declarations()
        .compare_concrete_keys(blob_comparator, replay_export_key, certification_export_key)
        .unwrap();

    assert!(replay_bundle.export_matches_root_and_lifecycle_identity());
    assert!(certification_bundle.export_matches_root_and_lifecycle_identity());
    assert_eq!(replay_primary_bytes, replay_import_bytes);
    assert_eq!(replay_primary_bytes, replay_export_bytes);
    assert_eq!(certification_lifecycle_bytes, certification_import_bytes);
    assert_eq!(certification_lifecycle_bytes, certification_export_bytes);
    assert_eq!(replay_primary_prefix, replay_import_prefix);
    assert_eq!(replay_primary_prefix, replay_export_prefix);
    assert_eq!(certification_lifecycle_prefix, certification_import_prefix);
    assert_eq!(certification_lifecycle_prefix, certification_export_prefix);
    assert_eq!(replay_primary_hash, replay_import_hash);
    assert_eq!(replay_primary_hash, replay_export_hash);
    assert_eq!(certification_lifecycle_hash, certification_import_hash);
    assert_eq!(certification_lifecycle_hash, certification_export_hash);
    assert_eq!(
        replay_primary_bytes,
        layout_declarations()
            .canonical_key_bytes(blob_comparator, replay_surface_rederived_key)
            .unwrap()
    );
    assert_eq!(replay_lifecycle_order, replay_import_order);
    assert_eq!(replay_lifecycle_order, replay_export_order);

    let reference_scope = admit_key_domain_scope(
        DurableArtifactFamilyId::ReachabilityEdge,
        &admitted_scope(
            StoreKeyScope::ArtifactEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let reference_domain = layout_declarations()
        .declare_physical_key_domain(reference_scope)
        .unwrap();
    let reference_encoding = layout_declarations().require_canonical_key_encoding(reference_domain);
    let reference_comparator = layout_declarations().declare_comparator_law(reference_encoding);
    let reference_prefix = layout_declarations()
        .require_prefix_law(reference_encoding)
        .unwrap();
    let reference_key = layout_declarations()
        .admit_physical_reference_key(reference_domain, page_slot_reference_admission(7, 11, 3, 9))
        .unwrap();
    let reference_bytes = layout_declarations()
        .canonical_key_bytes(reference_comparator, reference_key.clone())
        .unwrap();
    let reference_prefix_bytes = layout_declarations()
        .prefix_bytes(reference_prefix, reference_key)
        .unwrap();

    assert_eq!(
        reference_domain.domain(),
        PhysicalKeyDomain::PhysicalReferenceKey
    );
    assert_eq!(reference_bytes.as_bytes()[3], 0x01);
    assert_eq!(
        reference_prefix_bytes.as_bytes(),
        [vec![0x23, 0x02, 0x03, 0x01], 7u64.to_be_bytes().to_vec()]
            .concat()
            .as_slice()
    );
}
