use crate::runtime::{
    WorthUiInteractionOperabilityBasis, WorthUiInteractionOperabilityPosture,
    WorthUiInteractionOperabilityReceipt,
};

use super::{WorthUiPrimitiveFocusPosture, WorthUiPrimitiveResolvedCursorPosture};

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
pub enum WorthUiPrimitiveActivationPosture {
    Eligible,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveSelectionPosture {
    Unselected,
    Selected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveActivationAffordanceReceipt {
    activation_posture: WorthUiPrimitiveActivationPosture,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    focus: WorthUiPrimitiveFocusPosture,
    operability: WorthUiPrimitiveOperabilityReceipt,
}

impl WorthUiPrimitiveOperabilityReceipt {
    pub(super) fn from_interaction_operability(
        operability: &WorthUiInteractionOperabilityReceipt,
    ) -> Self {
        Self {
            posture: primitive_operability_posture(operability.posture()),
            basis: primitive_operability_basis(operability.basis()),
        }
    }

    pub fn posture(&self) -> WorthUiPrimitiveOperabilityPosture {
        self.posture
    }

    pub fn basis(&self) -> WorthUiPrimitiveOperabilityBasis {
        self.basis
    }

    pub(super) fn can_activate(&self) -> bool {
        self.posture == WorthUiPrimitiveOperabilityPosture::Enabled
    }

    fn can_focus(&self) -> bool {
        self.posture == WorthUiPrimitiveOperabilityPosture::Enabled
    }

    pub fn disabled_posture(&self) -> bool {
        self.posture == WorthUiPrimitiveOperabilityPosture::Disabled
    }
}

fn primitive_operability_posture(
    posture: WorthUiInteractionOperabilityPosture,
) -> WorthUiPrimitiveOperabilityPosture {
    match posture {
        WorthUiInteractionOperabilityPosture::Eligible => {
            WorthUiPrimitiveOperabilityPosture::Enabled
        }
        WorthUiInteractionOperabilityPosture::Disabled
        | WorthUiInteractionOperabilityPosture::ReadinessDisabled => {
            WorthUiPrimitiveOperabilityPosture::Disabled
        }
        WorthUiInteractionOperabilityPosture::Readonly => {
            WorthUiPrimitiveOperabilityPosture::Readonly
        }
        WorthUiInteractionOperabilityPosture::Inert => WorthUiPrimitiveOperabilityPosture::Inert,
        WorthUiInteractionOperabilityPosture::Unsupported
        | WorthUiInteractionOperabilityPosture::Denied => {
            WorthUiPrimitiveOperabilityPosture::Disabled
        }
    }
}

fn primitive_operability_basis(
    basis: WorthUiInteractionOperabilityBasis,
) -> WorthUiPrimitiveOperabilityBasis {
    match basis {
        WorthUiInteractionOperabilityBasis::Enabled => WorthUiPrimitiveOperabilityBasis::Enabled,
        WorthUiInteractionOperabilityBasis::PrimitiveDisabled => {
            WorthUiPrimitiveOperabilityBasis::PrimitiveDisabled
        }
        WorthUiInteractionOperabilityBasis::InteractionReadinessDisabled => {
            WorthUiPrimitiveOperabilityBasis::InteractionReadinessDisabled
        }
        WorthUiInteractionOperabilityBasis::UnsupportedCommandTarget
        | WorthUiInteractionOperabilityBasis::NonFocusableTarget
        | WorthUiInteractionOperabilityBasis::GestureMismatch
        | WorthUiInteractionOperabilityBasis::UnsupportedInteraction
        | WorthUiInteractionOperabilityBasis::GraphDenied => {
            WorthUiPrimitiveOperabilityBasis::InteractionReadinessDisabled
        }
    }
}

impl WorthUiPrimitiveSelectionPosture {
    pub(super) fn from_selected(selected: bool) -> Self {
        if selected {
            Self::Selected
        } else {
            Self::Unselected
        }
    }

    pub fn is_selected(self) -> bool {
        self == Self::Selected
    }
}

impl WorthUiPrimitiveActivationAffordanceReceipt {
    pub(super) fn resolve(
        resolved_cursor: WorthUiPrimitiveResolvedCursorPosture,
        authored_focus: WorthUiPrimitiveFocusPosture,
        operability: WorthUiPrimitiveOperabilityReceipt,
    ) -> Self {
        let activation_posture = if operability.can_activate() {
            WorthUiPrimitiveActivationPosture::Eligible
        } else {
            WorthUiPrimitiveActivationPosture::Denied
        };
        let cursor = if activation_posture == WorthUiPrimitiveActivationPosture::Denied {
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
            activation_posture,
            cursor,
            focus,
            operability,
        }
    }

    pub fn activation_posture(&self) -> WorthUiPrimitiveActivationPosture {
        self.activation_posture
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn focus(&self) -> WorthUiPrimitiveFocusPosture {
        self.focus
    }

    pub fn operability(&self) -> &WorthUiPrimitiveOperabilityReceipt {
        &self.operability
    }
}
