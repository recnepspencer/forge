use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::policy::{BridgePolicyDeclaration, BridgePolicyRejection};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeWritebackAuthoritativeStateBasis, BridgeWritebackAuthorityOutcome,
    BridgeWritebackDeclaration, BridgeWritebackEffectIdentity, BridgeWritebackEffectIntent,
    BridgeWritebackIdempotenceBasis, BridgeWritebackIdempotenceClass,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackNativeCausalityInputs,
    BridgeWritebackReplayBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAdmittedWritebackExecutionRequest {
    policy_declaration: BridgePolicyDeclaration,
    writeback_declaration: BridgeWritebackDeclaration,
    causality: BridgeWritebackNativeCausalityInputs,
    effect_identity: BridgeWritebackEffectIdentity,
    effect_intent: BridgeWritebackEffectIntent,
    authoritative_state_basis: BridgeWritebackAuthoritativeStateBasis,
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
        causality: BridgeWritebackNativeCausalityInputs,
        effect_identity: BridgeWritebackEffectIdentity,
        effect_intent: BridgeWritebackEffectIntent,
        idempotence_identity: BridgeWritebackIdempotenceIdentity,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> Self {
        let authoritative_state_basis =
            BridgeWritebackAuthoritativeStateBasis::from_effect_intent_and_causality(
                &effect_intent,
                &causality,
            );
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-admitted-writeback-execution-request|policy={}|writeback={}|causality={}|effect-identity={}|effect-intent={}|effect-intent-basis={}|authoritative-state={}|idempotence-identity={}|idempotence-class:{:?}",
            policy_declaration.declaration_identity().as_str(),
            writeback_declaration.declaration_identity().as_str(),
            causality.digest(),
            effect_identity.as_str(),
            effect_intent.digest(),
            effect_intent.patch_canonical_basis(),
            authoritative_state_basis.digest(),
            idempotence_identity.as_str(),
            idempotence_class,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            policy_declaration,
            writeback_declaration,
            causality,
            effect_identity,
            effect_intent,
            authoritative_state_basis,
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

    pub(crate) fn causality(&self) -> &BridgeWritebackNativeCausalityInputs {
        &self.causality
    }

    pub fn causality_digest(&self) -> &str {
        self.causality.digest()
    }

    pub fn effect_identity(&self) -> &BridgeWritebackEffectIdentity {
        &self.effect_identity
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        &self.effect_intent
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent.digest()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.effect_intent.patch_canonical_basis()
    }

    pub fn authoritative_state_basis(&self) -> &BridgeWritebackAuthoritativeStateBasis {
        &self.authoritative_state_basis
    }

    pub fn authoritative_state_digest(&self) -> &str {
        self.authoritative_state_basis.digest()
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
    admitted_execution_request: BridgeAdmittedWritebackExecutionRequest,
    contract_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    writeback_effect_artifact_digest: Arc<str>,
    effect_intent_digest: Arc<str>,
    effect_intent_patch_canonical_basis: Arc<str>,
    idempotence_digest: Arc<str>,
    authority_outcome_digest: Arc<str>,
    authority_receipt: crate::adapter::TruthWritebackReceipt,
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
            "bridge-admitted-writeback-execution-receipt|request={}|contract={}|lowered-policy={}|writeback-effect-artifact={}|effect-intent={}|effect-intent-basis={}|idempotence={}|authority-outcome={}|authority-receipt={}|replay-bundle={}",
            request.digest(),
            contract.digest(),
            contract.lowered_policy_digest(),
            effect.digest(),
            effect.effect_intent_digest(),
            effect.effect_intent().patch_canonical_basis(),
            idempotence.digest(),
            outcome.digest(),
            authority_receipt.digest(),
            replay_bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            admitted_execution_request: request.clone(),
            contract_digest: Arc::from(contract.digest().to_owned()),
            lowered_policy_digest: Arc::from(contract.lowered_policy_digest().to_owned()),
            writeback_effect_artifact_digest: Arc::from(effect.digest().to_owned()),
            effect_intent_digest: Arc::from(effect.effect_intent_digest().to_owned()),
            effect_intent_patch_canonical_basis: Arc::from(
                effect.effect_intent().patch_canonical_basis().to_owned(),
            ),
            idempotence_digest: Arc::from(idempotence.digest().to_owned()),
            authority_outcome_digest: Arc::from(outcome.digest().to_owned()),
            authority_receipt: authority_receipt.clone(),
            replay_bundle_digest: Arc::from(replay_bundle.digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-admitted-writeback-execution-receipt:sha256:{digest:x}"
            )),
        }
    }

    pub fn admitted_execution_request(&self) -> &BridgeAdmittedWritebackExecutionRequest {
        &self.admitted_execution_request
    }

    pub fn request_digest(&self) -> &str {
        self.admitted_execution_request.digest()
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }

    pub fn writeback_effect_artifact_digest(&self) -> &str {
        self.writeback_effect_artifact_digest.as_ref()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.effect_intent_patch_canonical_basis.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn authority_outcome_digest(&self) -> &str {
        self.authority_outcome_digest.as_ref()
    }

    pub fn authority_receipt(&self) -> &crate::adapter::TruthWritebackReceipt {
        &self.authority_receipt
    }

    pub fn authority_receipt_digest(&self) -> &str {
        self.authority_receipt.digest()
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
    execution_receipt: BridgeAdmittedWritebackExecutionReceipt,
    digest: Arc<str>,
}

impl BridgeAdmittedWritebackExecution {
    pub(crate) fn new(
        outcome: BridgeWritebackAuthorityOutcome,
        execution_receipt: BridgeAdmittedWritebackExecutionReceipt,
    ) -> Self {
        let canonical_basis = format!(
            "bridge-admitted-writeback-execution|authority-outcome={}|authority-receipt={}|execution-receipt={}",
            outcome.digest(),
            execution_receipt.authority_receipt_digest(),
            execution_receipt.digest(),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            outcome,
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
        self.execution_receipt.authority_receipt()
    }

    pub fn execution_receipt(&self) -> &BridgeAdmittedWritebackExecutionReceipt {
        &self.execution_receipt
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
