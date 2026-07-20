use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use worth_runtime_bridge::facade::{
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceBindingClass, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceOwner, BridgeCausalExplanationEnvelope,
};

use super::super::super::identity::{
    compose_bridge_causal_envelope_identity, compose_bridge_causal_envelope_receipt_identity,
    compose_bridge_causal_explanation_envelope_identity, CausalInspectionArtifactIdentity,
    CausalInspectionOutcomeIdentity,
};
use super::super::super::observation_identity::{
    CausalObservationReceiptIdentity, CausalResultShapeContextIdentity,
};
use super::super::{
    CausalBridgeReadmissionProof, CausalInspectionArtifactKind,
    CausalInspectionBoundaryEnvelopeCategory, CausalInspectionPerformanceEnvelope,
    CausalInspectionRedactionPolicy, CausalMaterializationReceipt,
    QueryCausalTemporalAsyncExplanation,
};
use super::built::BuiltBridgeBackedArtifact;
use super::denied::DeniedQueryCausalInspectionArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCausalEvidenceReferenceArtifact {
    owner: BridgeCausalEvidenceOwner,
    family: BridgeCausalEvidenceFamily,
    reference_identity: WorthQueryEvidenceIdentity,
    binding_identity: WorthQueryEvidenceIdentity,
    retained_record_identity: Option<WorthQueryEvidenceIdentity>,
    detail_redacted: bool,
    reference_receipt_identity: WorthQueryEvidenceIdentity,
}

impl QueryCausalEvidenceReferenceArtifact {
    pub(in crate::runtime::inspection::causal::materialization) fn from_bridge_binding(
        binding: &BridgeCausalEvidenceBinding,
        redaction_policy: CausalInspectionRedactionPolicy,
    ) -> Self {
        let detail_redacted = redaction_policy == CausalInspectionRedactionPolicy::DigestOnly;
        let retained_record_identity = if detail_redacted {
            None
        } else {
            binding.retained_record_evidence_identity()
        };
        let bridge_reference_identity = binding.reference_evidence_identity();
        let bridge_binding_identity = binding.binding_evidence_identity();
        let reference_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::CausalEvidenceReferenceReceipt,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "bridge-causal-evidence-reference",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("owner"),
            binding.owner().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            binding.family().as_str(),
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("reference"),
            &bridge_reference_identity,
        )
        .seal();
        let binding_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::CausalEvidenceReferenceReceipt,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "bridge-causal-evidence-binding",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("binding_class"),
            bridge_causal_evidence_binding_class_label(binding.binding_class()),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("reference"), &reference_identity)
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("binding"),
            &bridge_binding_identity,
        )
        .seal();
        let retained_record_identity = retained_record_identity.as_ref().map(|retained_record| {
            WorthQueryEvidenceIdentity::compose(
                WorthQueryEvidenceScope::CausalEvidenceReferenceReceipt,
            )
            .field_shape(WorthQueryEvidenceTag::new("role"), "retained-bridge-record")
            .field_shape(
                WorthQueryEvidenceTag::new("owner"),
                binding.owner().as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("family"),
                binding.family().as_str(),
            )
            .field_evidence_identity(WorthQueryEvidenceTag::new("reference"), &reference_identity)
            .field_evidence_identity(WorthQueryEvidenceTag::new("binding"), &binding_identity)
            .field_bridge_retained_evidence_identity(
                WorthQueryEvidenceTag::new("retained"),
                retained_record,
            )
            .seal()
        });
        let reference_receipt_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::CausalEvidenceReferenceReceipt,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("owner"),
            binding.owner().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            binding.family().as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("reference"), &reference_identity)
        .field_evidence_identity(WorthQueryEvidenceTag::new("binding"), &binding_identity)
        .optional_evidence_identity(
            WorthQueryEvidenceTag::new("retained"),
            retained_record_identity.as_ref(),
        )
        .field_bool(WorthQueryEvidenceTag::new("redacted"), detail_redacted)
        .seal();
        Self {
            owner: binding.owner(),
            family: binding.family(),
            reference_identity,
            binding_identity,
            retained_record_identity,
            detail_redacted,
            reference_receipt_identity,
        }
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn owner_kind(&self) -> BridgeCausalEvidenceOwner {
        self.owner
    }

    pub fn family(&self) -> &str {
        self.family.as_str()
    }

    pub fn family_kind(&self) -> BridgeCausalEvidenceFamily {
        self.family
    }

    pub fn reference_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.reference_identity
    }

    pub fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn retained_record_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.retained_record_identity.as_ref()
    }

    pub fn reference_evidence_for_reporting(&self) -> &str {
        self.reference_identity.as_str()
    }

    pub fn binding_for_reporting(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn retained_record_for_reporting(&self) -> Option<&str> {
        self.retained_record_identity
            .as_ref()
            .map(WorthQueryEvidenceIdentity::as_str)
    }

    pub fn detail_redacted(&self) -> bool {
        self.detail_redacted
    }

    pub fn reference_for_reporting(&self) -> &str {
        self.reference_receipt_identity.as_str()
    }

    pub(in crate::runtime::inspection::causal) fn reference_receipt_evidence_identity(
        &self,
    ) -> &WorthQueryEvidenceIdentity {
        &self.reference_receipt_identity
    }
}

