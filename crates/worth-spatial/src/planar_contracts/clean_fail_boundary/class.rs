#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarCleanFailClass {
    DirtyInput,
    UnboundedOrOpen,
}

impl PlanarCleanFailClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirtyInput => "dirty-input",
            Self::UnboundedOrOpen => "unbounded-or-open",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarCleanFailAction {
    InspectWithoutRepair,
    ClassifyWithoutBoundedConversion,
}

impl PlanarCleanFailAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectWithoutRepair => "inspect-without-repair",
            Self::ClassifyWithoutBoundedConversion => "classify-without-bounded-conversion",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRepairAttempt {
    NotAttempted,
    Attempted,
}

impl PlanarRepairAttempt {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "not-attempted",
            Self::Attempted => "attempted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBoundedConversion {
    NotAttempted,
    Attempted,
}

impl PlanarBoundedConversion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAttempted => "not-attempted",
            Self::Attempted => "attempted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarCleanFailTruthEffect {
    DoesNotChangePlanarTruth,
}

impl PlanarCleanFailTruthEffect {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoesNotChangePlanarTruth => "does-not-change-planar-truth",
        }
    }
}
