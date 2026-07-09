use worth_store_physical_format::PhysicalRootReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointRootPosture {
    RootPresent(PhysicalRootReference),
    MissingRoot,
    StaleRoot(PhysicalRootReference),
}

impl CheckpointRootPosture {
    pub const fn root_present(reference: PhysicalRootReference) -> Self {
        Self::RootPresent(reference)
    }

    pub const fn root_reference(self) -> Option<PhysicalRootReference> {
        match self {
            Self::RootPresent(reference) => Some(reference),
            Self::MissingRoot | Self::StaleRoot(_) => None,
        }
    }
}
