#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCapability {
    Media,
    PageRecord,
    WalCheckpoint,
    Recovery,
    Maintenance,
    Layout,
    Blob,
}

impl PhysicalCapability {
    pub(crate) const FAMILY_COUNT: u64 = 7;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityAvailability {
    Absent,
}

/// Immutable status of the physical capability families installed by C.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstalledCapabilityStatus {
    _private: (),
}

impl InstalledCapabilityStatus {
    pub(crate) const fn c3() -> Self {
        Self { _private: () }
    }

    pub const fn availability(self, capability: PhysicalCapability) -> CapabilityAvailability {
        match capability {
            PhysicalCapability::Media
            | PhysicalCapability::PageRecord
            | PhysicalCapability::WalCheckpoint
            | PhysicalCapability::Recovery
            | PhysicalCapability::Maintenance
            | PhysicalCapability::Layout
            | PhysicalCapability::Blob => CapabilityAvailability::Absent,
        }
    }

    pub const fn physical_media(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::Media)
    }

    pub const fn page_records(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::PageRecord)
    }

    pub const fn wal_and_checkpoint(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::WalCheckpoint)
    }

    pub const fn recovery(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::Recovery)
    }

    pub const fn maintenance(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::Maintenance)
    }

    pub const fn layout(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::Layout)
    }

    pub const fn blobs(self) -> CapabilityAvailability {
        self.availability(PhysicalCapability::Blob)
    }
}
