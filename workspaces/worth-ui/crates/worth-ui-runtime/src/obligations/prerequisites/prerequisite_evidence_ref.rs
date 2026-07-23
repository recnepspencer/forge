use worth_ui_host_contract::WorthUiHostCapabilityReport;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiObligationPrerequisiteEvidenceRef {
    Host(WorthUiHostCapabilityReport),
}

impl UiObligationPrerequisiteEvidenceRef {
    pub fn host(&self) -> Option<&WorthUiHostCapabilityReport> {
        match self {
            Self::Host(report) => Some(report),
        }
    }
}
