#[cfg(test)]
use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiOrdinaryLaneCounters, WorthUiRealtimeLaneCounters,
    WorthUiVirtualizedDataCounters,
};
use crate::runtime::{
    WorthUiCanvasSpatialFrameReceipt, WorthUiMeasurementCounterPacket,
    WorthUiOrdinaryLaneFrameReceipt, WorthUiRealtimeFrameReceipt, WorthUiRuntimeCounterFamily,
    WorthUiVirtualizedDataFrameReceipt,
};

use super::denial::{WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason};
use super::lane_rows;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiLaneFrameReceiptKind {
    Ordinary,
    VirtualizedData,
    CanvasSpatial,
    RealtimeOverlay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneFrameReceipt {
    kind: WorthUiLaneFrameReceiptKind,
    packet: WorthUiMeasurementCounterPacket,
    touched_plan_indexes: Vec<u32>,
    certification_evidence_digest: Option<u64>,
}

impl WorthUiLaneFrameReceipt {
    pub(crate) fn from_ordinary(
        active_plan_digest: u64,
        receipt: WorthUiOrdinaryLaneFrameReceipt,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::OrdinaryLaneExecution,
            lane_rows::ordinary_rows(receipt.counters()),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::Ordinary,
            packet,
            touched_plan_indexes: receipt.touched_plan_indexes().to_vec(),
            certification_evidence_digest: Some(ordinary_certification_digest(&receipt)),
        })
    }

    pub(crate) fn from_virtualized_data(
        active_plan_digest: u64,
        receipt: WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::VirtualizedDataExecution,
            lane_rows::virtualized_rows(receipt.counters()),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::VirtualizedData,
            packet,
            touched_plan_indexes: receipt.touched_plan_indexes().to_vec(),
            certification_evidence_digest: Some(virtualized_data_certification_digest(&receipt)),
        })
    }

    pub(crate) fn from_canvas_spatial(
        active_plan_digest: u64,
        receipt: WorthUiCanvasSpatialFrameReceipt,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::CanvasSpatialExecution,
            lane_rows::canvas_rows(receipt.counters()),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::CanvasSpatial,
            packet,
            touched_plan_indexes: receipt.touched_plan_indexes().to_vec(),
            certification_evidence_digest: Some(canvas_spatial_certification_digest(&receipt)),
        })
    }

    pub(crate) fn from_realtime_overlay(
        active_plan_digest: u64,
        receipt: WorthUiRealtimeFrameReceipt,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::RealtimeOverlayExecution,
            lane_rows::realtime_rows(receipt.counters()),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::RealtimeOverlay,
            packet,
            touched_plan_indexes: receipt.touched_plan_indexes().to_vec(),
            certification_evidence_digest: Some(realtime_certification_digest(&receipt)),
        })
    }

    #[cfg(test)]
    pub(crate) fn from_ordinary_counters_for_test(
        active_plan_digest: u64,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::OrdinaryLaneExecution,
            lane_rows::ordinary_rows(counters),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::Ordinary,
            packet,
            touched_plan_indexes: Vec::new(),
            certification_evidence_digest: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_realtime_counters_for_test(
        active_plan_digest: u64,
        counters: WorthUiRealtimeLaneCounters,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::RealtimeOverlayExecution,
            lane_rows::realtime_rows(counters),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::RealtimeOverlay,
            packet,
            touched_plan_indexes: Vec::new(),
            certification_evidence_digest: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_virtualized_counters_for_test(
        active_plan_digest: u64,
        counters: WorthUiVirtualizedDataCounters,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::VirtualizedDataExecution,
            lane_rows::virtualized_rows(counters),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::VirtualizedData,
            packet,
            touched_plan_indexes: Vec::new(),
            certification_evidence_digest: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_canvas_counters_for_test(
        active_plan_digest: u64,
        counters: WorthUiCanvasSpatialCounters,
    ) -> Result<Self, WorthUiSteadyFrameCounterDenial> {
        let packet = packet_for_family(
            active_plan_digest,
            WorthUiRuntimeCounterFamily::CanvasSpatialExecution,
            lane_rows::canvas_rows(counters),
        )?;
        Ok(Self {
            kind: WorthUiLaneFrameReceiptKind::CanvasSpatial,
            packet,
            touched_plan_indexes: Vec::new(),
            certification_evidence_digest: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn from_packet_for_test(
        kind: WorthUiLaneFrameReceiptKind,
        packet: WorthUiMeasurementCounterPacket,
    ) -> Self {
        Self {
            kind,
            packet,
            touched_plan_indexes: Vec::new(),
            certification_evidence_digest: None,
        }
    }

    pub fn kind(&self) -> WorthUiLaneFrameReceiptKind {
        self.kind
    }

    pub fn packet(&self) -> &WorthUiMeasurementCounterPacket {
        &self.packet
    }

    pub fn touched_plan_indexes(&self) -> &[u32] {
        &self.touched_plan_indexes
    }

    pub fn certification_evidence_digest(&self) -> Option<u64> {
        self.certification_evidence_digest
    }
}

fn ordinary_certification_digest(receipt: &WorthUiOrdinaryLaneFrameReceipt) -> u64 {
    let certification = receipt.certification();
    fold_u64(
        0x0D1A_110A_D1EE_0001,
        certification.ordinary_plan_digest()
            ^ certification.support_digest().rotate_left(11)
            ^ certification
                .handle_receipt()
                .basis_digest()
                .rotate_left(23),
    )
}

fn virtualized_data_certification_digest(receipt: &WorthUiVirtualizedDataFrameReceipt) -> u64 {
    let certification = receipt.certification();
    fold_u64(
        receipt.query_patch_posture().canonical_digest(),
        certification.data_plan_digest()
            ^ certification.support_digest().rotate_left(13)
            ^ certification
                .handle_receipt()
                .basis_digest()
                .rotate_left(29),
    )
}

fn canvas_spatial_certification_digest(receipt: &WorthUiCanvasSpatialFrameReceipt) -> u64 {
    let certification = receipt.certification();
    fold_u64(
        0xCA12_5CA1_0000_0001,
        certification.canvas_plan_digest()
            ^ certification.support_digest().rotate_left(17)
            ^ certification
                .handle_receipt()
                .basis_digest()
                .rotate_left(31),
    )
}

fn realtime_certification_digest(receipt: &WorthUiRealtimeFrameReceipt) -> u64 {
    let certification = receipt.certification();
    fold_u64(
        0xFEA1_71AE_0000_0001,
        certification.hud_plan_digest()
            ^ certification.support_digest().rotate_left(19)
            ^ certification.policy_digest().rotate_left(37)
            ^ certification
                .handle_receipt()
                .basis_digest()
                .rotate_left(41),
    )
}

fn fold_u64(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}

fn packet_for_family(
    active_plan_digest: u64,
    family: WorthUiRuntimeCounterFamily,
    rows: Vec<crate::runtime::WorthUiFrameCostCounter>,
) -> Result<WorthUiMeasurementCounterPacket, WorthUiSteadyFrameCounterDenial> {
    let mut builder = family
        .at_boundary(family.allowed_boundary())
        .with_active_plan_digest(active_plan_digest);
    for row in rows {
        builder = builder.record(row);
    }
    builder.seal().map_err(|denial| {
        WorthUiSteadyFrameCounterDenial::new(
            WorthUiSteadyFrameCounterDenialReason::MeasurementCertification(denial),
        )
    })
}
