use worth_store_physical_integrity::PhysicalIntegrityScrubWindow;

use crate::physical_runtime::{LifecycleGeneration, ScrubPhysicalAllocation};

use super::{
    progress::ManagedPhysicalIntegrityScrubProgress,
    request::{LazyIntegrityScrubWindows, ManagedPhysicalIntegrityScrubRequest},
    scheduling::{schedule_window, ScheduledIntegrityScrubWindow},
    state::{ManagedIntegrityScrubGate, ManagedIntegrityScrubLifecycle},
};

pub(in crate::physical_runtime) struct ManagedPhysicalIntegrityScrubHandle<'runtime, 'media> {
    allocation: ScrubPhysicalAllocation<'runtime>,
    lifecycle: ManagedIntegrityScrubLifecycle,
    windows: LazyIntegrityScrubWindows<'media>,
    pending_window: Option<PhysicalIntegrityScrubWindow<'media>>,
}

impl<'runtime, 'media> ManagedPhysicalIntegrityScrubHandle<'runtime, 'media> {
    pub(in crate::physical_runtime) fn start(
        request: ManagedPhysicalIntegrityScrubRequest<'runtime, 'media>,
    ) -> Self {
        let (allocation, windows, yield_after_windows) = request.into_parts();
        let generation = allocation.store_generation();
        Self {
            allocation,
            lifecycle: ManagedIntegrityScrubLifecycle::new(generation, yield_after_windows),
            windows,
            pending_window: None,
        }
    }

    pub(in crate::physical_runtime) fn next<F>(
        &mut self,
        current_generation: LifecycleGeneration,
        inspect: F,
    ) -> ManagedPhysicalIntegrityScrubProgress
    where
        F: for<'window> FnOnce(PhysicalIntegrityScrubWindow<'window>),
    {
        match self.lifecycle.gate(current_generation) {
            ManagedIntegrityScrubGate::Proceed => {}
            ManagedIntegrityScrubGate::Paused => {
                return ManagedPhysicalIntegrityScrubProgress::Paused;
            }
            ManagedIntegrityScrubGate::Cancelled => {
                return ManagedPhysicalIntegrityScrubProgress::Cancelled;
            }
            ManagedIntegrityScrubGate::Closed => {
                return ManagedPhysicalIntegrityScrubProgress::Closed;
            }
            ManagedIntegrityScrubGate::StaleRuntimeGeneration => {
                return ManagedPhysicalIntegrityScrubProgress::StaleRuntimeGeneration;
            }
        }
        let Some(window) = self.pending_window.take().or_else(|| self.windows.next()) else {
            return ManagedPhysicalIntegrityScrubProgress::Completed;
        };
        match schedule_window(
            window,
            self.allocation.store_identity(),
            self.allocation.bytes(),
        ) {
            ScheduledIntegrityScrubWindow::Inspect(window) => {
                let ordinal = window.ordinal();
                inspect(window.reborrow());
                self.lifecycle.record_completed_window();
                ManagedPhysicalIntegrityScrubProgress::WindowInspected { ordinal }
            }
            ScheduledIntegrityScrubWindow::DeferredAllocation(window) => {
                let progress = ManagedPhysicalIntegrityScrubProgress::DeferredAllocation {
                    ordinal: window.ordinal(),
                    requested_bytes: window.artifact().byte_count(),
                };
                self.pending_window = Some(window);
                progress
            }
            ScheduledIntegrityScrubWindow::RejectedStoreScope { ordinal } => {
                ManagedPhysicalIntegrityScrubProgress::RejectedStoreScope { ordinal }
            }
        }
    }

    pub(in crate::physical_runtime) fn cancel(&mut self) {
        self.lifecycle.cancel();
    }

    pub(in crate::physical_runtime) fn close(&mut self) {
        self.lifecycle.close();
    }

    pub(in crate::physical_runtime) const fn generation(&self) -> LifecycleGeneration {
        self.lifecycle.generation()
    }
}
