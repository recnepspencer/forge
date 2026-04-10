use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackAuthorityOutcome,
    BridgeWritebackIdempotenceBasis, BridgeWritebackOutcomeClass, BridgeWritebackRetryDisposition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackReplayBundle {
    contract_digest: Arc<str>,
    effect_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    causality_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    retry_disposition: BridgeWritebackRetryDisposition,
    outcome_digest: Arc<str>,
    outcome_class: BridgeWritebackOutcomeClass,
    authoritative_artifact_digest: Arc<str>,
    semantic_basis: Arc<str>,
    semantic_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackReplayBundle {
    pub fn from_canonical_records(
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        outcome: &BridgeWritebackAuthorityOutcome,
    ) -> Self {
        let retry_disposition = match idempotence.idempotence_class() {
            crate::writeback::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression => {
                BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
            }
            crate::writeback::BridgeWritebackIdempotenceClass::AllowRepeatedAuthorityAttempt => {
                panic!("Phase 2 replay bundle does not admit repeated-authority retry semantics")
            }
        };
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-replay-bundle|contract={}|effect={}|family:{:?}|strategy-class:{:?}|strategy={}|causality={}|idempotence={}|lowered-policy={}|retry:{retry_disposition:?}|outcome={}|outcome-class:{:?}|authoritative={}",
            contract.digest(),
            effect.digest(),
            effect.family_kind(),
            effect.strategy_class(),
            effect.strategy_descriptor_digest(),
            effect.causality_digest(),
            idempotence.digest(),
            idempotence.lowered_policy_digest(),
            outcome.digest(),
            outcome.outcome_class(),
            outcome.authoritative_artifact_digest(),
        ));
        let semantic_basis = Arc::<str>::from(format!(
            "bridge-writeback-replay-bundle-semantic|family:{:?}|effect:{:?}|effect-digest={}|strategy-class:{:?}|strategy={}|causality={}|idempotence-class:{:?}|authoritative-state={}|retry:{retry_disposition:?}|outcome-class:{:?}",
            effect.family_kind(),
            effect.effect_class(),
            effect.effect_digest(),
            effect.strategy_class(),
            effect.strategy_descriptor_digest(),
            effect.causality_digest(),
            idempotence.idempotence_class(),
            idempotence.authoritative_state_digest(),
            outcome.outcome_class(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let semantic_digest = Sha256::digest(semantic_basis.as_bytes());
        Self {
            contract_digest: Arc::from(contract.digest().to_owned()),
            effect_digest: Arc::from(effect.effect_digest().to_owned()),
            family_kind: effect.family_kind(),
            strategy_class: effect.strategy_class(),
            strategy_descriptor_digest: Arc::from(effect.strategy_descriptor_digest().to_owned()),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            idempotence_digest: Arc::from(idempotence.digest().to_owned()),
            lowered_policy_digest: Arc::from(idempotence.lowered_policy_digest().to_owned()),
            retry_disposition,
            outcome_digest: Arc::from(outcome.digest().to_owned()),
            outcome_class: outcome.outcome_class(),
            authoritative_artifact_digest: Arc::from(
                outcome.authoritative_artifact_digest().to_owned(),
            ),
            semantic_basis,
            semantic_digest: Arc::from(format!(
                "bridge-writeback-replay-bundle-semantic:sha256:{semantic_digest:x}"
            )),
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-replay-bundle:sha256:{digest:x}")),
        }
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }

    pub fn retry_disposition(&self) -> BridgeWritebackRetryDisposition {
        self.retry_disposition
    }

    pub fn outcome_digest(&self) -> &str {
        self.outcome_digest.as_ref()
    }

    pub fn outcome_class(&self) -> BridgeWritebackOutcomeClass {
        self.outcome_class
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn semantic_basis(&self) -> &str {
        self.semantic_basis.as_ref()
    }

    pub fn semantic_digest(&self) -> &str {
        self.semantic_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}
