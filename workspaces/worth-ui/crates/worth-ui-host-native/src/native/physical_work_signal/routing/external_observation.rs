use worth_signal::facade::ResourceRequestHandle;

use super::super::identity::UiNativePhysicalSignalRuntimeIdentity;
use super::UiNativePhysicalSignalWork;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalExternalStatus {
    Pending,
    Completed,
    RejectedBeforeEffects,
    RejectedAfterRasterization,
    EffectsIndeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalExternalOrigin {
    NativeExternalPort,
    #[cfg(feature = "certification-support")]
    QualifiedExternalPort,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalSignalExternalBasis {
    runtime: UiNativePhysicalSignalRuntimeIdentity,
    work: UiNativePhysicalSignalWork,
    handle: ResourceRequestHandle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalSignalExternalObservation {
    basis: UiNativePhysicalSignalExternalBasis,
    status: UiNativePhysicalSignalExternalStatus,
    origin: UiNativePhysicalSignalExternalOrigin,
}

impl UiNativePhysicalSignalExternalBasis {
    pub(crate) const fn new(
        runtime: UiNativePhysicalSignalRuntimeIdentity,
        work: UiNativePhysicalSignalWork,
        handle: ResourceRequestHandle,
    ) -> Self {
        Self {
            runtime,
            work,
            handle,
        }
    }

    pub(crate) const fn observe(
        self,
        status: UiNativePhysicalSignalExternalStatus,
    ) -> UiNativePhysicalSignalExternalObservation {
        UiNativePhysicalSignalExternalObservation {
            basis: self,
            status,
            origin: UiNativePhysicalSignalExternalOrigin::NativeExternalPort,
        }
    }

    #[cfg(feature = "certification-support")]
    pub(crate) const fn observe_qualified_external(
        self,
        status: UiNativePhysicalSignalExternalStatus,
    ) -> UiNativePhysicalSignalExternalObservation {
        UiNativePhysicalSignalExternalObservation {
            basis: self,
            status,
            origin: UiNativePhysicalSignalExternalOrigin::QualifiedExternalPort,
        }
    }
}

impl UiNativePhysicalSignalExternalObservation {
    pub(crate) const fn runtime(self) -> UiNativePhysicalSignalRuntimeIdentity {
        self.basis.runtime
    }

    pub(crate) const fn work(self) -> UiNativePhysicalSignalWork {
        self.basis.work
    }

    pub(crate) const fn status(self) -> UiNativePhysicalSignalExternalStatus {
        self.status
    }

    pub(crate) const fn origin(self) -> UiNativePhysicalSignalExternalOrigin {
        self.origin
    }

    pub(crate) const fn handle(self) -> ResourceRequestHandle {
        self.basis.handle
    }
}
