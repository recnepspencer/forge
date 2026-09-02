use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

use crate::state::SignalBranchId;

use super::basis::{
    admit_signal_branch_observation, AdmittedSignalBranchBasis, AdmittedSignalBranchBasisInner,
};
use super::{SignalBranchAdmissionLease, SignalBranchObservation};

/// The owner-local key includes the operational cell incarnation. The
/// observation encoding is only a lookup component; it is never accepted as
/// an admission or currentness proof on its own.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SignalBranchBasisRegistryKey {
    runtime_instance_id: u64,
    definition_basis: u64,
    branch_id: SignalBranchId,
    cell_incarnation: u64,
    observation_encoding: Vec<u8>,
}

impl SignalBranchBasisRegistryKey {
    fn new(
        runtime_instance_id: u64,
        definition_basis: u64,
        branch_id: SignalBranchId,
        cell_incarnation: u64,
        observation: &SignalBranchObservation,
    ) -> Self {
        Self {
            runtime_instance_id,
            definition_basis,
            branch_id,
            cell_incarnation,
            observation_encoding: observation.canonical_encoding(),
        }
    }
}

#[derive(Debug)]
struct SignalBranchBasisRegistryState {
    next_registration_id: u64,
    entries: HashMap<SignalBranchBasisRegistryKey, SignalBranchBasisRegistryEntry>,
}

#[derive(Debug)]
struct SignalBranchBasisRegistryEntry {
    registration_id: u64,
    basis: Weak<AdmittedSignalBranchBasisInner>,
}

/// Weak cleanup lease installed in one admitted basis. The registry owns only
/// weak entries, so canonicalization cannot keep an observation, owner, or
/// component lease alive by itself.
#[derive(Debug)]
pub(crate) struct SignalBranchBasisRegistryLease {
    state: Weak<Mutex<SignalBranchBasisRegistryState>>,
    registration_id: u64,
}

/// Signal-owner-local weak exact-basis canonicalization.
#[derive(Debug, Clone)]
pub(crate) struct SignalBranchBasisRegistry {
    state: Arc<Mutex<SignalBranchBasisRegistryState>>,
}

impl SignalBranchBasisRegistry {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(SignalBranchBasisRegistryState {
                next_registration_id: 0,
                entries: HashMap::new(),
            })),
        }
    }

    pub(crate) fn admit(
        &self,
        runtime_instance_id: u64,
        definition_basis: u64,
        branch_id: SignalBranchId,
        cell_incarnation: u64,
        observation: SignalBranchObservation,
        retention: SignalBranchAdmissionLease,
    ) -> AdmittedSignalBranchBasis {
        let key = SignalBranchBasisRegistryKey::new(
            runtime_instance_id,
            definition_basis,
            branch_id,
            cell_incarnation,
            &observation,
        );
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(existing) = existing_basis(&state, &key) {
            // The caller already owns this operation's reservation. It is
            // intentionally consumed here and released when the duplicate is
            // rejected as a new identity; the canonical basis remains the
            // sole holder for this exact live observation.
            let admitted = AdmittedSignalBranchBasis::from_inner(existing);
            drop(state);
            drop(retention);
            return admitted;
        }

        new_basis(
            &mut state,
            &self.state,
            key,
            observation,
            branch_id,
            retention,
        )
    }

    /// Admit lazily when the caller's owner lease can only be acquired after
    /// the weak canonical entry has been checked. This is the observation
    /// path's capacity boundary: an existing exact basis needs no second
    /// admitted lease, while a genuinely new exact basis must still reserve
    /// one before an identity is issued.
    pub(crate) fn admit_with_retention<E, Acquire>(
        &self,
        runtime_instance_id: u64,
        definition_basis: u64,
        branch_id: SignalBranchId,
        cell_incarnation: u64,
        observation: SignalBranchObservation,
        acquire_retention: Acquire,
    ) -> Result<AdmittedSignalBranchBasis, E>
    where
        Acquire: FnOnce() -> Result<SignalBranchAdmissionLease, E>,
    {
        let key = SignalBranchBasisRegistryKey::new(
            runtime_instance_id,
            definition_basis,
            branch_id,
            cell_incarnation,
            &observation,
        );
        let existing = {
            let state = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            existing_basis(&state, &key)
        };
        if let Some(existing) = existing {
            return Ok(AdmittedSignalBranchBasis::from_inner(existing));
        }

        // Capacity acquisition belongs to the Signal owner and may call back
        // into this registry. Never hold the registry mutex across that owner
        // operation; the second lookup below closes the concurrent admission
        // race without creating a second basis or lease.
        let retention = acquire_retention()?;
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(existing) = existing_basis(&state, &key) {
            let admitted = AdmittedSignalBranchBasis::from_inner(existing);
            drop(state);
            drop(retention);
            return Ok(admitted);
        }

        Ok(new_basis(
            &mut state,
            &self.state,
            key,
            observation,
            branch_id,
            retention,
        ))
    }

    /// Transfer pre-seal canonical entries onto the first sealed owner cell.
    /// The entry identity stays the same; only its owner lifecycle key gains
    /// the real cell incarnation that will distinguish later replacement.
    pub(crate) fn rebind_cell_incarnation(
        &self,
        runtime_instance_id: u64,
        definition_basis: u64,
        branch_id: SignalBranchId,
        previous_cell_incarnation: u64,
        cell_incarnation: u64,
    ) {
        if previous_cell_incarnation == cell_incarnation {
            return;
        }
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let keys = state
            .entries
            .keys()
            .filter(|key| {
                key.runtime_instance_id == runtime_instance_id
                    && key.definition_basis == definition_basis
                    && key.branch_id == branch_id
                    && key.cell_incarnation == previous_cell_incarnation
            })
            .cloned()
            .collect::<Vec<_>>();
        for mut key in keys {
            let entry = state
                .entries
                .remove(&key)
                .expect("a selected Signal basis registry key remains installed");
            key.cell_incarnation = cell_incarnation;
            let replaced = state.entries.insert(key, entry);
            assert!(
                replaced.is_none(),
                "sealing cannot collide with a pre-existing owner cell incarnation"
            );
        }
    }
}

