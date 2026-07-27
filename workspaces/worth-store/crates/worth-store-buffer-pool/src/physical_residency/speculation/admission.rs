use super::{PrefetchResidencyGrant, ReadAheadFrameGrant, ReadAheadResidencyGrant};
use crate::{
    ForegroundReadAllocationGrant, PhysicalFrameAccess, PhysicalOperationAllocationScope,
    PhysicalResidencyDenial, PhysicalResidencyPool, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::RecordFrameCoordinate;

impl PhysicalResidencyPool {
    pub fn admit_prefetch(
        &self,
        allocation: ForegroundReadAllocationGrant,
        coordinate: RecordFrameCoordinate,
    ) -> Result<PrefetchResidencyGrant, PhysicalResidencyDenial> {
        allocation.scope_for(&self.inner)?;
        let attempt = self
            .inner
            .begin_speculative_admission(PhysicalSpeculativeWorkKind::Prefetch);
        let count = self.validate_read_admission(
            &allocation,
            std::slice::from_ref(&coordinate),
            PhysicalSpeculativeWorkKind::Prefetch,
        )?;
        let permit = attempt.admit(PhysicalOperationAllocationScope::ForegroundRead, count)?;
        Ok(PrefetchResidencyGrant {
            permit,
            allocation,
            coordinate,
        })
    }

    pub fn admit_read_ahead<'coordinates>(
        &self,
        allocation: ForegroundReadAllocationGrant,
        coordinates: &'coordinates [RecordFrameCoordinate],
    ) -> Result<ReadAheadResidencyGrant<'coordinates>, PhysicalResidencyDenial> {
        allocation.scope_for(&self.inner)?;
        let attempt = self
            .inner
            .begin_speculative_admission(PhysicalSpeculativeWorkKind::ReadAhead);
        let count = self.validate_read_admission(
            &allocation,
            coordinates,
            PhysicalSpeculativeWorkKind::ReadAhead,
        )?;
        let permit = attempt.admit(PhysicalOperationAllocationScope::ForegroundRead, count)?;
        Ok(ReadAheadResidencyGrant {
            permit,
            allocation,
            coordinates,
        })
    }

    pub fn access_prefetch_frame(
        &self,
        grant: &PrefetchResidencyGrant,
    ) -> Result<PhysicalFrameAccess, PhysicalResidencyDenial> {
        self.access_frame(grant.allocation(), grant.frame())
    }

    pub fn access_read_ahead_frame(
        &self,
        grant: &ReadAheadFrameGrant<'_, '_>,
    ) -> Result<PhysicalFrameAccess, PhysicalResidencyDenial> {
        self.access_frame(grant.allocation(), grant.frame())
    }

    fn validate_read_admission(
        &self,
        allocation: &ForegroundReadAllocationGrant,
        coordinates: &[RecordFrameCoordinate],
        kind: PhysicalSpeculativeWorkKind,
    ) -> Result<u32, PhysicalResidencyDenial> {
        if coordinates.is_empty() {
            return Err(self
                .inner
                .record_denial(PhysicalResidencyDenial::EmptySpeculativeRead));
        }
        let count = u32::try_from(coordinates.len()).map_err(|_| {
            self.inner
                .record_denial(PhysicalResidencyDenial::AllocationFailed)
        })?;
        let required = coordinates.iter().try_fold(0_u64, |total, coordinate| {
            total
                .checked_add(u64::from(coordinate.length()))
                .ok_or_else(|| {
                    self.inner
                        .record_denial(PhysicalResidencyDenial::AllocationFailed)
                })
        })?;
        if allocation.bytes() != required {
            return Err(self.inner.record_denial(
                PhysicalResidencyDenial::SpeculativeAllocationMismatch {
                    granted: allocation.bytes(),
                    required,
                },
            ));
        }
        self.inner.require_bounded_speculative_validation(
            PhysicalOperationAllocationScope::ForegroundRead,
            kind,
            count,
        )?;
        for (index, coordinate) in coordinates.iter().enumerate() {
            if coordinates[..index].contains(coordinate) {
                return Err(self
                    .inner
                    .record_denial(PhysicalResidencyDenial::DuplicateSpeculativeFrame));
            }
        }
        Ok(count)
    }
}
