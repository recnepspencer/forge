#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedSemantics {
    LocalNeighborhood,
}

impl WorthQueryGroupedSemantics {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalNeighborhood => "local_neighborhood",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedOrdering {
    Declared,
}

impl WorthQueryGroupedOrdering {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Declared => "declared",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedAtomicity {
    Atomic,
    MemberIndependent,
}

impl WorthQueryGroupedAtomicity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Atomic => "atomic",
            Self::MemberIndependent => "member_independent",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedIntent {
    Exploratory,
    Authoritative,
}

impl WorthQueryGroupedIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exploratory => "exploratory",
            Self::Authoritative => "authoritative",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedContinuityAssumption {
    None,
    PreserveNeighborhood,
}

impl WorthQueryGroupedContinuityAssumption {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PreserveNeighborhood => "preserve_neighborhood",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGroupedSharedPostureClaim {
    SharedSelectionFocus,
    SharedMaterialPreview,
    SharedContinuity,
}

impl WorthQueryGroupedSharedPostureClaim {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SharedSelectionFocus => "shared_selection_focus",
            Self::SharedMaterialPreview => "shared_material_preview",
            Self::SharedContinuity => "shared_continuity",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGroupedMemberRole {
    Seed,
    Member,
}

impl WorthQueryGroupedMemberRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::Member => "member",
        }
    }
}
