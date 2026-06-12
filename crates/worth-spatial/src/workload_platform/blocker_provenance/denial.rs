#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadBlockerProvenanceDenialKind {
    OutcomeDidNotReportIntegrityMismatch,
    OutcomeDidNotExplainBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadBlockerProvenanceDenial {
    kind: WorkloadBlockerProvenanceDenialKind,
    human_reason: String,
}

impl WorkloadBlockerProvenanceDenial {
    pub(crate) fn new(
        kind: WorkloadBlockerProvenanceDenialKind,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> WorkloadBlockerProvenanceDenialKind {
        self.kind
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
