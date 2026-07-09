use worth_foundational::CanonicalBasisConstructionDenial;

use crate::{StoreCanonicalBasisFamily, StoreCanonicalBasisSourceDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreCanonicalBasisConstructionDenial {
    Source(StoreCanonicalBasisSourceDenial),
    MissingNativeSource { family: StoreCanonicalBasisFamily },
    ConflictingNativeSources { family: StoreCanonicalBasisFamily },
    MissingPhysicalWitness { family: StoreCanonicalBasisFamily },
    Foundational(CanonicalBasisConstructionDenial),
}

impl From<StoreCanonicalBasisSourceDenial> for StoreCanonicalBasisConstructionDenial {
    fn from(value: StoreCanonicalBasisSourceDenial) -> Self {
        Self::Source(value)
    }
}
