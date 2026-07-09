use worth_store_physical_format::PhysicalGenerationOwner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationIntegrityReport {
    SamePhysicalGeneration {
        owner: PhysicalGenerationOwner,
    },
    StalePhysicalGeneration {
        expected: PhysicalGenerationOwner,
        actual: PhysicalGenerationOwner,
    },
    MisplacedPhysicalIdentity {
        expected: PhysicalGenerationOwner,
        actual: PhysicalGenerationOwner,
    },
}

impl GenerationIntegrityReport {
    pub fn compare(expected: PhysicalGenerationOwner, actual: PhysicalGenerationOwner) -> Self {
        if expected == actual {
            return Self::SamePhysicalGeneration { owner: actual };
        }
        if same_physical_placement(expected, actual) {
            return Self::StalePhysicalGeneration { expected, actual };
        }
        Self::MisplacedPhysicalIdentity { expected, actual }
    }

    pub const fn expected_owner(self) -> PhysicalGenerationOwner {
        match self {
            Self::SamePhysicalGeneration { owner } => owner,
            Self::StalePhysicalGeneration { expected, .. }
            | Self::MisplacedPhysicalIdentity { expected, .. } => expected,
        }
    }

    pub const fn actual_owner(self) -> PhysicalGenerationOwner {
        match self {
            Self::SamePhysicalGeneration { owner } => owner,
            Self::StalePhysicalGeneration { actual, .. }
            | Self::MisplacedPhysicalIdentity { actual, .. } => actual,
        }
    }

    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::SamePhysicalGeneration { .. })
    }
}

fn same_physical_placement(
    expected: PhysicalGenerationOwner,
    actual: PhysicalGenerationOwner,
) -> bool {
    expected.domain() == actual.domain()
        && expected.segment_id() == actual.segment_id()
        && expected.page_id() == actual.page_id()
        && expected.extent_id() == actual.extent_id()
        && expected.slot() == actual.slot()
        && expected.root_reference() == actual.root_reference()
        && expected.allocation_class() == actual.allocation_class()
}
