use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiInteractionActivationRequest, WorthUiInteractionKind, WorthUiInteractionPayload,
    WorthUiInteractionReadiness, WorthUiInteractionReceipt, WorthUiMountedInteractionGesture,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveOperabilityPosture {
    Enabled,
    Disabled,
    Readonly,
    Inert,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveOperabilityBasis {
    Enabled,
    PrimitiveDisabled,
    InteractionReadinessDisabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveOperabilityReceipt {
    posture: WorthUiPrimitiveOperabilityPosture,
    basis: WorthUiPrimitiveOperabilityBasis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveActivationAffordanceReceipt {
    can_activate: bool,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    focus: WorthUiPrimitiveFocusPosture,
    operability: WorthUiPrimitiveOperabilityReceipt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveInteractionReceipt {
    kind: WorthUiPrimitiveInteractionKind,
    cursor: WorthUiPrimitiveCursorPosture,
    focus: WorthUiPrimitiveFocusPosture,
    disabled: bool,
    selected: bool,
    operability: WorthUiPrimitiveOperabilityReceipt,
    affordance: WorthUiPrimitiveActivationAffordanceReceipt,
    interaction: WorthUiInteractionReceipt,
}

impl WorthUiPrimitiveInteractionReceipt {
    pub(crate) fn new(
        kind: WorthUiInteractionKind,
        cursor: WorthUiPrimitiveCursorPosture,
        focus: WorthUiPrimitiveFocusPosture,
        disabled: bool,
        selected: bool,
        resolved_cursor: WorthUiPrimitiveResolvedCursorPosture,
        interaction: WorthUiInteractionReceipt,
    ) -> Self {
        let operability =
            WorthUiPrimitiveOperabilityReceipt::resolve(disabled, interaction.readiness());
        let affordance = WorthUiPrimitiveActivationAffordanceReceipt::resolve(
            resolved_cursor,
            focus,
            operability,
        );
        Self {
            kind: primitive_kind(kind),
            cursor,
            focus,
            disabled,
            selected,
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

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn selected(&self) -> bool {
        self.selected
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

    pub fn activation_request(
        &self,
        surface_id: &SurfaceId,
        gesture: WorthUiMountedInteractionGesture,
    ) -> WorthUiInteractionActivationRequest {
        WorthUiInteractionActivationRequest::new(
            surface_id.clone(),
            self.interaction_id(),
            self.interaction.kind(),
            gesture,
        )
    }
}

impl WorthUiPrimitiveOperabilityReceipt {
    fn resolve(primitive_disabled: bool, readiness: WorthUiInteractionReadiness) -> Self {
        if primitive_disabled {
            return Self {
                posture: WorthUiPrimitiveOperabilityPosture::Disabled,
                basis: WorthUiPrimitiveOperabilityBasis::PrimitiveDisabled,
            };
        }
        if readiness == WorthUiInteractionReadiness::Disabled {
            return Self {
                posture: WorthUiPrimitiveOperabilityPosture::Disabled,
                basis: WorthUiPrimitiveOperabilityBasis::InteractionReadinessDisabled,
            };
        }
        Self {
            posture: WorthUiPrimitiveOperabilityPosture::Enabled,
            basis: WorthUiPrimitiveOperabilityBasis::Enabled,
        }
    }

    pub fn posture(&self) -> WorthUiPrimitiveOperabilityPosture {
        self.posture
    }

    pub fn basis(&self) -> WorthUiPrimitiveOperabilityBasis {
        self.basis
    }

    pub fn can_activate(&self) -> bool {
        self.posture == WorthUiPrimitiveOperabilityPosture::Enabled
    }

    pub fn can_focus(&self) -> bool {
        self.posture == WorthUiPrimitiveOperabilityPosture::Enabled
    }

    pub fn disabled_posture(&self) -> bool {
        self.posture == WorthUiPrimitiveOperabilityPosture::Disabled
    }
}

impl WorthUiPrimitiveActivationAffordanceReceipt {
    fn resolve(
        resolved_cursor: WorthUiPrimitiveResolvedCursorPosture,
        authored_focus: WorthUiPrimitiveFocusPosture,
        operability: WorthUiPrimitiveOperabilityReceipt,
    ) -> Self {
        let cursor = if !operability.can_activate() {
            WorthUiPrimitiveResolvedCursorPosture::NotAllowed
        } else {
            resolved_cursor
        };
        let focus = if operability.can_focus() {
            authored_focus
        } else {
            WorthUiPrimitiveFocusPosture::None
        };
        Self {
            can_activate: operability.can_activate(),
            cursor,
            focus,
            operability,
        }
    }

    pub fn can_activate(&self) -> bool {
        self.can_activate
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn focus(&self) -> WorthUiPrimitiveFocusPosture {
        self.focus
    }

    pub fn disabled_posture(&self) -> bool {
        self.operability.disabled_posture()
    }

    pub fn operability(&self) -> &WorthUiPrimitiveOperabilityReceipt {
        &self.operability
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
