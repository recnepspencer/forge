#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLaneFrameCostCertificationCounters {
    lane_receipt_count: usize,
    certified_frame_receipt_count: usize,
    foundational_receipt_count: usize,
    scale_sample_count: usize,
    denial_count: usize,
}

impl WorthUiLaneFrameCostCertificationCounters {
    pub(crate) fn record_lane_receipts(&mut self, count: usize) {
        self.lane_receipt_count += count;
    }

    pub(crate) fn record_certified_frame_receipt(&mut self) {
        self.certified_frame_receipt_count += 1;
    }

    pub(crate) fn record_foundational_receipts(&mut self, count: usize) {
        self.foundational_receipt_count += count;
    }

    pub(crate) fn record_scale_samples(&mut self, count: usize) {
        self.scale_sample_count += count;
    }

    pub(crate) fn record_denial(&mut self) {
        self.denial_count += 1;
    }

    pub fn lane_receipt_count(self) -> usize {
        self.lane_receipt_count
    }

    pub fn certified_frame_receipt_count(self) -> usize {
        self.certified_frame_receipt_count
    }

    pub fn foundational_receipt_count(self) -> usize {
        self.foundational_receipt_count
    }

    pub fn scale_sample_count(self) -> usize {
        self.scale_sample_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}
