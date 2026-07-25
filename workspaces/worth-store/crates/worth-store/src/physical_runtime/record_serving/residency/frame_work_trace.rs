use crate::physical_runtime::PhysicalWorkIdentity;

/// Ordered causal summary of physical work admitted for one frame operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::physical_runtime::record_serving) struct FrameWorkTrace {
    count: u64,
    first: Option<PhysicalWorkIdentity>,
    last: Option<PhysicalWorkIdentity>,
}

impl FrameWorkTrace {
    pub(super) const fn none() -> Self {
        Self {
            count: 0,
            first: None,
            last: None,
        }
    }

    pub(super) const fn one(identity: Option<PhysicalWorkIdentity>) -> Self {
        match identity {
            Some(identity) => Self {
                count: 1,
                first: Some(identity),
                last: Some(identity),
            },
            None => Self::none(),
        }
    }

    pub(super) const fn then(self, next: Self) -> Self {
        Self {
            count: self.count.saturating_add(next.count),
            first: match self.first {
                Some(first) => Some(first),
                None => next.first,
            },
            last: match next.last {
                Some(last) => Some(last),
                None => self.last,
            },
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn count(self) -> u64 {
        self.count
    }

    pub(in crate::physical_runtime::record_serving) const fn first(
        self,
    ) -> Option<PhysicalWorkIdentity> {
        self.first
    }

    pub(in crate::physical_runtime::record_serving) const fn last(
        self,
    ) -> Option<PhysicalWorkIdentity> {
        self.last
    }
}
