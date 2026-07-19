#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationBridgeTruthContext {
    Current,
    Historical,
    Preview,
}

impl WorthQueryDeclarationBridgeTruthContext {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Historical => "historical",
            Self::Preview => "preview",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationBridgeContinuationMode {
    RuntimeRoute,
    TruthView,
    PreviewSession,
    PreviewPromotion,
    SubscriptionPreparation,
    WritebackPreparation,
}

impl WorthQueryDeclarationBridgeContinuationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeRoute => "runtime_route",
            Self::TruthView => "truth_view",
            Self::PreviewSession => "preview_session",
            Self::PreviewPromotion => "preview_promotion",
            Self::SubscriptionPreparation => "subscription_preparation",
            Self::WritebackPreparation => "writeback_preparation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBridgeContinuationRequest {
    mode: WorthQueryDeclarationBridgeContinuationMode,
    truth_context: WorthQueryDeclarationBridgeTruthContext,
}

impl WorthQueryDeclarationBridgeContinuationRequest {
    pub fn new(
        mode: WorthQueryDeclarationBridgeContinuationMode,
        truth_context: WorthQueryDeclarationBridgeTruthContext,
    ) -> Self {
        Self {
            mode,
            truth_context,
        }
    }

    pub fn mode(self) -> WorthQueryDeclarationBridgeContinuationMode {
        self.mode
    }

    pub fn truth_context(self) -> WorthQueryDeclarationBridgeTruthContext {
        self.truth_context
    }
}