fn existing_basis(
    state: &SignalBranchBasisRegistryState,
    key: &SignalBranchBasisRegistryKey,
) -> Option<Arc<AdmittedSignalBranchBasisInner>> {
    state
        .entries
        .get(key)
        .and_then(|entry| entry.basis.upgrade())
}

fn new_basis(
    state: &mut SignalBranchBasisRegistryState,
    registry_state: &Arc<Mutex<SignalBranchBasisRegistryState>>,
    key: SignalBranchBasisRegistryKey,
    observation: SignalBranchObservation,
    branch_id: SignalBranchId,
    retention: SignalBranchAdmissionLease,
) -> AdmittedSignalBranchBasis {
    let candidate = admit_signal_branch_observation(observation, branch_id, retention);
    let registration_id = state
        .next_registration_id
        .checked_add(1)
        .expect("Signal basis registry registration identities are non-repeatable");
    state.next_registration_id = registration_id;
    let lease = SignalBranchBasisRegistryLease {
        state: Arc::downgrade(registry_state),
        registration_id,
    };
    candidate
        .0
        .registry_lease
        .set(lease)
        .expect("a freshly issued Signal basis has one registry lease slot");
    state.entries.insert(
        key,
        SignalBranchBasisRegistryEntry {
            registration_id,
            basis: Arc::downgrade(&candidate.0),
        },
    );
    candidate
}

impl Default for SignalBranchBasisRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SignalBranchBasisRegistryLease {
    fn drop(&mut self) {
        let Some(state) = self.state.upgrade() else {
            return;
        };
        let mut state = state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let key = state.entries.iter().find_map(|(key, entry)| {
            (entry.registration_id == self.registration_id).then_some(key.clone())
        });
        if let Some(key) = key {
            state.entries.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use worth_foundational::{FoundationalBranchReferenceGeneration, FoundationalBranchTarget};

    use super::super::{signal_branch_observation, SignalBranchRetentionAcquisitionDenial};
    use super::*;

    fn observation(branch_id: u64) -> SignalBranchObservation {
        signal_branch_observation(
            "basis-registry-reentrancy",
            branch_id,
            format!("branch-{branch_id}"),
            FoundationalBranchTarget::empty(),
            FoundationalBranchReferenceGeneration::initial(),
        )
        .expect("test observation is valid")
    }

    #[test]
    fn retention_acquisition_can_reenter_the_registry_without_deadlock() {
        let registry = SignalBranchBasisRegistry::new();
        let retention = Arc::new(super::super::retention::SignalBranchRetentionRegistry::new(
            17,
        ));
        let nested_registry = registry.clone();
        let nested_retention = Arc::clone(&retention);
        let basis = registry
            .admit_with_retention(
                31,
                7,
                crate::state::SignalBranchId(1),
                2,
                observation(1),
                move || {
                    let nested = nested_registry.admit_with_retention(
                        31,
                        7,
                        crate::state::SignalBranchId(2),
                        2,
                        observation(2),
                        move || nested_retention.acquire_admitted(crate::state::SignalBranchId(2)),
                    )?;
                    drop(nested);
                    retention.acquire_admitted(crate::state::SignalBranchId(1))
                },
            )
            .expect("reentrant owner acquisition remains valid");
        let repeated = registry
            .admit_with_retention(
                31,
                7,
                crate::state::SignalBranchId(1),
                2,
                observation(1),
                || Err(SignalBranchRetentionAcquisitionDenial::IdentityExhausted),
            )
            .expect("the exact live basis is reused without reacquisition");

        assert_eq!(basis.admission_identity(), repeated.admission_identity());
    }
}
