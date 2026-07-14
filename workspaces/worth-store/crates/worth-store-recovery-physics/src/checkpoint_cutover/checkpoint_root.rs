use worth_store_physical_format::PhysicalReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointRootPosture {
    RootPresent(PhysicalReference),
    MissingRoot,
    StaleRoot(PhysicalReference),
}

impl CheckpointRootPosture {
    pub const fn root_present(reference: PhysicalReference) -> Self {
        Self::RootPresent(reference)
    }

    pub const fn root_reference(self) -> Option<PhysicalReference> {
        match self {
            Self::RootPresent(reference) => Some(reference),
            Self::MissingRoot | Self::StaleRoot(_) => None,
        }
    }
}
