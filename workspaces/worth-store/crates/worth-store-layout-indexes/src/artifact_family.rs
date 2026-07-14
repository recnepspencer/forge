pub(crate) use crate::catalog::{ArtifactFamilyInventoryRow, PhysicalArtifactFamilyDeclaration};

mod admission;
pub(crate) mod inventory_rows;
#[cfg(test)]
pub(crate) mod inventory_rows_tests;

pub use admission::{
    artifact_family_admission_cases, AdmittedPhysicalArtifactFamily, ArtifactFamilyAdmissionCaseId,
    ArtifactFamilyAdmissionOutcome, ArtifactFamilyAdmissionView,
};

pub(crate) fn artifact_family_inventory_rows() -> &'static [ArtifactFamilyInventoryRow] {
    inventory_rows::rows()
}
