use std::collections::BTreeSet;

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactDisposition,
    WorthQueryArtifactHandleGuard, WorthQueryArtifactLifecycleCounters,
    WorthQueryArtifactOwnerSnapshot, WorthQueryRuntimeArtifactOwner,
};

pub(super) struct WorthQueryRuntimeArtifactLifecycle {
    owner_generation: u64,
    owner_active: bool,
    next_borrow_generation: u64,
    active_borrows: BTreeSet<u64>,
    next_lease_generation: u64,
    active_leases: BTreeSet<u64>,
    disposed: bool,
    disposition: WorthQueryArtifactDisposition,
    counters: WorthQueryArtifactLifecycleCounters,
}

impl WorthQueryRuntimeArtifactLifecycle {
    pub(super) fn new(retained_bytes: usize) -> Self {
        Self {
            owner_generation: 1,
            owner_active: true,
            next_borrow_generation: 0,
            active_borrows: BTreeSet::new(),
            next_lease_generation: 0,
            active_leases: BTreeSet::new(),
            disposed: false,
            disposition: WorthQueryArtifactDisposition::Produced,
            counters: WorthQueryArtifactLifecycleCounters {
                production_admissions: 1,
                owner_registrations: 1,
                retained_bytes,
                peak_retained_bytes: retained_bytes,
                ..WorthQueryArtifactLifecycleCounters::default()
            },
        }
    }
}

impl WorthQueryRuntimeArtifactOwner {
    pub(super) fn snapshot(&self) -> WorthQueryArtifactOwnerSnapshot {
        let state = self.lifecycle();
        WorthQueryArtifactOwnerSnapshot::new(
            usize::from(state.owner_active),
            state.active_borrows.len(),
            state.active_leases.len(),
            state.owner_generation,
            state.disposed,
            state.counters,
        )
    }

    pub(super) fn disposition(&self) -> WorthQueryArtifactDisposition {
        self.lifecycle().disposition
    }

    pub(super) fn validate_guard(
        &self,
        guard: WorthQueryArtifactHandleGuard,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let mut state = self.lifecycle();
        validate_guard(self, &mut state, guard)
    }

