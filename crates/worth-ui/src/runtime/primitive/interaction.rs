mod activation;

use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionOperabilityReceipt, WorthUiInteractionPayload,
    WorthUiInteractionReadiness, WorthUiInteractionReceipt,
};

pub use activation::{
    WorthUiPrimitiveActivationAffordanceReceipt, WorthUiPrimitiveActivationPosture,
    WorthUiPrimitiveOperabilityBasis, WorthUiPrimitiveOperabilityPosture,
    WorthUiPrimitiveOperabilityReceipt, WorthUiPrimitiveSelectionPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveInteractionKind {
    None,
    Submit,
    Click,
    Command,
    Toggle,
    Open,
    Focus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveCursorPosture {
    Default,
    Pointer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveResolvedCursorPosture {
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
    Resize,
    NotAllowed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveFocusPosture {
    None,
    Focusable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveInteractionReceipt {
    kind: WorthUiPrimitiveInteractionKind,
    cursor: WorthUiPrimitiveCursorPosture,
    focus: WorthUiPrimitiveFocusPosture,
    selection_posture: WorthUiPrimitiveSelectionPosture,
    operability: WorthUiPrimitiveOperabilityReceipt,
    affordance: WorthUiPrimitiveActivationAffordanceReceipt,
    interaction: WorthUiInteractionReceipt,
}

impl WorthUiPrimitiveInteractionReceipt {
    pub(crate) fn from_graph_operability(
        kind: WorthUiInteractionKind,
        cursor: WorthUiPrimitiveCursorPosture,
        focus: WorthUiPrimitiveFocusPosture,
        selected: bool,
        resolved_cursor: WorthUiPrimitiveResolvedCursorPosture,
        interaction: WorthUiInteractionReceipt,
        graph_operability: &WorthUiInteractionOperabilityReceipt,
    ) -> Self {
        let operability =
            WorthUiPrimitiveOperabilityReceipt::from_interaction_operability(graph_operability);
        let affordance = WorthUiPrimitiveActivationAffordanceReceipt::resolve(
            resolved_cursor,
            focus,
            operability,
        );
        Self {
            kind: primitive_kind(kind),
            cursor,
            focus,
            selection_posture: WorthUiPrimitiveSelectionPosture::from_selected(selected),
            operability,
            affordance,
            interaction,
        }
    }

    pub fn kind(&self) -> WorthUiPrimitiveInteractionKind {
        self.kind
    }

    pub fn cursor(&self) -> WorthUiPrimitiveCursorPosture {
        self.cursor
    }

    pub fn focus(&self) -> WorthUiPrimitiveFocusPosture {
        self.focus
    }

    pub fn selection_posture(&self) -> WorthUiPrimitiveSelectionPosture {
        self.selection_posture
    }

    pub fn interaction_id(&self) -> &str {
        self.interaction.interaction_id()
    }

    pub fn submit_payload(&self) -> &WorthUiInteractionPayload {
        self.interaction.payload()
    }

    pub fn readiness(&self) -> WorthUiInteractionReadiness {
        self.interaction.readiness()
    }

    pub fn operability(&self) -> &WorthUiPrimitiveOperabilityReceipt {
        &self.operability
    }

    pub fn affordance(&self) -> &WorthUiPrimitiveActivationAffordanceReceipt {
        &self.affordance
    }

    pub fn lane_receipt(&self) -> &WorthUiInteractionReceipt {
        &self.interaction
    }
}

fn primitive_kind(kind: WorthUiInteractionKind) -> WorthUiPrimitiveInteractionKind {
    match kind {
        WorthUiInteractionKind::Click => WorthUiPrimitiveInteractionKind::Click,
        WorthUiInteractionKind::Submit => WorthUiPrimitiveInteractionKind::Submit,
        WorthUiInteractionKind::Command => WorthUiPrimitiveInteractionKind::Command,
        WorthUiInteractionKind::Toggle => WorthUiPrimitiveInteractionKind::Toggle,
        WorthUiInteractionKind::Open => WorthUiPrimitiveInteractionKind::Open,
        WorthUiInteractionKind::Focus => WorthUiPrimitiveInteractionKind::Focus,
    }
}
