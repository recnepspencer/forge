#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceReportFieldParticipation {
    Participating,
    DiagnosticNonParticipating,
}

impl EvidenceReportFieldParticipation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Participating => "participating",
            Self::DiagnosticNonParticipating => "diagnostic-nonparticipating",
        }
    }

    pub(crate) fn participates_in_report_identity(self) -> bool {
        self == Self::Participating
    }
}
