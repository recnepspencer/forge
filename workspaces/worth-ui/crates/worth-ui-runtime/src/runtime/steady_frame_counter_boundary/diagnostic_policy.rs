#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WorthUiSteadyFrameDiagnosticPolicy {
    #[default]
    Minimal,
    Standard,
    Support,
}

impl WorthUiSteadyFrameDiagnosticPolicy {
    pub(crate) fn allows_frame_path_report_materialization(self) -> bool {
        matches!(self, Self::Support)
    }
}
