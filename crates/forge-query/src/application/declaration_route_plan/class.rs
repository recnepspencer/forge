#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryLowerAuthorityRouteFamily {
    Relational,
    Bridge,
    Signal,
    Mixed,
    Deferred,
    Forbidden,
}

impl ForgeQueryLowerAuthorityRouteFamily {
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
pub enum ForgeQueryDeclarationRoutePlanClass {
    RelationalOnly,
    BridgeOnly,
    SignalOnly,
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRouteMultiplicity {
    Singular,
    PluralCapable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRouteIntentRequirement {
    Forbidden,
    Optional,
    Required,
}
