use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

macro_rules! define_id {
    ($name:ident, $prefix:literal) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(u128);

        impl $name {
            pub const fn new(raw: u128) -> Self {
                Self(raw)
            }

            pub const fn raw(self) -> u128 {
                self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!($prefix, "-{:032x}"), self.0)
            }
        }
    };
}

define_id!(SpecNodeId, "node");
define_id!(SpecRelationId, "rel");
define_id!(NamingAnchorId, "anchor");
