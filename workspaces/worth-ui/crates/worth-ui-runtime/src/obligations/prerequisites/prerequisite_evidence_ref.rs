use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiObligationPrerequisiteEvidenceRef {
    Query(Box<WorthUiQueryPrerequisiteEvidence>),
    Host(WorthUiHostCapabilityReport),
}

impl UiObligationPrerequisiteEvidenceRef {
    pub fn query(&self) -> Option<&WorthUiQueryPrerequisiteEvidence> {
        match self {
            Self::Query(evidence) => Some(evidence),
            Self::Host(_) => None,
        }
    }

    pub fn host(&self) -> Option<&WorthUiHostCapabilityReport> {
        match self {
            Self::Host(report) => Some(report),
            Self::Query(_) => None,
        }
    }
}
