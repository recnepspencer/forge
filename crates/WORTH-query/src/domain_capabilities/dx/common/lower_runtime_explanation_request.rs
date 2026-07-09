use worth_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

use crate::runtime::{
    CausalEvidenceFamily, CausalEvidenceReferenceSet, CausalInspectionMaterializationPolicy,
    CausalInspectionRedactionPolicy, CausalInspectionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeExplanationRequest {
    kind: WorthQueryLowerRuntimeExplanationRequestKind,
}

impl WorthQueryLowerRuntimeExplanationRequest {
    pub fn requires_cross_runtime_context(
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self {
            kind: WorthQueryLowerRuntimeExplanationRequestKind::CrossRuntimeContext {
                reference_set,
                target,
                requested_evidence_families,
                bridge_envelope,
                redaction_policy,
                materialization_policy,
            },
        }
    }

    pub fn explains_cross_runtime_fallback(
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self {
            kind: WorthQueryLowerRuntimeExplanationRequestKind::CrossRuntimeFallback {
                reference_set,
                target,
                requested_evidence_families,
                bridge_envelope,
                redaction_policy,
                materialization_policy,
            },
        }
    }

    pub fn explains_store_backed_replay_gap(
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self {
            kind: WorthQueryLowerRuntimeExplanationRequestKind::StoreBackedReplayGap {
                reference_set,
                target,
                requested_evidence_families,
                redaction_policy,
                materialization_policy,
            },
        }
    }

    pub(crate) fn kind(self) -> WorthQueryLowerRuntimeExplanationRequestKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryLowerRuntimeExplanationRequestKind {
    CrossRuntimeContext {
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    },
    CrossRuntimeFallback {
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    },
    StoreBackedReplayGap {
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    },
}
