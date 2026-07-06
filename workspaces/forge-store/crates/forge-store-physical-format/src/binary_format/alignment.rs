use crate::PhysicalBinaryFormatError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalAlignmentSite {
    PageStart,
    FrameStart,
    SlotDirectoryOffset,
    ExtentStart,
    ManifestRecord,
}

impl PhysicalAlignmentSite {
    pub const fn required_for_s1() -> [Self; 5] {
        [
            Self::PageStart,
            Self::FrameStart,
            Self::SlotDirectoryOffset,
            Self::ExtentStart,
            Self::ManifestRecord,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAlignmentClass {
    site: PhysicalAlignmentSite,
    bytes: u16,
}

impl PhysicalAlignmentClass {
    pub const fn page_start_4k() -> Self {
        Self::new(PhysicalAlignmentSite::PageStart, 4096)
    }

    pub const fn frame_start_8() -> Self {
        Self::new(PhysicalAlignmentSite::FrameStart, 8)
    }

    pub const fn slot_directory_offset_8() -> Self {
        Self::new(PhysicalAlignmentSite::SlotDirectoryOffset, 8)
    }

    pub const fn extent_start_4k() -> Self {
        Self::new(PhysicalAlignmentSite::ExtentStart, 4096)
    }

    pub const fn manifest_record_8() -> Self {
        Self::new(PhysicalAlignmentSite::ManifestRecord, 8)
    }

    pub(crate) const fn new(site: PhysicalAlignmentSite, bytes: u16) -> Self {
        Self { site, bytes }
    }

    pub(crate) fn from_bytes(
        site: PhysicalAlignmentSite,
        bytes: u16,
    ) -> Result<Self, PhysicalBinaryFormatError> {
        let alignment = Self::new(site, bytes);
        if alignment == expected_alignment_for_site(site) {
            Ok(alignment)
        } else {
            Err(PhysicalBinaryFormatError::AlignmentMismatch(site))
        }
    }

    pub const fn site(&self) -> PhysicalAlignmentSite {
        self.site
    }

    pub const fn bytes(&self) -> u16 {
        self.bytes
    }
}

pub(crate) const fn expected_alignment_for_site(
    site: PhysicalAlignmentSite,
) -> PhysicalAlignmentClass {
    match site {
        PhysicalAlignmentSite::PageStart => PhysicalAlignmentClass::page_start_4k(),
        PhysicalAlignmentSite::FrameStart => PhysicalAlignmentClass::frame_start_8(),
        PhysicalAlignmentSite::SlotDirectoryOffset => {
            PhysicalAlignmentClass::slot_directory_offset_8()
        }
        PhysicalAlignmentSite::ExtentStart => PhysicalAlignmentClass::extent_start_4k(),
        PhysicalAlignmentSite::ManifestRecord => PhysicalAlignmentClass::manifest_record_8(),
    }
}
