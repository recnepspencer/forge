use crate::identity::hash_parts;

use super::{BridgeExecutionOracle, RelationalExecutionOracle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionOracleVerificationKind {
    Mutation,
    Merge,
    Writeback,
    MutationBatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionOracleVerification {
    verification_kind: EffectExecutionOracleVerificationKind,
    execution_subject_digest: String,
    relational_oracle_digest: Option<String>,
    bridge_oracle_digest: Option<String>,
    verification_digest: String,
    component_count: usize,
}

impl EffectExecutionOracleVerification {
    pub(crate) fn relational(
        verification_kind: EffectExecutionOracleVerificationKind,
        execution_subject_digest: &str,
        oracle: &RelationalExecutionOracle,
        component_count: usize,
    ) -> Self {
        let verification_digest = hash_parts(&[
            "effect_execution_oracle_verification_v1".to_string(),
            format!("kind:{verification_kind:?}"),
            format!("subject:{execution_subject_digest}"),
            format!("relational_oracle:{}", oracle.relational_oracle_digest()),
            format!("components:{component_count}"),
        ]);
        Self {
            verification_kind,
            execution_subject_digest: execution_subject_digest.to_string(),
            relational_oracle_digest: Some(oracle.relational_oracle_digest().to_string()),
            bridge_oracle_digest: None,
            verification_digest,
            component_count,
        }
    }

    pub(crate) fn bridge(execution_subject_digest: &str, oracle: &BridgeExecutionOracle) -> Self {
        let verification_digest = hash_parts(&[
            "effect_execution_oracle_verification_v1".to_string(),
            "kind:Writeback".to_string(),
            format!("subject:{execution_subject_digest}"),
            format!("bridge_oracle:{}", oracle.bridge_oracle_digest()),
            "components:1".to_string(),
        ]);
        Self {
            verification_kind: EffectExecutionOracleVerificationKind::Writeback,
            execution_subject_digest: execution_subject_digest.to_string(),
            relational_oracle_digest: None,
            bridge_oracle_digest: Some(oracle.bridge_oracle_digest().to_string()),
            verification_digest,
            component_count: 1,
        }
    }

    pub fn verification_kind(&self) -> EffectExecutionOracleVerificationKind {
        self.verification_kind
    }

    pub fn execution_subject_digest(&self) -> &str {
        &self.execution_subject_digest
    }

    pub fn relational_oracle_digest(&self) -> Option<&str> {
        self.relational_oracle_digest.as_deref()
    }

    pub fn bridge_oracle_digest(&self) -> Option<&str> {
        self.bridge_oracle_digest.as_deref()
    }

    pub fn verification_digest(&self) -> &str {
        &self.verification_digest
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }
}
