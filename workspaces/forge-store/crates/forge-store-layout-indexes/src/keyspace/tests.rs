use super::tests_support::{
    admit_key_domain_scope, admitted_scope, page_id, root_reference, segment_id,
};
use crate::{
    layout_declarations, ArtifactFamilyDenial, CompositeKeyField, HashCollisionBehavior,
    PhysicalKeyDomain,
};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use std::cmp::Ordering;

#[test]
fn admitted_domain_binds_family_security_and_exact_applicable_law_suite() {
    let page_security = admitted_scope(
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let page_declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .unwrap();
    let page_family = layout_declarations()
        .admit_physical_artifact_family(page_declaration, page_security.witnesses())
        .unwrap();
    let page = layout_declarations()
        .admit_physical_key_domain(page_family, page_security.witnesses())
        .unwrap();

    assert_eq!(page.family(), page_family);
    assert_eq!(page.domain(), PhysicalKeyDomain::PageAddressKey);
    assert_eq!(page.encoding().domain(), page.witness());
    assert_eq!(page.comparator().encoding(), page.encoding());
    assert!(page.prefix().is_some());
    assert!(page.range().is_some());
    assert_eq!(page.hash_collision().domain(), page.witness());
    assert_eq!(page.composite_ordering().domain(), page.witness());
    assert_eq!(page.tenant_partition().domain(), page.witness());

    let root_security = admitted_scope(
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    assert_eq!(
        layout_declarations().admit_physical_key_domain(page_family, root_security.witnesses()),
        Err(ArtifactFamilyDenial::SecurityAuthorityMismatch),
    );
    let root_declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
        .unwrap();
    let root_family = layout_declarations()
        .admit_physical_artifact_family(root_declaration, root_security.witnesses())
        .unwrap();
    let root = layout_declarations()
        .admit_physical_key_domain(root_family, root_security.witnesses())
        .unwrap();

    assert_eq!(root.domain(), PhysicalKeyDomain::RootManifestKey);
    assert!(root.prefix().is_none());
    assert!(root.range().is_none());
}

#[test]
fn pages_admit_concrete_bytes_order_prefix_and_range() {
    let scope = admit_key_domain_scope(
        DurableArtifactFamilyId::PhysicalPage,
        &admitted_scope(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let domain = layout_declarations()
        .declare_physical_key_domain(scope)
        .unwrap();
    let encoding = layout_declarations().require_canonical_key_encoding(domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);
    let range = layout_declarations()
        .require_range_bound_law(comparator)
        .unwrap();
    let prefix = layout_declarations().require_prefix_law(encoding).unwrap();
    let composite = layout_declarations().declare_composite_key_ordering(domain);
    let first = layout_declarations()
        .admit_page_address_key(domain, segment_id(7), page_id(11))
        .unwrap();
    let second = layout_declarations()
        .admit_page_address_key(domain, segment_id(7), page_id(12))
        .unwrap();

    let first_bytes = layout_declarations()
        .canonical_key_bytes(comparator, first.clone())
        .unwrap();
    let expected = [
        vec![0x20, 0x02, 0x04],
        7u64.to_be_bytes().to_vec(),
        11u64.to_be_bytes().to_vec(),
    ]
    .concat();

    assert_eq!(domain.domain(), PhysicalKeyDomain::PageAddressKey);
    assert_eq!(first_bytes.as_bytes(), expected.as_slice());
    assert_eq!(
        layout_declarations()
            .compare_concrete_keys(comparator, first.clone(), second.clone())
            .unwrap(),
        Ordering::Less
    );
    assert_eq!(
        composite.fields(),
        &[
            CompositeKeyField::VersionByte,
            CompositeKeyField::TenantScope,
            CompositeKeyField::KeyScope,
            CompositeKeyField::SegmentId,
            CompositeKeyField::PageId,
        ]
    );

    let prefix_bytes = layout_declarations()
        .prefix_bytes(prefix, first.clone())
        .unwrap();
    let successor = layout_declarations().prefix_successor_bytes(&prefix_bytes);
    let start = layout_declarations()
        .range_start_bytes(range, first.clone())
        .unwrap();
    let end = layout_declarations().range_end_bytes(range, first).unwrap();

    assert_eq!(prefix_bytes.as_bytes(), &expected[..11]);
    assert!(successor.as_bytes() > prefix_bytes.as_bytes());
    assert_eq!(start.as_bytes(), expected.as_slice());
    assert!(end.as_bytes() > start.as_bytes());
}

#[test]
fn root_manifest_denies_prefix_and_range_but_keeps_exact_identity() {
    let scope = admit_key_domain_scope(
        DurableArtifactFamilyId::PhysicalRootManifest,
        &admitted_scope(
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let domain = layout_declarations()
        .declare_physical_key_domain(scope)
        .unwrap();
    let encoding = layout_declarations().require_canonical_key_encoding(domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);
    let hash_law = layout_declarations().declare_hash_collision_law(domain);
    let key = layout_declarations()
        .admit_root_manifest_key(domain, root_reference(9))
        .unwrap();

    assert_eq!(domain.domain(), PhysicalKeyDomain::RootManifestKey);
    assert_eq!(
        hash_law.behavior(),
        HashCollisionBehavior::ImpossibleByCanonicalIdentity
    );
    assert_eq!(
        layout_declarations().require_range_bound_law(comparator),
        Err(ArtifactFamilyDenial::PhysicalKeyDomainDoesNotSupportRangeBounds)
    );
    assert_eq!(
        layout_declarations().require_prefix_law(encoding),
        Err(ArtifactFamilyDenial::PhysicalKeyDomainDoesNotSupportPrefixBounds)
    );
    assert_eq!(
        layout_declarations().require_exact_hash_identity_claim(hash_law),
        Ok(hash_law)
    );
    assert!(
        layout_declarations()
            .hash_digest_for_key(hash_law, key.clone())
            .unwrap()
            > 0
    );
    layout_declarations()
        .verify_hash_identity(hash_law, key.clone(), key)
        .unwrap();
}

#[test]
fn denies_cross_domain_concrete_key_shortcuts() {
    let page_scope = admit_key_domain_scope(
        DurableArtifactFamilyId::PhysicalPage,
        &admitted_scope(
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let page_domain = layout_declarations()
        .declare_physical_key_domain(page_scope)
        .unwrap();

    assert_eq!(
        layout_declarations().admit_root_manifest_key(page_domain, root_reference(3)),
        Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain)
    );
}

#[test]
fn denies_families_without_explicit_concrete_key_law() {
    let repair_scope = admit_key_domain_scope(
        DurableArtifactFamilyId::RepairRecord,
        &admitted_scope(
            StoreKeyScope::RepairScopeEnvelope,
            StoreTenantScope::RepairBlastRadius,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedRepairRead,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    );
    let transfer_scope = admit_key_domain_scope(
        DurableArtifactFamilyId::ExportBundle,
        &admitted_scope(
            StoreKeyScope::BackupExportEnvelope,
            StoreTenantScope::BackupRestoreBoundary,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
            ),
            StoreCustodyPosture::ExportPrepared,
        ),
    );

    assert_eq!(
        layout_declarations().declare_physical_key_domain(repair_scope),
        Err(ArtifactFamilyDenial::PhysicalKeyDomainNotDeclaredForFamily)
    );
    assert_eq!(
        layout_declarations().declare_physical_key_domain(transfer_scope),
        Err(ArtifactFamilyDenial::PhysicalKeyDomainNotDeclaredForFamily)
    );
}
