/// Diagnostic posture for explaining plugin contributions admitted by a slot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PluginSlotDiagnostics {
    ExplainContributions,
    Minimal,
}

impl PluginSlotDiagnostics {
    pub fn explain_contributions() -> Self {
        Self::ExplainContributions
    }

    pub fn minimal() -> Self {
        Self::Minimal
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::ExplainContributions => "explain_contributions",
            Self::Minimal => "minimal",
        }
    }
}
