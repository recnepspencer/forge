use super::super::super::identity::CausalInspectionOutcomeIdentity;
use super::super::super::observation_identity::{
    CausalObservationReceiptIdentity, CausalResultShapeContextIdentity,
};
use super::super::{
    causal_identity_digest, CausalInspectionArtifactKind, CausalInspectionBoundaryEnvelopeCategory,
    CausalInspectionPerformanceEnvelope, CausalMaterializationReceipt,
    QueryCausalTemporalAsyncExplanation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeniedQueryCausalInspectionArtifact {
    query_denial_identity: CausalInspectionOutcomeIdentity,
    query_observation_identity: CausalObservationReceiptIdentity,
    result_shape_context_identity: CausalResultShapeContextIdentity,
    denial_reason: String,
    bridge_denial_digest: Option<String>,
    bridge_denial_kind: Option<String>,
    bridge_denial_family: Option<String>,
    temporal_async_explanation: QueryCausalTemporalAsyncExplanation,
    boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
    performance: CausalInspectionPerformanceEnvelope,
    receipt: CausalMaterializationReceipt,
    causal_identity_digest: String,
    artifact_digest: String,
}

impl DeniedQueryCausalInspectionArtifact {
    pub(in crate::runtime::inspection::causal::materialization) fn from_parts(
        query_denial_identity: &CausalInspectionOutcomeIdentity,
        denial_reason: String,
        query_observation_identity: &CausalObservationReceiptIdentity,
        result_shape_context_identity: &CausalResultShapeContextIdentity,
        bridge_denial_digest: Option<String>,
        bridge_denial_kind: Option<String>,
        bridge_denial_family: Option<String>,
        temporal_async_explanation: QueryCausalTemporalAsyncExplanation,
        boundary_categories: Vec<CausalInspectionBoundaryEnvelopeCategory>,
        performance: CausalInspectionPerformanceEnvelope,
        receipt: CausalMaterializationReceipt,
        artifact_digest: String,
    ) -> Self {
        let causal_identity_digest = causal_identity_digest(
            CausalInspectionArtifactKind::Denied,
            query_denial_identity.as_str(),
            query_observation_identity.as_str(),
            None,
            None,
        );
        Self {
            query_denial_identity: query_denial_identity.clone(),
            query_observation_identity: query_observation_identity.clone(),
            result_shape_context_identity: result_shape_context_identity.clone(),
            denial_reason,
            bridge_denial_digest,
            bridge_denial_kind,
            bridge_denial_family,
            temporal_async_explanation,
            boundary_categories,
            performance,
            receipt,
            causal_identity_digest,
            artifact_digest,
        }
    }

    pub fn query_denial_digest(&self) -> &str {
        self.query_denial_identity.as_str()
    }

    pub fn query_observation_digest(&self) -> &str {
        self.query_observation_identity.as_str()
    }

    pub fn result_shape_context_digest(&self) -> &str {
        self.result_shape_context_identity.as_str()
    }

    pub fn denial_reason(&self) -> &str {
        &self.denial_reason
    }

    pub fn bridge_denial_digest(&self) -> Option<&str> {
        self.bridge_denial_digest.as_deref()
    }

    pub fn bridge_denial_kind(&self) -> Option<&str> {
        self.bridge_denial_kind.as_deref()
    }

    pub fn bridge_denial_family(&self) -> Option<&str> {
        self.bridge_denial_family.as_deref()
    }

    pub fn temporal_async_explanation(&self) -> &QueryCausalTemporalAsyncExplanation {
        &self.temporal_async_explanation
    }

    pub fn boundary_categories(&self) -> &[CausalInspectionBoundaryEnvelopeCategory] {
        &self.boundary_categories
    }

    pub fn performance(&self) -> &CausalInspectionPerformanceEnvelope {
        &self.performance
    }

    pub fn receipt(&self) -> &CausalMaterializationReceipt {
        &self.receipt
    }

    pub fn causal_identity_digest(&self) -> &str {
        &self.causal_identity_digest
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}
