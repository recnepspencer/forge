use worth_store_buffer_pool::{
    BufferPoolQueueDeclarationContext, BufferPoolQueueGroupingScope,
    BufferPoolQueueWriteDurability, BufferPoolReadQueueExecutionDeclaration,
    BufferPoolWritebackQueueExecutionDeclaration, PhysicalFrameAccess, PhysicalFrameKey,
    PhysicalOperationAllocationScope as Scope, PhysicalResidencyLimits, PhysicalResidencyPool,
    PhysicalSpeculativeWorkKind as Speculation,
};
use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{
    store_namespace::{ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion},
    RecordArtifactFile, RecordFrameCoordinate,
};

pub(crate) fn buffer_pool_read_ahead_declaration(
    security: worth_store_security::StoreSecurityScopeIdentity,
    resource_shape: QueueProducerResourceShape,
) -> BufferPoolReadQueueExecutionDeclaration {
    BufferPoolDeclarationFixture::new(security, resource_shape).read_ahead()
}

pub(crate) fn buffer_pool_writeback_declaration(
    durability: ArtifactRangeWriteDurabilityRequirement,
    security: worth_store_security::StoreSecurityScopeIdentity,
    resource_shape: QueueProducerResourceShape,
) -> BufferPoolWritebackQueueExecutionDeclaration {
    BufferPoolDeclarationFixture::new(security, resource_shape).writeback(durability)
}

pub(crate) fn buffer_pool_prefetch_declaration(
    security: worth_store_security::StoreSecurityScopeIdentity,
    resource_shape: QueueProducerResourceShape,
) -> BufferPoolReadQueueExecutionDeclaration {
    BufferPoolDeclarationFixture::new(security, resource_shape).prefetch()
}

struct BufferPoolDeclarationFixture {
    pool: PhysicalResidencyPool,
    coordinate: RecordFrameCoordinate,
    context: BufferPoolQueueDeclarationContext,
}

impl BufferPoolDeclarationFixture {
    fn new(
        security: worth_store_security::StoreSecurityScopeIdentity,
        resource_shape: QueueProducerResourceShape,
    ) -> Self {
        let store = fixture_store();
        let pool = PhysicalResidencyPool::open(store, fixture_limits()).unwrap();
        let coordinate =
            RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap();
        let grouping = BufferPoolQueueGroupingScope::new(security);
        let context = BufferPoolQueueDeclarationContext::new(grouping, 7, resource_shape);
        Self {
            pool,
            coordinate,
            context,
        }
    }

    fn prefetch(self) -> BufferPoolReadQueueExecutionDeclaration {
        let allocation = self
            .pool
            .begin_foreground_read_operation(nonzero_bytes(64))
            .unwrap();
        let prefetch = self
            .pool
            .admit_prefetch(allocation, self.coordinate)
            .unwrap();
        BufferPoolReadQueueExecutionDeclaration::prefetch(&prefetch, self.context)
    }

    fn read_ahead(self) -> BufferPoolReadQueueExecutionDeclaration {
        let allocation = self
            .pool
            .begin_foreground_read_operation(nonzero_bytes(64))
            .unwrap();
        let coordinates = [self.coordinate];
        let read_ahead = self
            .pool
            .admit_read_ahead(allocation, &coordinates)
            .unwrap();
        BufferPoolReadQueueExecutionDeclaration::read_ahead(
            &read_ahead.frame(0).unwrap(),
            self.context,
        )
    }

    fn writeback(
        self,
        durability: ArtifactRangeWriteDurabilityRequirement,
    ) -> BufferPoolWritebackQueueExecutionDeclaration {
        let frame = PhysicalFrameKey::new(self.pool.store_identity(), self.coordinate);
        let allocation = self
            .pool
            .begin_foreground_write_operation(nonzero_bytes(64))
            .unwrap();
        let clean = match self.pool.access_frame(&allocation, frame).unwrap() {
            PhysicalFrameAccess::Fault(fault) => fault
                .load(|bytes| {
                    bytes.fill(0x51);
                    Ok::<_, std::convert::Infallible>(())
                })
                .unwrap(),
            _ => panic!("fresh scheduler fixture frame must fault"),
        };
        let _dirty = clean
            .begin_dirty_replacement(&allocation)
            .unwrap()
            .replace(|source, target| {
                target.copy_from_slice(source);
                Ok::<_, std::convert::Infallible>(())
            })
            .unwrap();
        let claim_allocation = self
            .pool
            .begin_foreground_write_operation(nonzero_bytes(64))
            .unwrap();
        let claim = self
            .pool
            .claim_writeback(claim_allocation, &[frame])
            .unwrap();
        BufferPoolWritebackQueueExecutionDeclaration::for_claim(
            &claim,
            self.context,
            queue_writeback_durability(durability),
        )
        .unwrap()
    }
}

fn fixture_store() -> worth_store_physical_format::store_namespace::StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([31; 16]).unwrap(),
    )
    .published_identity()
}

fn fixture_limits() -> PhysicalResidencyLimits {
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
        .unwrap()
}

const fn queue_writeback_durability(
    durability: ArtifactRangeWriteDurabilityRequirement,
) -> BufferPoolQueueWriteDurability {
    match durability {
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite => {
            BufferPoolQueueWriteDurability::BufferedWrite
        }
        ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization => {
            BufferPoolQueueWriteDurability::FileDataSynchronization
        }
    }
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}
