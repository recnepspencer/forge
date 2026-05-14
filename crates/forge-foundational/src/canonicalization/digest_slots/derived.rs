use super::algorithm::CanonicalDigestAlgorithmMetadata;
use super::evidence::{CanonicalDigestDerivationInput, CanonicalDigestInputId};
use super::material::{canonical_digest_material, stable_fixture_digest};
use super::CanonicalDigestDerivationReadyArtifact;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDerivedDigest {
    metadata: CanonicalDigestMetadata,
    value: CanonicalDigestValue,
    debt: Vec<CanonicalDigestDebt>,
}

impl CanonicalDerivedDigest {
    fn new(
        metadata: CanonicalDigestMetadata,
        value: CanonicalDigestValue,
        debt: Vec<CanonicalDigestDebt>,
    ) -> Self {
        Self {
            metadata,
            value,
            debt,
        }
    }

    pub fn metadata(&self) -> &CanonicalDigestMetadata {
        &self.metadata
    }

    pub fn value(&self) -> &CanonicalDigestValue {
        &self.value
    }

    pub fn debt(&self) -> &[CanonicalDigestDebt] {
        &self.debt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigestMetadata {
    algorithm: CanonicalDigestAlgorithmMetadata,
    input_id: CanonicalDigestInputId,
    entry_count: u32,
}

impl CanonicalDigestMetadata {
    fn new(
        algorithm: CanonicalDigestAlgorithmMetadata,
        input_id: CanonicalDigestInputId,
        entry_count: u32,
    ) -> Self {
        Self {
            algorithm,
            input_id,
            entry_count,
        }
    }

    pub fn algorithm(&self) -> &CanonicalDigestAlgorithmMetadata {
        &self.algorithm
    }

    pub fn input_id(&self) -> &CanonicalDigestInputId {
        &self.input_id
    }

    pub const fn entry_count(&self) -> u32 {
        self.entry_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigestValue {
    bytes: [u8; 32],
}

impl CanonicalDigestValue {
    fn new(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalDigestDebt {
    ProductionCryptographicPolicyDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalDigestDerivationDenial {
    UnsupportedAlgorithm,
    RuleVersionMismatch,
    InputDomainMismatch,
    InputShapeMismatch,
}

pub fn derive_canonical_digest(
    ready: CanonicalDigestDerivationReadyArtifact,
) -> CanonicalDerivedDigest {
    let (input, _proofs, _basis) = ready.into_parts().into_parts();
    let material = canonical_digest_material(&input);
    let value = CanonicalDigestValue::new(stable_fixture_digest(material.as_bytes()));
    let metadata = CanonicalDigestMetadata::new(
        input.algorithm().clone(),
        input.evidence().input_id(),
        input.evidence().entry_count(),
    );

    CanonicalDerivedDigest::new(
        metadata,
        value,
        vec![CanonicalDigestDebt::ProductionCryptographicPolicyDeferred],
    )
}

#[allow(dead_code)]
fn _input_type_is_owned(_: &CanonicalDigestDerivationInput) {}
