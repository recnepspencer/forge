use crate::capability::{ComponentId, SurfaceId};

use super::payload::{fold_digest, WorthUiInteractionKind, WorthUiInteractionPayload};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionReadiness {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionStatus {
    Emitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionTarget {
    Surface(String),
    Command(String),
    Toggle(String),
    Open(String),
    Focus(String),
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionReceipt {
    surface_id: String,
    component_id: String,
    interaction_id: String,
    kind: WorthUiInteractionKind,
    status: WorthUiInteractionStatus,
    readiness: WorthUiInteractionReadiness,
    target: WorthUiInteractionTarget,
    payload: WorthUiInteractionPayload,
    receipt_digest: u64,
}

impl WorthUiInteractionReceipt {
    pub(crate) fn new(
        surface_id: &SurfaceId,
        component_id: &ComponentId,
        interaction_id: impl Into<String>,
        readiness: WorthUiInteractionReadiness,
        target: WorthUiInteractionTarget,
        payload: WorthUiInteractionPayload,
    ) -> Self {
        let interaction_id = interaction_id.into();
        let kind = payload.kind();
        let receipt_digest = receipt_digest(
            surface_id.as_str(),
            component_id.as_str(),
            &interaction_id,
            readiness,
            &target,
            payload.digest(),
        );
        Self {
            surface_id: surface_id.as_str().to_owned(),
            component_id: component_id.as_str().to_owned(),
            interaction_id,
            kind,
            status: WorthUiInteractionStatus::Emitted,
            readiness,
            target,
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

    pub fn kind(&self) -> WorthUiInteractionKind {
        self.kind
    }

    pub fn status(&self) -> WorthUiInteractionStatus {
        self.status
    }

    pub fn readiness(&self) -> WorthUiInteractionReadiness {
        self.readiness
    }

    pub fn target(&self) -> &WorthUiInteractionTarget {
        &self.target
    }

    pub fn payload(&self) -> &WorthUiInteractionPayload {
        &self.payload
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiInteractionReadiness {
    pub fn is_enabled(self) -> bool {
        self == Self::Enabled
    }
}

fn receipt_digest(
    surface_id: &str,
    component_id: &str,
    interaction_id: &str,
    readiness: WorthUiInteractionReadiness,
    target: &WorthUiInteractionTarget,
    payload_digest: u64,
) -> u64 {
    let mut digest = payload_digest;
    for value in [
        surface_id,
        component_id,
        interaction_id,
        readiness_token(readiness),
        &target_basis(target),
    ] {
        digest = fold_digest(digest, value.as_bytes());
    }
    digest
}

fn readiness_token(readiness: WorthUiInteractionReadiness) -> &'static str {
    match readiness {
        WorthUiInteractionReadiness::Enabled => "enabled",
        WorthUiInteractionReadiness::Disabled => "disabled",
    }
}

fn target_basis(target: &WorthUiInteractionTarget) -> String {
    match target {
        WorthUiInteractionTarget::Surface(value) => format!("surface:{value}"),
        WorthUiInteractionTarget::Command(value) => format!("command:{value}"),
        WorthUiInteractionTarget::Toggle(value) => format!("toggle:{value}"),
        WorthUiInteractionTarget::Open(value) => format!("open:{value}"),
        WorthUiInteractionTarget::Focus(value) => format!("focus:{value}"),
        WorthUiInteractionTarget::None => "none".to_owned(),
    }
}
