use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::policy::{BridgePolicyDeclaration, BridgePolicyRejection};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackAuthorityOutcome,
    BridgeWritebackCausalityBasis, BridgeWritebackDeclaration, BridgeWritebackEffectIdentity,
    BridgeWritebackIdempotenceBasis, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackReplayBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAdmittedWritebackExecutionRequest {
    policy_declaration: BridgePolicyDeclaration,
    writeback_declaration: BridgeWritebackDeclaration,
    causality: BridgeWritebackCausalityBasis,
    effect_identity: BridgeWritebackEffectIdentity,
    effect_digest: Arc<str>,
    authoritative_state_digest: Arc<str>,
    idempotence_identity: BridgeWritebackIdempotenceIdentity,
    idempotence_class: BridgeWritebackIdempotenceClass,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAdmittedWritebackExecutionRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        policy_declaration: BridgePolicyDeclaration,
        writeback_declaration: BridgeWritebackDeclaration,
        causality: BridgeWritebackCausalityBasis,
        effect_identity: BridgeWritebackEffectIdentity,
        effect_digest: impl Into<Arc<str>>,
        authoritative_state_digest: impl Into<Arc<str>>,
        idempotence_identity: BridgeWritebackIdempotenceIdentity,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> Self {
        let effect_digest = effect_digest.into();
        let authoritative_state_digest = authoritative_state_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-admitted-writeback-execution-request|policy={}|writeback={}|causality={}|effect-identity={}|effect-digest={}|authoritative-state={}|idempotence-identity={}|idempotence-class:{:?}",
            policy_declaration.declaration_identity().as_str(),
            writeback_declaration.declaration_identity().as_str(),
            causality.digest(),
            effect_identity.as_str(),
            effect_digest.as_ref(),
            authoritative_state_digest.as_ref(),
            idempotence_identity.as_str(),
            idempotence_class,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            policy_declaration,
            writeback_declaration,
            causality,
            effect_identity,
            effect_digest,
            authoritative_state_digest,
            idempotence_identity,
            idempotence_class,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-admitted-writeback-execution-request:sha256:{digest:x}"
            )),
        }
    }

    pub fn policy_declaration(&self) -> &BridgePolicyDeclaration {
        &self.policy_declaration
    }

    pub fn writeback_declaration(&self) -> &BridgeWritebackDeclaration {
        &self.writeback_declaration
    }

    pub fn causality(&self) -> &BridgeWritebackCausalityBasis {
        &self.causality
    }

    pub fn effect_identity(&self) -> &BridgeWritebackEffectIdentity {
        &self.effect_identity
    }

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn authoritative_state_digest(&self) -> &str {
        self.authoritative_state_digest.as_ref()
    }

    pub fn idempotence_identity(&self) -> &BridgeWritebackIdempotenceIdentity {
        &self.idempotence_identity
    }

    pub fn idempotence_class(&self) -> BridgeWritebackIdempotenceClass {
        self.idempotence_class
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeAdmittedWritebackExecutionError {
    PolicyAdmission(BridgePolicyRejection),
    Writeback(Arc<crate::error::BridgeWritebackError>),
}

impl BridgeAdmittedWritebackExecutionError {
    pub fn policy_admission(error: BridgePolicyRejection) -> Self {
        Self::PolicyAdmission(error)
    }

    pub fn writeback(error: crate::error::BridgeWritebackError) -> Self {
        Self::Writeback(Arc::new(error))
    }
}

impl std::fmt::Display for BridgeAdmittedWritebackExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PolicyAdmission(error) => write!(f, "{error:?}"),
            Self::Writeback(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for BridgeAdmittedWritebackExecutionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAdmittedWritebackExecutionReceipt {
    request_digest: Arc<str>,
    contract_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    effect_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    authority_outcome_digest: Arc<str>,
    authority_receipt_digest: Arc<str>,
    replay_bundle_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAdmittedWritebackExecutionReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        request: &BridgeAdmittedWritebackExecutionRequest,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        outcome: &BridgeWritebackAuthorityOutcome,
        authority_receipt: &crate::adapter::TruthWritebackReceipt,
        replay_bundle: &BridgeWritebackReplayBundle,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-admitted-writeback-execution-receipt|request={}|contract={}|lowered-policy={}|effect={}|idempotence={}|authority-outcome={}|authority-receipt={}|replay-bundle={}",
            request.digest(),
            contract.digest(),
            contract.lowered_policy_digest(),
            effect.digest(),
            idempotence.digest(),
            outcome.digest(),
            authority_receipt.digest(),
            replay_bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            request_digest: Arc::from(request.digest().to_owned()),
            contract_digest: Arc::from(contract.digest().to_owned()),
            lowered_policy_digest: Arc::from(contract.lowered_policy_digest().to_owned()),
            effect_digest: Arc::from(effect.digest().to_owned()),
            idempotence_digest: Arc::from(idempotence.digest().to_owned()),
            authority_outcome_digest: Arc::from(outcome.digest().to_owned()),
            authority_receipt_digest: Arc::from(authority_receipt.digest().to_owned()),
            replay_bundle_digest: Arc::from(replay_bundle.digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-admitted-writeback-execution-receipt:sha256:{digest:x}"
            )),
        }
    }

    pub fn request_digest(&self) -> &str {
        self.request_digest.as_ref()
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn authority_outcome_digest(&self) -> &str {
        self.authority_outcome_digest.as_ref()
    }

    pub fn authority_receipt_digest(&self) -> &str {
        self.authority_receipt_digest.as_ref()
    }

    pub fn replay_bundle_digest(&self) -> &str {
        self.replay_bundle_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAdmittedWritebackExecution {
    outcome: BridgeWritebackAuthorityOutcome,
    authority_receipt: crate::adapter::TruthWritebackReceipt,
    execution_receipt: BridgeAdmittedWritebackExecutionReceipt,
    digest: Arc<str>,
}

impl BridgeAdmittedWritebackExecution {
    pub(crate) fn new(
        outcome: BridgeWritebackAuthorityOutcome,
        authority_receipt: crate::adapter::TruthWritebackReceipt,
        execution_receipt: BridgeAdmittedWritebackExecutionReceipt,
    ) -> Self {
        let canonical_basis = format!(
            "bridge-admitted-writeback-execution|authority-outcome={}|authority-receipt={}|execution-receipt={}",
            outcome.digest(),
            authority_receipt.digest(),
            execution_receipt.digest(),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            outcome,
            authority_receipt,
            execution_receipt,
            digest: Arc::from(format!(
                "bridge-admitted-writeback-execution:sha256:{digest:x}"
            )),
        }
    }

    pub fn outcome(&self) -> &BridgeWritebackAuthorityOutcome {
        &self.outcome
    }

    pub fn authority_receipt(&self) -> &crate::adapter::TruthWritebackReceipt {
        &self.authority_receipt
    }

    pub fn execution_receipt(&self) -> &BridgeAdmittedWritebackExecutionReceipt {
        &self.execution_receipt
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
