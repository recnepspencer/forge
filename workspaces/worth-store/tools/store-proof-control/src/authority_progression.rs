use crate::classification::ClassifiedInventory;
use crate::discovery::TestSurfaceInventory;

#[derive(Debug)]
pub struct DiscoveredTestSurface(TestSurfaceInventory);

impl DiscoveredTestSurface {
    pub(crate) fn from_repository(inventory: TestSurfaceInventory) -> Self {
        Self(inventory)
    }

    pub const fn inventory(&self) -> &TestSurfaceInventory {
        &self.0
    }

    pub(crate) fn into_inventory(self) -> TestSurfaceInventory {
        self.0
    }
}

#[derive(Debug)]
pub struct ClassifiedProofInventory(ClassifiedInventory);

impl ClassifiedProofInventory {
    pub(crate) fn from_discovered(inventory: ClassifiedInventory) -> Self {
        Self(inventory)
    }

    pub const fn inventory(&self) -> &ClassifiedInventory {
        &self.0
    }

    pub(crate) fn into_inventory(self) -> ClassifiedInventory {
        self.0
    }
}

#[derive(Debug)]
pub struct ValidatedProofInventory(ClassifiedInventory);

impl ValidatedProofInventory {
    pub(crate) fn from_classified(inventory: ClassifiedInventory) -> Self {
        Self(inventory)
    }

    pub const fn inventory(&self) -> &ClassifiedInventory {
        &self.0
    }
}
