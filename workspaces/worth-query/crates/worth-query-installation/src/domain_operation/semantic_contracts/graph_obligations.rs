use super::WorthQueryOperationNativeProjectionContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationGraphReadContract {
    NotRequired,
    Declared {
        roles: Vec<WorthQueryOperationGraphReadRole>,
    },
}

impl WorthQueryOperationGraphReadContract {
    pub fn roles(&self) -> &[WorthQueryOperationGraphReadRole] {
        match self {
            Self::NotRequired => &[],
            Self::Declared { roles } => roles,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationGraphReadRole {
    pub role: String,
    pub participation: WorthQueryOperationGraphParticipation,
    pub access: WorthQueryOperationGraphAccess,
    pub semantic_reads: Vec<WorthQueryOperationNativeProjectionContract>,
}

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

impl WorthQueryOperationGraphAccess {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Project => "project",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationTouchContract {
    NotRequired,
    Declared {
        graph_roles: Vec<String>,
        scopes: Vec<String>,
    },
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
