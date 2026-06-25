use super::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

impl WorthUiRuntimeFactId {
    pub fn live_view_declaration(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LiveViewDeclaration, identity)
    }

    pub fn live_view_state_binding(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LiveViewStateBinding, identity)
    }

    pub fn live_view_state_value(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LiveViewStateValue, identity)
    }

    pub fn live_view_control_projection(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewControlProjection,
            identity,
        )
    }

    pub fn live_view_control_options(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LiveViewControlOptions, identity)
    }

    pub fn live_view_conditional_projection(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewConditionalProjection,
            identity,
        )
    }

    pub fn live_view_participation(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LiveViewParticipation, identity)
    }

    pub fn live_view_expression_declaration(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewExpressionDeclaration,
            identity,
        )
    }

    pub fn live_view_expression_projection(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewExpressionProjection,
            identity,
        )
    }

    pub fn live_view_expression_output(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LiveViewExpressionOutput, identity)
    }

    pub fn live_view_readiness_projection(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewReadinessProjection,
            identity,
        )
    }

    pub fn live_view_interaction_intent(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewInteractionIntent,
            identity,
        )
    }

    pub fn live_view_payload_projection(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewPayloadProjection,
            identity,
        )
    }
}
