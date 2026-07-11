use super::grammar::PhysicalLayoutAccessFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFormatLayoutCloseout;

impl PhysicalFormatLayoutCloseout {
    pub const fn phase19() -> Self {
        Self
    }

    pub const fn phase20() -> Self {
        Self
    }

    pub const fn ordinary_family_lanes(self) -> [PhysicalLayoutAccessFamily; 4] {
        [
            PhysicalLayoutAccessFamily::Page,
            PhysicalLayoutAccessFamily::Frame,
            PhysicalLayoutAccessFamily::Segment,
            PhysicalLayoutAccessFamily::Extent,
        ]
    }

    pub const fn discovery_family_lanes(self) -> [PhysicalLayoutAccessFamily; 5] {
        [
            PhysicalLayoutAccessFamily::RootManifest,
            PhysicalLayoutAccessFamily::ManifestIndex,
            PhysicalLayoutAccessFamily::Allocation,
            PhysicalLayoutAccessFamily::FreeSpace,
            PhysicalLayoutAccessFamily::Fragmentation,
        ]
    }
}
