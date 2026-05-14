use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionOracleErrorKind {
    RelationalObservationMissingBranchHead,
    RelationalOracleUnsupportedEffect,
    RelationalOracleBranchMismatch,
    RelationalOracleCommitMismatch,
    BridgeObservationMissingWritebackRecord,
    BridgeObservationIncompleteWritebackRecord,
    BridgeOracleUnsupportedEffect,
    BridgeOracleOutcomeMismatch,
    BridgeOracleReceiptMismatch,
    BridgeOracleRequestMismatch,
    BatchOracleMixedTargetBranch,
}

impl EffectExecutionOracleErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RelationalObservationMissingBranchHead => {
                "relational_observation_missing_branch_head"
            }
            Self::RelationalOracleUnsupportedEffect => "relational_oracle_unsupported_effect",
            Self::RelationalOracleBranchMismatch => "relational_oracle_branch_mismatch",
            Self::RelationalOracleCommitMismatch => "relational_oracle_commit_mismatch",
            Self::BridgeObservationMissingWritebackRecord => {
                "bridge_observation_missing_writeback_record"
            }
            Self::BridgeObservationIncompleteWritebackRecord => {
                "bridge_observation_incomplete_writeback_record"
            }
            Self::BridgeOracleUnsupportedEffect => "bridge_oracle_unsupported_effect",
            Self::BridgeOracleOutcomeMismatch => "bridge_oracle_outcome_mismatch",
            Self::BridgeOracleReceiptMismatch => "bridge_oracle_receipt_mismatch",
            Self::BridgeOracleRequestMismatch => "bridge_oracle_request_mismatch",
            Self::BatchOracleMixedTargetBranch => "batch_oracle_mixed_target_branch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionOracleError {
    kind: EffectExecutionOracleErrorKind,
    message: String,
    execution_subject_digest: String,
    oracle_digest: Option<String>,
    error_digest: String,
}

impl EffectExecutionOracleError {
    pub(crate) fn new(
        kind: EffectExecutionOracleErrorKind,
        message: impl Into<String>,
        execution_subject_digest: impl Into<String>,
        oracle_digest: Option<&str>,
    ) -> Self {
        let message = message.into();
        let execution_subject_digest = execution_subject_digest.into();
        let error_digest = hash_parts(&[
            "effect_execution_oracle_error_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("subject:{execution_subject_digest}"),
            format!("oracle:{}", oracle_digest.unwrap_or("none")),
            format!("message:{message}"),
        ]);
        Self {
            kind,
            message,
            execution_subject_digest,
            oracle_digest: oracle_digest.map(str::to_string),
            error_digest,
        }
    }

    pub fn kind(&self) -> EffectExecutionOracleErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn execution_subject_digest(&self) -> &str {
        &self.execution_subject_digest
    }

    pub fn oracle_digest(&self) -> Option<&str> {
        self.oracle_digest.as_deref()
    }

    pub fn error_digest(&self) -> &str {
        &self.error_digest
    }
}
