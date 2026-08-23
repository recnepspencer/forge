#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationGraphParticipation {
    PrimaryLogicalGraph,
    SeparateAuthority { role: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationGraphAccess {
    Observe,
    Project,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationEffectContract {
    NotRequired,
    Declared {
        effect_families: Vec<WorthQueryOperationEffectFamily>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryOperationEffectFamily {
    Mutation,
    Merge,
    Writeback,
}

impl WorthQueryOperationEffectFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Merge => "merge",
            Self::Writeback => "writeback",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationInvariantContract {
    NotRequired,
    Declared { invariant_slots: Vec<String> },
}
