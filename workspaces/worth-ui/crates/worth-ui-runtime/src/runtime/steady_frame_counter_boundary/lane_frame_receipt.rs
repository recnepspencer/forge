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
    touches: WorthUiLaneFrameTouches,
    certification_evidence_digest: Option<u64>,
    work_scope: crate::runtime::WorthUiFrameWorkScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiLaneFrameTouches {
    Ordinary(crate::runtime::WorthUiOrdinaryLaneTouchReceipt),
    Indexed(Vec<u32>),
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
            touches: WorthUiLaneFrameTouches::Ordinary(receipt.touch().clone()),
            certification_evidence_digest: Some(ordinary_certification_digest(&receipt)),
            work_scope: receipt.work_scope(),
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
            touches: WorthUiLaneFrameTouches::Indexed(vec![receipt.touched_plan_index()]),
            certification_evidence_digest: Some(virtualized_data_certification_digest(&receipt)),
            work_scope: receipt.work_scope(),
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
            touches: WorthUiLaneFrameTouches::Indexed(receipt.touched_plan_indexes().to_vec()),
            certification_evidence_digest: Some(canvas_spatial_certification_digest(&receipt)),
            work_scope: receipt.work_scope(),
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
            touches: WorthUiLaneFrameTouches::Indexed(receipt.touched_plan_indexes().to_vec()),
            certification_evidence_digest: Some(realtime_certification_digest(&receipt)),
            work_scope: receipt.work_scope(),
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
            touches: WorthUiLaneFrameTouches::Indexed(Vec::new()),
            certification_evidence_digest: None,
            work_scope: crate::runtime::WorthUiFrameWorkScope::new(0, 0),
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
            touches: WorthUiLaneFrameTouches::Indexed(Vec::new()),
            certification_evidence_digest: None,
            work_scope: crate::runtime::WorthUiFrameWorkScope::new(0, 0),
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
            touches: WorthUiLaneFrameTouches::Indexed(Vec::new()),
            certification_evidence_digest: None,
            work_scope: crate::runtime::WorthUiFrameWorkScope::new(0, 0),
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
            touches: WorthUiLaneFrameTouches::Indexed(Vec::new()),
            certification_evidence_digest: None,
            work_scope: crate::runtime::WorthUiFrameWorkScope::new(0, 0),
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
            touches: WorthUiLaneFrameTouches::Indexed(Vec::new()),
            certification_evidence_digest: None,
            work_scope: crate::runtime::WorthUiFrameWorkScope::new(0, 0),
        }
    }

    pub fn kind(&self) -> WorthUiLaneFrameReceiptKind {
        self.kind
    }

    pub fn packet(&self) -> &WorthUiMeasurementCounterPacket {
        &self.packet
    }

    pub fn touched_plan_index_count(&self) -> usize {
        match &self.touches {
            WorthUiLaneFrameTouches::Ordinary(touch) => touch.row_count(),
            WorthUiLaneFrameTouches::Indexed(indexes) => indexes.len(),
        }
    }

    #[cfg(test)]
    pub(crate) fn replace_work_scope_for_test(
        &mut self,
        work_scope: crate::runtime::WorthUiFrameWorkScope,
    ) {
        self.work_scope = work_scope;
    }

    /// Reports whether this compact receipt explicitly names `plan_index`.
    ///
    /// Ordinary subtree descendants are covered by the receipt's breadth,
    /// row count, and digest without materializing their indexes per frame.
    pub fn names_plan_index(&self, plan_index: u32) -> bool {
        match &self.touches {
            WorthUiLaneFrameTouches::Ordinary(touch) => touch.names_plan_index(plan_index),
            WorthUiLaneFrameTouches::Indexed(indexes) => indexes.contains(&plan_index),
        }
    }

    pub fn certification_evidence_digest(&self) -> Option<u64> {
        self.certification_evidence_digest
    }

    pub fn work_scope(&self) -> crate::runtime::WorthUiFrameWorkScope {
        self.work_scope
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
        receipt.evidence().evidence_identity_digest(),
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
