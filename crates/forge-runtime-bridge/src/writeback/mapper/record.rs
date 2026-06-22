use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeWritebackMapperRecordIdentity, BridgeWritebackMapperWitness};
use crate::writeback::{
    BridgeValidatedWritebackCandidate, BridgeWritebackEffectClass, BridgeWritebackFamilyKind,
    BridgeWritebackStrategyClass, BridgeWritebackStrategyDescriptorBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperRecord {
    record_identity: BridgeWritebackMapperRecordIdentity,
    mapper_envelope_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    witness_digest: Arc<str>,
    candidate_digest: Arc<str>,
    family_kind: BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: BridgeWritebackStrategyClass,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    causality_digest: Arc<str>,
    effect_intent_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackMapperRecord {
    pub fn new(
        witness: &BridgeWritebackMapperWitness,
        candidate: &BridgeValidatedWritebackCandidate,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-record|mapper-envelope={}|mapped-input={}|witness={}|candidate={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|effect-intent={}|retry:{:?}",
            witness.mapper_envelope_digest(),
            witness.mapped_input_digest(),
            witness.digest(),
            candidate.digest(),
            witness.family_kind(),
            witness.effect_class(),
            witness.strategy_class(),
            witness.strategy_descriptor_basis().digest(),
            witness.causality_digest(),
            witness.effect_intent_digest(),
            candidate.retry_disposition(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgeWritebackMapperRecordIdentity::admit_bridge_owned(format!(
                "bridge-writeback-mapper-record:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(witness.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(witness.mapped_input_digest().to_owned()),
            witness_digest: Arc::from(witness.digest().to_owned()),
            candidate_digest: Arc::from(candidate.digest().to_owned()),
            family_kind: witness.family_kind(),
            effect_class: witness.effect_class(),
            strategy_class: witness.strategy_class(),
            strategy_descriptor_basis: witness.strategy_descriptor_basis().clone(),
            causality_digest: Arc::from(witness.causality_digest().to_owned()),
            effect_intent_digest: Arc::from(witness.effect_intent_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-mapper-record:sha256:{digest:x}")),
        }
    }

    pub fn record_identity(&self) -> &BridgeWritebackMapperRecordIdentity {
        &self.record_identity
    }

    pub fn witness_digest(&self) -> &str {
        self.witness_digest.as_ref()
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        self.mapper_envelope_digest.as_ref()
    }

    pub fn mapped_input_digest(&self) -> &str {
        self.mapped_input_digest.as_ref()
    }

    pub fn candidate_digest(&self) -> &str {
        self.candidate_digest.as_ref()
    }

    pub fn family_kind(&self) -> BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_basis(&self) -> &BridgeWritebackStrategyDescriptorBasis {
        &self.strategy_descriptor_basis
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
