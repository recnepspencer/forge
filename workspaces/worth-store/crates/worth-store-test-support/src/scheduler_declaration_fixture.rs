use worth_store_buffer_pool::{
    BufferPoolQueueExecutionDeclaration, BufferPoolQueueGroupingScope, PhysicalFrameKey,
    PhysicalResidencyLimits, PhysicalResidencyPool,
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
) -> BufferPoolQueueExecutionDeclaration {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([93; 16]).unwrap(),
    )
    .published_identity();
    let pool = PhysicalResidencyPool::open(
        store,
        PhysicalResidencyLimits::new_with_metadata_budget(4096, 4096, 2, 2, 4096, 4).unwrap(),
    )
    .unwrap();
    let frame = PhysicalFrameKey::new(
        store,
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap(),
    );
    BufferPoolQueueExecutionDeclaration::read_ahead(
        &pool,
        frame,
        BufferPoolQueueGroupingScope::new(security),
        flush_epoch,
        resource_shape,
    )
    .unwrap()
}
