use super::WorthUiPresentationAsyncObservation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationAsyncTerminalProjection {
    posture: &'static str,
    signal_graph_instance: u64,
}

impl WorthUiPresentationAsyncTerminalProjection {
    pub fn from_observation(observation: WorthUiPresentationAsyncObservation) -> Self {
        let posture = match observation.posture() {
            super::WorthUiPresentationAsyncPosture::Pending => "pending",
            super::WorthUiPresentationAsyncPosture::Current => "current",
            super::WorthUiPresentationAsyncPosture::Stale => "stale",
            super::WorthUiPresentationAsyncPosture::Failed => "failed",
            super::WorthUiPresentationAsyncPosture::Cancelled => "cancelled",
            super::WorthUiPresentationAsyncPosture::Superseded => "superseded",
            super::WorthUiPresentationAsyncPosture::Unresolved => "unresolved",
        };
        Self {
            posture,
            signal_graph_instance: observation.signal_graph_instance(),
        }
    }

    pub const fn posture(&self) -> &'static str {
        self.posture
    }

    pub const fn signal_graph_instance(&self) -> u64 {
        self.signal_graph_instance
    }
}
