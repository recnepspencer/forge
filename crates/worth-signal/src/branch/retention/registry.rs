use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use worth_foundational::{FoundationalBranchTargetBasis, FoundationalBranchTargetEncoding};

use crate::branch::SignalBranchBasisDescriptor;
use crate::state::SignalBranchId;

use super::accounting::{
    decrement_obligation_count, increment_obligation_count, obligation_count,
    SignalRetentionReleaseAccounting, SignalRetentionTerminalAccounting,
    SignalRetentionTerminalCause,
};
use super::lease::{SignalBranchAdmissionLease, SignalBranchRetentionLease};
use super::outcome::{
    SignalBranchRetentionAcquisitionDenial, SignalBranchRetentionOwnerPosture,
    SignalBranchRetentionTerminalCounts,
};

const DEFAULT_MAXIMUM_ACTIVE_SIGNAL_BRANCH_LEASES: usize = 4_096;

/// Identity of the narrow Signal retention owner.
///
/// The registry holds the only strong reference, so acquisition capability is
/// bound to a live owner and closes strictly before owner state is dropped.
/// Outstanding obligations hold a weak reference plus the cleanup ledger, so
/// they can still terminate after the runtime is gone without keeping any
/// Signal observation or mutation capability open.
#[derive(Debug)]
pub(crate) struct SignalRetentionOwner {
    runtime_instance_id: u64,
}

/// The exact immutable Signal target one external obligation retains.
///
/// Identity is the owner's canonical target encoding: domain-tagged,
/// schema-versioned bytes. Two obligations share a key only when they retain
/// the same immutable component state, so releasing one obligation can never
/// discharge one taken over a newer or sibling target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SignalRetainedTargetKey(FoundationalBranchTargetEncoding);

/// Owner-held Signal retention authority.
///
/// Deliberately not `Clone`: owner liveness is exactly the strong reference
/// this value holds. Narrow cleanup bindings are issued through
/// [`SignalBranchRetentionRegistry::binding`].
#[derive(Debug)]
pub(crate) struct SignalBranchRetentionRegistry {
    owner: Arc<SignalRetentionOwner>,
    ledger: Arc<SignalRetentionLedger>,
}

/// Cloneable narrow binding to the Signal retention owner.
///
/// It carries weak owner liveness plus the surviving cleanup ledger, and grants
/// no observation, mutation, or acquisition capability of its own.
#[derive(Debug, Clone)]
pub(crate) struct SignalBranchRetentionBinding {
    owner: Weak<SignalRetentionOwner>,
    ledger: Arc<SignalRetentionLedger>,
}

/// How one obligation's issuing owner relates to another owner binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalBranchRetentionOwnerRelationship {
    SameOwner,
    DifferentOwner,
    OwnerLost,
}

#[derive(Debug)]
struct SignalRetentionLedger {
    maximum_active_leases: usize,
    state: Mutex<SignalRetentionLedgerState>,
    terminality: SignalRetentionTerminalAccounting,
}

#[derive(Debug)]
struct SignalRetentionLedgerState {
    next_lease_id: u64,
    admitted_leases: HashMap<u64, SignalBranchId>,
    external_leases: HashMap<u64, SignalExternalObligationRecord>,
    admitted_count_by_branch: HashMap<SignalBranchId, u32>,
    external_count_by_branch: HashMap<SignalBranchId, u32>,
    external_count_by_target: HashMap<SignalRetainedTargetKey, u32>,
}

#[derive(Debug)]
struct SignalExternalObligationRecord {
    branch_id: SignalBranchId,
    target: SignalRetainedTargetKey,
}

impl SignalBranchRetentionRegistry {
    pub(crate) fn new(runtime_instance_id: u64) -> Self {
        Self {
            owner: Arc::new(SignalRetentionOwner {
                runtime_instance_id,
            }),
            ledger: Arc::new(SignalRetentionLedger {
                maximum_active_leases: DEFAULT_MAXIMUM_ACTIVE_SIGNAL_BRANCH_LEASES,
                state: Mutex::new(SignalRetentionLedgerState {
                    next_lease_id: 0,
                    admitted_leases: HashMap::new(),
                    external_leases: HashMap::new(),
                    admitted_count_by_branch: HashMap::new(),
                    external_count_by_branch: HashMap::new(),
                    external_count_by_target: HashMap::new(),
                }),
                terminality: SignalRetentionTerminalAccounting::default(),
            }),
        }
    }

