use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{WorthUiComponentInteractionKind, WorthUiComponentInteractionPayload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiComponentInteractionReceipt {
    surface_id: String,
    component_id: String,
    interaction_id: String,
    status: WorthUiComponentInteractionStatus,
    payload: WorthUiComponentInteractionPayload,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiComponentInteractionStatus {
    Submitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiComponentInteractionDenial {
    MissingSurface {
        surface_id: String,
    },
    MissingAuthoringSnapshot,
    MissingAuthoredSurface {
        surface_id: String,
    },
    ComponentMismatch {
        surface_id: String,
        expected_component_id: String,
        actual_component_id: String,
    },
    UnsupportedInteraction {
        surface_id: String,
        component_id: String,
        kind: WorthUiComponentInteractionKind,
    },
}

impl WorthUiComponentInteractionReceipt {
    pub(crate) fn new(
        surface_id: &SurfaceId,
        component_id: &ComponentId,
        interaction_id: impl Into<String>,
        payload: WorthUiComponentInteractionPayload,
    ) -> Self {
        let interaction_id = interaction_id.into();
        let receipt_digest = receipt_digest(
            surface_id.as_str(),
            component_id.as_str(),
            &interaction_id,
            payload.digest(),
        );
        Self {
            surface_id: surface_id.as_str().to_owned(),
            component_id: component_id.as_str().to_owned(),
            interaction_id,
            status: WorthUiComponentInteractionStatus::Submitted,
            payload,
            receipt_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn status(&self) -> WorthUiComponentInteractionStatus {
        self.status
    }

    pub fn payload(&self) -> &WorthUiComponentInteractionPayload {
        &self.payload
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn receipt_digest(
    surface_id: &str,
    component_id: &str,
    interaction_id: &str,
    payload_digest: u64,
) -> u64 {
    let mut digest = payload_digest;
    for value in [surface_id, component_id, interaction_id] {
        for byte in value.as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}
