use super::{CopyOnWritePublicationPlan, PhysicalPublicationDenial, PhysicalPublicationReceipt};
use crate::CurrentPhysicalRoot;

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
    pub fn publish(plan: CopyOnWritePublicationPlan) -> Result<Self, PhysicalPublicationDenial> {
        let root_swap = PublishedCopyOnWriteRootSwap::from_plan(&plan);
        let receipt = PhysicalPublicationReceipt::from_publish(
            plan.intent(),
            plan.publish_ordering(),
            plan.release_posture(),
            plan.readiness().free_reuse(),
            plan.counters().with_root_swap(),
        );
        Ok(Self { root_swap, receipt })
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
