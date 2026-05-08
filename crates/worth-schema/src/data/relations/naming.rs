use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NamingRelationKind {
    PersistentNameTargetsEntity,
}

impl NamingRelationKind {
    pub const WRAPPED_ALL: [super::RelationKind; 1] = [super::RelationKind::Naming(
        Self::PersistentNameTargetsEntity,
    )];

    pub const ALL: [Self; 1] = [Self::PersistentNameTargetsEntity];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::PersistentNameTargetsEntity => KindId(301),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::PersistentNameTargetsEntity => ".persistent_name_targets_entity",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(301) => Self::PersistentNameTargetsEntity,
            _ => return None,
        })
    }
}
