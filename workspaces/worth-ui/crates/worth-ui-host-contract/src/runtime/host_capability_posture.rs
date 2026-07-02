#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostCapabilityPosture {
    Available,
    Missing,
    Ambiguous,
    DiagnosticOnly,
}
