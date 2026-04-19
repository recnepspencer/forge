use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, WritebackMappedFamilyInputIdentityTag, WritebackMapperEnvelopeIdentityTag,
    WritebackMapperRecordIdentityTag, WritebackMapperWitnessIdentityTag,
};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect,
    BridgeValidatedWritebackCandidate, BridgeWritebackCausalityBasis,
};

pub type BridgeMappedWritebackFamilyInputIdentity =
    BridgeIdentity<WritebackMappedFamilyInputIdentityTag>;
pub type BridgeWritebackMapperEnvelopeIdentity = BridgeIdentity<WritebackMapperEnvelopeIdentityTag>;
pub type BridgeWritebackMapperWitnessIdentity = BridgeIdentity<WritebackMapperWitnessIdentityTag>;
pub type BridgeWritebackMapperRecordIdentity = BridgeIdentity<WritebackMapperRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperEnvelope {
    envelope_identity: BridgeWritebackMapperEnvelopeIdentity,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    causality_digest: Arc<str>,
    domain_payload_digest: Arc<str>,
    domain_evidence_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackMapperEnvelope {
    pub(crate) fn new(
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackCausalityBasis,
        domain_payload_digest: impl Into<Arc<str>>,
        domain_evidence_digest: impl Into<Arc<str>>,
    ) -> Self {
        let family_basis = contract
            .validated_declaration()
            .family_basis()
            .expect("admitted writeback contract must preserve family basis");
        let strategy_basis = contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract must preserve strategy basis");
        let declaration = contract.validated_declaration().declaration();
        let domain_payload_digest = domain_payload_digest.into();
        let domain_evidence_digest = domain_evidence_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-envelope|contract={}|family:{:?}|family-basis={}|effect-class:{:?}|strategy-class:{:?}|strategy-basis={}|strategy={}|causality={}|domain-payload={}|domain-evidence={}",
            contract.digest(),
            family_basis.family_kind(),
            family_basis.digest(),
            declaration.effect_class(),
            strategy_basis.strategy_class(),
            strategy_basis.digest(),
            strategy_basis.strategy_descriptor_digest(),
            causality.digest(),
            domain_payload_digest.as_ref(),
            domain_evidence_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            envelope_identity: BridgeWritebackMapperEnvelopeIdentity::new(format!(
                "bridge-writeback-mapper-envelope:sha256:{digest:x}"
            )),
            contract_digest: Arc::from(contract.digest().to_owned()),
            family_kind: family_basis.family_kind(),
            effect_class: declaration.effect_class(),
            strategy_class: strategy_basis.strategy_class(),
            strategy_descriptor_digest: Arc::from(
                strategy_basis.strategy_descriptor_digest().to_owned(),
            ),
            causality_digest: Arc::from(causality.digest().to_owned()),
            domain_payload_digest,
            domain_evidence_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-mapper-envelope:sha256:{digest:x}"
            )),
        }
    }

    pub fn envelope_identity(&self) -> &BridgeWritebackMapperEnvelopeIdentity {
        &self.envelope_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn domain_payload_digest(&self) -> &str {
        self.domain_payload_digest.as_ref()
    }

    pub fn domain_evidence_digest(&self) -> &str {
        self.domain_evidence_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappedWritebackFamilyInput {
    mapped_input_identity: BridgeMappedWritebackFamilyInputIdentity,
    mapper_envelope_digest: Arc<str>,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    causality_digest: Arc<str>,
    domain_payload_digest: Arc<str>,
    domain_evidence_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMappedWritebackFamilyInput {
    pub(crate) fn from_mapper_envelope(envelope: &BridgeWritebackMapperEnvelope) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-mapped-writeback-family-input|mapper-envelope={}|contract={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|domain-payload={}|domain-evidence={}",
            envelope.digest(),
            envelope.contract_digest(),
            envelope.family_kind(),
            envelope.effect_class(),
            envelope.strategy_class(),
            envelope.strategy_descriptor_digest(),
            envelope.causality_digest(),
            envelope.domain_payload_digest(),
            envelope.domain_evidence_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            mapped_input_identity: BridgeMappedWritebackFamilyInputIdentity::new(format!(
                "bridge-mapped-writeback-family-input:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(envelope.digest().to_owned()),
            contract_digest: Arc::from(envelope.contract_digest().to_owned()),
            family_kind: envelope.family_kind(),
            effect_class: envelope.effect_class(),
            strategy_class: envelope.strategy_class(),
            strategy_descriptor_digest: Arc::from(envelope.strategy_descriptor_digest().to_owned()),
            causality_digest: Arc::from(envelope.causality_digest().to_owned()),
            domain_payload_digest: Arc::from(envelope.domain_payload_digest().to_owned()),
            domain_evidence_digest: Arc::from(envelope.domain_evidence_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-mapped-writeback-family-input:sha256:{digest:x}"
            )),
        }
    }

    pub fn mapped_input_identity(&self) -> &BridgeMappedWritebackFamilyInputIdentity {
        &self.mapped_input_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        self.mapper_envelope_digest.as_ref()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn domain_payload_digest(&self) -> &str {
        self.domain_payload_digest.as_ref()
    }

    pub fn domain_evidence_digest(&self) -> &str {
        self.domain_evidence_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperWitness {
    witness_identity: BridgeWritebackMapperWitnessIdentity,
    mapper_envelope_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    causality_digest: Arc<str>,
    proposed_effect_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackMapperWitness {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn issue(mapped_input: &BridgeMappedWritebackFamilyInput) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-witness|mapper-envelope={}|mapped-input={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|proposed-effect={}",
            mapped_input.mapper_envelope_digest(),
            mapped_input.digest(),
            mapped_input.family_kind(),
            mapped_input.effect_class(),
            mapped_input.strategy_class(),
            mapped_input.strategy_descriptor_digest(),
            mapped_input.causality_digest(),
            mapped_input.domain_payload_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            witness_identity: BridgeWritebackMapperWitnessIdentity::new(format!(
                "bridge-writeback-mapper-witness:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(mapped_input.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(mapped_input.digest().to_owned()),
            family_kind: mapped_input.family_kind(),
            effect_class: mapped_input.effect_class(),
            strategy_class: mapped_input.strategy_class(),
            strategy_descriptor_digest: Arc::from(
                mapped_input.strategy_descriptor_digest().to_owned(),
            ),
            causality_digest: Arc::from(mapped_input.causality_digest().to_owned()),
            proposed_effect_digest: Arc::from(mapped_input.domain_payload_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-mapper-witness:sha256:{digest:x}")),
        }
    }

    pub(crate) fn issue_from_effect(effect: &BridgeDerivedWritebackEffect) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-witness|mapper-envelope={}|mapped-input={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|proposed-effect={}",
            effect.mapper_envelope_digest(),
            effect.mapped_input_digest(),
            effect.family_kind(),
            effect.effect_class(),
            effect.strategy_class(),
            effect.strategy_descriptor_digest(),
            effect.causality_digest(),
            effect.effect_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            witness_identity: BridgeWritebackMapperWitnessIdentity::new(format!(
                "bridge-writeback-mapper-witness:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(effect.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(effect.mapped_input_digest().to_owned()),
            family_kind: effect.family_kind(),
            effect_class: effect.effect_class(),
            strategy_class: effect.strategy_class(),
            strategy_descriptor_digest: Arc::from(effect.strategy_descriptor_digest().to_owned()),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            proposed_effect_digest: Arc::from(effect.effect_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-mapper-witness:sha256:{digest:x}")),
        }
    }

    pub fn witness_identity(&self) -> &BridgeWritebackMapperWitnessIdentity {
        &self.witness_identity
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        self.mapper_envelope_digest.as_ref()
    }

    pub fn mapped_input_digest(&self) -> &str {
        self.mapped_input_digest.as_ref()
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn proposed_effect_digest(&self) -> &str {
        self.proposed_effect_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperRecord {
    record_identity: BridgeWritebackMapperRecordIdentity,
    mapper_envelope_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    witness_digest: Arc<str>,
    candidate_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    causality_digest: Arc<str>,
    proposed_effect_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackMapperRecord {
    pub fn new(
        witness: &BridgeWritebackMapperWitness,
        candidate: &BridgeValidatedWritebackCandidate,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-record|mapper-envelope={}|mapped-input={}|witness={}|candidate={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|proposed-effect={}|retry:{:?}",
            witness.mapper_envelope_digest(),
            witness.mapped_input_digest(),
            witness.digest(),
            candidate.digest(),
            witness.family_kind(),
            witness.effect_class(),
            witness.strategy_class(),
            witness.strategy_descriptor_digest(),
            witness.causality_digest(),
            witness.proposed_effect_digest(),
            candidate.retry_disposition(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgeWritebackMapperRecordIdentity::new(format!(
                "bridge-writeback-mapper-record:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(witness.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(witness.mapped_input_digest().to_owned()),
            witness_digest: Arc::from(witness.digest().to_owned()),
            candidate_digest: Arc::from(candidate.digest().to_owned()),
            family_kind: witness.family_kind(),
            effect_class: witness.effect_class(),
            strategy_class: witness.strategy_class(),
            strategy_descriptor_digest: Arc::from(witness.strategy_descriptor_digest().to_owned()),
            causality_digest: Arc::from(witness.causality_digest().to_owned()),
            proposed_effect_digest: Arc::from(witness.proposed_effect_digest().to_owned()),
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

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn proposed_effect_digest(&self) -> &str {
        self.proposed_effect_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
