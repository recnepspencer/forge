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

pub(crate) struct TruthWritebackRequestEvidence<'a> {
    pub(crate) contract: &'a crate::writeback::AdmittedBridgeWritebackContract,
    pub(crate) candidate: &'a crate::writeback::BridgeValidatedWritebackCandidate,
    pub(crate) effect: &'a crate::writeback::BridgeDerivedWritebackEffect,
    pub(crate) mapper_witness: &'a crate::writeback::BridgeWritebackMapperWitness,
    pub(crate) feedback_provenance: &'a crate::writeback::BridgeWritebackFeedbackProvenance,
    pub(crate) loop_prevention: &'a crate::writeback::BridgeWritebackLoopPreventionReport,
    pub(crate) strategy_coherence: &'a crate::writeback::BridgeWritebackStrategyCoherenceReport,
    pub(crate) idempotence: &'a crate::writeback::BridgeWritebackIdempotenceBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthWritebackRequest {
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    contract_digest: Arc<str>,
    candidate_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    mapper_witness_digest: Arc<str>,
    writeback_effect_artifact_digest: Arc<str>,
    effect_intent: crate::writeback::BridgeWritebackEffectIntent,
    effect_intent_digest: Arc<str>,
    effect_intent_patch_canonical_basis: Arc<str>,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    feedback_provenance_digest: Arc<str>,
    loop_prevention_digest: Arc<str>,
    loop_prevention_disposition: crate::writeback::BridgeWritebackLoopDisposition,
    strategy_coherence_digest: Arc<str>,
    causality_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    idempotence_class: crate::writeback::BridgeWritebackIdempotenceClass,
    strategy_descriptor_basis: crate::writeback::BridgeWritebackStrategyDescriptorBasis,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl TruthWritebackRequest {
    pub(crate) fn from_evidence(evidence: TruthWritebackRequestEvidence<'_>) -> Self {
        let family_kind = evidence.effect.family_kind();
        let contract_digest = Arc::<str>::from(evidence.contract.digest().to_owned());
        let candidate_digest = Arc::<str>::from(evidence.candidate.digest().to_owned());
        let mapped_input_digest =
            Arc::<str>::from(evidence.mapper_witness.mapped_input_digest().to_owned());
        let mapper_witness_digest = Arc::<str>::from(evidence.mapper_witness.digest().to_owned());
        let writeback_effect_artifact_digest =
            Arc::<str>::from(evidence.effect.digest().to_owned());
        let effect_intent = evidence.effect.effect_intent().clone();
        let effect_intent_digest = Arc::<str>::from(effect_intent.digest().to_owned());
        let effect_intent_patch_canonical_basis =
            Arc::<str>::from(effect_intent.patch_canonical_basis().to_owned());
        let effect_class = evidence.effect.effect_class();
        let strategy_class = evidence.effect.strategy_class();
        let feedback_provenance_digest =
            Arc::<str>::from(evidence.feedback_provenance.digest().to_owned());
        let loop_prevention_digest = Arc::<str>::from(evidence.loop_prevention.digest().to_owned());
        let loop_prevention_disposition = evidence.loop_prevention.disposition();
        let strategy_coherence_digest =
            Arc::<str>::from(evidence.strategy_coherence.digest().to_owned());
        let causality_digest = Arc::<str>::from(evidence.idempotence.causality_digest().to_owned());
        let idempotence_digest = Arc::<str>::from(evidence.idempotence.digest().to_owned());
        let idempotence_class = evidence.idempotence.idempotence_class();
        let strategy_descriptor_basis = evidence.effect.strategy_descriptor_basis().clone();
        let canonical_basis = Arc::<str>::from(format!(
            "truth-writeback-request|family:{family_kind:?}|contract={}|candidate={}|mapped-input={}|mapper-witness={}|writeback-effect-artifact={}|effect-intent={}|effect-intent-basis={}|effect-class:{effect_class:?}|strategy-class:{strategy_class:?}|feedback-provenance={}|loop-prevention={}|loop-disposition:{loop_prevention_disposition:?}|strategy-coherence={}|causality={}|idempotence={}|idempotence-class:{idempotence_class:?}|strategy={}",
            contract_digest.as_ref(),
            candidate_digest.as_ref(),
            mapped_input_digest.as_ref(),
            mapper_witness_digest.as_ref(),
            writeback_effect_artifact_digest.as_ref(),
            effect_intent_digest.as_ref(),
            effect_intent_patch_canonical_basis.as_ref(),
            feedback_provenance_digest.as_ref(),
            loop_prevention_digest.as_ref(),
            strategy_coherence_digest.as_ref(),
            causality_digest.as_ref(),
            idempotence_digest.as_ref(),
            strategy_descriptor_basis.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            family_kind,
            contract_digest,
            candidate_digest,
            mapped_input_digest,
            mapper_witness_digest,
            writeback_effect_artifact_digest,
            effect_intent,
            effect_intent_digest,
            effect_intent_patch_canonical_basis,
            effect_class,
            strategy_class,
            feedback_provenance_digest,
            loop_prevention_digest,
            loop_prevention_disposition,
            strategy_coherence_digest,
            causality_digest,
            idempotence_digest,
            idempotence_class,
            strategy_descriptor_basis,
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

    pub fn writeback_effect_artifact_digest(&self) -> &str {
        self.writeback_effect_artifact_digest.as_ref()
    }

    pub fn effect_intent(&self) -> &crate::writeback::BridgeWritebackEffectIntent {
        &self.effect_intent
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.effect_intent_patch_canonical_basis.as_ref()
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

    pub fn strategy_coherence_digest(&self) -> &str {
        self.strategy_coherence_digest.as_ref()
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
        self.strategy_descriptor_basis.digest()
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
    effect_intent: crate::writeback::BridgeWritebackEffectIntent,
    authoritative_artifact_digest: Arc<str>,
    request_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl TruthWritebackReceipt {
    pub fn new(
        outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
        request: &TruthWritebackRequest,
    ) -> Self {
        Self::new_with_failure_class(outcome_class, None, request)
    }

    pub fn new_with_failure_class(
        outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
        failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
        request: &TruthWritebackRequest,
    ) -> Self {
        let authoritative_artifact_digest = derive_writeback_receipt_authoritative_artifact_digest(
            outcome_class,
            failure_class,
            request,
            None,
        );
        Self::from_derived_authoritative_artifact(
            outcome_class,
            failure_class,
            authoritative_artifact_digest,
            request,
            None,
        )
    }

    pub fn canonical_noop_from_prior_receipt(
        request: &TruthWritebackRequest,
        prior_receipt: &TruthWritebackReceipt,
    ) -> Self {
        Self::from_derived_authoritative_artifact(
            crate::writeback::BridgeWritebackOutcomeClass::CanonicalNoop,
            None,
            Arc::from(prior_receipt.authoritative_artifact_digest()),
            request,
            Some(prior_receipt),
        )
    }

    fn from_derived_authoritative_artifact(
        outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
        failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
        authoritative_artifact_digest: Arc<str>,
        request: &TruthWritebackRequest,
        prior_receipt: Option<&TruthWritebackReceipt>,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "truth-writeback-receipt|request={}|outcome:{outcome_class:?}|failure:{}|authoritative={}|prior-receipt={}",
            request.digest(),
            failure_class
                .map(|class| format!("{class:?}"))
                .unwrap_or_else(|| "none".to_string()),
            authoritative_artifact_digest.as_ref(),
            prior_receipt.map_or("none", TruthWritebackReceipt::digest),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            outcome_class,
            failure_class,
            effect_intent: request.effect_intent().clone(),
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

    pub fn effect_intent(&self) -> &crate::writeback::BridgeWritebackEffectIntent {
        &self.effect_intent
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

fn derive_writeback_receipt_authoritative_artifact_digest(
    outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
    failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
    request: &TruthWritebackRequest,
    prior_receipt: Option<&TruthWritebackReceipt>,
) -> Arc<str> {
    let canonical_basis = format!(
        "truth-writeback-authoritative-artifact|request={}|outcome:{outcome_class:?}|failure:{}|effect-intent={}|effect-intent-basis={}|idempotence={}|prior-receipt={}",
        request.digest(),
        failure_class
            .map(|class| format!("{class:?}"))
            .unwrap_or_else(|| "none".to_string()),
        request.effect_intent_digest(),
        request.effect_intent_patch_canonical_basis(),
        request.idempotence_digest(),
        prior_receipt.map_or("none", TruthWritebackReceipt::digest),
    );
    let digest = Sha256::digest(canonical_basis.as_bytes());
    Arc::from(format!(
        "truth-writeback-authoritative-artifact:sha256:{digest:x}"
    ))
}
