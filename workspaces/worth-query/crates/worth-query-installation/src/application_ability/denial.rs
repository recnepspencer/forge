#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryAbilityInstallationDenialKind {
    AbilityNotInstalled,
    AbilityPolicyNotInstalled,
    AbilityMeaningChanged,
    SchemaMeaningChanged,
    ForeignRuntime,
    StaleGeneration,
    PackageIdentityChanged,
    AuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAbilityInstallationDenial {
    kind: WorthQueryAbilityInstallationDenialKind,
    ability: String,
}

impl WorthQueryAbilityInstallationDenial {
    pub(crate) fn new(
        kind: WorthQueryAbilityInstallationDenialKind,
        ability: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            ability: ability.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryAbilityInstallationDenialKind {
        self.kind
    }

    pub fn ability(&self) -> &str {
        &self.ability
    }
}

impl std::fmt::Display for WorthQueryAbilityInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "installed application ability denied: {:?} ({})",
            self.kind, self.ability
        )
    }
}

impl std::error::Error for WorthQueryAbilityInstallationDenial {}
