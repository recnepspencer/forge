use crate::{ForgeServerOperationFamily, ForgeServerSurfaceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationInventory {
    registered_families: Vec<ForgeServerOperationFamily>,
    rows: Vec<ForgeServerOperationInventoryRow>,
}

impl ForgeServerOperationInventory {
    pub(crate) fn new(rows: Vec<ForgeServerOperationInventoryRow>) -> Self {
        let registered_families = rows.iter().map(|row| row.family).collect();
        Self {
            registered_families,
            rows,
        }
    }

    pub fn registered_families(&self) -> &[ForgeServerOperationFamily] {
        &self.registered_families
    }

    pub fn rows(&self) -> &[ForgeServerOperationInventoryRow] {
        &self.rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationInventoryRow {
    family: ForgeServerOperationFamily,
    enabled: bool,
    exposed_surfaces: Vec<ForgeServerSurfaceFamily>,
}

impl ForgeServerOperationInventoryRow {
    pub(crate) fn new(
        family: ForgeServerOperationFamily,
        enabled: bool,
        exposed_surfaces: Vec<ForgeServerSurfaceFamily>,
    ) -> Self {
        Self {
            family,
            enabled,
            exposed_surfaces,
        }
    }

    pub fn family(&self) -> ForgeServerOperationFamily {
        self.family
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn exposed_surfaces(&self) -> &[ForgeServerSurfaceFamily] {
        &self.exposed_surfaces
    }
}
