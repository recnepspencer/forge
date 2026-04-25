use serde::{Deserialize, Serialize};

use super::lifecycle::ResourceLifecycleTransition;
use super::request::{ResourceRequestHandle, ResourceSupersessionOrdinal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSupersessionRecord {
    supersession_ordinal: ResourceSupersessionOrdinal,
    previous: ResourceRequestHandle,
    replacing: ResourceRequestHandle,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl ResourceSupersessionRecord {
    pub(crate) fn new(
        supersession_ordinal: ResourceSupersessionOrdinal,
        previous: ResourceRequestHandle,
        replacing: ResourceRequestHandle,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            supersession_ordinal,
            previous,
            replacing,
            lifecycle_transition,
        }
    }

    pub fn supersession_ordinal(self) -> ResourceSupersessionOrdinal {
        self.supersession_ordinal
    }

    pub fn previous(self) -> ResourceRequestHandle {
        self.previous
    }

    pub fn replacing(self) -> ResourceRequestHandle {
        self.replacing
    }

    pub fn lifecycle_transition(self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}
