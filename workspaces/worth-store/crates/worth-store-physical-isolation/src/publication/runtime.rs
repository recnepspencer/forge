use super::{CopyOnWritePublicationPlan, PhysicalPublicationDenial, ReadCopyUpdateRootPublication};
use crate::CurrentPhysicalRoot;

/// Owns the mutable root against which copy-on-write plans execute.
#[derive(Debug, Clone)]
pub struct PhysicalRootPublicationRuntime {
    current_root: CurrentPhysicalRoot,
}

impl PhysicalRootPublicationRuntime {
    pub const fn from_current_root(current_root: CurrentPhysicalRoot) -> Self {
        Self { current_root }
    }

    pub const fn current_root(&self) -> CurrentPhysicalRoot {
        self.current_root
    }

    pub fn publish(
        &mut self,
        plan: CopyOnWritePublicationPlan,
    ) -> Result<ReadCopyUpdateRootPublication, PhysicalPublicationDenial> {
        let planned_old_root = plan.intent().old_root();
        if planned_old_root.store_authority_identity()
            != self.current_root.store_authority_identity()
        {
            return Err(PhysicalPublicationDenial::StoreAuthorityMismatch);
        }
        if planned_old_root != self.current_root {
            return Err(PhysicalPublicationDenial::StaleRootPublicationEpoch);
        }

        let published = ReadCopyUpdateRootPublication::publish(plan)?;
        self.current_root = published.root_swap().post_swap_reader_root();
        Ok(published)
    }
}
