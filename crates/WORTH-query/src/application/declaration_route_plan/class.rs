#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryLowerAuthorityRouteFamily {
    Relational,
    Bridge,
    Signal,
    Mixed,
    Deferred,
    Forbidden,
}

impl WorthQueryLowerAuthorityRouteFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Relational => "relational",
            Self::Bridge => "bridge",
            Self::Signal => "signal",
            Self::Mixed => "mixed",
            Self::Deferred => "deferred",
            Self::Forbidden => "forbidden",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationRoutePlanClass {
    RelationalOnly,
    BridgeOnly,
    SignalOnly,
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationRouteMultiplicity {
    Singular,
    PluralCapable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationRouteIntentRequirement {
    Forbidden,
    Optional,
    Required,
}
