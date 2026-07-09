#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResidentFrameSlot(u32);

impl ResidentFrameSlot {
    pub(crate) const fn from_index(index: u32) -> Self {
        Self(index)
    }

    pub const fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResidentFrameGeneration(u64);

impl ResidentFrameGeneration {
    pub(crate) const fn initial() -> Self {
        Self(1)
    }

    pub(crate) fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameIdentity {
    slot: ResidentFrameSlot,
    generation: ResidentFrameGeneration,
}

impl ResidentFrameIdentity {
    pub(crate) const fn new(slot: ResidentFrameSlot, generation: ResidentFrameGeneration) -> Self {
        Self { slot, generation }
    }

    pub const fn slot(self) -> ResidentFrameSlot {
        self.slot
    }

    pub const fn generation(self) -> ResidentFrameGeneration {
        self.generation
    }

    pub const fn token(self) -> ResidentFrameToken {
        ResidentFrameToken::new(self.slot, self.generation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameToken {
    slot: ResidentFrameSlot,
    generation: ResidentFrameGeneration,
}

impl ResidentFrameToken {
    pub(crate) const fn new(slot: ResidentFrameSlot, generation: ResidentFrameGeneration) -> Self {
        Self { slot, generation }
    }

    pub const fn slot(self) -> ResidentFrameSlot {
        self.slot
    }

    pub const fn resident_generation(self) -> ResidentFrameGeneration {
        self.generation
    }
}
