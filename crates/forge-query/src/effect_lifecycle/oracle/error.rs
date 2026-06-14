use crate::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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
    execution_subject_identity: ForgeQueryEvidenceIdentity,
    oracle_identity: Option<ForgeQueryEvidenceIdentity>,
    error_identity: ForgeQueryEvidenceIdentity,
}

impl EffectExecutionOracleError {
    pub(crate) fn new(
        kind: EffectExecutionOracleErrorKind,
        message: impl Into<String>,
        execution_subject_identity: &ForgeQueryEvidenceIdentity,
        oracle_identity: Option<&ForgeQueryEvidenceIdentity>,
    ) -> Self {
        let message = message.into();
        let error_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::EffectIntentReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_execution_oracle_error_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("execution_subject"),
            execution_subject_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("oracle"),
            oracle_identity,
        )
        .field_shape(ForgeQueryEvidenceTag::new("message"), message.as_str())
        .seal();
        Self {
            kind,
            message,
            execution_subject_identity: execution_subject_identity.clone(),
            oracle_identity: oracle_identity.cloned(),
            error_identity,
        }
    }

    pub fn kind(&self) -> EffectExecutionOracleErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn execution_subject_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.execution_subject_identity
    }

    pub fn execution_subject_for_reporting(&self) -> &str {
        self.execution_subject_identity.as_str()
    }

    pub fn oracle_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.oracle_identity.as_ref()
    }

    pub fn oracle_for_reporting(&self) -> Option<&str> {
        self.oracle_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn error_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.error_identity
    }

    pub fn error_for_reporting(&self) -> &str {
        self.error_identity.as_str()
    }
}

pub(crate) fn bridge_oracle_observation_subject() -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "bridge_oracle_observation_subject_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), "last_writeback")
        .seal()
}
