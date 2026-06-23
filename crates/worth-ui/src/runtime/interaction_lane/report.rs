use crate::capability::{ComponentId, SurfaceId};

use super::denial_receipt::WorthUiInteractionValueDenialReceipt;
use super::payload::{
    WorthUiInteractionField, WorthUiInteractionFieldValue, WorthUiInteractionKind,
    WorthUiInteractionPayload,
};
use super::receipt::{
    WorthUiInteractionReadiness, WorthUiInteractionReceipt, WorthUiInteractionTarget,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInteractionAdmissionReport {
    surface_id: String,
    status: WorthUiInteractionAdmissionStatus,
    counters: WorthUiInteractionAdmissionCounters,
    schema_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiInteractionAdmissionStatus {
    Accepted(WorthUiInteractionAdmissionReceipt),
    Rejected(WorthUiInteractionValueDenialSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInteractionAdmissionReceipt {
    surface_id: String,
    prop_set: WorthUiValidatedInteractionPropSet,
    authored_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiValidatedInteractionPropSet {
    kind: WorthUiInteractionKind,
    interaction_id: String,
    payload_value: WorthUiInteractionFieldValue,
    target: WorthUiInteractionTarget,
    readiness: WorthUiInteractionReadiness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionAdmissionCounters {
    schema_count: usize,
    authored_props_seen: usize,
    defaults_applied: usize,
    values_validated: usize,
    denials_emitted: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInteractionValueDenialSet {
    surface_id: String,
    denials: Vec<WorthUiInteractionValueDenialReceipt>,
    denial_set_digest: u64,
}

impl WorthUiInteractionAdmissionReport {
    pub(crate) fn accepted(
        surface_id: impl Into<String>,
        receipt: WorthUiInteractionAdmissionReceipt,
        counters: WorthUiInteractionAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = receipt.admission_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiInteractionAdmissionStatus::Accepted(receipt),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub(crate) fn rejected(
        surface_id: impl Into<String>,
        denial_set: WorthUiInteractionValueDenialSet,
        counters: WorthUiInteractionAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = denial_set.denial_set_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiInteractionAdmissionStatus::Rejected(denial_set),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn status(&self) -> &WorthUiInteractionAdmissionStatus {
        &self.status
    }

    pub fn counters(&self) -> WorthUiInteractionAdmissionCounters {
        self.counters
    }

    pub fn schema_digest(&self) -> u64 {
        self.schema_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiInteractionAdmissionStatus {
    pub fn accepted_receipt(&self) -> Option<&WorthUiInteractionAdmissionReceipt> {
        match self {
            Self::Accepted(receipt) => Some(receipt),
            Self::Rejected(_) => None,
        }
    }

    pub fn denial_set(&self) -> Option<&WorthUiInteractionValueDenialSet> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(denial_set) => Some(denial_set),
        }
    }
}

impl WorthUiInteractionAdmissionReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        prop_set: WorthUiValidatedInteractionPropSet,
        authored_digest: u64,
        admission_digest: u64,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            prop_set,
            authored_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn prop_set(&self) -> &WorthUiValidatedInteractionPropSet {
        &self.prop_set
    }

    pub fn authored_digest(&self) -> u64 {
        self.authored_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }

    pub(crate) fn emit_receipt(
        &self,
        surface_id: &SurfaceId,
        component_id: &ComponentId,
    ) -> WorthUiInteractionReceipt {
        WorthUiInteractionReceipt::new(
            surface_id,
            component_id,
            self.prop_set.interaction_id(),
            self.prop_set.readiness(),
            self.prop_set.target().clone(),
            self.prop_set.payload(self.authored_digest),
        )
    }
}

impl WorthUiValidatedInteractionPropSet {
    pub(crate) fn new(
        kind: WorthUiInteractionKind,
        interaction_id: impl Into<String>,
        payload_value: WorthUiInteractionFieldValue,
        target: WorthUiInteractionTarget,
        readiness: WorthUiInteractionReadiness,
    ) -> Self {
        Self {
            kind,
            interaction_id: interaction_id.into(),
            payload_value,
            target,
            readiness,
        }
    }

    pub fn kind(&self) -> WorthUiInteractionKind {
        self.kind
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn payload_value(&self) -> &WorthUiInteractionFieldValue {
        &self.payload_value
    }

    pub fn target(&self) -> &WorthUiInteractionTarget {
        &self.target
    }

    pub fn readiness(&self) -> WorthUiInteractionReadiness {
        self.readiness
    }

    pub fn payload(&self, authored_digest: u64) -> WorthUiInteractionPayload {
        WorthUiInteractionPayload::new(
            self.kind,
            vec![
                WorthUiInteractionField::new("payload", self.payload_value.clone()),
                WorthUiInteractionField::new(
                    "target",
                    WorthUiInteractionFieldValue::Text(format!("{:?}", self.target)),
                ),
            ],
            authored_digest,
        )
    }
}

impl WorthUiInteractionAdmissionCounters {
    pub(crate) fn new(
        schema_count: usize,
        authored_props_seen: usize,
        defaults_applied: usize,
        values_validated: usize,
        denials_emitted: usize,
    ) -> Self {
        Self {
            schema_count,
            authored_props_seen,
            defaults_applied,
            values_validated,
            denials_emitted,
        }
    }

    pub fn schema_count(self) -> usize {
        self.schema_count
    }

    pub fn authored_props_seen(self) -> usize {
        self.authored_props_seen
    }

    pub fn defaults_applied(self) -> usize {
        self.defaults_applied
    }

    pub fn values_validated(self) -> usize {
        self.values_validated
    }

    pub fn denials_emitted(self) -> usize {
        self.denials_emitted
    }
}

impl WorthUiInteractionValueDenialSet {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        denials: Vec<WorthUiInteractionValueDenialReceipt>,
        denial_set_digest: u64,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            denials,
            denial_set_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn denials(&self) -> &[WorthUiInteractionValueDenialReceipt] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}
