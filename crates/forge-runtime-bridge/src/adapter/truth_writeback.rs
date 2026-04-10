use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TruthWritebackAuthorityErrorTag {}
pub type TruthWritebackAuthorityError = BridgeMessageError<TruthWritebackAuthorityErrorTag>;

pub trait TruthWritebackAuthority: Send + Sync + 'static {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthWritebackRequest {
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    contract_digest: Arc<str>,
    candidate_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    mapper_witness_digest: Arc<str>,
    derived_effect_digest: Arc<str>,
    proposed_effect_digest: Arc<str>,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    feedback_provenance_digest: Arc<str>,
    loop_prevention_digest: Arc<str>,
    loop_prevention_disposition: crate::writeback::BridgeWritebackLoopDisposition,
    strategy_compatibility_digest: Arc<str>,
    causality_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    idempotence_class: crate::writeback::BridgeWritebackIdempotenceClass,
    strategy_descriptor_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl TruthWritebackRequest {
    pub fn new(
        family_kind: crate::writeback::BridgeWritebackFamilyKind,
        contract_digest: impl Into<Arc<str>>,
        candidate_digest: impl Into<Arc<str>>,
        mapped_input_digest: impl Into<Arc<str>>,
        mapper_witness_digest: impl Into<Arc<str>>,
        derived_effect_digest: impl Into<Arc<str>>,
        proposed_effect_digest: impl Into<Arc<str>>,
        effect_class: crate::writeback::BridgeWritebackEffectClass,
        strategy_class: crate::writeback::BridgeWritebackStrategyClass,
        feedback_provenance_digest: impl Into<Arc<str>>,
        loop_prevention_digest: impl Into<Arc<str>>,
        loop_prevention_disposition: crate::writeback::BridgeWritebackLoopDisposition,
        strategy_compatibility_digest: impl Into<Arc<str>>,
        causality_digest: impl Into<Arc<str>>,
        idempotence_digest: impl Into<Arc<str>>,
        idempotence_class: crate::writeback::BridgeWritebackIdempotenceClass,
        strategy_descriptor_digest: impl Into<Arc<str>>,
    ) -> Self {
        let contract_digest = contract_digest.into();
        let candidate_digest = candidate_digest.into();
        let mapped_input_digest = mapped_input_digest.into();
        let mapper_witness_digest = mapper_witness_digest.into();
        let derived_effect_digest = derived_effect_digest.into();
        let proposed_effect_digest = proposed_effect_digest.into();
        let feedback_provenance_digest = feedback_provenance_digest.into();
        let loop_prevention_digest = loop_prevention_digest.into();
        let strategy_compatibility_digest = strategy_compatibility_digest.into();
        let causality_digest = causality_digest.into();
        let idempotence_digest = idempotence_digest.into();
        let strategy_descriptor_digest = strategy_descriptor_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "truth-writeback-request|family:{family_kind:?}|contract={}|candidate={}|mapped-input={}|mapper-witness={}|derived-effect={}|proposed-effect={}|effect-class:{effect_class:?}|strategy-class:{strategy_class:?}|feedback-provenance={}|loop-prevention={}|loop-disposition:{loop_prevention_disposition:?}|strategy-compatibility={}|causality={}|idempotence={}|idempotence-class:{idempotence_class:?}|strategy={}",
            contract_digest.as_ref(),
            candidate_digest.as_ref(),
            mapped_input_digest.as_ref(),
            mapper_witness_digest.as_ref(),
            derived_effect_digest.as_ref(),
            proposed_effect_digest.as_ref(),
            feedback_provenance_digest.as_ref(),
            loop_prevention_digest.as_ref(),
            strategy_compatibility_digest.as_ref(),
            causality_digest.as_ref(),
            idempotence_digest.as_ref(),
            strategy_descriptor_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            family_kind,
            contract_digest,
            candidate_digest,
            mapped_input_digest,
            mapper_witness_digest,
            derived_effect_digest,
            proposed_effect_digest,
            effect_class,
            strategy_class,
            feedback_provenance_digest,
            loop_prevention_digest,
            loop_prevention_disposition,
            strategy_compatibility_digest,
            causality_digest,
            idempotence_digest,
            idempotence_class,
            strategy_descriptor_digest,
            canonical_basis,
            digest: Arc::from(format!("truth-writeback-request:sha256:{digest:x}")),
        }
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn candidate_digest(&self) -> &str {
        self.candidate_digest.as_ref()
    }

    pub fn mapper_witness_digest(&self) -> &str {
        self.mapper_witness_digest.as_ref()
    }

    pub fn mapped_input_digest(&self) -> &str {
        self.mapped_input_digest.as_ref()
    }

    pub fn derived_effect_digest(&self) -> &str {
        self.derived_effect_digest.as_ref()
    }

    pub fn proposed_effect_digest(&self) -> &str {
        self.proposed_effect_digest.as_ref()
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn feedback_provenance_digest(&self) -> &str {
        self.feedback_provenance_digest.as_ref()
    }

    pub fn loop_prevention_digest(&self) -> &str {
        self.loop_prevention_digest.as_ref()
    }

    pub fn loop_prevention_disposition(&self) -> crate::writeback::BridgeWritebackLoopDisposition {
        self.loop_prevention_disposition
    }

    pub fn strategy_compatibility_digest(&self) -> &str {
        self.strategy_compatibility_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn idempotence_class(&self) -> crate::writeback::BridgeWritebackIdempotenceClass {
        self.idempotence_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthWritebackReceipt {
    outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
    failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
    authoritative_artifact_digest: Arc<str>,
    request_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl TruthWritebackReceipt {
    pub fn new(
        outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
        authoritative_artifact_digest: impl Into<Arc<str>>,
        request: &TruthWritebackRequest,
    ) -> Self {
        Self::new_with_failure_class(outcome_class, None, authoritative_artifact_digest, request)
    }

    pub fn new_with_failure_class(
        outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
        failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
        authoritative_artifact_digest: impl Into<Arc<str>>,
        request: &TruthWritebackRequest,
    ) -> Self {
        let authoritative_artifact_digest = authoritative_artifact_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "truth-writeback-receipt|request={}|outcome:{outcome_class:?}|failure:{}|authoritative={}",
            request.digest(),
            failure_class
                .map(|class| format!("{class:?}"))
                .unwrap_or_else(|| "none".to_string()),
            authoritative_artifact_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            outcome_class,
            failure_class,
            authoritative_artifact_digest,
            request_digest: Arc::from(request.digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!("truth-writeback-receipt:sha256:{digest:x}")),
        }
    }

    pub fn outcome_class(&self) -> crate::writeback::BridgeWritebackOutcomeClass {
        self.outcome_class
    }

    pub fn failure_class(&self) -> Option<crate::writeback::BridgeWritebackFailureClass> {
        self.failure_class
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn request_digest(&self) -> &str {
        self.request_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
