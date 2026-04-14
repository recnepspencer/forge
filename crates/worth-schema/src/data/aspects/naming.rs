use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthNamingAspect {
    PersistentName,
}

impl WorthNamingAspect {
    pub const ALL: [Self; 1] = [Self::PersistentName];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentName => "naming.persistent_name",
        }
    }
}
