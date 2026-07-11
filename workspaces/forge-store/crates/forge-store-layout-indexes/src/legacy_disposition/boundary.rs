use super::{
    inventory::{legacy_surface_rows, LegacyAccessPathBypassInventory},
    LegacySurfaceDisposition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacySurfaceDispositionAndDedicatedWorkspaceBoundary {
    inventory: LegacyAccessPathBypassInventory,
}

impl LegacySurfaceDispositionAndDedicatedWorkspaceBoundary {
    pub fn current() -> Self {
        Self {
            inventory: LegacyAccessPathBypassInventory::new(legacy_surface_rows()),
        }
    }

    pub const fn inventory(self) -> LegacyAccessPathBypassInventory {
        self.inventory
    }

    pub const fn dedicated_workspace_crate(self) -> &'static str {
        "forge-store"
    }

    pub const fn dedicated_workspace_facade(self) -> &'static str {
        "forge_store::layout_boundary"
    }

    pub const fn legacy_topology_is_precedent(self) -> bool {
        false
    }

    pub fn forbids_legacy_authority(self, surface: &str) -> bool {
        matches!(
            self.inventory().disposition_for(surface).disposition(),
            LegacySurfaceDisposition::ForbiddenAsAuthority
                | LegacySurfaceDisposition::SupersededAndForbidden
                | LegacySurfaceDisposition::CertificationOnly
                | LegacySurfaceDisposition::TerminalOnly
        )
    }
}
