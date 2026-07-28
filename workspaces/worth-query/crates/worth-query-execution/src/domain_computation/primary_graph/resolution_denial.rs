#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPrincipalResolutionDenialKind {
    PrimaryGraphNotInstalled,
    BindingNotInstalled,
    ForeignRuntime,
    StaleInstalledSchema,
    ExpiredAuthentication,
    Cancelled,
    DeadlineExceeded,
    IdentityIndexUnavailable,
    CorruptIdentityIndex,
    UnknownPrincipal,
    DisabledPrincipal,
    AmbiguousPrincipal,
    MissingPrincipalTarget,
    AmbiguousPrincipalTarget,
    WrongPrincipalTargetKind,
    StalePrincipalProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPrincipalResolutionDenial {
    kind: WorthQueryPrincipalResolutionDenialKind,
    binding: String,
}

impl WorthQueryPrincipalResolutionDenial {
    pub(super) fn new(
        kind: WorthQueryPrincipalResolutionDenialKind,
        binding: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            binding: binding.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryPrincipalResolutionDenialKind {
        self.kind
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }
}

impl std::fmt::Display for WorthQueryPrincipalResolutionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application principal resolution denied: {:?} ({})",
            self.kind, self.binding
        )
    }
}

impl std::error::Error for WorthQueryPrincipalResolutionDenial {}
