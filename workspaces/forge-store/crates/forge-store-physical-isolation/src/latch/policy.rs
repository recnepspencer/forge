use super::PhysicalLatchClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLatchDeadlockPolicy {
    PreventByCanonicalOrder,
    DetectWithBoundedWaitForGraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalLatchFamilyDeadlockPolicy {
    class: PhysicalLatchClass,
    policy: PhysicalLatchDeadlockPolicy,
}

impl PhysicalLatchFamilyDeadlockPolicy {
    pub const fn for_class(class: PhysicalLatchClass) -> Self {
        Self {
            class,
            policy: PhysicalLatchDeadlockPolicy::PreventByCanonicalOrder,
        }
    }

    pub const fn detect_with_bounded_wait_for_graph(class: PhysicalLatchClass) -> Self {
        Self {
            class,
            policy: PhysicalLatchDeadlockPolicy::DetectWithBoundedWaitForGraph,
        }
    }

    pub const fn class(self) -> PhysicalLatchClass {
        self.class
    }

    pub const fn policy(self) -> PhysicalLatchDeadlockPolicy {
        self.policy
    }
}
