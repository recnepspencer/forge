use core::fmt;

use super::{CapabilitySupportId, CapabilitySupportKind};

/// Structured support failure for a capability that did not meet a requirement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilitySupportRejection<T: CapabilitySupportId> {
    id: T,
    required: CapabilitySupportKind,
    actual: CapabilitySupportKind,
}

impl<T: CapabilitySupportId> CapabilitySupportRejection<T> {
    pub(crate) fn new(
        id: T,
        required: CapabilitySupportKind,
        actual: CapabilitySupportKind,
    ) -> Self {
        Self {
            id,
            required,
            actual,
        }
    }

    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn required(&self) -> CapabilitySupportKind {
        self.required
    }

    pub fn actual(&self) -> CapabilitySupportKind {
        self.actual
    }
}

impl<T: CapabilitySupportId + fmt::Display> fmt::Display for CapabilitySupportRejection<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "capability '{}' required {:?} support but had {:?} support",
            self.id, self.required, self.actual
        )
    }
}

impl<T: CapabilitySupportId + fmt::Debug + fmt::Display> std::error::Error
    for CapabilitySupportRejection<T>
{
}
