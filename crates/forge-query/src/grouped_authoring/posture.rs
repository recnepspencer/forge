#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedSemantics {
    LocalNeighborhood,
}

impl ForgeQueryGroupedSemantics {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalNeighborhood => "local_neighborhood",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedOrdering {
    Declared,
}

impl ForgeQueryGroupedOrdering {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declared => "declared",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedAtomicity {
    Atomic,
    MemberIndependent,
}

impl ForgeQueryGroupedAtomicity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Atomic => "atomic",
            Self::MemberIndependent => "member_independent",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedIntent {
    Exploratory,
    Authoritative,
}

impl ForgeQueryGroupedIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exploratory => "exploratory",
            Self::Authoritative => "authoritative",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedContinuityAssumption {
    None,
    PreserveNeighborhood,
}

impl ForgeQueryGroupedContinuityAssumption {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PreserveNeighborhood => "preserve_neighborhood",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ForgeQueryGroupedSharedPostureClaim {
    SharedSelectionFocus,
    SharedMaterialPreview,
    SharedContinuity,
}

impl ForgeQueryGroupedSharedPostureClaim {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SharedSelectionFocus => "shared_selection_focus",
            Self::SharedMaterialPreview => "shared_material_preview",
            Self::SharedContinuity => "shared_continuity",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedMemberRole {
    Seed,
    Member,
}

impl ForgeQueryGroupedMemberRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::Member => "member",
        }
    }
}
