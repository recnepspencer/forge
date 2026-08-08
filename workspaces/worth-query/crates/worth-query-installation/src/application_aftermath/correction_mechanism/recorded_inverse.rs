//! Installed recorded-inverse mechanism.

use worth_query_declaration::facade::application_aftermath::{
    DeclaredPreImageDemand, DeclaredRecordedInverse,
};

use super::super::lowering_correspondence::InstalledLoweringCorrespondence;
use super::super::postcondition::InstalledAftermathPostcondition;

/// Installed Bridge lowering correspondence — resolved typed value (R8.9).
///
/// The diagnostic slot string is retained for explanation only. Binding identity
/// is the resolved correspondence digest, generation, and graph participation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstalledLoweringCorrespondenceRef {
    resolved: InstalledLoweringCorrespondence,
}

impl InstalledLoweringCorrespondenceRef {
    pub(crate) fn from_resolved(resolved: InstalledLoweringCorrespondence) -> Self {
        Self { resolved }
    }

    pub const fn resolved(&self) -> &InstalledLoweringCorrespondence {
        &self.resolved
    }

    /// Diagnostic label only — never use as binding identity.
    pub fn correspondence_slot(&self) -> &str {
        self.resolved.correspondence_slot()
    }
}

/// Installed pre-image demand bound by the operation's declared reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledPreImageDemand {
    field_slots: Vec<String>,
    maximum_encoded_bytes: usize,
}

impl InstalledPreImageDemand {
    pub fn from_declared(declared: &DeclaredPreImageDemand) -> Self {
        Self {
            field_slots: declared.field_slots().to_vec(),
            maximum_encoded_bytes: declared.maximum_encoded_bytes(),
        }
    }

    pub fn field_slots(&self) -> &[String] {
        &self.field_slots
    }

    pub const fn maximum_encoded_bytes(&self) -> usize {
        self.maximum_encoded_bytes
    }
}

/// Installed recorded-inverse correction mechanism.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledRecordedInverse {
    inverse_operation_slot: String,
    lowering_correspondence: InstalledLoweringCorrespondenceRef,
    postcondition: InstalledAftermathPostcondition,
    preimage_demand: InstalledPreImageDemand,
}

impl InstalledRecordedInverse {
    pub(crate) fn from_declared(
        declared: &DeclaredRecordedInverse,
        lowering_correspondence: InstalledLoweringCorrespondence,
    ) -> Self {
        Self {
            inverse_operation_slot: declared.inverse_operation_slot().to_owned(),
            lowering_correspondence: InstalledLoweringCorrespondenceRef::from_resolved(
                lowering_correspondence,
            ),
            postcondition: InstalledAftermathPostcondition::from_declared(declared.postcondition()),
            preimage_demand: InstalledPreImageDemand::from_declared(declared.preimage_demand()),
        }
    }

    pub fn inverse_operation_slot(&self) -> &str {
        &self.inverse_operation_slot
    }

    pub const fn lowering_correspondence(&self) -> &InstalledLoweringCorrespondenceRef {
        &self.lowering_correspondence
    }

    pub const fn postcondition(&self) -> &InstalledAftermathPostcondition {
        &self.postcondition
    }

    pub const fn preimage_demand(&self) -> &InstalledPreImageDemand {
        &self.preimage_demand
    }
}
