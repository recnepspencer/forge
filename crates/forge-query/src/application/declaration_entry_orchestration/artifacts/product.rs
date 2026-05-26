#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryOrchestrationProduct {
    RoutePlan,
    Receipt,
    Envelope,
}

impl ForgeQueryDeclarationEntryOrchestrationProduct {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoutePlan => "route_plan",
            Self::Receipt => "receipt",
            Self::Envelope => "envelope",
        }
    }
}
