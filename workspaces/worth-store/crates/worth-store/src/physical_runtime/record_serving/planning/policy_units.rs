#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordByteLimit(pub(in crate::physical_runtime::record_serving) u32);

impl RecordByteLimit {
    pub const fn new(bytes: u32) -> Option<Self> {
        if bytes == 0 {
            None
        } else {
            Some(Self(bytes))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentPageCount(pub(in crate::physical_runtime::record_serving) u32);

impl SegmentPageCount {
    pub const fn new(pages: u32) -> Option<Self> {
        if pages == 0 {
            None
        } else {
            Some(Self(pages))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestEntryCapacity(pub(in crate::physical_runtime::record_serving) u16);

impl ManifestEntryCapacity {
    pub const fn new(entries: u16) -> Option<Self> {
        if !manifest_capacity_can_branch(entries) {
            None
        } else {
            Some(Self(entries))
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

pub(in crate::physical_runtime::record_serving) const fn manifest_capacity_can_branch(
    entries: u16,
) -> bool {
    entries >= 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFillPercent(pub(in crate::physical_runtime::record_serving) u8);

impl PageFillPercent {
    pub const fn new(percent: u8) -> Option<Self> {
        if percent == 0 || percent > 100 {
            None
        } else {
            Some(Self(percent))
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordCountLimit(pub(in crate::physical_runtime::record_serving) u32);

impl RecordCountLimit {
    pub const fn new(records: u32) -> Option<Self> {
        if records == 0 {
            None
        } else {
            Some(Self(records))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}
