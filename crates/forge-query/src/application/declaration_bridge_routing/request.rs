#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationBridgeTruthContext {
    Current,
    Historical,
    Preview,
}

impl ForgeQueryDeclarationBridgeTruthContext {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Historical => "historical",
            Self::Preview => "preview",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationBridgeContinuationMode {
    RuntimeRoute,
    TruthView,
    PreviewSession,
    PreviewPromotion,
    SubscriptionPreparation,
    WritebackPreparation,
}

impl ForgeQueryDeclarationBridgeContinuationMode {
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
pub struct ForgeQueryDeclarationBridgeContinuationRequest {
    mode: ForgeQueryDeclarationBridgeContinuationMode,
    truth_context: ForgeQueryDeclarationBridgeTruthContext,
}

impl ForgeQueryDeclarationBridgeContinuationRequest {
    pub fn new(
        mode: ForgeQueryDeclarationBridgeContinuationMode,
        truth_context: ForgeQueryDeclarationBridgeTruthContext,
    ) -> Self {
        Self {
            mode,
            truth_context,
        }
    }

    pub fn mode(self) -> ForgeQueryDeclarationBridgeContinuationMode {
        self.mode
    }

    pub fn truth_context(self) -> ForgeQueryDeclarationBridgeTruthContext {
        self.truth_context
    }
}
