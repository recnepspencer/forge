#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphObligationRegistrationDenialKind {
    EmptySelectorValue,
    InvalidAspectPath,
    EmptyRegistrationCatalog,
    ConflictingRegistrationForRule,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationRegistrationDenial {
    kind: WorthQueryGraphObligationRegistrationDenialKind,
    message: String,
}

impl WorthQueryGraphObligationRegistrationDenial {
    pub(super) fn new(
        kind: WorthQueryGraphObligationRegistrationDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &WorthQueryGraphObligationRegistrationDenialKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for WorthQueryGraphObligationRegistrationDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WorthQueryGraphObligationRegistrationDenial {}
