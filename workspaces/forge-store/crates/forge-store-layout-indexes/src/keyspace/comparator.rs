use super::encoding::{encode_concrete_physical_key, CanonicalKeyBytes, CanonicalKeyEncoding};
use super::value::ConcretePhysicalKeyWitness;
use crate::catalog::ArtifactFamilyDenial;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparatorBehavior {
    CanonicalByteLexicographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComparatorLaw {
    encoding: CanonicalKeyEncoding,
    behavior: ComparatorBehavior,
}

impl ComparatorLaw {
    pub(crate) const fn new(encoding: CanonicalKeyEncoding, behavior: ComparatorBehavior) -> Self {
        Self { encoding, behavior }
    }

    pub const fn encoding(self) -> CanonicalKeyEncoding {
        self.encoding
    }

    pub const fn behavior(self) -> ComparatorBehavior {
        self.behavior
    }
}

pub(crate) const fn declare_comparator_law(encoding: CanonicalKeyEncoding) -> ComparatorLaw {
    ComparatorLaw::new(encoding, ComparatorBehavior::CanonicalByteLexicographic)
}

pub(crate) fn compare_concrete_physical_keys(
    law: ComparatorLaw,
    left: ConcretePhysicalKeyWitness,
    right: ConcretePhysicalKeyWitness,
) -> Result<Ordering, ArtifactFamilyDenial> {
    let left_bytes = encode_concrete_physical_key(law.encoding(), left)?;
    let right_bytes = encode_concrete_physical_key(law.encoding(), right)?;
    Ok(left_bytes.as_bytes().cmp(right_bytes.as_bytes()))
}

pub(crate) fn canonical_bytes_for_key(
    law: ComparatorLaw,
    key: ConcretePhysicalKeyWitness,
) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
    encode_concrete_physical_key(law.encoding(), key)
}
