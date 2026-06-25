use super::super::expression::WorthUiLiveViewExpressionProjectionReceipt;
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewControlProjectionReceipt, WorthUiLiveViewStateBindingReceipt,
    WorthUiQueryGraphExecutionReceipt,
};

use super::declaration::{
    WorthUiLiveViewConditionExpression, WorthUiLiveViewConditionalProjectionDeclaration,
    WorthUiLiveViewParticipationPosture,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionAdmissionCounters {
    conditional_count: usize,
    state_fact_lookup_count: usize,
    denial_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionReceipt {
    live_view_id: String,
    control: WorthUiLiveViewControlProjectionReceipt,
    condition: WorthUiLiveViewConditionExpression,
    consumed_binding: WorthUiLiveViewStateBindingReceipt,
    expression_projection: WorthUiLiveViewExpressionProjectionReceipt,
    participation: WorthUiLiveViewParticipationReceipt,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    conditional_projection_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewRetainedStatePosture {
    Retained,
    Dropped,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewParticipationReceipt {
    posture: WorthUiLiveViewParticipationPosture,
    layout: bool,
    paint: bool,
    events: bool,
    accessibility: bool,
    retained_state: WorthUiLiveViewRetainedStatePosture,
    participation_digest: u64,
}

impl WorthUiLiveViewConditionalProjectionAdmissionCounters {
    pub(crate) fn new(
        conditional_count: usize,
        state_fact_lookup_count: usize,
        denial_count: usize,
    ) -> Self {
        Self {
            conditional_count,
            state_fact_lookup_count,
            denial_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn conditional_count(self) -> usize {
        self.conditional_count
    }

    pub fn state_fact_lookup_count(self) -> usize {
        self.state_fact_lookup_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

impl WorthUiLiveViewConditionalProjectionReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        declaration: &WorthUiLiveViewConditionalProjectionDeclaration,
        control: WorthUiLiveViewControlProjectionReceipt,
        consumed_binding: WorthUiLiveViewStateBindingReceipt,
        expression_projection: WorthUiLiveViewExpressionProjectionReceipt,
        active_posture: WorthUiLiveViewParticipationPosture,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let participation = WorthUiLiveViewParticipationReceipt::from_posture(active_posture);
        let participation_digest_token = participation.participation_digest().to_string();
        let conditional_projection_digest = digest_parts([
            live_view_id,
            declaration.control_id(),
            declaration.condition().token(),
            consumed_binding.binding_digest().to_string().as_str(),
            expression_projection
                .expression_digest()
                .to_string()
                .as_str(),
            participation_digest_token.as_str(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            control,
            condition: declaration.condition().clone(),
            consumed_binding,
            expression_projection,
            participation,
            graph_execution,
            conditional_projection_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn control(&self) -> &WorthUiLiveViewControlProjectionReceipt {
        &self.control
    }

    pub fn condition(&self) -> &WorthUiLiveViewConditionExpression {
        &self.condition
    }

    pub fn consumed_binding(&self) -> &WorthUiLiveViewStateBindingReceipt {
        &self.consumed_binding
    }

    pub fn expression_projection(&self) -> &WorthUiLiveViewExpressionProjectionReceipt {
        &self.expression_projection
    }

    pub fn participation(&self) -> &WorthUiLiveViewParticipationReceipt {
        &self.participation
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn conditional_projection_digest(&self) -> u64 {
        self.conditional_projection_digest
    }
}

impl WorthUiLiveViewParticipationReceipt {
    pub(crate) fn from_posture(posture: WorthUiLiveViewParticipationPosture) -> Self {
        let (layout, paint, events, accessibility, retained_state) = match posture {
            WorthUiLiveViewParticipationPosture::Present => (
                true,
                true,
                true,
                true,
                WorthUiLiveViewRetainedStatePosture::Retained,
            ),
            WorthUiLiveViewParticipationPosture::AbsentRetainingState => (
                false,
                false,
                false,
                false,
                WorthUiLiveViewRetainedStatePosture::Retained,
            ),
            WorthUiLiveViewParticipationPosture::Unsupported => (
                false,
                false,
                false,
                false,
                WorthUiLiveViewRetainedStatePosture::Dropped,
            ),
        };
        let participation_digest = digest_parts([
            posture.token(),
            layout.to_string().as_str(),
            paint.to_string().as_str(),
            events.to_string().as_str(),
            accessibility.to_string().as_str(),
            retained_state.token(),
        ]);
        Self {
            posture,
            layout,
            paint,
            events,
            accessibility,
            retained_state,
            participation_digest,
        }
    }

    pub fn posture(&self) -> WorthUiLiveViewParticipationPosture {
        self.posture
    }

    pub fn participates_in_layout(&self) -> bool {
        self.layout
    }

    pub fn participates_in_paint(&self) -> bool {
        self.paint
    }

    pub fn participates_in_events(&self) -> bool {
        self.events
    }

    pub fn participates_in_accessibility(&self) -> bool {
        self.accessibility
    }

    pub fn retained_state(&self) -> WorthUiLiveViewRetainedStatePosture {
        self.retained_state
    }

    pub fn participation_digest(&self) -> u64 {
        self.participation_digest
    }
}

impl WorthUiLiveViewRetainedStatePosture {
    pub fn token(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::Dropped => "dropped",
        }
    }
}
