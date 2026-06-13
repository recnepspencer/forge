use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::super::identity::CausalInspectionOutcomeIdentity;
use super::{
    CausalInspectionMaterializationPolicy, CausalInspectionPerformanceEnvelope,
    CausalInspectionRedactionPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalMaterializationReceipt {
    query_admission_identity: CausalInspectionOutcomeIdentity,
    bridge_envelope_identity: Option<ForgeQueryEvidenceIdentity>,
    bridge_receipt_identity: Option<ForgeQueryEvidenceIdentity>,
    policy_identity: ForgeQueryEvidenceIdentity,
    performance_identity: ForgeQueryEvidenceIdentity,
    materialization_identity: ForgeQueryEvidenceIdentity,
    receipt_identity: ForgeQueryEvidenceIdentity,
}

impl CausalMaterializationReceipt {
    pub(super) fn new(
        query_admission_identity: &CausalInspectionOutcomeIdentity,
        bridge_envelope_identity: Option<&ForgeQueryEvidenceIdentity>,
        bridge_receipt_identity: Option<&ForgeQueryEvidenceIdentity>,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
        performance: &CausalInspectionPerformanceEnvelope,
        detail_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        let policy_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalInspectionArtifact)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "materialization-policy")
                .field_shape(
                    ForgeQueryEvidenceTag::new("redaction"),
                    redaction_policy.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("materialization"),
                    materialization_policy.as_str(),
                )
                .seal();
        let materialization_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalInspectionArtifact)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "materialization")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("query_admission"),
                    query_admission_identity.evidence_identity(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("bridge_envelope"),
                    bridge_envelope_identity,
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("bridge_receipt"),
                    bridge_receipt_identity,
                )
                .field_evidence_identity(ForgeQueryEvidenceTag::new("policy"), &policy_identity)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("performance"),
                    performance.performance_identity(),
                )
                .field_evidence_identity(ForgeQueryEvidenceTag::new("detail"), detail_identity)
                .seal();
        let receipt_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalInspectionArtifact)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "materialization-receipt",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("materialization"),
                    &materialization_identity,
                )
                .seal();
        Self {
            query_admission_identity: query_admission_identity.clone(),
            bridge_envelope_identity: bridge_envelope_identity.cloned(),
            bridge_receipt_identity: bridge_receipt_identity.cloned(),
            policy_identity,
            performance_identity: performance.performance_identity().clone(),
            materialization_identity,
            receipt_identity,
        }
    }

    pub fn query_admission_for_reporting(&self) -> &str {
        self.query_admission_identity.as_str()
    }

    pub fn query_admission_identity(&self) -> &CausalInspectionOutcomeIdentity {
        &self.query_admission_identity
    }

    pub fn bridge_envelope_digest(&self) -> Option<&str> {
        self.bridge_envelope_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn bridge_envelope_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.bridge_envelope_identity.as_ref()
    }

    pub fn bridge_receipt_digest(&self) -> Option<&str> {
        self.bridge_receipt_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn bridge_receipt_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.bridge_receipt_identity.as_ref()
    }

    pub fn policy_for_reporting(&self) -> &str {
        self.policy_identity.as_str()
    }

    pub fn policy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.policy_identity
    }

    pub fn performance_for_reporting(&self) -> &str {
        self.performance_identity.as_str()
    }

    pub fn performance_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.performance_identity
    }

    pub fn materialization_for_reporting(&self) -> &str {
        self.materialization_identity.as_str()
    }

    pub fn materialization_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.materialization_identity
    }

    pub fn receipt_for_reporting(&self) -> &str {
        self.receipt_identity.as_str()
    }

    pub(super) fn receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_identity
    }
}
