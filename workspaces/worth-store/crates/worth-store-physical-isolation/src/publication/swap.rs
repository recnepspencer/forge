use super::{CopyOnWritePublicationPlan, PhysicalPublicationReceipt};
use crate::CurrentPhysicalRoot;
use worth_store_physical_backend::StorageBoundaryExecutionIdentity;

#[derive(Debug, Clone, Copy)]
pub struct PublishedCopyOnWriteRootSwap {
    pre_swap_reader_root: CurrentPhysicalRoot,
    post_swap_reader_root: CurrentPhysicalRoot,
}

#[derive(Debug, Clone)]
pub struct ReadCopyUpdateRootPublication {
    root_swap: PublishedCopyOnWriteRootSwap,
    receipt: PhysicalPublicationReceipt,
}

impl ReadCopyUpdateRootPublication {
    #[cfg(any(test, feature = "certification-authority"))]
    pub fn publish(
        plan: CopyOnWritePublicationPlan,
    ) -> Result<Self, super::PhysicalPublicationDenial> {
        let current_root = plan.intent().old_root();
        super::PhysicalRootPublicationRuntime::open_for_testing(current_root)?.publish(plan)
    }

    pub(super) fn from_durable_publication(
        plan: CopyOnWritePublicationPlan,
        storage_boundary_execution: Option<StorageBoundaryExecutionIdentity>,
    ) -> Self {
        let root_swap = PublishedCopyOnWriteRootSwap::from_plan(&plan);
        let receipt = PhysicalPublicationReceipt::from_publish(
            plan.intent(),
            plan.publish_ordering(),
            plan.release_posture(),
            plan.readiness().free_reuse(),
            plan.counters().with_root_swap(),
            storage_boundary_execution,
        );
        Self { root_swap, receipt }
    }

    pub const fn receipt(&self) -> &PhysicalPublicationReceipt {
        &self.receipt
    }

    pub const fn root_swap(&self) -> PublishedCopyOnWriteRootSwap {
        self.root_swap
    }
}

impl PublishedCopyOnWriteRootSwap {
    pub(crate) fn from_plan(plan: &CopyOnWritePublicationPlan) -> Self {
        Self {
            pre_swap_reader_root: plan.intent().old_root(),
            post_swap_reader_root: plan.intent().new_root(),
        }
    }

    pub const fn pre_swap_reader_root(self) -> CurrentPhysicalRoot {
        self.pre_swap_reader_root
    }

    pub const fn post_swap_reader_root(self) -> CurrentPhysicalRoot {
        self.post_swap_reader_root
    }
}
