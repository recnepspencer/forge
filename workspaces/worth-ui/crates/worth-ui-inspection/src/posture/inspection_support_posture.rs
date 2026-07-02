#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSupportPosture {
    Supported,
    DiagnosticOnly,
    Unsupported,
    WrongWorld,
    Deferred,
}
