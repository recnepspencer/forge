use super::WorthUiPresentationAsyncPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationAsyncObservation {
    posture: WorthUiPresentationAsyncPosture,
    signal_graph_instance: u64,
}

impl WorthUiPresentationAsyncObservation {
    pub(crate) const fn new(
        posture: WorthUiPresentationAsyncPosture,
        signal_graph_instance: u64,
    ) -> Self {
        Self {
            posture,
            signal_graph_instance,
        }
    }

    pub const fn posture(self) -> WorthUiPresentationAsyncPosture {
        self.posture
    }

    pub const fn signal_graph_instance(self) -> u64 {
        self.signal_graph_instance
    }
}
