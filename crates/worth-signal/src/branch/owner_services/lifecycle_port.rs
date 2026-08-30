use std::sync::{Arc, Weak};

use super::{SignalOwner, SignalOwnerUnavailable};

/// Package-private Phase 3 slot for the concrete weak lifecycle service.
pub struct SignalBranchLifecyclePort<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    owner: Weak<SignalOwner<D, I, T>>,
    diagnostic_owner_runtime_instance_id: u64,
}

impl<D, I, T> Clone for SignalBranchLifecyclePort<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            diagnostic_owner_runtime_instance_id: self.diagnostic_owner_runtime_instance_id,
        }
    }
}

impl<D, I, T> SignalBranchLifecyclePort<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(
        owner: Weak<SignalOwner<D, I, T>>,
        diagnostic_owner_runtime_instance_id: u64,
    ) -> Self {
        Self {
            owner,
            diagnostic_owner_runtime_instance_id,
        }
    }

    pub(crate) fn diagnostic_owner_runtime_instance_id(&self) -> u64 {
        self.diagnostic_owner_runtime_instance_id
    }

    pub(super) fn upgrade_owner(
        &self,
    ) -> Result<Arc<SignalOwner<D, I, T>>, SignalOwnerUnavailable> {
        SignalOwner::upgrade(&self.owner)
    }
}
