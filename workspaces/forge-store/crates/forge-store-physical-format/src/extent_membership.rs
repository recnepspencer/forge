use crate::{AllocationClassKind, ExtentGenerationCell, PhysicalGenerationOwner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtentMembership {
    Missing,
    LargeRecord {
        cell: ExtentGenerationCell,
        declared_extent_length: usize,
    },
}

impl ExtentMembership {
    pub const fn missing() -> Self {
        Self::Missing
    }

    pub const fn large_record(cell: ExtentGenerationCell, declared_extent_length: usize) -> Self {
        Self::LargeRecord {
            cell,
            declared_extent_length,
        }
    }

    pub const fn allocation_class(self) -> Option<AllocationClassKind> {
        match self {
            Self::Missing => None,
            Self::LargeRecord { .. } => Some(AllocationClassKind::LargeRecordExtent),
        }
    }

    pub const fn cell(self) -> Option<ExtentGenerationCell> {
        match self {
            Self::Missing => None,
            Self::LargeRecord { cell, .. } => Some(cell),
        }
    }

    pub const fn owner(self) -> Option<PhysicalGenerationOwner> {
        match self {
            Self::Missing => None,
            Self::LargeRecord { cell, .. } => Some(cell.owner()),
        }
    }

    pub const fn declared_extent_length(self) -> Option<usize> {
        match self {
            Self::Missing => None,
            Self::LargeRecord {
                declared_extent_length,
                ..
            } => Some(declared_extent_length),
        }
    }
}
