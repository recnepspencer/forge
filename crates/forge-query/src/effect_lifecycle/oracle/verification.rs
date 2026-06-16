use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::{BridgeExecutionOracle, RelationalExecutionOracle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExecutionOracleVerificationKind {
    Mutation,
    Merge,
    Writeback,
    MutationBatch,
}

impl EffectExecutionOracleVerificationKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Merge => "merge",
            Self::Writeback => "writeback",
            Self::MutationBatch => "mutation_batch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectExecutionOracleVerification {
    verification_kind: EffectExecutionOracleVerificationKind,
    execution_subject_identity: ForgeQueryEvidenceIdentity,
    relational_oracle_identity: Option<ForgeQueryEvidenceIdentity>,
    bridge_oracle_identity: Option<ForgeQueryEvidenceIdentity>,
    verification_identity: ForgeQueryEvidenceIdentity,
    component_count: usize,
}

impl EffectExecutionOracleVerification {
    pub(crate) fn relational(
        verification_kind: EffectExecutionOracleVerificationKind,
        execution_subject_identity: &ForgeQueryEvidenceIdentity,
        oracle: &RelationalExecutionOracle,
        component_count: usize,
    ) -> Self {
        let verification_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "effect_execution_oracle_verification_v1",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("kind"),
                    verification_kind.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_subject"),
                    execution_subject_identity,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("relational_oracle"),
                    oracle.relational_oracle_identity(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("component_count"),
                    component_count,
                )
                .seal();
        Self {
            verification_kind,
            execution_subject_identity: execution_subject_identity.clone(),
            relational_oracle_identity: Some(oracle.relational_oracle_identity().clone()),
            bridge_oracle_identity: None,
            verification_identity,
            component_count,
        }
    }

    pub(crate) fn bridge(
        execution_subject_identity: &ForgeQueryEvidenceIdentity,
        oracle: &BridgeExecutionOracle,
    ) -> Self {
        let verification_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::EffectIntentReceipt)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "effect_execution_oracle_verification_v1",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("kind"),
                    EffectExecutionOracleVerificationKind::Writeback.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_subject"),
                    execution_subject_identity,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("bridge_oracle"),
                    oracle.bridge_oracle_identity(),
                )
                .field_usize(ForgeQueryEvidenceTag::new("component_count"), 1)
                .seal();
        Self {
            verification_kind: EffectExecutionOracleVerificationKind::Writeback,
            execution_subject_identity: execution_subject_identity.clone(),
            relational_oracle_identity: None,
            bridge_oracle_identity: Some(oracle.bridge_oracle_identity().clone()),
            verification_identity,
            component_count: 1,
        }
    }

    pub fn verification_kind(&self) -> EffectExecutionOracleVerificationKind {
        self.verification_kind
    }

    pub fn execution_subject_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.execution_subject_identity
    }

    pub fn execution_subject_for_reporting(&self) -> &str {
        self.execution_subject_identity.as_str()
    }

    pub fn relational_oracle_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.relational_oracle_identity.as_ref()
    }

    pub fn relational_oracle_for_reporting(&self) -> Option<&str> {
        self.relational_oracle_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn bridge_oracle_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.bridge_oracle_identity.as_ref()
    }

    pub fn bridge_oracle_for_reporting(&self) -> Option<&str> {
        self.bridge_oracle_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn verification_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.verification_identity
    }

    pub fn verification_for_reporting(&self) -> &str {
        self.verification_identity.as_str()
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }
}
