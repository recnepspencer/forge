use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalRootManifestRebuildSource, PhysicalSegmentId, PlatformPhysicalAppendRequest,
};
use forge_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind, BlobWalReplayRebuildWitness,
    DurablePublicationDeclaration, WalFrameDurablePublicationScope, WalSecurityMetadataCarrier,
    WalSecurityMetadataEnvelope,
};

pub(crate) fn root_manifest_source_witness(
    segment: u64,
    page: u64,
) -> PhysicalRootManifestRebuildSource {
    root_manifest_rebuild_source_rows_for_store(
        segment,
        page,
        1,
        crate::strategy::tests_support::strategy_test_store_identity(),
    )
}

pub(crate) fn root_manifest_source_witness_rows(
    segment: u64,
    row_count: u64,
) -> PhysicalRootManifestRebuildSource {
    root_manifest_rebuild_source_rows_for_store(
        segment,
        1,
        row_count,
        crate::strategy::tests_support::strategy_test_store_identity(),
    )
}

pub(crate) fn root_manifest_source_witness_for_store(
    segment: u64,
    page: u64,
    store_identity: forge_store_physical_format::PhysicalStoreIdentity,
) -> PhysicalRootManifestRebuildSource {
    root_manifest_rebuild_source_rows_for_store(segment, page, 1, store_identity)
}

fn root_manifest_rebuild_source_rows_for_store(
    segment: u64,
    first_page: u64,
    row_count: u64,
    store_identity: forge_store_physical_format::PhysicalStoreIdentity,
) -> PhysicalRootManifestRebuildSource {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment_id = PhysicalSegmentId::from_raw(segment).unwrap();
    let mut runtime =
        crate::bootstrap::test_support::open_physical_facade_for_store(store_identity);
    for page in first_page..first_page + row_count {
        let slot = generations
            .slot_cell(
                segment_id,
                PhysicalPageId::from_raw(page).unwrap(),
                PhysicalRecordSlot::from_raw(1).unwrap(),
            )
            .with_slot_generation(PhysicalGeneration::from_raw(7).unwrap());
        runtime
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot,
                format!("rebuild-row-{segment}-{page}").as_bytes(),
            ))
            .expect("rebuild fixture page append");
    }
    runtime
        .publish_physical_root()
        .expect("rebuild fixture root publication");
    runtime
        .root_manifest_rebuild_source()
        .expect("opened Store must issue its root-manifest rebuild source")
}

pub(crate) fn wal_replay_source_witness(
    materialization: &crate::AdmittedLayoutMaterialization,
    kind: BlobWalRecordKind,
) -> BlobWalReplayRebuildWitness {
    let security =
        forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    wal_replay_source_witness_with_security(materialization, kind, security.witnesses())
}

pub(crate) fn wal_replay_source_witness_with_security(
    materialization: &crate::AdmittedLayoutMaterialization,
    kind: BlobWalRecordKind,
    security: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
) -> BlobWalReplayRebuildWitness {
    let crate::LayoutMaterializationSourceKind::LsmReplacement(replacement) =
        materialization.source().kind()
    else {
        panic!("WAL rebuild fixture requires an LSM replacement materialization");
    };
    wal_replay_source_witness_for_identity(
        BlobWalRecordIdentity::new(replacement.sequence(), kind).unwrap(),
        security,
    )
}

pub(crate) fn wal_replay_source_witness_for_identity(
    identity: BlobWalRecordIdentity,
    security: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
) -> BlobWalReplayRebuildWitness {
    let record = BlobWalRecordEnvelope::new(
        identity,
        DurablePublicationDeclaration::wal_frame(
            WalFrameDurablePublicationScope::new(1, 1, 10, 20, "sha256:wal-frame", 4096).unwrap(),
        ),
        "sha256:payload",
    )
    .unwrap();
    let metadata = WalSecurityMetadataCarrier::for_wal_record(
        security,
        forge_store_security::StoreKeyVersionPosture::Current,
        forge_store_security::StoreLegacySecurityPosture::NativeScoped,
    );
    BlobWalReplayRebuildWitness::admit(WalSecurityMetadataEnvelope::wal_record(record, metadata))
}
