use std::num::NonZeroU64;

use worth_store_physical_integrity::PhysicalIntegrityScrubWindow;

use crate::physical_runtime::ScrubPhysicalAllocation;

pub(super) struct LazyIntegrityScrubWindows<'media> {
    windows: Box<dyn Iterator<Item = PhysicalIntegrityScrubWindow<'media>> + 'media>,
}

pub(in crate::physical_runtime) struct ManagedPhysicalIntegrityScrubRequest<'runtime, 'media> {
    allocation: ScrubPhysicalAllocation<'runtime>,
    windows: LazyIntegrityScrubWindows<'media>,
    yield_after_windows: Option<NonZeroU64>,
}

impl<'runtime, 'media> ManagedPhysicalIntegrityScrubRequest<'runtime, 'media> {
    pub(in crate::physical_runtime) fn new<I>(
        allocation: ScrubPhysicalAllocation<'runtime>,
        windows: I,
    ) -> Self
    where
        I: Iterator<Item = PhysicalIntegrityScrubWindow<'media>> + 'media,
    {
        Self {
            allocation,
            windows: LazyIntegrityScrubWindows::new(windows),
            yield_after_windows: None,
        }
    }

    pub(in crate::physical_runtime) fn with_yield_after_windows(
        mut self,
        windows: NonZeroU64,
    ) -> Self {
        self.yield_after_windows = Some(windows);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ScrubPhysicalAllocation<'runtime>,
        LazyIntegrityScrubWindows<'media>,
        Option<NonZeroU64>,
    ) {
        (self.allocation, self.windows, self.yield_after_windows)
    }
}

impl<'media> LazyIntegrityScrubWindows<'media> {
    fn new<I>(windows: I) -> Self
    where
        I: Iterator<Item = PhysicalIntegrityScrubWindow<'media>> + 'media,
    {
        Self {
            windows: Box::new(windows),
        }
    }

    pub(super) fn next(&mut self) -> Option<PhysicalIntegrityScrubWindow<'media>> {
        self.windows.next()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::LazyIntegrityScrubWindows;

    #[test]
    fn wrapping_a_source_does_not_pull_or_precollect_windows() {
        let pulls = Cell::new(0_u64);
        let source = std::iter::from_fn(|| {
            pulls.set(pulls.get() + 1);
            None
        });

        let mut windows = LazyIntegrityScrubWindows::new(source);
        assert_eq!(pulls.get(), 0);
        assert!(windows.next().is_none());
        assert_eq!(pulls.get(), 1);
    }
}
