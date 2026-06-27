#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IconFamily {
    Command,
    Surface,
    Status,
    RuntimeOutcome,
    Navigation,
    Toolbar,
    CustomAdmitted,
    Unknown(String),
}

impl IconFamily {
    pub fn command() -> Self {
        Self::Command
    }

    pub fn surface() -> Self {
        Self::Surface
    }

    pub fn status() -> Self {
        Self::Status
    }

    pub fn runtime_outcome() -> Self {
        Self::RuntimeOutcome
    }

    pub fn navigation() -> Self {
        Self::Navigation
    }

    pub fn toolbar() -> Self {
        Self::Toolbar
    }

    pub fn custom_admitted() -> Self {
        Self::CustomAdmitted
    }

    pub fn unknown_for_diagnostics(name: impl Into<String>) -> Self {
        Self::Unknown(name.into())
    }

    pub(crate) fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Command => "command".to_string(),
            Self::Surface => "surface".to_string(),
            Self::Status => "status".to_string(),
            Self::RuntimeOutcome => "runtime_outcome".to_string(),
            Self::Navigation => "navigation".to_string(),
            Self::Toolbar => "toolbar".to_string(),
            Self::CustomAdmitted => "custom_admitted".to_string(),
            Self::Unknown(name) => format!("unknown:{}", length_prefixed(name)),
        }
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
