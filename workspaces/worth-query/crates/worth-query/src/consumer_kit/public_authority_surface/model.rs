#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryPublicAuthorityOwner {
    Identity,
    IdentityEvolution,
    Historical,
    BasisLifecycle,
    IntentAdmission,
    Subscription,
    CausalInspection,
    Preview,
    Facade,
}

impl WorthQueryPublicAuthorityOwner {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::IdentityEvolution => "identity-evolution",
            Self::Historical => "historical",
            Self::BasisLifecycle => "basis-lifecycle",
            Self::IntentAdmission => "intent-admission",
            Self::Subscription => "subscription",
            Self::CausalInspection => "causal-inspection",
            Self::Preview => "preview",
            Self::Facade => "facade",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryPublicAuthoritySurfaceClass {
    OrdinaryDeclarativeApi,
    SealedPhaseApi,
    ReadOnlyProjection,
    CertificationOnlyApi,
    InternalAdapter,
    DeleteBeforeCloseout,
    RemovedSurface,
}

impl WorthQueryPublicAuthoritySurfaceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryDeclarativeApi => "ordinary-declarative-api",
            Self::SealedPhaseApi => "sealed-phase-api",
            Self::ReadOnlyProjection => "read-only-projection",
            Self::CertificationOnlyApi => "certification-only-api",
            Self::InternalAdapter => "internal-adapter",
            Self::DeleteBeforeCloseout => "delete-before-closeout",
            Self::RemovedSurface => "removed-surface",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicAuthoritySurfaceRow {
    symbol: &'static str,
    source_path: &'static str,
    source_probe: &'static str,
    facade_path: Option<&'static str>,
    facade_probe: Option<&'static str>,
    operational_consumer: &'static str,
    owner: WorthQueryPublicAuthorityOwner,
    current_class: WorthQueryPublicAuthoritySurfaceClass,
    target_class: WorthQueryPublicAuthoritySurfaceClass,
    replacement: &'static str,
}

impl WorthQueryPublicAuthoritySurfaceRow {
    pub(crate) const fn new(
        symbol: &'static str,
        source_path: &'static str,
        source_probe: &'static str,
        facade_path: Option<&'static str>,
        facade_probe: Option<&'static str>,
        operational_consumer: &'static str,
        owner: WorthQueryPublicAuthorityOwner,
        current_class: WorthQueryPublicAuthoritySurfaceClass,
        target_class: WorthQueryPublicAuthoritySurfaceClass,
        replacement: &'static str,
    ) -> Self {
        Self {
            symbol,
            source_path,
            source_probe,
            facade_path,
            facade_probe,
            operational_consumer,
            owner,
            current_class,
            target_class,
            replacement,
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.symbol
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn source_probe(&self) -> &'static str {
        self.source_probe
    }

    pub fn facade_path(&self) -> Option<&'static str> {
        self.facade_path
    }

    pub fn facade_probe(&self) -> Option<&'static str> {
        self.facade_probe
    }

    pub fn operational_consumer(&self) -> &'static str {
        self.operational_consumer
    }

    pub fn owner(&self) -> WorthQueryPublicAuthorityOwner {
        self.owner
    }

    pub fn current_class(&self) -> WorthQueryPublicAuthoritySurfaceClass {
        self.current_class
    }

    pub fn target_class(&self) -> WorthQueryPublicAuthoritySurfaceClass {
        self.target_class
    }

    pub fn replacement(&self) -> &'static str {
        self.replacement
    }
}
