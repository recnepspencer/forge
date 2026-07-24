#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainHandleDenialKind {
    DomainNotInstalled,
    ForeignRuntime,
    StaleInstallationGeneration,
    PackageIdentityChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainHandleDenial {
    kind: WorthQueryDomainHandleDenialKind,
}

impl WorthQueryDomainHandleDenial {
    pub const fn new(kind: WorthQueryDomainHandleDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthQueryDomainHandleDenialKind {
        self.kind
    }
}

impl std::fmt::Display for WorthQueryDomainHandleDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "installed domain handle denied: {:?}", self.kind)
    }
}

impl std::error::Error for WorthQueryDomainHandleDenial {}
