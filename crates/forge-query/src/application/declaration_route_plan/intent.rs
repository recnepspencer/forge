#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationRouteIntent {
    Auto,
    RelationalOnly,
    BridgeOnly,
    SignalOnly,
    RelationalAndBridge,
    DeferredRouting,
}

impl ForgeQueryDeclarationRouteIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::RelationalOnly => "relational_only",
            Self::BridgeOnly => "bridge_only",
            Self::SignalOnly => "signal_only",
            Self::RelationalAndBridge => "relational_and_bridge",
            Self::DeferredRouting => "deferred_routing",
        }
    }
}
