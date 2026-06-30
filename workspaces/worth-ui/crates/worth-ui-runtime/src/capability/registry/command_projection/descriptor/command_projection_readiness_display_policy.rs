/// Display policy for command-owned readiness.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandProjectionReadinessDisplayPolicy {
    HideReadiness,
    ShowReadiness,
    DisableUnavailableCommands,
}

impl CommandProjectionReadinessDisplayPolicy {
    pub fn digest_basis(self) -> &'static str {
        match self {
            Self::HideReadiness => "hide_readiness",
            Self::ShowReadiness => "show_readiness",
            Self::DisableUnavailableCommands => "disable_unavailable_commands",
        }
    }
}
