//! Public-read recorded-inverse meaning after schema association.

use super::super::postcondition::DeclaredAftermathPostcondition;
use super::recorded_inverse::DeclaredLoweringCorrespondenceRef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PortablePreImageLocus {
    pub(super) entity: String,
    pub(super) aspect: String,
    pub(super) field: String,
}

impl PortablePreImageLocus {
    pub fn entity(&self) -> &str {
        &self.entity
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PortablePreImageDemand {
    pub(super) loci: Vec<PortablePreImageLocus>,
    pub(super) maximum_encoded_bytes: usize,
}

impl PortablePreImageDemand {
    pub fn loci(&self) -> &[PortablePreImageLocus] {
        &self.loci
    }

    pub const fn maximum_encoded_bytes(&self) -> usize {
        self.maximum_encoded_bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PortableRecordedInverse {
    pub(super) inverse_operation_slot: String,
    pub(super) lowering_correspondence: DeclaredLoweringCorrespondenceRef,
    pub(super) postcondition: DeclaredAftermathPostcondition,
    pub(super) preimage_demand: PortablePreImageDemand,
}

impl PortableRecordedInverse {
    pub fn inverse_operation_slot(&self) -> &str {
        &self.inverse_operation_slot
    }

    pub const fn lowering_correspondence(&self) -> &DeclaredLoweringCorrespondenceRef {
        &self.lowering_correspondence
    }

    pub const fn postcondition(&self) -> &DeclaredAftermathPostcondition {
        &self.postcondition
    }

    pub const fn preimage_demand(&self) -> &PortablePreImageDemand {
        &self.preimage_demand
    }
}
