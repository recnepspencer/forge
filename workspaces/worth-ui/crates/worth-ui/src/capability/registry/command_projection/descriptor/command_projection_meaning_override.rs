/// Diagnostic-only proof that a projection tried to define command meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandProjectionMeaningOverride {
    Label,
    Readiness,
    Handler,
}

impl CommandProjectionMeaningOverride {
    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::Readiness => "readiness",
            Self::Handler => "handler",
        }
    }
}
