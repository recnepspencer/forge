use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionReadiness, WorthUiInteractionTarget,
    WorthUiMountedInteractionGesture, WorthUiPrimitiveFocusPosture,
    WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

use super::classification::classify_interaction_operability;
use super::digest::operability_digest;
use super::{WorthUiInteractionOperabilityBasis, WorthUiInteractionOperabilityPosture};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiInteractionOperabilityReceipt {
    posture: WorthUiInteractionOperabilityPosture,
    basis: WorthUiInteractionOperabilityBasis,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

impl WorthUiInteractionOperabilityReceipt {
    pub(super) fn resolve(
        graph_authority: &WorthUiRuntimeGraphAuthority,
        surface_id: &SurfaceId,
        interaction_id: &str,
        primitive_disabled: bool,
        readiness: WorthUiInteractionReadiness,
        kind: WorthUiInteractionKind,
        target: &WorthUiInteractionTarget,
        gesture: WorthUiMountedInteractionGesture,
        primitive_focus: WorthUiPrimitiveFocusPosture,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let (posture, basis) = classify_interaction_operability(
            primitive_disabled,
            readiness,
            kind,
            target,
            gesture,
            primitive_focus,
        );
        let query_graph_execution = graph_authority
            .plan_mounted_interaction_graph_operation(
                surface_id,
                interaction_id,
                consumed_facts.clone(),
                basis,
                readiness,
                kind,
                target,
                primitive_focus,
            )
            .into_execution_receipt();
        let receipt_digest = operability_digest(
            posture,
            basis,
            query_graph_execution.execution_digest(),
            &consumed_facts,
        );
        Self {
            posture,
            basis,
            query_graph_execution,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn posture(&self) -> WorthUiInteractionOperabilityPosture {
        self.posture
    }

    pub fn basis(&self) -> WorthUiInteractionOperabilityBasis {
        self.basis
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn query_graph_execution_digest(&self) -> u64 {
        self.query_graph_execution.execution_digest()
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }

    pub fn is_eligible(&self) -> bool {
        self.posture == WorthUiInteractionOperabilityPosture::Eligible
    }
}
