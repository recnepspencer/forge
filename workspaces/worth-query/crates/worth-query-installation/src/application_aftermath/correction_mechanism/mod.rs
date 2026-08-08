//! Installed correction-mechanism axis.

mod compensation;
mod recorded_inverse;

pub use compensation::InstalledCompensation;
pub use recorded_inverse::{
    InstalledLoweringCorrespondenceRef, InstalledPreImageDemand, InstalledRecordedInverse,
};

use worth_query_declaration::facade::application_aftermath::DeclaredCorrectionMechanism;

use super::lowering_correspondence::InstalledLoweringCorrespondence;

/// Installed correction mechanism for one aftermath contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstalledCorrectionMechanism {
    RecordedInverse(InstalledRecordedInverse),
    Compensation(InstalledCompensation),
}

impl InstalledCorrectionMechanism {
    pub(crate) fn from_declared_compensation(
        declared: &worth_query_declaration::facade::application_aftermath::DeclaredCompensation,
    ) -> Self {
        Self::Compensation(InstalledCompensation::from_declared(declared))
    }

    pub(crate) fn from_declared_recorded_inverse(
        declared: &worth_query_declaration::facade::application_aftermath::DeclaredRecordedInverse,
        lowering_correspondence: InstalledLoweringCorrespondence,
    ) -> Self {
        Self::RecordedInverse(InstalledRecordedInverse::from_declared(
            declared,
            lowering_correspondence,
        ))
    }

    pub(crate) fn from_declared(
        declared: &DeclaredCorrectionMechanism,
        lowering_correspondence: Option<InstalledLoweringCorrespondence>,
    ) -> Result<Self, &'static str> {
        match declared {
            DeclaredCorrectionMechanism::RecordedInverse(inverse) => {
                let correspondence = lowering_correspondence
                    .ok_or("recorded-inverse-requires-resolved-lowering-correspondence")?;
                Ok(Self::from_declared_recorded_inverse(
                    inverse,
                    correspondence,
                ))
            }
            DeclaredCorrectionMechanism::Compensation(compensation) => {
                Ok(Self::from_declared_compensation(compensation))
            }
        }
    }
}
