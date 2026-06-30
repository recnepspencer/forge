use super::FrozenCapabilityFamily;
use crate::capability::{RegistryFamily, RegistryFamilyInventoryAudit};

/// Canonical summary produced by the snapshot freeze boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotFreezeReport {
    families: Vec<FrozenCapabilityFamily>,
}

impl SnapshotFreezeReport {
    pub(crate) fn new(mut families: Vec<FrozenCapabilityFamily>) -> Self {
        families.sort_by_key(FrozenCapabilityFamily::family_name);
        Self { families }
    }

    pub fn families(&self) -> &[FrozenCapabilityFamily] {
        &self.families
    }

    pub fn family_width(&self, family_name: &'static str) -> Option<usize> {
        self.families
            .binary_search_by_key(&family_name, FrozenCapabilityFamily::family_name)
            .ok()
            .map(|index| self.families[index].width())
    }

    pub fn registry_family_width(&self, registry_family: RegistryFamily) -> Option<usize> {
        self.family_width(registry_family.name())
    }

    pub fn omitted_registry_families(&self) -> Vec<RegistryFamily> {
        self.registry_family_inventory_audit()
            .omitted_families()
            .to_vec()
    }

    pub fn has_complete_registry_family_inventory(&self) -> bool {
        self.registry_family_inventory_audit().is_complete()
    }

    pub fn registry_family_inventory_audit(&self) -> RegistryFamilyInventoryAudit {
        RegistryFamilyInventoryAudit::from_reported_family_names(
            self.families
                .iter()
                .map(FrozenCapabilityFamily::family_name),
        )
    }
}
