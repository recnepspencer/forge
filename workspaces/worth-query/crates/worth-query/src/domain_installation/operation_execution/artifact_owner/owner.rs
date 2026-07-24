use std::sync::{Arc, Mutex};

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactDisposition,
    WorthQueryArtifactLifecycleCounters, WorthQueryArtifactOwnerSnapshot,
    WorthQueryArtifactSemanticProjection, WorthQueryErasedArtifactProviderResource,
    WorthQueryPreparedArtifactResource,
};

pub(crate) struct WorthQueryRuntimeArtifactOwner {
    binding: WorthQueryRuntimeArtifactBinding,
    semantic_projection: WorthQueryArtifactSemanticProjection,
    retained_bytes: usize,
    created_thread: std::thread::ThreadId,
    resource: Mutex<Option<Box<dyn WorthQueryErasedArtifactProviderResource>>>,
    lifecycle: Mutex<WorthQueryRuntimeArtifactLifecycle>,
}

pub(crate) struct WorthQueryRuntimeArtifactBinding {
    pub(super) contract:
        Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>,
    pub(super) domain_authority:
        Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(super) operation_identity: String,
    pub(super) binding_identity: String,
    pub(super) run_identity: String,
    pub(super) producing_stage: String,
    pub(super) basis_identity: String,
    pub(super) provenance_identity: String,
    pub(super) dependency_identity: String,
    pub(super) owner_identity: String,
    pub(super) occurrence_identity: String,
}

struct WorthQueryRuntimeArtifactLifecycle {
    owner_count: usize,
    borrow_count: usize,
    lease_count: usize,
    lifecycle_generation: u64,
    borrow_generation: u64,
    lease_generation: u64,
    disposed: bool,
    disposition: WorthQueryArtifactDisposition,
    counters: WorthQueryArtifactLifecycleCounters,
}

impl WorthQueryRuntimeArtifactOwner {
    pub(super) fn register(
        binding: WorthQueryRuntimeArtifactBinding,
        prepared: WorthQueryPreparedArtifactResource,
    ) -> Arc<Self> {
        let retained_bytes = prepared.retained_bytes;
        Arc::new(Self {
            binding,
            semantic_projection: prepared.semantic_projection,
            retained_bytes,
            created_thread: std::thread::current().id(),
            resource: Mutex::new(Some(prepared.resource)),
            lifecycle: Mutex::new(WorthQueryRuntimeArtifactLifecycle {
                owner_count: 1,
                borrow_count: 0,
                lease_count: 0,
                lifecycle_generation: 1,
                borrow_generation: 0,
                lease_generation: 0,
                disposed: false,
                disposition: WorthQueryArtifactDisposition::Produced,
                counters: WorthQueryArtifactLifecycleCounters {
                    production_admissions: 1,
                    owner_registrations: 1,
                    retained_bytes,
                    peak_retained_bytes: retained_bytes,
                    ..WorthQueryArtifactLifecycleCounters::default()
                },
            }),
        })
    }

    pub(super) fn binding(&self) -> &WorthQueryRuntimeArtifactBinding {
        &self.binding
    }