    pub(crate) fn binding(&self) -> SignalBranchRetentionBinding {
        SignalBranchRetentionBinding {
            owner: Arc::downgrade(&self.owner),
            ledger: Arc::clone(&self.ledger),
        }
    }

    pub(crate) fn acquire_admitted(
        &self,
        branch_id: SignalBranchId,
    ) -> Result<SignalBranchAdmissionLease, SignalBranchRetentionAcquisitionDenial> {
        let lease_id = self.ledger.admit_lease_identity(branch_id)?;
        Ok(SignalBranchAdmissionLease::owner_issued(
            self.binding(),
            lease_id,
            branch_id,
        ))
    }

    /// Open one external obligation over the exact immutable target the
    /// supplied descriptor names.
    ///
    /// Only owner affinity and bounded owner resources are decided here.
    /// Branch lifecycle, definition agreement, and continued target
    /// availability are decided by the runtime operation that owns those
    /// truths, and currentness is decided by nobody: an exact obligation over
    /// a historical target is legitimate.
    pub(crate) fn acquire_external(
        &self,
        descriptor: SignalBranchBasisDescriptor,
    ) -> Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial> {
        let Some(target) = descriptor.observation().target().as_basis() else {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        };
        if target.graph_instance_id() != self.owner.runtime_instance_id.to_string() {
            return Err(SignalBranchRetentionAcquisitionDenial::ForeignBasis);
        }
        let key = SignalRetainedTargetKey(target.canonical_encoding());
        let branch_id = descriptor.owner_branch_id();
        let lease_id = self.ledger.retain_exact_target(branch_id, key)?;
        Ok(SignalBranchRetentionLease::owner_issued(
            descriptor,
            self.binding(),
            lease_id,
        ))
    }

    pub(crate) fn admitted_count(&self, branch_id: SignalBranchId) -> u32 {
        obligation_count(&self.ledger.lock().admitted_count_by_branch, &branch_id)
    }

    pub(crate) fn external_count(&self, branch_id: SignalBranchId) -> u32 {
        obligation_count(&self.ledger.lock().external_count_by_branch, &branch_id)
    }

    pub(crate) fn terminal_counts(&self) -> SignalBranchRetentionTerminalCounts {
        self.ledger.terminality.counts()
    }
}

impl SignalBranchRetentionBinding {
    pub(crate) fn owner_posture(&self) -> SignalBranchRetentionOwnerPosture {
        match self.owner.upgrade() {
            Some(_) => SignalBranchRetentionOwnerPosture::Live,
            None => SignalBranchRetentionOwnerPosture::Lost,
        }
    }

    pub(crate) fn owner_relationship(
        &self,
        owner: &Self,
    ) -> SignalBranchRetentionOwnerRelationship {
        let (Some(issuing_owner), Some(observed_owner)) =
            (self.owner.upgrade(), owner.owner.upgrade())
        else {
            return SignalBranchRetentionOwnerRelationship::OwnerLost;
        };
        if Arc::ptr_eq(&issuing_owner, &observed_owner) {
            SignalBranchRetentionOwnerRelationship::SameOwner
        } else {
            SignalBranchRetentionOwnerRelationship::DifferentOwner
        }
    }

    pub(crate) fn holds_external_obligation(&self, lease_id: u64) -> bool {
        self.ledger.lock().external_leases.contains_key(&lease_id)
    }

    pub(crate) fn terminal_counts(&self) -> SignalBranchRetentionTerminalCounts {
        self.ledger.terminality.counts()
    }

