use crate::runtime::{
    WorthUiFrameExecutionReceipt, WorthUiLaneFrameReceipt, WorthUiLaneFrameReceiptKind,
};

use super::denial::WorthUiLaneFrameCostCertificationDenialReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLaneScaleVariationProof {
    virtualized_data_sample_count: usize,
    realtime_sample_count: usize,
}

impl WorthUiLaneScaleVariationProof {
    pub(crate) fn certify(
        primary: &WorthUiFrameExecutionReceipt,
        data_samples: &[WorthUiFrameExecutionReceipt],
        realtime_samples: &[WorthUiFrameExecutionReceipt],
    ) -> Result<Self, WorthUiLaneFrameCostCertificationDenialReason> {
        reject_forbidden_data_or_realtime_counters(primary)?;
        for sample in data_samples {
            reject_forbidden_data_or_realtime_counters(sample)?;
            require_scale_lane_certification_evidence(
                sample,
                WorthUiLaneFrameReceiptKind::VirtualizedData,
            )?;
        }
        for sample in realtime_samples {
            reject_forbidden_data_or_realtime_counters(sample)?;
            require_scale_lane_certification_evidence(
                sample,
                WorthUiLaneFrameReceiptKind::RealtimeOverlay,
            )?;
        }

        let data_widths = distinct_data_width_count(primary, data_samples);
        let realtime_widths = distinct_realtime_width_count(primary, realtime_samples);
        if data_widths < 2 || realtime_widths < 2 {
            return Err(WorthUiLaneFrameCostCertificationDenialReason::MissingScaleVariation);
        }
        Ok(Self {
            virtualized_data_sample_count: data_samples.len() + 1,
            realtime_sample_count: realtime_samples.len() + 1,
        })
    }

    pub fn virtualized_data_sample_count(self) -> usize {
        self.virtualized_data_sample_count
    }

    pub fn realtime_sample_count(self) -> usize {
        self.realtime_sample_count
    }
}

fn require_scale_lane_certification_evidence(
    receipt: &WorthUiFrameExecutionReceipt,
    kind: WorthUiLaneFrameReceiptKind,
) -> Result<(), WorthUiLaneFrameCostCertificationDenialReason> {
    if receipt
        .lane_receipts()
        .iter()
        .any(|lane| is_certified_lane_receipt(lane, kind))
    {
        return Ok(());
    }

    Err(WorthUiLaneFrameCostCertificationDenialReason::MissingLaneCertificationEvidence)
}

fn is_certified_lane_receipt(
    receipt: &WorthUiLaneFrameReceipt,
    kind: WorthUiLaneFrameReceiptKind,
) -> bool {
    receipt.kind() == kind && receipt.certification_evidence_digest().is_some()
}

fn reject_forbidden_data_or_realtime_counters(
    receipt: &WorthUiFrameExecutionReceipt,
) -> Result<(), WorthUiLaneFrameCostCertificationDenialReason> {
    if receipt
        .counters()
        .virtualized_data()
        .full_collection_scan_count()
        > 0
    {
        return Err(WorthUiLaneFrameCostCertificationDenialReason::FullCollectionDataScan);
    }
    if receipt
        .counters()
        .realtime_overlay()
        .ordinary_layout_pass_count()
        > 0
    {
        return Err(WorthUiLaneFrameCostCertificationDenialReason::RealtimeOrdinaryTraversal);
    }
    Ok(())
}

fn distinct_data_width_count(
    primary: &WorthUiFrameExecutionReceipt,
    samples: &[WorthUiFrameExecutionReceipt],
) -> usize {
    let mut widths = Vec::new();
    widths.push(data_width(primary));
    widths.extend(samples.iter().map(data_width));
    widths.sort();
    widths.dedup();
    widths.len()
}

fn distinct_realtime_width_count(
    primary: &WorthUiFrameExecutionReceipt,
    samples: &[WorthUiFrameExecutionReceipt],
) -> usize {
    let mut widths = Vec::new();
    widths.push(realtime_width(primary));
    widths.extend(samples.iter().map(realtime_width));
    widths.sort();
    widths.dedup();
    widths.len()
}

fn data_width(receipt: &WorthUiFrameExecutionReceipt) -> (usize, usize) {
    let counters = receipt.counters().virtualized_data();
    (
        counters.visible_row_touch_count(),
        counters.visible_column_touch_count(),
    )
}

fn realtime_width(receipt: &WorthUiFrameExecutionReceipt) -> (usize, usize) {
    let counters = receipt.counters().realtime_overlay();
    (
        counters.targeted_overlay_row_count(),
        counters.renderer_surface_handoff_count(),
    )
}
