use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

use crate::runtime::{
    CausalEvidenceFamily, CausalEvidenceReferenceSet, CausalInspectionMaterializationPolicy,
    CausalInspectionRedactionPolicy, CausalInspectionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeExplanationRequest {
    kind: ForgeQueryLowerRuntimeExplanationRequestKind,
}

impl ForgeQueryLowerRuntimeExplanationRequest {
    pub fn requires_cross_runtime_context(
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self {
            kind: ForgeQueryLowerRuntimeExplanationRequestKind::CrossRuntimeContext {
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
            kind: ForgeQueryLowerRuntimeExplanationRequestKind::CrossRuntimeFallback {
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
            kind: ForgeQueryLowerRuntimeExplanationRequestKind::StoreBackedReplayGap {
                reference_set,
                target,
                requested_evidence_families,
                redaction_policy,
                materialization_policy,
            },
        }
    }

    pub(crate) fn kind(self) -> ForgeQueryLowerRuntimeExplanationRequestKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ForgeQueryLowerRuntimeExplanationRequestKind {
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
