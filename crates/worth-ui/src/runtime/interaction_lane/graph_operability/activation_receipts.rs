use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionReceipt, WorthUiInteractionTarget,
    WorthUiMountedInteractionGesture,
};

use super::digest::activation_digest;
use super::WorthUiInteractionOperabilityReceipt;

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiMountedInteractionActivation {
    Eligible(WorthUiMountedInteractionActivationEligibleReceipt),
    Denied(WorthUiMountedInteractionActivationDeniedReceipt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedInteractionActivationEligibleReceipt {
    surface_id: SurfaceId,
    component_id: ComponentId,
    interaction_id: String,
    kind: WorthUiInteractionKind,
    gesture: WorthUiMountedInteractionGesture,
    receipt: WorthUiInteractionReceipt,
    operability: WorthUiInteractionOperabilityReceipt,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedInteractionActivationDeniedReceipt {
    surface_id: String,
    interaction_id: String,
    kind: WorthUiInteractionKind,
    gesture: WorthUiMountedInteractionGesture,
    target: WorthUiInteractionTarget,
    operability: WorthUiInteractionOperabilityReceipt,
    receipt_digest: u64,
}

impl WorthUiMountedInteractionActivation {
    pub fn eligible(&self) -> Option<&WorthUiMountedInteractionActivationEligibleReceipt> {
        match self {
            Self::Eligible(receipt) => Some(receipt),
            Self::Denied(_) => None,
        }
    }

    pub fn denied(&self) -> Option<&WorthUiMountedInteractionActivationDeniedReceipt> {
        match self {
            Self::Eligible(_) => None,
            Self::Denied(receipt) => Some(receipt),
        }
    }
}

impl WorthUiMountedInteractionActivationEligibleReceipt {
    pub(super) fn new(
        surface_id: SurfaceId,
        component_id: ComponentId,
        interaction_id: String,
        kind: WorthUiInteractionKind,
        gesture: WorthUiMountedInteractionGesture,
        receipt: WorthUiInteractionReceipt,
        operability: WorthUiInteractionOperabilityReceipt,
    ) -> Self {
        let receipt_digest = activation_digest(
            surface_id.as_str(),
            &interaction_id,
            kind,
            gesture,
            operability.receipt_digest(),
            receipt.receipt_digest(),
        );
        Self {
            surface_id,
            component_id,
            interaction_id,
            kind,
            gesture,
            receipt,
            operability,
            receipt_digest,
        }
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn kind(&self) -> WorthUiInteractionKind {
        self.kind
    }

    pub fn gesture(&self) -> WorthUiMountedInteractionGesture {
        self.gesture
    }

    pub fn operability(&self) -> &WorthUiInteractionOperabilityReceipt {
        &self.operability
    }

    pub fn query_graph_execution_digest(&self) -> u64 {
        self.operability.query_graph_execution_digest()
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }

    pub(crate) fn emit_interaction_receipt(self) -> WorthUiInteractionReceipt {
        self.receipt
    }
}

impl WorthUiMountedInteractionActivationDeniedReceipt {
    pub(super) fn new(
        surface_id: &str,
        interaction_id: &str,
        kind: WorthUiInteractionKind,
        gesture: WorthUiMountedInteractionGesture,
        target: WorthUiInteractionTarget,
        operability: WorthUiInteractionOperabilityReceipt,
    ) -> Self {
        let receipt_digest = activation_digest(
            surface_id,
            interaction_id,
            kind,
            gesture,
            operability.receipt_digest(),
            0,
        );
        Self {
            surface_id: surface_id.to_owned(),
            interaction_id: interaction_id.to_owned(),
            kind,
            gesture,
            target,
            operability,
            receipt_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn kind(&self) -> WorthUiInteractionKind {
        self.kind
    }

    pub fn gesture(&self) -> WorthUiMountedInteractionGesture {
        self.gesture
    }

    pub fn target(&self) -> &WorthUiInteractionTarget {
        &self.target
    }

    pub fn operability(&self) -> &WorthUiInteractionOperabilityReceipt {
        &self.operability
    }

    pub fn query_graph_execution_digest(&self) -> u64 {
        self.operability.query_graph_execution_digest()
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}
