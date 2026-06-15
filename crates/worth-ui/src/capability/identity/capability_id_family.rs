use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

use super::capability_id_error::CapabilityIdError;
use super::capability_id_text::CapabilityIdText;

pub(super) struct CapabilityId<Family> {
    text: CapabilityIdText,
    family: PhantomData<fn() -> Family>,
}

impl<Family> CapabilityId<Family> {
    pub(super) fn new(raw_text: impl AsRef<str>) -> Result<Self, CapabilityIdError> {
        Ok(Self {
            text: CapabilityIdText::new(raw_text)?,
            family: PhantomData,
        })
    }

    pub(super) fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

impl<Family> Clone for CapabilityId<Family> {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            family: PhantomData,
        }
    }
}

impl<Family> fmt::Debug for CapabilityId<Family> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CapabilityId")
            .field(&self.as_str())
            .finish()
    }
}

impl<Family> Eq for CapabilityId<Family> {}

impl<Family> Hash for CapabilityId<Family> {
    fn hash<State: Hasher>(&self, state: &mut State) {
        self.text.hash(state);
    }
}

impl<Family> PartialEq for CapabilityId<Family> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl<Family> Ord for CapabilityId<Family> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.text.cmp(&other.text)
    }
}

impl<Family> PartialOrd for CapabilityId<Family> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! define_capability_id_family {
    ($id_type:ident, $family_type:ident) => {
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $id_type {
            id: CapabilityId<$family_type>,
        }

        impl $id_type {
            pub fn new(raw_text: impl AsRef<str>) -> Result<Self, CapabilityIdError> {
                Ok(Self {
                    id: CapabilityId::new(raw_text)?,
                })
            }

            pub fn as_str(&self) -> &str {
                self.id.as_str()
            }
        }

        impl core::fmt::Debug for $id_type {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter
                    .debug_tuple(stringify!($id_type))
                    .field(&self.as_str())
                    .finish()
            }
        }

        impl core::fmt::Display for $id_type {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $id_type {
            type Err = CapabilityIdError;

            fn from_str(raw_text: &str) -> Result<Self, Self::Err> {
                Self::new(raw_text)
            }
        }

        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        struct $family_type;
    };
}

pub(super) use define_capability_id_family;
