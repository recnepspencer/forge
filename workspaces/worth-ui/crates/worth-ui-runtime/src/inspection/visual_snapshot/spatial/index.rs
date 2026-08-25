use super::interval_index::UiImmutableIntervalIndex;
use super::record::{UiHitTestRegionRecord, UiVisibleRegionRecord};

pub(crate) struct UiVisibleRegionIndex {
    identity: worth_ui_inspection::UiVisibleRegionIndexIdentity,
    index: UiImmutableIntervalIndex<UiVisibleRegionRecord>,
    supported_len: usize,
}

pub(crate) struct UiHitTestRegionIndex {
    identity: worth_ui_inspection::UiHitTestRegionIndexIdentity,
    index: UiImmutableIntervalIndex<UiHitTestRegionRecord>,
}

impl UiVisibleRegionIndex {
    pub(crate) fn build(capture_identity: u64, records: Vec<UiVisibleRegionRecord>) -> Self {
        let supported_len = records
            .iter()
            .filter(|record| record.opacity() != super::UiVisibleOpacity::Unsupported)
            .count();
        let index = UiImmutableIntervalIndex::build(records);
        Self {
            identity: worth_ui_inspection::UiVisibleRegionIndexIdentity::from_runtime_projection(
                capture_identity,
                index.structural_digest(),
            ),
            index,
            supported_len,
        }
    }

    pub(crate) const fn identity(&self) -> worth_ui_inspection::UiVisibleRegionIndexIdentity {
        self.identity
    }

    pub(crate) fn rebind_snapshot(mut self, capture_identity: u64) -> Self {
        self.identity = worth_ui_inspection::UiVisibleRegionIndexIdentity::from_runtime_projection(
            capture_identity,
            self.identity.structural_digest(),
        );
        self
    }

    pub(crate) fn len(&self) -> usize {
        self.index.len()
    }

    pub(crate) const fn supported_len(&self) -> usize {
        self.supported_len
    }

    pub(crate) fn retained_structural_bytes(&self) -> Option<usize> {
        Self::estimated_retained_structural_bytes(self.len())
    }

    pub(crate) fn estimated_retained_structural_bytes(record_count: usize) -> Option<usize> {
        super::interval_index::estimated_retained_structural_bytes::<UiVisibleRegionRecord>(
            record_count,
        )
        .and_then(|bytes| std::mem::size_of::<Self>().checked_add(bytes))
    }

    pub(crate) fn point_candidates(
        &self,
        point: worth_ui_inspection::UiClientPhysicalPixel,
        maximum_candidates: usize,
    ) -> super::interval_index::UiBoundedPointCandidates<'_, UiVisibleRegionRecord> {
        self.index.point_candidates(point, maximum_candidates)
    }

    pub(crate) fn region_candidates(
        &self,
        region: worth_ui_inspection::UiClientPhysicalRect,
        maximum_candidates: usize,
    ) -> super::interval_index::UiBoundedRegionCandidates<'_, UiVisibleRegionRecord> {
        self.index.region_candidates(region, maximum_candidates)
    }
}

impl UiHitTestRegionIndex {
    pub(crate) fn build(capture_identity: u64, records: Vec<UiHitTestRegionRecord>) -> Self {
        let index = UiImmutableIntervalIndex::build(records);
        Self {
            identity: worth_ui_inspection::UiHitTestRegionIndexIdentity::from_runtime_projection(
                capture_identity,
                index.structural_digest(),
            ),
            index,
        }
    }

    pub(crate) const fn identity(&self) -> worth_ui_inspection::UiHitTestRegionIndexIdentity {
        self.identity
    }

    pub(crate) fn rebind_snapshot(mut self, capture_identity: u64) -> Self {
        self.identity = worth_ui_inspection::UiHitTestRegionIndexIdentity::from_runtime_projection(
            capture_identity,
            self.identity.structural_digest(),
        );
        self
    }

    pub(crate) fn len(&self) -> usize {
        self.index.len()
    }

    pub(crate) fn retained_structural_bytes(&self) -> Option<usize> {
        Self::estimated_retained_structural_bytes(self.len())
    }

    pub(crate) fn estimated_retained_structural_bytes(record_count: usize) -> Option<usize> {
        super::interval_index::estimated_retained_structural_bytes::<UiHitTestRegionRecord>(
            record_count,
        )
        .and_then(|bytes| std::mem::size_of::<Self>().checked_add(bytes))
    }

    pub(crate) fn point_candidates(
        &self,
        point: worth_ui_inspection::UiClientPhysicalPixel,
        maximum_candidates: usize,
    ) -> super::interval_index::UiBoundedPointCandidates<'_, UiHitTestRegionRecord> {
        self.index.point_candidates(point, maximum_candidates)
    }

    pub(crate) fn target_record(
        &self,
        node_receipt: u64,
        total_order: u32,
    ) -> Option<UiHitTestRegionRecord> {
        self.index.records().iter().copied().find(|record| {
            record.node_receipt().diagnostic_value() == node_receipt
                && record.total_order().rank() == total_order
        })
    }
}
