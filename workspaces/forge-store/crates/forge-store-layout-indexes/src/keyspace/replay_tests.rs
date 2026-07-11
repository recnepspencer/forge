use super::tests_support::{admit_phase_four_scope, admitted_scope, page_id, segment_id};
use crate::layout_families::layout_declarations;
use crate::ArtifactFamilyDenial;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

#[test]
fn phase_four_scope_partitioning_denies_cross_scope_key_reuse_on_the_public_lane() {
    let page_declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalPage)
        .unwrap();
    let page_classification = layout_declarations().classify_family(page_declaration);
    let page_role = layout_declarations().declare_authority_role(page_classification);
    let page_accuracy = layout_declarations().declare_derived_accuracy_class(page_role);

    assert_eq!(
        layout_declarations().require_scope_partition(
            page_accuracy,
            admitted_scope(
                StoreKeyScope::PageEnvelope,
                StoreTenantScope::MultiTenantPhysicalBoundary,
                StoreAuthenticityRequirement::required(
                    StoreAuthenticityRequirementClass::AuthenticatedFrame,
                ),
                StoreCustodyPosture::InternalStoreCustody,
            )
            .witnesses(),
        ),
        Err(ArtifactFamilyDenial::CrossTenantScopePartitionDenied)
    );
    assert_eq!(
        layout_declarations().require_scope_partition(
            page_accuracy,
            admitted_scope(
                StoreKeyScope::ArtifactEnvelope,
                StoreTenantScope::TenantPhysicalBoundary,
                StoreAuthenticityRequirement::required(
                    StoreAuthenticityRequirementClass::AuthenticatedFrame,
                ),
                StoreCustodyPosture::InternalStoreCustody,
            )
            .witnesses(),
        ),
        Err(ArtifactFamilyDenial::CrossKeyScopePartitionDenied)
    );

    let blob_declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::DedupeIndex)
        .unwrap();
    let blob_classification = layout_declarations().classify_family(blob_declaration);
    let blob_role = layout_declarations().declare_authority_role(blob_classification);
    let blob_accuracy = layout_declarations().declare_derived_accuracy_class(blob_role);

    assert_eq!(
        layout_declarations().require_scope_partition(
            blob_accuracy,
            admitted_scope(
                StoreKeyScope::BlobChunkEnvelope,
                StoreTenantScope::MultiTenantPhysicalBoundary,
                StoreAuthenticityRequirement::required(
                    StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
                ),
                StoreCustodyPosture::InternalStoreCustody,
            )
            .witnesses(),
        ),
        Err(ArtifactFamilyDenial::CrossTenantScopePartitionDenied)
    );
    assert_eq!(
        layout_declarations().require_scope_partition(
            blob_accuracy,
            admitted_scope(
                StoreKeyScope::ArtifactEnvelope,
                StoreTenantScope::TenantPhysicalBoundary,
                StoreAuthenticityRequirement::required(
                    StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
                ),
                StoreCustodyPosture::InternalStoreCustody,
            )
            .witnesses(),
        ),
        Err(ArtifactFamilyDenial::CrossKeyScopePartitionDenied)
    );

    let page_scope = admit_phase_four_scope(
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
    let page_partition = layout_declarations().declare_tenant_scoped_key_domain(page_domain);
    let page_encoding = layout_declarations().require_canonical_key_encoding(page_domain);
    let page_comparator = layout_declarations().declare_comparator_law(page_encoding);
    let page_prefix = layout_declarations()
        .require_prefix_law(page_encoding)
        .unwrap();
    let page_range = layout_declarations()
        .require_range_bound_law(page_comparator)
        .unwrap();
    let page_key = layout_declarations()
        .admit_page_address_key(page_domain, segment_id(9), page_id(13))
        .unwrap();
    let page_bytes = layout_declarations()
        .canonical_key_bytes(page_comparator, page_key.clone())
        .unwrap();
    let page_prefix_bytes = layout_declarations()
        .prefix_bytes(page_prefix, page_key.clone())
        .unwrap();
    let page_start = layout_declarations()
        .range_start_bytes(page_range, page_key.clone())
        .unwrap();
    let page_end = layout_declarations()
        .range_end_bytes(page_range, page_key)
        .unwrap();

    let blob_scope = admit_phase_four_scope(
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
    let blob_partition = layout_declarations().declare_tenant_scoped_key_domain(blob_domain);

    assert_ne!(
        page_partition.key_partition(),
        blob_partition.key_partition()
    );
    assert_eq!(page_start.as_bytes(), page_bytes.as_bytes());
    assert_eq!(page_prefix_bytes.as_bytes(), &page_bytes.as_bytes()[..11]);
    assert!(page_end.as_bytes() > page_start.as_bytes());
    assert_eq!(page_bytes.as_bytes()[..3], [0x20, 0x02, 0x04]);
}
