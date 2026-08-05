#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPrincipalBindingInstallationDenialKind {
    BindingNotInstalled,
    BindingMeaningChanged,
    ForeignRuntime,
    StaleGeneration,
    PackageIdentityChanged,
    SchemaMeaningChanged,
    AuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPrincipalBindingInstallationDenial {
    kind: WorthQueryPrincipalBindingInstallationDenialKind,
    binding: String,
}

impl WorthQueryPrincipalBindingInstallationDenial {
    pub(crate) fn new(
        kind: WorthQueryPrincipalBindingInstallationDenialKind,
        binding: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            binding: binding.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryPrincipalBindingInstallationDenialKind {
        self.kind
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }
}

impl std::fmt::Display for WorthQueryPrincipalBindingInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "principal binding installation denied: {:?} ({})",
            self.kind, self.binding
        )
    }
}

impl std::error::Error for WorthQueryPrincipalBindingInstallationDenial {}
