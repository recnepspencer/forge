use worth_store_buffer_pool::{
    BufferPoolQueueExecutionDeclaration, BufferPoolQueueGroupingScope, PhysicalFrameKey,
    PhysicalResidencyLimits, PhysicalResidencyPool,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_format::{
    store_namespace::{ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion},
    RecordArtifactFile, RecordFrameCoordinate,
};

pub(crate) fn buffer_pool_declaration(
    write_back: bool,
    security: worth_store_security::StoreSecurityScopeIdentity,
    resource_shape: QueueProducerResourceShape,
) -> BufferPoolQueueExecutionDeclaration {
    let store = StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([31; 16]).unwrap(),
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
    let grouping = BufferPoolQueueGroupingScope::new(security);
    if write_back {
        BufferPoolQueueExecutionDeclaration::write_back(&pool, frame, grouping, 7, resource_shape)
            .unwrap()
    } else {
        BufferPoolQueueExecutionDeclaration::read_ahead(&pool, frame, grouping, 7, resource_shape)
            .unwrap()
    }
}
