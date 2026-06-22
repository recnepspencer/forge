#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphObligationRegistrationDenialKind {
    EmptySelectorValue,
    EmptyRegistrationCatalog,
    ConflictingRegistrationForRule,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationRegistrationDenial {
    kind: ForgeQueryGraphObligationRegistrationDenialKind,
    message: String,
}

impl ForgeQueryGraphObligationRegistrationDenial {
    pub(super) fn new(
        kind: ForgeQueryGraphObligationRegistrationDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &ForgeQueryGraphObligationRegistrationDenialKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ForgeQueryGraphObligationRegistrationDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ForgeQueryGraphObligationRegistrationDenial {}
