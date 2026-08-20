#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalShutdown {
    Disposed,
    RetainedObligations { active_requests: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalLifecycle {
    Running,
    Draining,
    Disposed,
}

impl super::UiNativePhysicalSignalOwner {
    pub(crate) fn shutdown(&mut self) -> UiNativePhysicalSignalShutdown {
        if self.lifecycle == UiNativePhysicalSignalLifecycle::Running {
            self.lifecycle = UiNativePhysicalSignalLifecycle::Draining;
        }
        let active_requests = self.worker.as_ref().map_or(
            0,
            super::worker::UiNativePhysicalSignalWorker::active_requests,
        );
        if active_requests != 0 {
            return UiNativePhysicalSignalShutdown::RetainedObligations { active_requests };
        }
        if let Some(worker) = self.worker.take() {
            self.terminal_telemetry = worker.telemetry();
            self.terminal_performed_transitions = worker.performed_transitions();
            self.terminal_performed_nodes = worker.performed_nodes();
        }
        self.route.clear();
        self.wake.clear();
        self.transition_observations.clear();
        self.lifecycle = UiNativePhysicalSignalLifecycle::Disposed;
        UiNativePhysicalSignalShutdown::Disposed
    }
}