fn bridge_causal_evidence_binding_class_label(
    binding_class: BridgeCausalEvidenceBindingClass,
) -> &'static str {
    match binding_class {
        BridgeCausalEvidenceBindingClass::RetainedBridgeRecord => "retained_bridge_record",
        BridgeCausalEvidenceBindingClass::ExternalAuthorityReference => {
            "external_authority_reference"
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedQueryCausalInspectionArtifact {
    query_admission_identity: CausalInspectionOutcomeIdentity,
    query_observation_identity: CausalObservationReceiptIdentity,
    result_shape_context_identity: CausalResultShapeContextIdentity,
    bridge_identity: WorthQueryEvidenceIdentity,
    bridge_envelope_identity: WorthQueryEvidenceIdentity,
    bridge_receipt_identity: WorthQueryEvidenceIdentity,
    boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
    evidence_references: Vec<QueryCausalEvidenceReferenceArtifact>,
    temporal_async_explanation: QueryCausalTemporalAsyncExplanation,
    performance: CausalInspectionPerformanceEnvelope,
    receipt: CausalMaterializationReceipt,
    readmission_proof: CausalBridgeReadmissionProof,
    causal_identity: CausalInspectionArtifactIdentity,
    artifact_identity: CausalInspectionArtifactIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryQueryCausalInspectionArtifact {
    query_advisory_identity: CausalInspectionOutcomeIdentity,
    query_observation_identity: CausalObservationReceiptIdentity,
    result_shape_context_identity: CausalResultShapeContextIdentity,
    advisory_reason: String,
    bridge_identity: WorthQueryEvidenceIdentity,
    bridge_envelope_identity: WorthQueryEvidenceIdentity,
    bridge_receipt_identity: WorthQueryEvidenceIdentity,
    boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
    evidence_references: Vec<QueryCausalEvidenceReferenceArtifact>,
    temporal_async_explanation: QueryCausalTemporalAsyncExplanation,
    performance: CausalInspectionPerformanceEnvelope,
    receipt: CausalMaterializationReceipt,
    readmission_proof: CausalBridgeReadmissionProof,
    causal_identity: CausalInspectionArtifactIdentity,
    artifact_identity: CausalInspectionArtifactIdentity,
}

impl AdmittedQueryCausalInspectionArtifact {
    pub(in crate::runtime::inspection::causal::materialization) fn from_parts(
        query_admission_identity: &CausalInspectionOutcomeIdentity,
        query_observation_identity: &CausalObservationReceiptIdentity,
        result_shape_context_identity: &CausalResultShapeContextIdentity,
        envelope: &BridgeCausalExplanationEnvelope,
        temporal_async_explanation: QueryCausalTemporalAsyncExplanation,
        built: BuiltBridgeBackedArtifact,
    ) -> Self {
        let bridge_identity = compose_bridge_causal_envelope_identity(envelope.identity());
        let bridge_envelope_identity =
            compose_bridge_causal_explanation_envelope_identity(envelope);
        let bridge_receipt_identity =
            compose_bridge_causal_envelope_receipt_identity(envelope.receipt());
        Self {
            query_admission_identity: query_admission_identity.clone(),
            query_observation_identity: query_observation_identity.clone(),
            result_shape_context_identity: result_shape_context_identity.clone(),
            bridge_identity,
            bridge_envelope_identity,
            bridge_receipt_identity,
            boundary_categories: built.boundary_categories,
            evidence_references: built.evidence_references,
            temporal_async_explanation,
            performance: built.performance,
            receipt: built.receipt,
            readmission_proof: built.readmission_proof,
            causal_identity: built.causal_identity,
            artifact_identity: built.artifact_identity,
        }
    }

    pub fn query_admission_for_reporting(&self) -> &str {
        self.query_admission_identity.as_str()
    }

    pub fn query_admission_identity(&self) -> &CausalInspectionOutcomeIdentity {
        &self.query_admission_identity
    }

    pub fn query_observation_for_reporting(&self) -> &str {
        self.query_observation_identity.as_str()
    }

    pub(in crate::runtime) fn query_observation_identity(
        &self,
    ) -> &CausalObservationReceiptIdentity {
        &self.query_observation_identity
    }

    pub fn result_shape_context_for_reporting(&self) -> &str {
        self.result_shape_context_identity.as_str()
    }
}

impl AdvisoryQueryCausalInspectionArtifact {
    pub(in crate::runtime::inspection::causal::materialization) fn from_parts(
        query_advisory_identity: &CausalInspectionOutcomeIdentity,
        query_observation_identity: &CausalObservationReceiptIdentity,
        result_shape_context_identity: &CausalResultShapeContextIdentity,
        advisory_reason: String,
        envelope: &BridgeCausalExplanationEnvelope,
        temporal_async_explanation: QueryCausalTemporalAsyncExplanation,
        built: BuiltBridgeBackedArtifact,
    ) -> Self {
        let bridge_identity = compose_bridge_causal_envelope_identity(envelope.identity());
        let bridge_envelope_identity =
            compose_bridge_causal_explanation_envelope_identity(envelope);
        let bridge_receipt_identity =
            compose_bridge_causal_envelope_receipt_identity(envelope.receipt());
        Self {
            query_advisory_identity: query_advisory_identity.clone(),
            query_observation_identity: query_observation_identity.clone(),
            result_shape_context_identity: result_shape_context_identity.clone(),
            advisory_reason,
            bridge_identity,
            bridge_envelope_identity,
            bridge_receipt_identity,
            boundary_categories: built.boundary_categories,
            evidence_references: built.evidence_references,
            temporal_async_explanation,
            performance: built.performance,
            receipt: built.receipt,
            readmission_proof: built.readmission_proof,
            causal_identity: built.causal_identity,
            artifact_identity: built.artifact_identity,
        }
    }

    pub fn query_advisory_for_reporting(&self) -> &str {
        self.query_advisory_identity.as_str()
    }

    pub fn query_observation_for_reporting(&self) -> &str {
        self.query_observation_identity.as_str()
    }

    pub(in crate::runtime) fn query_observation_identity(
        &self,
    ) -> &CausalObservationReceiptIdentity {
        &self.query_observation_identity
    }

    pub fn result_shape_context_for_reporting(&self) -> &str {
        self.result_shape_context_identity.as_str()
    }

    pub fn advisory_reason(&self) -> &str {
        &self.advisory_reason
    }
}

macro_rules! bridge_backed_accessors {
    ($ty:ty) => {
        impl $ty {
            pub fn evidence_references(&self) -> &[QueryCausalEvidenceReferenceArtifact] {
                &self.evidence_references
            }

            pub fn temporal_async_explanation(&self) -> &QueryCausalTemporalAsyncExplanation {
                &self.temporal_async_explanation
            }

            pub fn boundary_categories(&self) -> &[CausalInspectionBoundaryEnvelopeCategory] {
                &self.boundary_categories
            }

            pub fn bridge_envelope_for_reporting(&self) -> &str {
                self.bridge_envelope_identity.as_str()
            }

            pub fn bridge_envelope_identity(&self) -> &WorthQueryEvidenceIdentity {
                &self.bridge_envelope_identity
            }

            pub fn bridge_receipt_for_reporting(&self) -> &str {
                self.bridge_receipt_identity.as_str()
            }

            pub fn bridge_receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
                &self.bridge_receipt_identity
            }

            pub fn performance(&self) -> &CausalInspectionPerformanceEnvelope {
                &self.performance
            }

            pub fn receipt(&self) -> &CausalMaterializationReceipt {
                &self.receipt
            }

            pub fn readmission_proof(&self) -> &CausalBridgeReadmissionProof {
                &self.readmission_proof
            }

            pub fn bridge_readmission_proof_for_reporting(&self) -> &str {
                self.readmission_proof.readmission_proof_for_reporting()
            }

            pub(in crate::runtime) fn bridge_readmission_proof_identity(
                &self,
            ) -> &WorthQueryEvidenceIdentity {
                self.readmission_proof.readmission_proof_identity()
            }

            pub fn causal_identity_for_reporting(&self) -> &str {
                self.causal_identity.as_str()
            }

            pub fn causal_identity(&self) -> &CausalInspectionArtifactIdentity {
                &self.causal_identity
            }

            pub fn artifact_for_reporting(&self) -> &str {
                self.artifact_identity.as_str()
            }

            pub fn artifact_identity(&self) -> &CausalInspectionArtifactIdentity {
                &self.artifact_identity
            }
        }
    };
}

bridge_backed_accessors!(AdmittedQueryCausalInspectionArtifact);
bridge_backed_accessors!(AdvisoryQueryCausalInspectionArtifact);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryCausalInspectionArtifact {
    Admitted(AdmittedQueryCausalInspectionArtifact),
    Advisory(AdvisoryQueryCausalInspectionArtifact),
    Denied(DeniedQueryCausalInspectionArtifact),
}

impl QueryCausalInspectionArtifact {
    pub fn kind(&self) -> CausalInspectionArtifactKind {
        match self {
            Self::Admitted(_) => CausalInspectionArtifactKind::Admitted,
            Self::Advisory(_) => CausalInspectionArtifactKind::Advisory,
            Self::Denied(_) => CausalInspectionArtifactKind::Denied,
        }
    }

    pub fn artifact_for_reporting(&self) -> &str {
        match self {
            Self::Admitted(artifact) => artifact.artifact_for_reporting(),
            Self::Advisory(artifact) => artifact.artifact_for_reporting(),
            Self::Denied(artifact) => artifact.artifact_for_reporting(),
        }
    }

    pub fn artifact_identity(&self) -> &CausalInspectionArtifactIdentity {
        match self {
            Self::Admitted(artifact) => artifact.artifact_identity(),
            Self::Advisory(artifact) => artifact.artifact_identity(),
            Self::Denied(artifact) => artifact.artifact_identity(),
        }
    }

    pub fn causal_identity_for_reporting(&self) -> &str {
        match self {
            Self::Admitted(artifact) => artifact.causal_identity_for_reporting(),
            Self::Advisory(artifact) => artifact.causal_identity_for_reporting(),
            Self::Denied(artifact) => artifact.causal_identity_for_reporting(),
        }
    }

    pub fn causal_identity(&self) -> &CausalInspectionArtifactIdentity {
        match self {
            Self::Admitted(artifact) => artifact.causal_identity(),
            Self::Advisory(artifact) => artifact.causal_identity(),
            Self::Denied(artifact) => artifact.causal_identity(),
        }
    }

    pub fn query_observation_for_reporting(&self) -> &str {
        match self {
            Self::Admitted(artifact) => artifact.query_observation_for_reporting(),
            Self::Advisory(artifact) => artifact.query_observation_for_reporting(),
            Self::Denied(artifact) => artifact.query_observation_for_reporting(),
        }
    }

    pub(in crate::runtime) fn query_observation_identity(
        &self,
    ) -> &CausalObservationReceiptIdentity {
        match self {
            Self::Admitted(artifact) => artifact.query_observation_identity(),
            Self::Advisory(artifact) => artifact.query_observation_identity(),
            Self::Denied(artifact) => artifact.query_observation_identity(),
        }
    }

    pub fn bridge_envelope_for_reporting(&self) -> Option<&str> {
        match self {
            Self::Admitted(artifact) => Some(artifact.bridge_envelope_for_reporting()),
            Self::Advisory(artifact) => Some(artifact.bridge_envelope_for_reporting()),
            Self::Denied(_) => None,
        }
    }

    pub fn evidence_reference_count(&self) -> usize {
        match self {
            Self::Admitted(artifact) => artifact.evidence_references().len(),
            Self::Advisory(artifact) => artifact.evidence_references().len(),
            Self::Denied(_) => 0,
        }
    }

    pub fn performance(&self) -> &CausalInspectionPerformanceEnvelope {
        match self {
            Self::Admitted(artifact) => artifact.performance(),
            Self::Advisory(artifact) => artifact.performance(),
            Self::Denied(artifact) => artifact.performance(),
        }
    }

    pub fn temporal_async_explanation(&self) -> &QueryCausalTemporalAsyncExplanation {
        match self {
            Self::Admitted(artifact) => artifact.temporal_async_explanation(),
            Self::Advisory(artifact) => artifact.temporal_async_explanation(),
            Self::Denied(artifact) => artifact.temporal_async_explanation(),
        }
    }

    pub fn bridge_readmission_proof_for_reporting(&self) -> Option<&str> {
        match self {
            Self::Admitted(artifact) => Some(artifact.bridge_readmission_proof_for_reporting()),
            Self::Advisory(artifact) => Some(artifact.bridge_readmission_proof_for_reporting()),
            Self::Denied(_) => None,
        }
    }

    pub(in crate::runtime) fn bridge_readmission_proof_identity(
        &self,
    ) -> Option<&WorthQueryEvidenceIdentity> {
        match self {
            Self::Admitted(artifact) => Some(artifact.bridge_readmission_proof_identity()),
            Self::Advisory(artifact) => Some(artifact.bridge_readmission_proof_identity()),
            Self::Denied(_) => None,
        }
    }

    pub fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted(_))
    }

    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Denied(_))
    }
}
