use forge_relational::facade::identity::KindId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthNamingEntityKind {
    PersistentName,
}

impl WorthNamingEntityKind {
    pub const WRAPPED_ALL: [super::WorthEntityKind; 1] =
        [super::WorthEntityKind::Naming(Self::PersistentName)];

    pub const ALL: [Self; 1] = [Self::PersistentName];

    pub const fn kind_id(self) -> KindId {
        match self {
            Self::PersistentName => KindId(201),
        }
    }

    pub const fn kind_name(self) -> &'static str {
        match self {
            Self::PersistentName => "worth.persistent_name",
        }
    }

    pub fn from_kind_id(kind_id: KindId) -> Option<Self> {
        Some(match kind_id {
            KindId(201) => Self::PersistentName,
            _ => return None,
        })
    }
}
