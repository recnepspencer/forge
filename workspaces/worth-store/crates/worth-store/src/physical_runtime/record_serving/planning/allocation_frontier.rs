use worth_store_physical_format::{
    DurableFreeSpaceManifestHeader, PhysicalExtentId, PhysicalPageId, PhysicalSegmentId,
};

#[derive(Clone)]
pub(in crate::physical_runtime) struct RecordAllocationFrontier {
    next_segment: u64,
    next_page: u64,
    next_extent: u64,
}

impl RecordAllocationFrontier {
    pub(in crate::physical_runtime::record_serving) fn new(
        free_space: &DurableFreeSpaceManifestHeader,
    ) -> Self {
        Self {
            next_segment: free_space.next_segment(),
            next_page: free_space.next_page(),
            next_extent: free_space.next_extent(),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn allocate_segment(
        &mut self,
    ) -> Option<PhysicalSegmentId> {
        if self.next_segment == u64::MAX {
            return None;
        }
        let value = PhysicalSegmentId::from_raw(self.next_segment).ok()?;
        self.next_segment = self.next_segment.checked_add(1)?;
        Some(value)
    }

    pub(in crate::physical_runtime::record_serving) fn allocate_page(
        &mut self,
    ) -> Option<PhysicalPageId> {
        if self.next_page == u64::MAX {
            return None;
        }
        let value = PhysicalPageId::from_raw(self.next_page).ok()?;
        self.next_page = self.next_page.checked_add(1)?;
        Some(value)
    }

    pub(in crate::physical_runtime::record_serving) fn allocate_extent(
        &mut self,
    ) -> Option<PhysicalExtentId> {
        if self.next_extent == u64::MAX {
            return None;
        }
        let value = PhysicalExtentId::from_raw(self.next_extent).ok()?;
        self.next_extent = self.next_extent.checked_add(1)?;
        Some(value)
    }

    pub(in crate::physical_runtime::record_serving) const fn next_segment(&self) -> u64 {
        self.next_segment
    }
    pub(in crate::physical_runtime::record_serving) const fn next_page(&self) -> u64 {
        self.next_page
    }
    pub(in crate::physical_runtime::record_serving) const fn next_extent(&self) -> u64 {
        self.next_extent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_identity_value_is_reserved_and_never_advertised_as_allocatable() {
        let free =
            DurableFreeSpaceManifestHeader::new(1, 1, 2, 0, u64::MAX, u64::MAX, u64::MAX, 1, None)
                .unwrap();
        let mut frontier = RecordAllocationFrontier::new(&free);
        assert_eq!(frontier.allocate_segment(), None);
        assert_eq!(frontier.allocate_page(), None);
        assert_eq!(frontier.allocate_extent(), None);
    }
}
