mod reachability;

pub use reachability::{UiEguiRawInputIngressOutcome, UiEguiRawInputReachability};

pub(super) fn observe_raw_input(raw_input: &egui::RawInput) -> UiEguiRawInputIngressOutcome {
    UiEguiRawInputIngressOutcome::Unsupported(UiEguiRawInputReachability::inspect(raw_input))
}
