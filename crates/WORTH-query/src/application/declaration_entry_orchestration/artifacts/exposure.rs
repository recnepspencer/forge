#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationExposureLevel {
    Ordinary,
    Checked,
    ProofVisible,
}

impl WorthQueryDeclarationEntryOrchestrationExposureLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::Checked => "checked",
            Self::ProofVisible => "proof_visible",
        }
    }
}
