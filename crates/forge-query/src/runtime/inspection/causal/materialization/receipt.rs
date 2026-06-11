use crate::identity::hash_parts;

use super::super::identity::CausalInspectionOutcomeIdentity;
use super::{
    CausalInspectionMaterializationPolicy, CausalInspectionPerformanceEnvelope,
    CausalInspectionRedactionPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalMaterializationReceipt {
    query_admission_identity: CausalInspectionOutcomeIdentity,
    bridge_envelope_digest: Option<String>,
    bridge_receipt_digest: Option<String>,
    policy_digest: String,
    performance_digest: String,
    materialization_digest: String,
    receipt_digest: String,
}

impl CausalMaterializationReceipt {
    pub(super) fn new(
        query_admission_identity: &CausalInspectionOutcomeIdentity,
        bridge_envelope_digest: Option<&str>,
        bridge_receipt_digest: Option<&str>,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
        performance: &CausalInspectionPerformanceEnvelope,
        detail_digest: &str,
    ) -> Self {
        let policy_digest = hash_parts(&[
            "causal_inspection_materialization_policy_v1".to_string(),
            format!("redaction:{}", redaction_policy.as_str()),
            format!("materialization:{}", materialization_policy.as_str()),
        ]);
        let materialization_digest = hash_parts(&[
            "causal_inspection_materialization_v1".to_string(),
            format!("query-admission:{}", query_admission_identity.as_str()),
            format!(
                "bridge-envelope:{}",
                bridge_envelope_digest.unwrap_or("none")
            ),
            format!("bridge-receipt:{}", bridge_receipt_digest.unwrap_or("none")),
            format!("policy:{policy_digest}"),
            format!("performance:{}", performance.performance_digest()),
            format!("detail:{detail_digest}"),
        ]);
        let receipt_digest = hash_parts(&[
            "causal_materialization_receipt_v1".to_string(),
            materialization_digest.clone(),
        ]);
        Self {
            query_admission_identity: query_admission_identity.clone(),
            bridge_envelope_digest: bridge_envelope_digest.map(str::to_string),
            bridge_receipt_digest: bridge_receipt_digest.map(str::to_string),
            policy_digest,
            performance_digest: performance.performance_digest().to_string(),
            materialization_digest,
            receipt_digest,
        }
    }

    pub fn query_admission_digest(&self) -> &str {
        self.query_admission_identity.as_str()
    }

    pub fn bridge_envelope_digest(&self) -> Option<&str> {
        self.bridge_envelope_digest.as_deref()
    }

    pub fn bridge_receipt_digest(&self) -> Option<&str> {
        self.bridge_receipt_digest.as_deref()
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn performance_digest(&self) -> &str {
        &self.performance_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