    /// Terminate exactly one external obligation through the owner ledger.
    ///
    /// The accounting is `None` when no live obligation is registered, which is
    /// the typed registry defense behind the consuming lease. The posture is
    /// returned alongside it so the caller's evidence and the ledger's record
    /// describe the same owner, rather than two separate observations of it.
    pub(crate) fn terminate_external(
        &self,
        lease_id: u64,
        cause: SignalRetentionTerminalCause,
    ) -> (
        SignalBranchRetentionOwnerPosture,
        Option<SignalRetentionReleaseAccounting>,
    ) {
        let posture = self.owner_posture();
        let accounting = self.ledger.release_exact_target(lease_id);
        self.ledger
            .terminality
            .record(cause, posture, accounting.is_some());
        (posture, accounting)
    }

    pub(crate) fn release_admitted(&self, lease_id: u64, branch_id: SignalBranchId) {
        self.ledger.release_admitted(lease_id, branch_id);
    }

    pub(crate) fn rebind_admitted(
        &self,
        lease_id: u64,
        previous_branch_id: SignalBranchId,
        branch_id: SignalBranchId,
    ) {
        self.ledger
            .rebind_admitted(lease_id, previous_branch_id, branch_id);
    }
}

impl SignalRetentionLedger {
    fn admit_lease_identity(
        &self,
        branch_id: SignalBranchId,
    ) -> Result<u64, SignalBranchRetentionAcquisitionDenial> {
        let mut state = self.lock();
        let lease_id = self.reserve_lease_identity(&mut state)?;
        increment_obligation_count(&mut state.admitted_count_by_branch, branch_id);
        state.admitted_leases.insert(lease_id, branch_id);
        Ok(lease_id)
    }

    fn retain_exact_target(
        &self,
        branch_id: SignalBranchId,
        target: SignalRetainedTargetKey,
    ) -> Result<u64, SignalBranchRetentionAcquisitionDenial> {
        let mut state = self.lock();
        let lease_id = self.reserve_lease_identity(&mut state)?;
        increment_obligation_count(&mut state.external_count_by_branch, branch_id);
        increment_obligation_count(&mut state.external_count_by_target, target.clone());
        state.external_leases.insert(
            lease_id,
            SignalExternalObligationRecord { branch_id, target },
        );
        Ok(lease_id)
    }

    fn release_exact_target(&self, lease_id: u64) -> Option<SignalRetentionReleaseAccounting> {
        let mut state = self.lock();
        let record = state.external_leases.remove(&lease_id)?;
        let remaining_branch_leases =
            decrement_obligation_count(&mut state.external_count_by_branch, &record.branch_id);
        let remaining_target_leases =
            decrement_obligation_count(&mut state.external_count_by_target, &record.target);
        Some(SignalRetentionReleaseAccounting {
            branch_id: record.branch_id,
            remaining_target_leases,
            remaining_branch_leases,
        })
    }

    fn release_admitted(&self, lease_id: u64, branch_id: SignalBranchId) {
        let mut state = self.lock();
        if state.admitted_leases.remove(&lease_id) == Some(branch_id) {
            decrement_obligation_count(&mut state.admitted_count_by_branch, &branch_id);
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
        decrement_obligation_count(&mut state.admitted_count_by_branch, &previous_branch_id);
        increment_obligation_count(&mut state.admitted_count_by_branch, branch_id);
    }

    fn reserve_lease_identity(
        &self,
        state: &mut SignalRetentionLedgerState,
    ) -> Result<u64, SignalBranchRetentionAcquisitionDenial> {
        if state.admitted_leases.len() + state.external_leases.len() >= self.maximum_active_leases {
            return Err(SignalBranchRetentionAcquisitionDenial::CapacityExhausted {
                maximum_active_leases: self.maximum_active_leases,
            });
        }
        let lease_id = state
            .next_lease_id
            .checked_add(1)
            .ok_or(SignalBranchRetentionAcquisitionDenial::IdentityExhausted)?;
        state.next_lease_id = lease_id;
        Ok(lease_id)
    }

    fn lock(&self) -> MutexGuard<'_, SignalRetentionLedgerState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