    pub(super) fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        &self.semantic_projection
    }

    pub(super) const fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    pub(super) fn created_thread(&self) -> std::thread::ThreadId {
        self.created_thread
    }

    pub(super) fn snapshot(&self) -> WorthQueryArtifactOwnerSnapshot {
        let state = self.lifecycle();
        WorthQueryArtifactOwnerSnapshot::new(
            state.owner_count,
            state.borrow_count,
            state.lease_count,
            state.lifecycle_generation,
            state.disposed,
            state.counters,
        )
    }

    pub(super) fn disposition(&self) -> WorthQueryArtifactDisposition {
        self.lifecycle().disposition
    }

    pub(super) fn validate_generation(
        &self,
        generation: u64,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let mut state = self.lifecycle();
        state.counters.lifecycle_generation_checks += 1;
        if state.disposed {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::AlreadyDisposed,
                "artifact owner has already been disposed",
            ));
        }
        if state.lifecycle_generation != generation {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
                "artifact handle carries a stale lifecycle generation",
            ));
        }
        Ok(())
    }

    pub(super) fn validate_live(&self) -> Result<(), WorthQueryArtifactDenial> {
        if self.lifecycle().disposed {
            Err(self.denial(
                WorthQueryArtifactDenialKind::AlreadyDisposed,
                "artifact owner has already been disposed",
            ))
        } else {
            Ok(())
        }
    }

    pub(super) fn admit_transfer(&self, generation: u64) -> Result<u64, WorthQueryArtifactDenial> {
        use worth_query_installation::facade::WorthQueryArtifactMovePosture;

        if self.binding.contract.contract().carriage().movement()
            != WorthQueryArtifactMovePosture::Required
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::MovementForbidden,
                "installed artifact contract forbids ownership movement",
            ));
        }
        let mut state = self.lifecycle();
        ensure_live_generation(self, &mut state, generation)?;
        if state.borrow_count != 0 {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ActiveBorrow,
                "artifact cannot transfer while a borrow is active",
            ));
        }
        state.lifecycle_generation += 1;
        state.disposition = WorthQueryArtifactDisposition::Transferred;
        state.counters.transfer_admissions += 1;
        Ok(state.lifecycle_generation)
    }

    pub(super) fn admit_borrow(&self, generation: u64) -> Result<u64, WorthQueryArtifactDenial> {
        use worth_query_installation::facade::WorthQueryArtifactBorrowPosture;

        if self.binding.contract.contract().carriage().borrowing()
            != WorthQueryArtifactBorrowPosture::SharedReadOnly
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::BorrowForbidden,
                "installed artifact contract forbids shared borrowing",
            ));
        }
        let mut state = self.lifecycle();
        ensure_live_generation(self, &mut state, generation)?;
        state.borrow_count += 1;
        state.borrow_generation += 1;
        state.disposition = WorthQueryArtifactDisposition::Borrowed;
        state.counters.borrow_admissions += 1;
        Ok(state.borrow_generation)
    }

    pub(super) fn release_borrow(&self) {
        let mut state = self.lifecycle();
        debug_assert!(state.borrow_count > 0);
        state.borrow_count -= 1;
        state.borrow_generation += 1;
        if state.borrow_count == 0 {
            state.disposition = WorthQueryArtifactDisposition::Released;
        }
    }

    pub(super) fn admit_lease(&self, generation: u64) -> Result<u64, WorthQueryArtifactDenial> {
        use worth_query_installation::facade::{
            WorthQueryArtifactBorrowPosture, WorthQueryArtifactClonePosture,
        };

        let contract = self.binding.contract.contract();
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
        ensure_live_generation(self, &mut state, generation)?;
        state.lease_count += 1;
        state.lease_generation += 1;
        state.disposition = WorthQueryArtifactDisposition::Leased;
        state.counters.lease_admissions += 1;
        Ok(state.lease_generation)
    }

    pub(super) fn release_lease(&self, disposition: WorthQueryArtifactDisposition) -> bool {
        let should_dispose = {
            let mut state = self.lifecycle();
            debug_assert!(state.lease_count > 0);
            state.lease_count -= 1;
            state.lease_generation += 1;
            state.disposition = disposition;
            mark_disposed_if_unowned(&mut state)
        };
        self.dispose_provider_if_required(should_dispose);
        should_dispose
    }

    pub(super) fn release_owner(
        &self,
        disposition: WorthQueryArtifactDisposition,
        require_no_lease: bool,
    ) -> Result<bool, WorthQueryArtifactDenial> {
        let should_dispose = {
            let mut state = self.lifecycle();
            if state.disposed {
                return Err(self.denial(
                    WorthQueryArtifactDenialKind::AlreadyDisposed,
                    "artifact owner has already been disposed",
                ));
            }
            if state.borrow_count != 0 {
                return Err(self.denial(
                    WorthQueryArtifactDenialKind::ActiveBorrow,
                    "artifact cannot be disposed while a borrow is active",
                ));
            }
            if require_no_lease && state.lease_count != 0 {
                return Err(self.denial(
                    WorthQueryArtifactDenialKind::ActiveLease,
                    "artifact cannot be explicitly disposed while a lease is active",
                ));
            }
            debug_assert_eq!(state.owner_count, 1);
            state.owner_count = 0;
            state.lifecycle_generation += 1;
            state.disposition = disposition;
            mark_disposed_if_unowned(&mut state)
        };
        self.dispose_provider_if_required(should_dispose);
        Ok(should_dispose)
    }

    fn dispose_provider_if_required(&self, should_dispose: bool) {
        if !should_dispose {
            return;
        }
        let resource = self
            .resource
            .lock()
            .expect("artifact provider resource lock must remain available")
            .take()
            .expect("live artifact owner retains exactly one provider resource");
        resource.dispose();
    }

    fn denial(
        &self,
        kind: WorthQueryArtifactDenialKind,
        detail: &'static str,
    ) -> WorthQueryArtifactDenial {
        WorthQueryArtifactDenial::new(
            kind,
            Some(self.binding.contract.contract().family().as_str()),
            detail,
        )
    }

    fn lifecycle(&self) -> std::sync::MutexGuard<'_, WorthQueryRuntimeArtifactLifecycle> {
        self.lifecycle
            .lock()
            .expect("artifact lifecycle lock must remain available")
    }
}

fn ensure_live_generation(
    owner: &WorthQueryRuntimeArtifactOwner,
    state: &mut WorthQueryRuntimeArtifactLifecycle,
    generation: u64,
) -> Result<(), WorthQueryArtifactDenial> {
    state.counters.lifecycle_generation_checks += 1;
    if state.disposed {
        return Err(owner.denial(
            WorthQueryArtifactDenialKind::AlreadyDisposed,
            "artifact owner has already been disposed",
        ));
    }
    if state.lifecycle_generation != generation {
        return Err(owner.denial(
            WorthQueryArtifactDenialKind::StaleLifecycleGeneration,
            "artifact handle carries a stale lifecycle generation",
        ));
    }
    Ok(())
}

fn mark_disposed_if_unowned(state: &mut WorthQueryRuntimeArtifactLifecycle) -> bool {
    if state.owner_count == 0 && state.lease_count == 0 && !state.disposed {
        state.disposed = true;
        state.disposition = WorthQueryArtifactDisposition::Disposed;
        state.counters.provider_disposals += 1;
        true
    } else {
        false
    }
}

impl Drop for WorthQueryRuntimeArtifactOwner {
    fn drop(&mut self) {
        let Some(resource) = self
            .resource
            .get_mut()
            .expect("artifact provider resource lock must remain available")
            .take()
        else {
            return;
        };
        resource.dispose();
    }
}
