use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use serde::{Deserialize, Serialize};
use worth_foundational::FoundationalBranchReferenceMismatchAxis;

use crate::state::SignalBranchId;

const DEFAULT_MAXIMUM_ACTIVE_SIGNAL_BRANCH_LEASES: usize = 4_096;

#[derive(Debug, Clone)]
pub(crate) struct SignalBranchRetentionRegistry {
    state: Arc<Mutex<SignalBranchRetentionState>>,
}

#[derive(Debug)]
struct SignalBranchRetentionState {
    next_lease_id: u64,
    maximum_active_leases: usize,
    admitted_leases: HashMap<u64, SignalBranchId>,
    external_leases: HashMap<u64, SignalBranchId>,
    admitted_count_by_branch: HashMap<SignalBranchId, u32>,
    external_count_by_branch: HashMap<SignalBranchId, u32>,
}

#[derive(Debug)]
pub(crate) struct SignalBranchAdmissionLease {
    registry: SignalBranchRetentionRegistry,
    lease_id: u64,
    branch_id: SignalBranchId,
}

#[derive(Debug)]
pub struct SignalBranchRetentionLease {
    pub(crate) runtime_instance_id: String,
    pub(crate) lease_id: u64,
    pub(crate) branch_id: SignalBranchId,
}

impl SignalBranchRetentionLease {
    pub const fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub(crate) fn owner_issued(
        runtime_instance_id: String,
        lease_id: u64,
        branch_id: SignalBranchId,
    ) -> Self {
        Self {
            runtime_instance_id,
            lease_id,
            branch_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchRetentionAcquisitionDenial {
    ForeignBasis,
    UnknownBranch {
        branch_id: SignalBranchId,
    },
    RetiredBranch {
        branch_id: SignalBranchId,
    },
    StaleBasis {
        axes: Vec<FoundationalBranchReferenceMismatchAxis>,
    },
    CapacityExhausted {
        maximum_active_leases: usize,
    },
    IdentityExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalBranchRetentionReleaseDenial {
    ForeignRuntime,
    UnknownLease,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalBranchRetentionReleaseReceipt {
    branch_id: SignalBranchId,
    remaining_branch_leases: u32,
}

impl SignalBranchRetentionReleaseReceipt {
    pub const fn branch_id(&self) -> SignalBranchId {
        self.branch_id
    }

    pub const fn remaining_branch_leases(&self) -> u32 {
        self.remaining_branch_leases
    }

    pub(crate) const fn owner_issued(
        branch_id: SignalBranchId,
        remaining_branch_leases: u32,
    ) -> Self {
        Self {
            branch_id,
            remaining_branch_leases,
        }
    }
}

#[derive(Debug)]
pub enum SignalBranchRetentionReleaseOutcome {
    Released(SignalBranchRetentionReleaseReceipt),
    Denied {
        lease: SignalBranchRetentionLease,
        denial: SignalBranchRetentionReleaseDenial,
    },
}

impl Default for SignalBranchRetentionRegistry {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(SignalBranchRetentionState {
                next_lease_id: 0,
                maximum_active_leases: DEFAULT_MAXIMUM_ACTIVE_SIGNAL_BRANCH_LEASES,
                admitted_leases: HashMap::new(),
                external_leases: HashMap::new(),
                admitted_count_by_branch: HashMap::new(),
                external_count_by_branch: HashMap::new(),
            })),
        }
    }
}

impl SignalBranchRetentionRegistry {
    pub(crate) fn acquire_admitted(
        &self,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchAdmissionLease, SignalBranchRetentionAcquisitionDenial> {
        let lease_id = self.acquire_lease(branch_id, true)?;
        Ok(SignalBranchAdmissionLease {
            registry: self.clone(),
            lease_id,
            branch_id,
        })
    }

    pub(crate) fn acquire_external(
        &self,
        runtime_instance_id: String,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial> {
        let lease_id = self.acquire_lease(branch_id, false)?;
        Ok(SignalBranchRetentionLease::owner_issued(
            runtime_instance_id,
            lease_id,
            branch_id,
        ))
    }

    pub(crate) fn release_external(
        &self,
        runtime_instance_id: &str,
        lease: SignalBranchRetentionLease,
    ) -> SignalBranchRetentionReleaseOutcome {
        if lease.runtime_instance_id != runtime_instance_id {
            return SignalBranchRetentionReleaseOutcome::Denied {
                lease,
                denial: SignalBranchRetentionReleaseDenial::ForeignRuntime,
            };
        }
        let mut state = self.lock();
        let Some(branch_id) = state.external_leases.remove(&lease.lease_id) else {
            return SignalBranchRetentionReleaseOutcome::Denied {
                lease,
                denial: SignalBranchRetentionReleaseDenial::UnknownLease,
            };
        };
        let remaining = decrement_branch_count(&mut state.external_count_by_branch, branch_id);
        SignalBranchRetentionReleaseOutcome::Released(
            SignalBranchRetentionReleaseReceipt::owner_issued(branch_id, remaining),
        )
    }

    pub(crate) fn admitted_count(&self, branch_id: SignalBranchId) -> u32 {
        self.lock()
            .admitted_count_by_branch
            .get(&branch_id)
            .copied()
            .unwrap_or(0)
    }

    pub(crate) fn external_count(&self, branch_id: SignalBranchId) -> u32 {
        self.lock()
            .external_count_by_branch
            .get(&branch_id)
            .copied()
            .unwrap_or(0)
    }

    fn acquire_lease(
        &self,
        branch_id: SignalBranchId,
        admitted: bool,
    ) -> Result<u64, SignalBranchRetentionAcquisitionDenial> {
        let mut state = self.lock();
        if state.admitted_leases.len() + state.external_leases.len() >= state.maximum_active_leases
        {
            return Err(SignalBranchRetentionAcquisitionDenial::CapacityExhausted {
                maximum_active_leases: state.maximum_active_leases,
            });
        }
        let lease_id = state
            .next_lease_id
            .checked_add(1)
            .ok_or(SignalBranchRetentionAcquisitionDenial::IdentityExhausted)?;
        let maximum_active_leases = state.maximum_active_leases;
        let counts = if admitted {
            &mut state.admitted_count_by_branch
        } else {
            &mut state.external_count_by_branch
        };
        let count = counts.entry(branch_id).or_default();
        *count = count.checked_add(1).ok_or(
            SignalBranchRetentionAcquisitionDenial::CapacityExhausted {
                maximum_active_leases,
            },
        )?;
        state.next_lease_id = lease_id;
        if admitted {
            state.admitted_leases.insert(lease_id, branch_id);
        } else {
            state.external_leases.insert(lease_id, branch_id);
        }
        Ok(lease_id)
    }

    fn release_admitted(&self, lease_id: u64, branch_id: SignalBranchId) {
        let mut state = self.lock();
        if state.admitted_leases.remove(&lease_id) == Some(branch_id) {
            decrement_branch_count(&mut state.admitted_count_by_branch, branch_id);
        }
    }

    fn rebind_admitted(
        &self,
        lease_id: u64,
        previous_branch_id: SignalBranchId,
        branch_id: SignalBranchId,
    ) {
        let mut state = self.lock();
        if state.admitted_leases.get(&lease_id) != Some(&previous_branch_id) {
            return;
        }
        state.admitted_leases.insert(lease_id, branch_id);
        decrement_branch_count(&mut state.admitted_count_by_branch, previous_branch_id);
        let count = state.admitted_count_by_branch.entry(branch_id).or_default();
        *count = count.saturating_add(1);
    }

    fn lock(&self) -> MutexGuard<'_, SignalBranchRetentionState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl Drop for SignalBranchAdmissionLease {
    fn drop(&mut self) {
        self.registry
            .release_admitted(self.lease_id, self.branch_id);
    }
}

impl SignalBranchAdmissionLease {
    pub(crate) const fn lease_id(&self) -> u64 {
        self.lease_id
    }

    pub(crate) fn rebind_branch(&mut self, branch_id: SignalBranchId) {
        if self.branch_id == branch_id {
            return;
        }
        self.registry
            .rebind_admitted(self.lease_id, self.branch_id, branch_id);
        self.branch_id = branch_id;
    }
}

fn decrement_branch_count(
    counts: &mut HashMap<SignalBranchId, u32>,
    branch_id: SignalBranchId,
) -> u32 {
    let remaining = counts
        .get_mut(&branch_id)
        .map(|count| {
            *count = count.saturating_sub(1);
            *count
        })
        .unwrap_or(0);
    if remaining == 0 {
        counts.remove(&branch_id);
    }
    remaining
}
