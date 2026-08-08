//! Declared recorded-inverse correction mechanism.

use super::super::postcondition::DeclaredAftermathPostcondition;

/// Bound demand for retained pre-image bytes required by an inverse.
///
/// Field identities reuse the existing application unit slot; aftermath does
/// not mint a parallel unit, measure, amount, or currency vocabulary.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeclaredPreImageDemand {
    field_slots: Vec<String>,
    maximum_encoded_bytes: usize,
}

impl DeclaredPreImageDemand {
    pub fn new(
        field_slots: impl IntoIterator<Item = impl Into<String>>,
        maximum_encoded_bytes: usize,
    ) -> Result<Self, DeclaredPreImageDemandDenial> {
        let field_slots = field_slots
            .into_iter()
            .map(|slot| {
                let slot = slot.into();
                if slot.trim().is_empty() {
                    Err(DeclaredPreImageDemandDenial::EmptyFieldSlot)
                } else {
                    Ok(slot)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        if field_slots.is_empty() {
            return Err(DeclaredPreImageDemandDenial::EmptyDemand);
        }
        if maximum_encoded_bytes == 0 {
            return Err(DeclaredPreImageDemandDenial::ZeroByteBound);
        }
        Ok(Self {
            field_slots,
            maximum_encoded_bytes,
        })
    }

    pub fn field_slots(&self) -> &[String] {
        &self.field_slots
    }

    pub const fn maximum_encoded_bytes(&self) -> usize {
        self.maximum_encoded_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DeclaredPreImageDemandDenial {
    EmptyDemand,
    EmptyFieldSlot,
    ZeroByteBound,
}

/// Typed reference to an installed Bridge lowering correspondence.
///
/// A free string may appear only as a diagnostic label. Binding identity is the
/// sealed correspondence slot validated at installation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeclaredLoweringCorrespondenceRef {
    correspondence_slot: String,
}

impl DeclaredLoweringCorrespondenceRef {
    pub fn new(correspondence_slot: impl Into<String>) -> Result<Self, &'static str> {
        let correspondence_slot = correspondence_slot.into();
        if correspondence_slot.trim().is_empty() {
            return Err("empty-lowering-correspondence");
        }
        Ok(Self {
            correspondence_slot,
        })
    }

    pub fn correspondence_slot(&self) -> &str {
        &self.correspondence_slot
    }
}

/// Correction by restoring prior truth from recorded inverse data.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeclaredRecordedInverse {
    inverse_operation_slot: String,
    lowering_correspondence: DeclaredLoweringCorrespondenceRef,
    postcondition: DeclaredAftermathPostcondition,
    preimage_demand: DeclaredPreImageDemand,
}

impl DeclaredRecordedInverse {
    pub fn new(
        inverse_operation_slot: impl Into<String>,
        lowering_correspondence: DeclaredLoweringCorrespondenceRef,
        postcondition: DeclaredAftermathPostcondition,
        preimage_demand: DeclaredPreImageDemand,
    ) -> Result<Self, &'static str> {
        let inverse_operation_slot = inverse_operation_slot.into();
        if inverse_operation_slot.trim().is_empty() {
            return Err("empty-inverse-operation");
        }
        if !matches!(
            postcondition,
            DeclaredAftermathPostcondition::ExactPriorTruth
        ) {
            return Err("recorded-inverse-requires-exact-prior-truth");
        }
        Ok(Self {
            inverse_operation_slot,
            lowering_correspondence,
            postcondition,
            preimage_demand,
        })
    }

    pub fn inverse_operation_slot(&self) -> &str {
        &self.inverse_operation_slot
    }

    pub const fn lowering_correspondence(&self) -> &DeclaredLoweringCorrespondenceRef {
        &self.lowering_correspondence
    }

    pub const fn postcondition(&self) -> &DeclaredAftermathPostcondition {
        &self.postcondition
    }

    pub const fn preimage_demand(&self) -> &DeclaredPreImageDemand {
        &self.preimage_demand
    }
}
