/// State preservation class declared by a surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceStateClass {
    Ephemeral,
    Restorable,
    Persistent,
    InvalidForDiagnostics(String),
}

impl SurfaceStateClass {
    pub fn ephemeral() -> Self {
        Self::Ephemeral
    }

    pub fn restorable() -> Self {
        Self::Restorable
    }

    pub fn persistent() -> Self {
        Self::Persistent
    }

    pub fn invalid_for_diagnostics(name: impl Into<String>) -> Self {
        Self::InvalidForDiagnostics(name.into())
    }

    pub(crate) fn is_invalid(&self) -> bool {
        matches!(self, Self::InvalidForDiagnostics(_))
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Ephemeral => "ephemeral".to_owned(),
            Self::Restorable => "restorable".to_owned(),
            Self::Persistent => "persistent".to_owned(),
            Self::InvalidForDiagnostics(name) => format!("invalid:{name}"),
        }
    }
}
