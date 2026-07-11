use super::{
    bypass::LegacyAccessPathBypass, disposition::LegacySurfaceDisposition,
    surface_row::LegacySurfaceInventoryRow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyAccessPathBypassInventory {
    rows: &'static [LegacySurfaceInventoryRow],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacySurfaceDispositionOutcome {
    disposition: LegacySurfaceDisposition,
}

impl LegacySurfaceDispositionOutcome {
    pub const fn disposition(self) -> LegacySurfaceDisposition {
        self.disposition
    }
}

impl PartialEq<LegacySurfaceDisposition> for LegacySurfaceDispositionOutcome {
    fn eq(&self, other: &LegacySurfaceDisposition) -> bool {
        self.disposition == *other
    }
}

impl LegacyAccessPathBypassInventory {
    pub const fn new(rows: &'static [LegacySurfaceInventoryRow]) -> Self {
        Self { rows }
    }

    pub const fn rows(self) -> &'static [LegacySurfaceInventoryRow] {
        self.rows
    }

    pub fn disposition_for(self, surface: &str) -> LegacySurfaceDispositionOutcome {
        let disposition = self
            .rows
            .iter()
            .find(|row| row.surface() == surface)
            .unwrap_or_else(|| panic!("missing legacy surface disposition for {surface}"))
            .disposition();
        LegacySurfaceDispositionOutcome { disposition }
    }

    pub fn bypass_for(self, surface: &str) -> LegacyAccessPathBypass {
        self.rows
            .iter()
            .find(|row| row.surface() == surface)
            .unwrap_or_else(|| panic!("missing legacy surface bypass posture for {surface}"))
            .bypass()
    }
}

pub(crate) use super::rows::legacy_surface_rows;
