use crate::runtime::{WorthUiFrameExecutionReceipt, WorthUiLaneFrameReceiptKind};

use super::denial::WorthUiLaneFrameCostCertificationDenialReason;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneCertification {
    covers_ordinary: bool,
    covers_virtualized_data: bool,
    covers_canvas_spatial: bool,
    covers_realtime_overlay: bool,
}

impl WorthUiLaneCertification {
    pub(crate) fn certify(
        receipt: &WorthUiFrameExecutionReceipt,
    ) -> Result<Self, WorthUiLaneFrameCostCertificationDenialReason> {
        let mut ordinary = 0;
        let mut virtualized_data = 0;
        let mut canvas_spatial = 0;
        let mut realtime_overlay = 0;

        for lane_receipt in receipt.lane_receipts() {
            match lane_receipt.kind() {
                WorthUiLaneFrameReceiptKind::Ordinary => ordinary += 1,
                WorthUiLaneFrameReceiptKind::VirtualizedData => virtualized_data += 1,
                WorthUiLaneFrameReceiptKind::CanvasSpatial => canvas_spatial += 1,
                WorthUiLaneFrameReceiptKind::RealtimeOverlay => realtime_overlay += 1,
            }
        }

        if [ordinary, virtualized_data, canvas_spatial, realtime_overlay]
            .into_iter()
            .any(|count| count > 1)
        {
            return Err(WorthUiLaneFrameCostCertificationDenialReason::DuplicateLaneEvidence);
        }
        if [ordinary, virtualized_data, canvas_spatial, realtime_overlay]
            .into_iter()
            .any(|count| count == 0)
        {
            return Err(WorthUiLaneFrameCostCertificationDenialReason::MissingLaneEvidence);
        }
        if receipt
            .lane_receipts()
            .iter()
            .any(|lane_receipt| lane_receipt.certification_evidence_digest().is_none())
        {
            return Err(
                WorthUiLaneFrameCostCertificationDenialReason::MissingLaneCertificationEvidence,
            );
        }

        Ok(Self {
            covers_ordinary: true,
            covers_virtualized_data: true,
            covers_canvas_spatial: true,
            covers_realtime_overlay: true,
        })
    }

    pub fn covers_all_platform_lanes(&self) -> bool {
        self.covers_ordinary
            && self.covers_virtualized_data
            && self.covers_canvas_spatial
            && self.covers_realtime_overlay
    }
}
