use serde::Serialize;

use super::{
    WorthServerCompletedExternalResourceExecution, WorthServerExternalResourceExecutionCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerAdmittedExternalResourceResult {
    request_identity: String,
    contract_identity: String,
    schema_identity: String,
    schema_version: u32,
    result_digest: String,
    transport_evidence_identity: String,
    counters: WorthServerExternalResourceExecutionCounters,
}

impl WorthServerCompletedExternalResourceExecution {
    pub fn admit_json_result<T: Serialize>(
        self,
        schema_identity: impl Into<String>,
        schema_version: u32,
        result: &T,
    ) -> Result<
        WorthServerAdmittedExternalResourceResult,
        WorthServerExternalResourceResultAdmissionError,
    > {
        let schema_identity = schema_identity.into();
        if schema_identity.trim().is_empty() || schema_version == 0 {
            return Err(WorthServerExternalResourceResultAdmissionError::InvalidSchema);
        }
        let canonical_json = serde_json::to_string(result)
            .map_err(|_| WorthServerExternalResourceResultAdmissionError::SerializationFailed)?;
        let (plan, transport_evidence_identity, counters) = self.into_parts();
        let result_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-external-resource-result-v1",
        )
        .field("request_identity", plan.request_identity())
        .field("plan_digest", plan.canonical_digest())
        .field("contract_identity", plan.contract_identity())
        .field("schema_identity", &schema_identity)
        .field("schema_version", &schema_version.to_string())
        .field("canonical_json", &canonical_json)
        .finish();
        Ok(WorthServerAdmittedExternalResourceResult {
            request_identity: plan.request_identity().to_string(),
            contract_identity: plan.contract_identity().to_string(),
            schema_identity,
            schema_version,
            result_digest,
            transport_evidence_identity,
            counters,
        })
    }
}

impl WorthServerAdmittedExternalResourceResult {
    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    pub fn schema_identity(&self) -> &str {
        &self.schema_identity
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn transport_evidence_identity(&self) -> &str {
        &self.transport_evidence_identity
    }

    pub fn counters(&self) -> WorthServerExternalResourceExecutionCounters {
        self.counters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerExternalResourceResultAdmissionError {
    InvalidSchema,
    SerializationFailed,
}
