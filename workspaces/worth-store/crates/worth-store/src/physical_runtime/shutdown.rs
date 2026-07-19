use super::{
    diagnostics::{RuntimeCounterCells, RuntimeCounterSnapshot},
    lifecycle::{LifecycleCoordinator, LifecycleState, LifecycleStateSnapshot},
    root_admission::RootAdmission,
    DeclaredStoreRoot, RuntimeIdentity,
};
use std::sync::Arc;

pub(crate) struct ShutdownCoordinator {
    lifecycle: LifecycleCoordinator,
    root_admission: Option<RootAdmission>,
    counters: Arc<RuntimeCounterCells>,
}

impl ShutdownCoordinator {
    pub(crate) fn admitted(
        root_admission: RootAdmission,
        counters: Arc<RuntimeCounterCells>,
    ) -> Self {
        Self {
            lifecycle: LifecycleCoordinator::admitted(),
            root_admission: Some(root_admission),
            counters,
        }
    }

    pub(crate) fn declared_root(&self) -> &DeclaredStoreRoot {
        self.root_admission
            .as_ref()
            .expect("an admitted shutdown coordinator owns its root")
            .declared_root()
    }

    pub(crate) fn lifecycle_state(&self) -> Arc<LifecycleState> {
        self.lifecycle.observation_state()
    }

    pub(crate) fn lifecycle_snapshot(&self) -> LifecycleStateSnapshot {
        self.lifecycle.snapshot()
    }

    pub(crate) fn close(mut self, runtime_identity: RuntimeIdentity) -> ClosedRuntime {
        let declared_root = self.declared_root().clone();
        let root_admission = self
            .root_admission
            .take()
            .expect("close consumes the admitted root exactly once");
        let terminal = root_admission.release_after(|| {
            let terminal = self.lifecycle.finish_closed();
            self.counters.record_explicit_close();
            terminal
        });
        ClosedRuntime::new(
            runtime_identity,
            declared_root,
            self.counters.snapshot(terminal.generation),
        )
    }

    pub(crate) fn abort(mut self, runtime_identity: RuntimeIdentity) -> AbortedRuntime {
        let declared_root = self.declared_root().clone();
        let root_admission = self
            .root_admission
            .take()
            .expect("abort consumes the admitted root exactly once");
        let terminal = root_admission.release_after(|| {
            let terminal = self.lifecycle.finish_aborted();
            self.counters.record_explicit_abort();
            terminal
        });
        AbortedRuntime::new(
            runtime_identity,
            declared_root,
            self.counters.snapshot(terminal.generation),
        )
    }
}

impl Drop for ShutdownCoordinator {
    fn drop(&mut self) {
        let Some(root_admission) = self.root_admission.take() else {
            return;
        };

        root_admission.release_after(|| {
            self.lifecycle.finish_aborted();
            if std::thread::panicking() {
                self.counters.record_panic_termination();
            } else {
                self.counters.record_unexpected_drop();
            }
        });
    }
}

/// Non-authoritative final summary produced by an ordinary close.
pub struct ClosedRuntime(TerminalRuntimeSummary);

impl ClosedRuntime {
    fn new(
        runtime_identity: RuntimeIdentity,
        declared_root: DeclaredStoreRoot,
        counters: RuntimeCounterSnapshot,
    ) -> Self {
        Self(TerminalRuntimeSummary::new(
            runtime_identity,
            declared_root,
            counters,
        ))
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.0.runtime_identity
    }

    pub fn declared_store_root(&self) -> &DeclaredStoreRoot {
        &self.0.declared_root
    }

    pub const fn counters(&self) -> RuntimeCounterSnapshot {
        self.0.counters
    }
}

/// Non-authoritative final summary produced by an explicit abort.
pub struct AbortedRuntime(TerminalRuntimeSummary);

impl AbortedRuntime {
    fn new(
        runtime_identity: RuntimeIdentity,
        declared_root: DeclaredStoreRoot,
        counters: RuntimeCounterSnapshot,
    ) -> Self {
        Self(TerminalRuntimeSummary::new(
            runtime_identity,
            declared_root,
            counters,
        ))
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.0.runtime_identity
    }

    pub fn declared_store_root(&self) -> &DeclaredStoreRoot {
        &self.0.declared_root
    }

    pub const fn counters(&self) -> RuntimeCounterSnapshot {
        self.0.counters
    }
}

struct TerminalRuntimeSummary {
    runtime_identity: RuntimeIdentity,
    declared_root: DeclaredStoreRoot,
    counters: RuntimeCounterSnapshot,
}

impl TerminalRuntimeSummary {
    const fn new(
        runtime_identity: RuntimeIdentity,
        declared_root: DeclaredStoreRoot,
        counters: RuntimeCounterSnapshot,
    ) -> Self {
        Self {
            runtime_identity,
            declared_root,
            counters,
        }
    }
}
