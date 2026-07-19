use super::{
    availability::InstalledCapabilityStatus,
    diagnostics::RuntimeDiagnostics,
    observation::ObservationHandle,
    resource_lifecycle::ResourceLifecycle,
    root_admission::RootAdmission,
    shutdown::{AbortedRuntime, ClosedRuntime, ShutdownCoordinator},
    DeclaredStoreRoot, PhysicalCapability, RuntimeCounterSnapshot, RuntimeIdentity,
};

/// Sole move-only owner of the responsibilities installed at admission.
pub struct AdmittedPhysicalRuntime {
    runtime_identity: RuntimeIdentity,
    resource_lifecycle: ResourceLifecycle,
    diagnostics: RuntimeDiagnostics,
    shutdown: ShutdownCoordinator,
}

impl AdmittedPhysicalRuntime {
    pub(crate) fn from_admission(
        runtime_identity: RuntimeIdentity,
        root_admission: RootAdmission,
    ) -> Self {
        let diagnostics = RuntimeDiagnostics::admitted(runtime_identity);
        let resource_lifecycle = ResourceLifecycle::new(diagnostics.counter_cells());
        let shutdown = ShutdownCoordinator::admitted(root_admission, diagnostics.counter_cells());
        Self {
            runtime_identity,
            resource_lifecycle,
            diagnostics,
            shutdown,
        }
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.runtime_identity
    }

    pub fn declared_store_root(&self) -> &DeclaredStoreRoot {
        self.shutdown.declared_root()
    }

    pub fn observe(&self) -> ObservationHandle {
        ObservationHandle::new(
            self.runtime_identity,
            self.shutdown.lifecycle_state(),
            self.resource_lifecycle.acquire_observation(),
        )
    }

    pub fn installed_capabilities(&self) -> InstalledCapabilityStatus {
        self.diagnostics
            .record_capability_observations(PhysicalCapability::FAMILY_COUNT);
        InstalledCapabilityStatus::c3()
    }

    pub fn counters(&self) -> RuntimeCounterSnapshot {
        let lifecycle = self.shutdown.lifecycle_snapshot();
        self.diagnostics.snapshot(lifecycle.generation)
    }

    pub fn close(self) -> ClosedRuntime {
        let Self {
            runtime_identity,
            resource_lifecycle,
            diagnostics,
            shutdown,
        } = self;
        let closed = shutdown.close(runtime_identity);
        drop((resource_lifecycle, diagnostics));
        closed
    }

    pub fn abort(self) -> AbortedRuntime {
        let Self {
            runtime_identity,
            resource_lifecycle,
            diagnostics,
            shutdown,
        } = self;
        let aborted = shutdown.abort(runtime_identity);
        drop((resource_lifecycle, diagnostics));
        aborted
    }
}
