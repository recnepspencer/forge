pub(crate) use crate::catalog::{
    ArtifactFamilyInventoryRow, PhysicalArtifactFamilyDeclaration, S8ArtifactFamilyInventory,
};

pub(crate) mod inventory_rows;
#[cfg(test)]
pub(crate) mod inventory_rows_tests;

pub(crate) fn artifact_family_inventory_rows() -> &'static [ArtifactFamilyInventoryRow] {
    inventory_rows::rows()
}
