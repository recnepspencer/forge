use std::ops::{Deref, DerefMut};

use worth_store_physical_isolation::{
    CopyOnWritePublicationPlan, CurrentPhysicalRoot, PhysicalPublicationDenial,
    PhysicalRootPublicationRuntime, ReadCopyUpdateRootPublication,
};

/// Owns both a physical publication runtime and the filesystem root backing it.
pub struct PhysicalRootPublicationFixture {
    runtime: PhysicalRootPublicationRuntime,
    _directory: crate::TemporaryDirectory,
}

pub fn publish_in_temporary_store(
    plan: CopyOnWritePublicationPlan,
) -> Result<ReadCopyUpdateRootPublication, PhysicalPublicationDenial> {
    let mut fixture = PhysicalRootPublicationFixture::open(plan.binding().old_root())?;
    fixture.publish(plan)
}

impl PhysicalRootPublicationFixture {
    pub fn open(current_root: CurrentPhysicalRoot) -> Result<Self, PhysicalPublicationDenial> {
        let directory = crate::TemporaryDirectory::create("root-publication")
            .map_err(|_| PhysicalPublicationDenial::PublicationStoreIo)?;
        let runtime = PhysicalRootPublicationRuntime::open(directory.path(), current_root)?;
        Ok(Self {
            runtime,
            _directory: directory,
        })
    }
}

impl Deref for PhysicalRootPublicationFixture {
    type Target = PhysicalRootPublicationRuntime;

    fn deref(&self) -> &Self::Target {
        &self.runtime
    }
}

impl DerefMut for PhysicalRootPublicationFixture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.runtime
    }
}
