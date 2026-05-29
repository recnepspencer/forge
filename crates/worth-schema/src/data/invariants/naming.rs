use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NamingInvariantGroup {
    PersistentNameStability,
    PersistentNameUniqueness,
}

impl NamingInvariantGroup {
    pub const ALL: [Self; 2] = [
        Self::PersistentNameStability,
        Self::PersistentNameUniqueness,
    ];
}
