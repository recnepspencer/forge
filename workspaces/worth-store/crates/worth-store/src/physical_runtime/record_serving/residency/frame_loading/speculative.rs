use worth_store_buffer_pool::{PrefetchResidencyGrant, ReadAheadFrameGrant};

use super::{
    read_source::CanonicalFrameReadSource, BoundedFrameLoader, FrameLoadFailure,
    FrameLoadFailureKind, LoadedPhysicalFrame,
};

impl BoundedFrameLoader {
    pub(in crate::physical_runtime::record_serving::residency) fn load_prefetch(
        &self,
        grant: &PrefetchResidencyGrant,
        source: &CanonicalFrameReadSource,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let coordinate = grant.frame().coordinate();
        let access = self
            .pool
            .access_prefetch_frame(grant)
            .map_err(|reason| FrameLoadFailure::new(FrameLoadFailureKind::Residency(reason)))?;
        self.load_admitted_exact(coordinate, access, || source.prepare_prefetch(grant))
    }

    pub(in crate::physical_runtime::record_serving::residency) fn load_read_ahead(
        &self,
        grant: &ReadAheadFrameGrant<'_, '_>,
        source: &CanonicalFrameReadSource,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let coordinate = grant.frame().coordinate();
        let access = self
            .pool
            .access_read_ahead_frame(grant)
            .map_err(|reason| FrameLoadFailure::new(FrameLoadFailureKind::Residency(reason)))?;
        self.load_admitted_exact(coordinate, access, || source.prepare_read_ahead(grant))
    }
}
