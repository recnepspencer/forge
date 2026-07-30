use super::SpeculativeResidencyPermit;
use crate::{ForegroundReadAllocationGrant, PhysicalFrameKey, PhysicalResidencyIncarnation};
use worth_store_physical_format::RecordFrameCoordinate;

/// Exact one-frame prefetch authority issued by the owning residency pool.
#[derive(Debug)]
pub struct PrefetchResidencyGrant {
    pub(super) permit: SpeculativeResidencyPermit,
    pub(super) allocation: ForegroundReadAllocationGrant,
    pub(super) coordinate: RecordFrameCoordinate,
}

/// Ordered, nonempty read-ahead authority issued by the owning residency pool.
#[derive(Debug)]
pub struct ReadAheadResidencyGrant<'coordinates> {
    pub(super) permit: SpeculativeResidencyPermit,
    pub(super) allocation: ForegroundReadAllocationGrant,
    pub(super) coordinates: &'coordinates [RecordFrameCoordinate],
}

/// Borrowed authority for one exact frame within a read-ahead grant.
#[derive(Debug)]
pub struct ReadAheadFrameGrant<'grant, 'coordinates> {
    grant: &'grant ReadAheadResidencyGrant<'coordinates>,
    index: usize,
}

impl PrefetchResidencyGrant {
    pub fn frame(&self) -> PhysicalFrameKey {
        PhysicalFrameKey::new(self.store_identity(), self.coordinate)
    }

    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.permit.owner.store_identity()
    }

    pub fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.permit.owner.incarnation()
    }

    pub(in crate::physical_residency) const fn allocation(&self) -> &ForegroundReadAllocationGrant {
        &self.allocation
    }
}

impl<'coordinates> ReadAheadResidencyGrant<'coordinates> {
    pub const fn coordinates(&self) -> &'coordinates [RecordFrameCoordinate] {
        self.coordinates
    }

    pub fn frame(&self, index: usize) -> Option<ReadAheadFrameGrant<'_, 'coordinates>> {
        self.coordinates
            .get(index)
            .map(|_| ReadAheadFrameGrant { grant: self, index })
    }

    pub fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.permit.owner.store_identity()
    }

    pub fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.permit.owner.incarnation()
    }
}

impl ReadAheadFrameGrant<'_, '_> {
    pub fn frame(&self) -> PhysicalFrameKey {
        PhysicalFrameKey::new(self.store_identity(), self.coordinate())
    }

    pub fn coordinate(&self) -> RecordFrameCoordinate {
        self.grant.coordinates[self.index]
    }

    pub fn store_identity(
        &self,
    ) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        self.grant.store_identity()
    }

    pub fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.grant.pool_incarnation()
    }

    pub(in crate::physical_residency) const fn allocation(&self) -> &ForegroundReadAllocationGrant {
        &self.grant.allocation
    }
}
