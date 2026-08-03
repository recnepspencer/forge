use std::sync::Arc;

use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    pool::PoolInner, PhysicalResidencyDenial, PhysicalResidencyLimits, PhysicalResidencyPool,
};

#[derive(Debug)]
pub struct PhysicalResidencyPoolOwner {
    pool: PhysicalResidencyPool,
    candidate_clean: CandidateFrameCleanAuthority,
    writeback_clean: FrameWritebackCleanAuthority,
}

#[derive(Debug)]
pub struct CandidateFrameCleanAuthority {
    owner: Arc<PoolInner>,
}

#[derive(Debug)]
pub struct FrameWritebackCleanAuthority {
    owner: Arc<PoolInner>,
}

impl PhysicalResidencyPoolOwner {
    pub fn open(
        store: StableStoreIdentity,
        limits: PhysicalResidencyLimits,
    ) -> Result<Self, PhysicalResidencyDenial> {
        let pool = PhysicalResidencyPool::open(store, limits)?;
        Ok(Self {
            candidate_clean: CandidateFrameCleanAuthority {
                owner: Arc::clone(&pool.inner),
            },
            writeback_clean: FrameWritebackCleanAuthority {
                owner: Arc::clone(&pool.inner),
            },
            pool,
        })
    }

    pub fn into_parts(
        self,
    ) -> (
        PhysicalResidencyPool,
        CandidateFrameCleanAuthority,
        FrameWritebackCleanAuthority,
    ) {
        (self.pool, self.candidate_clean, self.writeback_clean)
    }
}

impl CandidateFrameCleanAuthority {
    pub(in crate::physical_residency) fn authorizes(&self, owner: &Arc<PoolInner>) -> bool {
        Arc::ptr_eq(&self.owner, owner)
    }
}

impl FrameWritebackCleanAuthority {
    pub(in crate::physical_residency) fn authorizes(&self, owner: &Arc<PoolInner>) -> bool {
        Arc::ptr_eq(&self.owner, owner)
    }
}
