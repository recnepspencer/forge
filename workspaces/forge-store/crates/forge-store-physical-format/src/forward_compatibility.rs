#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalFormatEvolutionPosture {
    Admission,
    Rejection,
    Preservation,
    MigrationReserved,
    DowngradeRefusal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalForwardCompatibilityPolicy {
    RejectUnknownKind,
    PreserveUnknownBytes,
    MigrationReserved,
}

impl PhysicalForwardCompatibilityPolicy {
    pub const fn reject_unknown_kind() -> Self {
        Self::RejectUnknownKind
    }

    pub const fn code(self) -> u8 {
        match self {
            Self::RejectUnknownKind => 1,
            Self::PreserveUnknownBytes => 2,
            Self::MigrationReserved => 3,
        }
    }

    pub const fn posture(self) -> PhysicalFormatEvolutionPosture {
        match self {
            Self::RejectUnknownKind => PhysicalFormatEvolutionPosture::Rejection,
            Self::PreserveUnknownBytes => PhysicalFormatEvolutionPosture::Preservation,
            Self::MigrationReserved => PhysicalFormatEvolutionPosture::MigrationReserved,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalForwardCompatibilityDeclaration {
    Known(PhysicalForwardCompatibilityPolicy),
    Unsupported,
}

impl From<PhysicalForwardCompatibilityPolicy> for PhysicalForwardCompatibilityDeclaration {
    fn from(value: PhysicalForwardCompatibilityPolicy) -> Self {
        Self::Known(value)
    }
}
