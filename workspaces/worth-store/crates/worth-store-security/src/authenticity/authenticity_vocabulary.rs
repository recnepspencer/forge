#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAuthenticityRequirementClass {
    AuthenticatedFrame,
    AuthenticatedWalRecord,
    AuthenticatedManifest,
    AuthenticatedBlobChunk,
    AuthenticatedBackupCapsule,
    AuthenticatedRepairRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAuthenticityRequirement {
    NotRequired,
    Required(StoreAuthenticityRequirementClass),
}

impl StoreAuthenticityRequirement {
    pub const fn required(class: StoreAuthenticityRequirementClass) -> Self {
        Self::Required(class)
    }

    pub const fn not_required() -> Self {
        Self::NotRequired
    }

    pub const fn class(self) -> Option<StoreAuthenticityRequirementClass> {
        match self {
            Self::NotRequired => None,
            Self::Required(class) => Some(class),
        }
    }

    pub const fn requires_admission_before_result(self) -> bool {
        matches!(self, Self::Required(_))
    }
}
