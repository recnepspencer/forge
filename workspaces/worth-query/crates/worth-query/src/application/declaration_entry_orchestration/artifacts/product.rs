#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryOrchestrationProduct {
    RoutePlan,
    Receipt,
    Envelope,
}

impl WorthQueryDeclarationEntryOrchestrationProduct {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoutePlan => "route_plan",
            Self::Receipt => "receipt",
            Self::Envelope => "envelope",
        }
    }
}
