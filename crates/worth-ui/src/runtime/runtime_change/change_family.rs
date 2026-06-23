use std::cmp::Ordering;

use crate::runtime::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStatus, WorthUiChangedRuntimeFacts,
    WorthUiComponentCompatibility, WorthUiComponentInteractionReceipt,
    WorthUiComponentReloadReceipt, WorthUiDropdownSelectionInteractionReceipt,
    WorthUiDropdownSelectionInteractionStatus, WorthUiQueryRuntimeFactLoweringReceipt,
    WorthUiQueryRuntimeFactLoweringStatus, WorthUiRuntimeFactId, WorthUiRuntimeFactSet,
    WorthUiValidationChangedFacts, WorthUiValidationReloadEvidence, WorthUiValidationReloadStatus,
};

use super::WorthUiRuntimeInstanceWitness;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiRuntimeChangeFamily {
    ValidationSource,
    Capability,
    InteractionState,
    QueryBinding,
    DurableState,
}

impl WorthUiRuntimeChangeFamily {
    pub(crate) fn token(self) -> &'static str {
        match self {
            Self::ValidationSource => "validation_source",
            Self::Capability => "capability",
            Self::InteractionState => "interaction_state",
            Self::QueryBinding => "query_binding",
            Self::DurableState => "durable_state",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeChangeFamilyStatus {
    EquivalentNoOp,
    ReadyForFrameBoundary,
    Activated,
    Denied,
}

impl WorthUiRuntimeChangeFamilyStatus {
    pub(crate) fn token(self) -> &'static str {
        match self {
            Self::EquivalentNoOp => "equivalent",
            Self::ReadyForFrameBoundary => "ready",
            Self::Activated => "activated",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeChangeFamilyRow {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    family: WorthUiRuntimeChangeFamily,
    status: WorthUiRuntimeChangeFamilyStatus,
    changed_facts: WorthUiChangedRuntimeFacts,
    denial_detail: Option<String>,
    payload_digest: u64,
    component_reload_receipt: Option<WorthUiComponentReloadReceipt>,
}

impl WorthUiRuntimeChangeFamilyRow {
    pub(crate) fn from_validation_evidence(evidence: &WorthUiValidationReloadEvidence) -> Self {
        let changed_facts = WorthUiValidationChangedFacts::from_reload_evidence(evidence);
        Self {
            runtime_instance: WorthUiRuntimeInstanceWitness::from_raw(
                evidence.runtime_instance_witness(),
            ),
            family: WorthUiRuntimeChangeFamily::ValidationSource,
            status: validation_status(evidence.status()),
            changed_facts: changed_facts.changed_facts().clone(),
            denial_detail: evidence.denial_detail().map(str::to_owned),
            payload_digest: validation_payload_digest(evidence),
            component_reload_receipt: None,
        }
    }

    pub(crate) fn from_capability_evidence(evidence: &WorthUiCapabilityReloadEvidence) -> Self {
        Self {
            runtime_instance: WorthUiRuntimeInstanceWitness::from_raw(
                evidence.runtime_instance_witness(),
            ),
            family: WorthUiRuntimeChangeFamily::Capability,
            status: capability_status(evidence.status()),
            changed_facts: evidence.capability_changed_facts().changed_facts().clone(),
            denial_detail: evidence.denial_detail().map(str::to_owned),
            payload_digest: capability_payload_digest(evidence),
            component_reload_receipt: evidence.component_reload_receipt().cloned(),
        }
    }

    pub(crate) fn from_query_lowering_receipt(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiQueryRuntimeFactLoweringReceipt,
    ) -> Self {
        Self {
            runtime_instance,
            family: WorthUiRuntimeChangeFamily::QueryBinding,
            status: query_status(receipt.status()),
            changed_facts: receipt.changed_facts().changed_facts().clone(),
            denial_detail: query_denial_detail(receipt),
            payload_digest: receipt.receipt_digest(),
            component_reload_receipt: None,
        }
    }

    pub(crate) fn from_dropdown_selection_interaction(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiDropdownSelectionInteractionReceipt,
    ) -> Self {
        Self {
            runtime_instance,
            family: WorthUiRuntimeChangeFamily::InteractionState,
            status: interaction_status(receipt.status()),
            changed_facts: interaction_changed_facts(receipt),
            denial_detail: None,
            payload_digest: interaction_payload_digest(receipt),
            component_reload_receipt: None,
        }
    }

    pub(crate) fn from_component_interaction(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiComponentInteractionReceipt,
    ) -> Self {
        Self {
            runtime_instance,
            family: WorthUiRuntimeChangeFamily::InteractionState,
            status: WorthUiRuntimeChangeFamilyStatus::Activated,
            changed_facts: component_interaction_changed_facts(receipt),
            denial_detail: None,
            payload_digest: receipt.receipt_digest(),
            component_reload_receipt: None,
        }
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn family(&self) -> WorthUiRuntimeChangeFamily {
        self.family
    }

    pub fn status(&self) -> WorthUiRuntimeChangeFamilyStatus {
        self.status
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }

    pub fn payload_digest(&self) -> u64 {
        self.payload_digest
    }

    pub fn component_compatibility(&self) -> Option<&WorthUiComponentCompatibility> {
        self.component_reload_receipt
            .as_ref()
            .map(WorthUiComponentReloadReceipt::compatibility)
    }

    pub fn component_reload_receipt(&self) -> Option<&WorthUiComponentReloadReceipt> {
        self.component_reload_receipt.as_ref()
    }

    pub(crate) fn canonical_cmp(&self, other: &Self) -> Ordering {
        self.family
            .cmp(&other.family)
            .then_with(|| self.status.token().cmp(other.status.token()))
            .then_with(|| self.payload_digest.cmp(&other.payload_digest))
            .then_with(|| {
                self.changed_facts
                    .digest()
                    .value()
                    .cmp(&other.changed_facts.digest().value())
            })
    }
}

fn validation_status(status: WorthUiValidationReloadStatus) -> WorthUiRuntimeChangeFamilyStatus {
    match status {
        WorthUiValidationReloadStatus::EquivalentNoOp => {
            WorthUiRuntimeChangeFamilyStatus::EquivalentNoOp
        }
        WorthUiValidationReloadStatus::ReadyForFrameBoundary => {
            WorthUiRuntimeChangeFamilyStatus::ReadyForFrameBoundary
        }
        WorthUiValidationReloadStatus::Activated => WorthUiRuntimeChangeFamilyStatus::Activated,
        WorthUiValidationReloadStatus::Denied(_) => WorthUiRuntimeChangeFamilyStatus::Denied,
    }
}

fn capability_status(status: WorthUiCapabilityReloadStatus) -> WorthUiRuntimeChangeFamilyStatus {
    match status {
        WorthUiCapabilityReloadStatus::EquivalentNoOp => {
            WorthUiRuntimeChangeFamilyStatus::EquivalentNoOp
        }
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => {
            WorthUiRuntimeChangeFamilyStatus::ReadyForFrameBoundary
        }
        WorthUiCapabilityReloadStatus::Activated => WorthUiRuntimeChangeFamilyStatus::Activated,
        WorthUiCapabilityReloadStatus::Denied(_) => WorthUiRuntimeChangeFamilyStatus::Denied,
    }
}

fn query_status(status: WorthUiQueryRuntimeFactLoweringStatus) -> WorthUiRuntimeChangeFamilyStatus {
    match status {
        WorthUiQueryRuntimeFactLoweringStatus::AdmittedChanged => {
            WorthUiRuntimeChangeFamilyStatus::Activated
        }
        WorthUiQueryRuntimeFactLoweringStatus::EquivalentNoOp => {
            WorthUiRuntimeChangeFamilyStatus::EquivalentNoOp
        }
        WorthUiQueryRuntimeFactLoweringStatus::Denied => WorthUiRuntimeChangeFamilyStatus::Denied,
    }
}

fn query_denial_detail(receipt: &WorthUiQueryRuntimeFactLoweringReceipt) -> Option<String> {
    let denial = receipt.support_denials().first()?;
    Some(format!(
        "query reload denied: {:?}; support={:?}; runtime_hooks={}; denied_bindings={}",
        denial.kind(),
        denial.support_status(),
        denial.runtime_hook_count(),
        denial.denied_binding_count()
    ))
}

fn interaction_status(
    status: &WorthUiDropdownSelectionInteractionStatus,
) -> WorthUiRuntimeChangeFamilyStatus {
    match status {
        WorthUiDropdownSelectionInteractionStatus::SelectedSingle
        | WorthUiDropdownSelectionInteractionStatus::AddedMultiSelection => {
            WorthUiRuntimeChangeFamilyStatus::Activated
        }
        WorthUiDropdownSelectionInteractionStatus::AlreadySelected => {
            WorthUiRuntimeChangeFamilyStatus::EquivalentNoOp
        }
    }
}

fn interaction_changed_facts(
    receipt: &WorthUiDropdownSelectionInteractionReceipt,
) -> WorthUiChangedRuntimeFacts {
    let mut facts = WorthUiRuntimeFactSet::empty();
    if receipt.previous_selection_state() != receipt.next_selection_state() {
        facts.insert(WorthUiRuntimeFactId::dropdown_selection_state(
            &crate::capability::CommandProjectionId::new(receipt.projection_id())
                .expect("interaction receipt preserves valid projection ids"),
        ));
    }
    WorthUiChangedRuntimeFacts::from_runtime(facts)
}

fn interaction_payload_digest(receipt: &WorthUiDropdownSelectionInteractionReceipt) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for byte in receipt.projection_id().as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    for byte in receipt.command_id().as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    for command_id in receipt.next_selection_state().selected_command_ids() {
        for byte in command_id.as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}

fn component_interaction_changed_facts(
    receipt: &WorthUiComponentInteractionReceipt,
) -> WorthUiChangedRuntimeFacts {
    let identity = format!(
        "{}|{}|{}",
        receipt.surface_id(),
        receipt.component_id(),
        receipt.interaction_id()
    );
    WorthUiChangedRuntimeFacts::from_runtime(WorthUiRuntimeFactSet::single(
        WorthUiRuntimeFactId::component_interaction_state(identity),
    ))
}

fn validation_payload_digest(evidence: &WorthUiValidationReloadEvidence) -> u64 {
    evidence
        .source_revision_digest()
        .or_else(|| evidence.candidate_artifact_digest())
        .or_else(|| evidence.candidate_plan_digest())
        .unwrap_or(evidence.active_artifact_digest_before())
}

fn capability_payload_digest(evidence: &WorthUiCapabilityReloadEvidence) -> u64 {
    evidence
        .candidate_snapshot_digest()
        .unwrap_or(evidence.theme_source_digest())
}