    pub(super) fn validate_borrow_generation(
        &self,
        generation: u64,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let mut state = self.lifecycle();
        state.counters.lifecycle_generation_checks += 1;
        ensure_live(self, &state)?;
        if !state.active_borrows.contains(&generation) {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
                "artifact view carries an inactive borrow generation",
            ));
        }
        Ok(())
    }

    pub(super) fn admit_transfer(&self, generation: u64) -> Result<u64, WorthQueryArtifactDenial> {
        use worth_query_installation::facade::WorthQueryArtifactMovePosture;

        if self.binding().contract.contract().carriage().movement()
            != WorthQueryArtifactMovePosture::Required
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::MovementForbidden,
                "installed artifact contract forbids ownership movement",
            ));
        }
        let mut state = self.lifecycle();
        validate_guard(
            self,
            &mut state,
            WorthQueryArtifactHandleGuard::Owner(generation),
        )?;
        if !state.active_borrows.is_empty() {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ActiveBorrow,
                "artifact cannot transfer while a borrow is active",
            ));
        }
        state.owner_generation += 1;
        state.disposition = WorthQueryArtifactDisposition::Transferred;
        state.counters.transfer_admissions += 1;
        Ok(state.owner_generation)
    }

    pub(super) fn admit_borrow(
        &self,
        guard: WorthQueryArtifactHandleGuard,
    ) -> Result<u64, WorthQueryArtifactDenial> {
        use worth_query_installation::facade::WorthQueryArtifactBorrowPosture;

        if self.binding().contract.contract().carriage().borrowing()
            != WorthQueryArtifactBorrowPosture::SharedReadOnly
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::BorrowForbidden,
                "installed artifact contract forbids shared borrowing",
            ));
        }
        let mut state = self.lifecycle();
        validate_guard(self, &mut state, guard)?;
        state.next_borrow_generation += 1;
        let generation = state.next_borrow_generation;
        let inserted = state.active_borrows.insert(generation);
        debug_assert!(inserted);
        state.disposition = WorthQueryArtifactDisposition::Borrowed;
        state.counters.borrow_admissions += 1;
        Ok(generation)
    }

    pub(super) fn release_borrow(&self, generation: u64) -> Result<(), WorthQueryArtifactDenial> {
        let mut state = self.lifecycle();
        if !state.active_borrows.remove(&generation) {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
                "artifact borrow generation is no longer active",
            ));
        }
        if state.active_borrows.is_empty() {
            state.disposition = WorthQueryArtifactDisposition::Released;
        }
        Ok(())
    }

    pub(super) fn admit_lease(&self, generation: u64) -> Result<u64, WorthQueryArtifactDenial> {
        use worth_query_installation::facade::{
            WorthQueryArtifactBorrowPosture, WorthQueryArtifactClonePosture,
        };

        let contract = self.binding().contract.contract();
        let lease_is_declared = contract.lifecycle().is_reusable()
            && (contract.carriage().borrowing() == WorthQueryArtifactBorrowPosture::SharedReadOnly
                || matches!(
                    contract.carriage().clone_posture(),
                    WorthQueryArtifactClonePosture::Declared { .. }
                ));
        if !lease_is_declared {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::LeaseForbidden,
                "installed artifact contract does not admit a retained lease",
            ));
        }
        let mut state = self.lifecycle();
        validate_guard(
            self,
            &mut state,
            WorthQueryArtifactHandleGuard::Owner(generation),
        )?;
        state.next_lease_generation += 1;
        let generation = state.next_lease_generation;
        let inserted = state.active_leases.insert(generation);
        debug_assert!(inserted);
        state.disposition = WorthQueryArtifactDisposition::Leased;
        state.counters.lease_admissions += 1;
        Ok(generation)
    }

    pub(super) fn release_lease(
        &self,
        generation: u64,
        disposition: WorthQueryArtifactDisposition,
    ) -> Result<bool, WorthQueryArtifactDenial> {
        let should_dispose = {
            let mut state = self.lifecycle();
            if !state.active_leases.remove(&generation) {
                return Err(self.denial(
                    WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
                    "artifact lease generation is no longer active",
                ));
            }
            state.disposition = disposition;
            mark_disposed_if_unowned(&mut state)
        };
        self.dispose_provider_if_required(should_dispose);
        Ok(should_dispose)
    }

    pub(super) fn release_owner(
        &self,
        generation: u64,
        disposition: WorthQueryArtifactDisposition,
        require_no_lease: bool,
    ) -> Result<bool, WorthQueryArtifactDenial> {
        let should_dispose = {
            let mut state = self.lifecycle();
            validate_guard(
                self,
                &mut state,
                WorthQueryArtifactHandleGuard::Owner(generation),
            )?;
            if !state.active_borrows.is_empty() {
                return Err(self.denial(
                    WorthQueryArtifactDenialKind::ActiveBorrow,
                    "artifact cannot be disposed while a borrow is active",
                ));
            }
            if require_no_lease && !state.active_leases.is_empty() {
                return Err(self.denial(
                    WorthQueryArtifactDenialKind::ActiveLease,
                    "artifact cannot be explicitly disposed while a lease is active",
                ));
            }
            state.owner_active = false;
            state.owner_generation += 1;
            state.disposition = disposition;
            mark_disposed_if_unowned(&mut state)
        };
        self.dispose_provider_if_required(should_dispose);
        Ok(should_dispose)
    }

    fn lifecycle(&self) -> std::sync::MutexGuard<'_, WorthQueryRuntimeArtifactLifecycle> {
        self.lifecycle
            .lock()
            .expect("artifact lifecycle lock must remain available")
    }
}

fn validate_guard(
    owner: &WorthQueryRuntimeArtifactOwner,
    state: &mut WorthQueryRuntimeArtifactLifecycle,
    guard: WorthQueryArtifactHandleGuard,
) -> Result<(), WorthQueryArtifactDenial> {
    state.counters.lifecycle_generation_checks += 1;
    ensure_live(owner, state)?;
    let active = match guard {
        WorthQueryArtifactHandleGuard::Owner(generation) => {
            state.owner_active && state.owner_generation == generation
        }
        WorthQueryArtifactHandleGuard::Lease(generation) => {
            state.active_leases.contains(&generation)
        }
    };
    if !active {
        return Err(owner.denial(
            WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
            "artifact handle carries an inactive lifecycle generation",
        ));
    }
    Ok(())
}

fn ensure_live(
    owner: &WorthQueryRuntimeArtifactOwner,
    state: &WorthQueryRuntimeArtifactLifecycle,
) -> Result<(), WorthQueryArtifactDenial> {
    if state.disposed {
        Err(owner.denial(
            WorthQueryArtifactDenialKind::AlreadyDisposed,
            "artifact owner has already been disposed",
        ))
    } else {
        Ok(())
    }
}

fn mark_disposed_if_unowned(state: &mut WorthQueryRuntimeArtifactLifecycle) -> bool {
    if !state.owner_active
        && state.active_borrows.is_empty()
        && state.active_leases.is_empty()
        && !state.disposed
    {
        state.disposed = true;
        state.disposition = WorthQueryArtifactDisposition::Disposed;
        state.counters.provider_disposals += 1;
        true
    } else {
        false
    }
}
