use std::path::Path;

use super::store::PhysicalRootPublicationStore;
use super::{CopyOnWritePublicationPlan, PhysicalPublicationDenial, ReadCopyUpdateRootPublication};
use crate::CurrentPhysicalRoot;
use worth_store_physical_backend::{
    ProductionStorageBoundaryControl, UninterruptedStorageBoundaryControl,
};

/// The single path-bound owner of durable physical-root publication.
#[derive(Debug)]
pub struct PhysicalRootPublicationRuntime {
    current_root: CurrentPhysicalRoot,
    store: PhysicalRootPublicationStore,
}

impl PhysicalRootPublicationRuntime {
    pub fn open(
        publication_directory: &Path,
        current_root: CurrentPhysicalRoot,
    ) -> Result<Self, PhysicalPublicationDenial> {
        let store = PhysicalRootPublicationStore::open(publication_directory, current_root)?;
        Ok(Self {
            current_root,
            store,
        })
    }

    pub const fn current_root(&self) -> CurrentPhysicalRoot {
        self.current_root
    }

    pub fn publish(
        &mut self,
        plan: CopyOnWritePublicationPlan,
    ) -> Result<ReadCopyUpdateRootPublication, PhysicalPublicationDenial> {
        self.publish_controlled(plan, &UninterruptedStorageBoundaryControl)
    }

    fn publish_controlled(
        &mut self,
        plan: CopyOnWritePublicationPlan,
        control: &impl ProductionStorageBoundaryControl,
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
        let new_root = plan.intent().new_root();
        self.store
            .publish_with_boundary_control(planned_old_root, new_root, control)?;
        let published = ReadCopyUpdateRootPublication::from_durable_publication(
            plan,
            control.execution_identity(),
        );
        self.current_root = new_root;
        Ok(published)
    }

    pub(crate) fn publish_recovery(
        &mut self,
        plan: CopyOnWritePublicationPlan,
        recovery_binding: [u8; 32],
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
        let control = UninterruptedStorageBoundaryControl;
        let new_root = plan.intent().new_root();
        self.store.publish_recovery_with_boundary_control(
            planned_old_root,
            new_root,
            recovery_binding,
            &control,
        )?;
        let published = ReadCopyUpdateRootPublication::from_durable_publication(
            plan,
            control.execution_identity(),
        );
        self.current_root = new_root;
        Ok(published)
    }

    pub(crate) fn current_recovery_binding(
        &self,
    ) -> Result<Option<[u8; 32]>, PhysicalPublicationDenial> {
        self.store.current_recovery_binding()
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn publish_with_boundary_control(
        &mut self,
        plan: CopyOnWritePublicationPlan,
        control: &impl ProductionStorageBoundaryControl,
    ) -> Result<ReadCopyUpdateRootPublication, PhysicalPublicationDenial> {
        self.publish_controlled(plan, control)
    }

    #[cfg(any(test, feature = "certification-authority"))]
    pub fn attempt_with_boundary_control(
        &mut self,
        plan: CopyOnWritePublicationPlan,
        control: &impl ProductionStorageBoundaryControl,
    ) -> super::PhysicalRootPublicationAttempt {
        let storage_boundary_execution = control.execution_identity();
        super::PhysicalRootPublicationAttempt::from_outcome(
            self.publish_controlled(plan, control),
            storage_boundary_execution,
        )
    }

    #[cfg(any(
        test,
        feature = "certification-authority",
        feature = "phase20-layout-rule-construction"
    ))]
    pub fn open_for_testing(
        current_root: CurrentPhysicalRoot,
    ) -> Result<Self, PhysicalPublicationDenial> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT_STORE: AtomicU64 = AtomicU64::new(1);
        let id = NEXT_STORE.fetch_add(1, Ordering::Relaxed);
        let creation_nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("test publication directory clock must follow Unix epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "worth-store-root-publication-test-{}-{id}-{creation_nonce}",
            std::process::id(),
        ));
        Self::open(&directory, current_root)
    }

    #[cfg(any(
        test,
        feature = "certification-authority",
        feature = "phase20-layout-rule-construction"
    ))]
    pub fn from_current_root(current_root: CurrentPhysicalRoot) -> Self {
        Self::open_for_testing(current_root)
            .expect("isolated test publication store must open through the production owner")
    }
}
