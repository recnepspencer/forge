use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeWritebackError, BridgeWritebackErrorKind};
use crate::identity::{BridgeIdentity, WritebackCandidateIdentityTag};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackIdempotenceBasis,
    BridgeWritebackLoopDisposition, BridgeWritebackLoopPreventionReport,
    BridgeWritebackRetryDisposition, BridgeWritebackStrategyCompatibilityDisposition,
    BridgeWritebackStrategyCompatibilityReport,
};

pub type BridgeWritebackCandidateIdentity = BridgeIdentity<WritebackCandidateIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeValidatedWritebackCandidate {
    candidate_identity: BridgeWritebackCandidateIdentity,
    contract_digest: Arc<str>,
    effect_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    loop_prevention_digest: Arc<str>,
    strategy_compatibility_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    retry_disposition: BridgeWritebackRetryDisposition,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeValidatedWritebackCandidate {
    pub(crate) fn new(
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        loop_prevention: &BridgeWritebackLoopPreventionReport,
        strategy_compatibility: &BridgeWritebackStrategyCompatibilityReport,
    ) -> Result<Self, BridgeWritebackError> {
        match loop_prevention.disposition() {
            BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt => {}
            BridgeWritebackLoopDisposition::CanonicalNoop => {
                return Err(BridgeWritebackError::new(
                    BridgeWritebackErrorKind::InvariantRejected,
                    format!(
                        "writeback candidate validation rejected canonical noop loop disposition: {}",
                        loop_prevention.digest()
                    ),
                ));
            }
            BridgeWritebackLoopDisposition::RejectAsUnsafeFeedback => {
                return Err(BridgeWritebackError::new(
                    BridgeWritebackErrorKind::InvariantRejected,
                    format!(
                        "writeback candidate validation rejected unsafe feedback disposition: {}",
                        loop_prevention.digest()
                    ),
                ));
            }
        }

        match strategy_compatibility.disposition() {
            BridgeWritebackStrategyCompatibilityDisposition::Compatible => {}
            BridgeWritebackStrategyCompatibilityDisposition::FamilyKindMismatch
            | BridgeWritebackStrategyCompatibilityDisposition::StrategyClassMismatch
            | BridgeWritebackStrategyCompatibilityDisposition::StrategyDescriptorMismatch
            | BridgeWritebackStrategyCompatibilityDisposition::EffectClassMismatch => {
                return Err(BridgeWritebackError::new(
                    match strategy_compatibility.disposition() {
                        BridgeWritebackStrategyCompatibilityDisposition::FamilyKindMismatch => {
                            BridgeWritebackErrorKind::FamilyBindingMismatch
                        }
                        _ => BridgeWritebackErrorKind::StrategyDescriptorMismatch,
                    },
                    format!(
                        "writeback candidate validation rejected incompatible strategy contract: {}",
                        strategy_compatibility.digest()
                    ),
                ));
            }
            BridgeWritebackStrategyCompatibilityDisposition::IdempotenceClassMismatch => {
                return Err(BridgeWritebackError::new(
                    BridgeWritebackErrorKind::IdempotenceBasisMismatch,
                    format!(
                        "writeback candidate validation rejected incompatible idempotence basis: {}",
                        strategy_compatibility.digest()
                    ),
                ));
            }
        }

        let retry_disposition = match idempotence.idempotence_class() {
            crate::writeback::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression => {
                BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
            }
            crate::writeback::BridgeWritebackIdempotenceClass::AllowRepeatedAuthorityAttempt => {
                return Err(BridgeWritebackError::new(
                    BridgeWritebackErrorKind::IdempotenceBasisMismatch,
                    format!(
                        "writeback candidate validation rejected unsupported retry semantics for idempotence basis `{}`",
                        idempotence.digest()
                    ),
                ));
            }
        };

        let contract_digest = Arc::<str>::from(contract.digest().to_owned());
        let effect_digest = Arc::<str>::from(effect.digest().to_owned());
        let idempotence_digest = Arc::<str>::from(idempotence.digest().to_owned());
        let loop_prevention_digest = Arc::<str>::from(loop_prevention.digest().to_owned());
        let strategy_compatibility_digest =
            Arc::<str>::from(strategy_compatibility.digest().to_owned());
        let family_kind = effect.family_kind();
        let strategy_descriptor_digest =
            Arc::<str>::from(effect.strategy_descriptor_digest().to_owned());
        let strategy_class = effect.strategy_class();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-validated-writeback-candidate|contract={}|effect={}|idempotence={}|loop-prevention={}|strategy-compatibility={}|family:{family_kind:?}|strategy-class:{strategy_class:?}|strategy={}|retry:{retry_disposition:?}|lowered-policy={}|causality={}",
            contract_digest.as_ref(),
            effect_digest.as_ref(),
            idempotence_digest.as_ref(),
            loop_prevention_digest.as_ref(),
            strategy_compatibility_digest.as_ref(),
            strategy_descriptor_digest.as_ref(),
            idempotence.lowered_policy_digest(),
            idempotence.causality_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            candidate_identity: BridgeWritebackCandidateIdentity::new(format!(
                "bridge-writeback-candidate:sha256:{digest:x}"
            )),
            contract_digest,
            effect_digest,
            idempotence_digest,
            loop_prevention_digest,
            strategy_compatibility_digest,
            family_kind,
            strategy_class,
            strategy_descriptor_digest,
            retry_disposition,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-candidate:sha256:{digest:x}")),
        })
    }

    pub fn candidate_identity(&self) -> &BridgeWritebackCandidateIdentity {
        &self.candidate_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn loop_prevention_digest(&self) -> &str {
        self.loop_prevention_digest.as_ref()
    }

    pub fn strategy_compatibility_digest(&self) -> &str {
        self.strategy_compatibility_digest.as_ref()
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

    pub fn retry_disposition(&self) -> BridgeWritebackRetryDisposition {
        self.retry_disposition
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}
