use super::super::expression::{
    WorthUiLiveViewExpressionOutputValue, WorthUiLiveViewExpressionProjectionReceipt,
};
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewStateBindingReceipt, WorthUiLiveViewStateValue, WorthUiLiveViewTargetBinding,
    WorthUiQueryGraphExecutionReceipt,
};

use super::declaration::WorthUiLiveViewReadinessProjectionDeclaration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewValuePresencePosture {
    Present,
    Missing,
    Hidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewReadinessPosture {
    Enabled,
    DeniedMissingRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewValuePresenceReceipt {
    binding: WorthUiLiveViewStateBindingReceipt,
    posture: WorthUiLiveViewValuePresencePosture,
    presence_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewReadinessProjectionReceipt {
    live_view_id: String,
    target_binding: WorthUiLiveViewTargetBinding,
    readiness_id: String,
    required_bindings: Vec<WorthUiLiveViewValuePresenceReceipt>,
    expression_projection: WorthUiLiveViewExpressionProjectionReceipt,
    consumed_facts: Vec<crate::runtime::WorthUiRuntimeFactId>,
    posture: WorthUiLiveViewReadinessPosture,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    readiness_digest: u64,
}

impl WorthUiLiveViewValuePresenceReceipt {
    pub(crate) fn new(
        binding: WorthUiLiveViewStateBindingReceipt,
        value: Option<&WorthUiLiveViewStateValue>,
        participates: bool,
    ) -> Self {
        let posture = if !participates {
            WorthUiLiveViewValuePresencePosture::Hidden
        } else if value
            .map(WorthUiLiveViewStateValue::as_display_text)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
        {
            WorthUiLiveViewValuePresencePosture::Present
        } else {
            WorthUiLiveViewValuePresencePosture::Missing
        };
        let presence_digest = digest_parts([
            binding.live_view_id(),
            binding.binding_id(),
            binding.state_fact().as_str(),
            posture.token(),
        ]);
        Self {
            binding,
            posture,
            presence_digest,
        }
    }

    pub fn binding(&self) -> &WorthUiLiveViewStateBindingReceipt {
        &self.binding
    }

    pub fn posture(&self) -> WorthUiLiveViewValuePresencePosture {
        self.posture
    }

    pub fn presence_digest(&self) -> u64 {
        self.presence_digest
    }
}

impl WorthUiLiveViewReadinessProjectionReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        target_binding: WorthUiLiveViewTargetBinding,
        declaration: &WorthUiLiveViewReadinessProjectionDeclaration,
        required_bindings: Vec<WorthUiLiveViewValuePresenceReceipt>,
        expression_projection: WorthUiLiveViewExpressionProjectionReceipt,
        consumed_facts: Vec<crate::runtime::WorthUiRuntimeFactId>,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let posture = match expression_projection.output().value() {
            WorthUiLiveViewExpressionOutputValue::Boolean(true) => {
                WorthUiLiveViewReadinessPosture::Enabled
            }
            WorthUiLiveViewExpressionOutputValue::Boolean(false)
            | WorthUiLiveViewExpressionOutputValue::PayloadShape(_)
            | WorthUiLiveViewExpressionOutputValue::Text(_) => {
                WorthUiLiveViewReadinessPosture::DeniedMissingRequired
            }
        };
        let mut digest_parts_input = vec![
            live_view_id.to_owned(),
            target_binding.binding_digest().to_string(),
            declaration.readiness_id().to_owned(),
        ];
        digest_parts_input.extend(
            required_bindings
                .iter()
                .map(|receipt| receipt.presence_digest().to_string()),
        );
        digest_parts_input.push(expression_projection.expression_digest().to_string());
        digest_parts_input.push(posture.token().to_owned());
        let readiness_digest = digest_parts(digest_parts_input);
        Self {
            live_view_id: live_view_id.to_owned(),
            target_binding,
            readiness_id: declaration.readiness_id().to_owned(),
            required_bindings,
            expression_projection,
            consumed_facts,
            posture,
            graph_execution,
            readiness_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn target_binding(&self) -> &WorthUiLiveViewTargetBinding {
        &self.target_binding
    }

    pub fn readiness_id(&self) -> &str {
        &self.readiness_id
    }

    pub fn required_bindings(&self) -> &[WorthUiLiveViewValuePresenceReceipt] {
        &self.required_bindings
    }

    pub fn expression_projection(&self) -> &WorthUiLiveViewExpressionProjectionReceipt {
        &self.expression_projection
    }

    pub fn consumed_facts(&self) -> &[crate::runtime::WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn posture(&self) -> WorthUiLiveViewReadinessPosture {
        self.posture
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn readiness_digest(&self) -> u64 {
        self.readiness_digest
    }
}

impl WorthUiLiveViewValuePresencePosture {
    pub fn token(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
            Self::Hidden => "hidden",
        }
    }
}

impl WorthUiLiveViewReadinessPosture {
    pub fn token(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::DeniedMissingRequired => "denied_missing_required",
        }
    }

    pub fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}
