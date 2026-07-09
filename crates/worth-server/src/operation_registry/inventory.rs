use crate::{WorthServerOperationFamily, WorthServerSurfaceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationInventory {
    registered_families: Vec<WorthServerOperationFamily>,
    rows: Vec<WorthServerOperationInventoryRow>,
}

impl WorthServerOperationInventory {
    pub(crate) fn new(rows: Vec<WorthServerOperationInventoryRow>) -> Self {
        let registered_families = rows.iter().map(|row| row.family).collect();
        Self {
            registered_families,
            rows,
        }
    }

    pub fn registered_families(&self) -> &[WorthServerOperationFamily] {
        &self.registered_families
    }

    pub fn rows(&self) -> &[WorthServerOperationInventoryRow] {
        &self.rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationInventoryRow {
    family: WorthServerOperationFamily,
    enabled: bool,
    exposed_surfaces: Vec<WorthServerSurfaceFamily>,
}

impl WorthServerOperationInventoryRow {
    pub(crate) fn new(
        family: WorthServerOperationFamily,
        enabled: bool,
        exposed_surfaces: Vec<WorthServerSurfaceFamily>,
    ) -> Self {
        Self {
            family,
            enabled,
            exposed_surfaces,
        }
    }

    pub fn family(&self) -> WorthServerOperationFamily {
        self.family
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn exposed_surfaces(&self) -> &[WorthServerSurfaceFamily] {
        &self.exposed_surfaces
    }
}
