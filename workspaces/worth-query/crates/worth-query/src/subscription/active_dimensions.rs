#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveRegistryLookupWidth(u64);

impl ActiveRegistryLookupWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveFanoutWidth(u64);

impl ActiveFanoutWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveAllocationScopeWidth(u64);

impl ActiveAllocationScopeWidth {
    pub fn measured(width: u64) -> Self {
        Self(width)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}
