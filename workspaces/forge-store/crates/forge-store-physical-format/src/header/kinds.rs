#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHeaderKind {
    Page(PhysicalPageKind),
    Frame(PhysicalFrameKind),
}

impl PhysicalHeaderKind {
    pub const fn tag(self) -> u8 {
        match self {
            Self::Page(kind) => kind.tag(),
            Self::Frame(kind) => kind.tag(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPageKind {
    DataPage,
    ManifestPage,
}

impl PhysicalPageKind {
    pub const fn tag(self) -> u8 {
        match self {
            Self::DataPage => 0x10,
            Self::ManifestPage => 0x11,
        }
    }

    pub const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0x10 => Some(Self::DataPage),
            0x11 => Some(Self::ManifestPage),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFrameKind {
    RecordFrame,
    ExtentRecordFrame,
}

impl PhysicalFrameKind {
    pub const fn tag(self) -> u8 {
        match self {
            Self::RecordFrame => 0x20,
            Self::ExtentRecordFrame => 0x21,
        }
    }

    pub const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0x20 => Some(Self::RecordFrame),
            0x21 => Some(Self::ExtentRecordFrame),
            _ => None,
        }
    }
}
