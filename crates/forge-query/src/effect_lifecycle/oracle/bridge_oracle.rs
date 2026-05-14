use forge_runtime_bridge::facade::RuntimeBridge;

use crate::identity::hash_parts;

use super::{EffectExecutionOracleError, EffectExecutionOracleErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeExecutionOracle {
    execution_record_digest: String,
    outcome_digest: String,
    outcome_class: forge_runtime_bridge::facade::BridgeWritebackOutcomeClass,
    request_digest: String,
    receipt_digest: String,
    bridge_oracle_digest: String,
}

impl BridgeExecutionOracle {
    pub fn new(
        execution_record_digest: impl Into<String>,
        outcome_digest: impl Into<String>,
        outcome_class: forge_runtime_bridge::facade::BridgeWritebackOutcomeClass,
        request_digest: impl Into<String>,
        receipt_digest: impl Into<String>,
    ) -> Self {
        let execution_record_digest = execution_record_digest.into();
        let outcome_digest = outcome_digest.into();
        let request_digest = request_digest.into();
        let receipt_digest = receipt_digest.into();
        let bridge_oracle_digest = hash_parts(&[
            "bridge_execution_oracle_v1".to_string(),
            format!("record:{execution_record_digest}"),
            format!("outcome:{outcome_digest}"),
            format!("outcome_class:{outcome_class:?}"),
            format!("request:{request_digest}"),
            format!("receipt:{receipt_digest}"),
        ]);
        Self {
            execution_record_digest,
            outcome_digest,
            outcome_class,
            request_digest,
            receipt_digest,
            bridge_oracle_digest,
        }
    }

    pub fn observe_last_writeback(
        runtime: &RuntimeBridge,
    ) -> Result<Self, EffectExecutionOracleError> {
        let record = runtime
            .diagnostics()
            .last_writeback_execution_record()
            .ok_or_else(|| {
                EffectExecutionOracleError::new(
                    EffectExecutionOracleErrorKind::BridgeObservationMissingWritebackRecord,
                    "independent bridge oracle inspection could not find a writeback execution record",
                    "bridge:last-writeback".to_string(),
                    None,
                )
            })?;
        let outcome_digest = record
            .outcome_digest()
            .ok_or_else(|| incomplete_bridge_record_error("outcome", record.digest()))?;
        let request_digest = record
            .request_digest()
            .ok_or_else(|| incomplete_bridge_record_error("request", record.digest()))?;
        let receipt_digest = record
            .receipt_digest()
            .ok_or_else(|| incomplete_bridge_record_error("receipt", record.digest()))?;
        Ok(Self::new(
            record.digest(),
            outcome_digest,
            record
                .outcome_class()
                .ok_or_else(|| incomplete_bridge_record_error("outcome_class", record.digest()))?,
            request_digest,
            receipt_digest,
        ))
    }

    pub fn execution_record_digest(&self) -> &str {
        &self.execution_record_digest
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub fn outcome_class(&self) -> forge_runtime_bridge::facade::BridgeWritebackOutcomeClass {
        self.outcome_class
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn bridge_oracle_digest(&self) -> &str {
        &self.bridge_oracle_digest
    }
}

fn incomplete_bridge_record_error(
    missing_field: &str,
    record_digest: &str,
) -> EffectExecutionOracleError {
    EffectExecutionOracleError::new(
        EffectExecutionOracleErrorKind::BridgeObservationIncompleteWritebackRecord,
        format!(
            "independent bridge oracle inspection found writeback record `{record_digest}` without `{missing_field}`"
        ),
        "bridge:last-writeback".to_string(),
        Some(record_digest),
    )
}
