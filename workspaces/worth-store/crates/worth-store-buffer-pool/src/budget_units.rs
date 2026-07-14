#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentMemoryBudget {
    bytes: u64,
}

impl ResidentMemoryBudget {
    pub fn bytes(bytes: u64) -> Result<Self, BudgetUnitDenial> {
        non_zero_u64(bytes, BudgetUnitDenial::ResidentMemoryBudgetIsZero)
            .map(|bytes| Self { bytes })
    }

    pub const fn as_bytes(&self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinnedPageBudget {
    pages: u32,
}

impl PinnedPageBudget {
    pub fn pages(pages: u32) -> Result<Self, BudgetUnitDenial> {
        non_zero_u32(pages, BudgetUnitDenial::PinnedPageBudgetIsZero).map(|pages| Self { pages })
    }

    pub const fn as_pages(&self) -> u32 {
        self.pages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPageBudget {
    pages: u32,
}

impl DirtyPageBudget {
    pub fn pages(pages: u32) -> Result<Self, BudgetUnitDenial> {
        non_zero_u32(pages, BudgetUnitDenial::DirtyPageBudgetIsZero).map(|pages| Self { pages })
    }

    pub const fn as_pages(&self) -> u32 {
        self.pages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentByteCount {
    bytes: u64,
}

impl ResidentByteCount {
    pub const fn from_observed_bytes(bytes: u64) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(&self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinnedPageCount {
    pages: u32,
}

impl PinnedPageCount {
    pub const fn from_observed_pages(pages: u32) -> Self {
        Self { pages }
    }

    pub const fn as_pages(&self) -> u32 {
        self.pages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPageCount {
    pages: u32,
}

impl DirtyPageCount {
    pub const fn from_observed_pages(pages: u32) -> Self {
        Self { pages }
    }

    pub const fn as_pages(&self) -> u32 {
        self.pages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyByteCount {
    bytes: u64,
}

impl DirtyByteCount {
    pub const fn from_observed_bytes(bytes: u64) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(&self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopiedByteCount {
    bytes: u64,
}

impl CopiedByteCount {
    pub const fn from_observed_bytes(bytes: u64) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(&self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterializedByteCount {
    bytes: u64,
}

impl MaterializedByteCount {
    pub const fn from_observed_bytes(bytes: u64) -> Self {
        Self { bytes }
    }

    pub const fn as_bytes(&self) -> u64 {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetUnitDenial {
    ResidentMemoryBudgetIsZero,
    PinnedPageBudgetIsZero,
    DirtyPageBudgetIsZero,
}

fn non_zero_u64(value: u64, denial: BudgetUnitDenial) -> Result<u64, BudgetUnitDenial> {
    if value == 0 {
        Err(denial)
    } else {
        Ok(value)
    }
}

fn non_zero_u32(value: u32, denial: BudgetUnitDenial) -> Result<u32, BudgetUnitDenial> {
    if value == 0 {
        Err(denial)
    } else {
        Ok(value)
    }
}
