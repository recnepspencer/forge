mod basis;
mod comparison;
mod digest;
mod export;
mod readiness;

pub use basis::{CanonicalBasisFrontDoor, CanonicalBasisVersionStep};
pub use comparison::{
    CanonicalComparisonBasisStep, CanonicalComparisonFrontDoor, CanonicalComparisonRightStep,
};
pub use digest::CanonicalDigestFrontDoor;
pub use export::{
    CanonicalExportBasisStep, CanonicalExportFrontDoor, CanonicalExportNameStep,
    CanonicalExportShapeStep,
};
pub use readiness::CanonicalReadinessFrontDoor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalizationFrontDoor;

pub const fn canonicalization() -> CanonicalizationFrontDoor {
    CanonicalizationFrontDoor
}

impl CanonicalizationFrontDoor {
    pub const fn basis(self) -> CanonicalBasisFrontDoor {
        CanonicalBasisFrontDoor
    }

    pub const fn compare(self) -> CanonicalComparisonFrontDoor {
        CanonicalComparisonFrontDoor
    }

    pub const fn export(self) -> CanonicalExportFrontDoor {
        CanonicalExportFrontDoor
    }

    pub const fn digest(self) -> CanonicalDigestFrontDoor {
        CanonicalDigestFrontDoor
    }

    pub const fn readiness(self) -> CanonicalReadinessFrontDoor {
        CanonicalReadinessFrontDoor
    }
}
