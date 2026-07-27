use worth_store_buffer_pool::{
    BufferPoolQueueDeclarationContext, BufferPoolQueueGroupingScope,
    BufferPoolReadQueueExecutionDeclaration, PhysicalOperationAllocationScope as Scope,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalSpeculativeWorkKind as Speculation,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_format::{
    store_namespace::{ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion},
    RecordArtifactFile, RecordFrameCoordinate,
};
use worth_store_security::StoreSecurityScopeIdentity;

pub fn read_ahead_declaration_for_real_pool(
    security: StoreSecurityScopeIdentity,
    flush_epoch: u64,
    resource_shape: QueueProducerResourceShape,
) -> BufferPoolReadQueueExecutionDeclaration {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([93; 16]).unwrap(),
    )
    .published_identity();
    let pool = PhysicalResidencyPool::open(
        store,
        PhysicalResidencyLimits::builder()
            .total_bytes(nonzero_bytes(16_384))
            .resident_bytes(nonzero_bytes(4096))
            .metadata_bytes(nonzero_bytes(4096))
            .frame_entries(nonzero_count(4))
            .pinned_frames(nonzero_count(2))
            .pin_leases(nonzero_count(2))
            .dirty_frames(nonzero_count(2))
            .dirty_replacement_bytes(nonzero_bytes(4096))
            .operation_bytes(nonzero_bytes(4096))
            .scope_bytes(Scope::ForegroundRead, nonzero_bytes(4096))
            .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(4096))
            .scope_bytes(Scope::Recovery, nonzero_bytes(4096))
            .scope_bytes(Scope::Scrub, nonzero_bytes(4096))
            .scope_bytes(Scope::Maintenance, nonzero_bytes(4096))
            .scope_bytes(Scope::Verification, nonzero_bytes(4096))
            .scope_bytes(Scope::Blob, nonzero_bytes(4096))
            .speculative_frames(Speculation::Prefetch, nonzero_count(2))
            .speculative_frames(Speculation::ReadAhead, nonzero_count(2))
            .speculative_frames(Speculation::WriteBehind, nonzero_count(2))
            .admit(nonzero_bytes(64))
            .unwrap(),
    )
    .unwrap();
    let coordinate =
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap();
    let allocation = pool
        .begin_foreground_read_operation(nonzero_bytes(64))
        .unwrap();
    let coordinates = [coordinate];
    let read_ahead = pool.admit_read_ahead(allocation, &coordinates).unwrap();
    let context = BufferPoolQueueDeclarationContext::new(
        BufferPoolQueueGroupingScope::new(security),
        flush_epoch,
        resource_shape,
    );
    BufferPoolReadQueueExecutionDeclaration::read_ahead(&read_ahead.frame(0).unwrap(), context)
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
