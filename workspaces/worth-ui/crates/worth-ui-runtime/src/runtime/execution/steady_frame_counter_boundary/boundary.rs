#[cfg(test)]
use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiLaneFrameReceiptKind, WorthUiOrdinaryLaneCounters,
    WorthUiRealtimeLaneCounters, WorthUiVirtualizedDataCounters,
};
use crate::runtime::{
    WorthUiCanvasSpatialFrameReceipt, WorthUiCounterCaptureRichness,
    WorthUiMeasurementCounterPacket, WorthUiOrdinaryLaneFrameReceipt, WorthUiRealtimeFrameReceipt,
    WorthUiRuntimeCounterFamily, WorthUiSteadyFrameCounters, WorthUiVirtualizedDataFrameReceipt,
};

use super::counter_schema;
use super::denial::{WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason};
use super::diagnostic_policy::WorthUiSteadyFrameDiagnosticPolicy;
use super::frame_receipt::WorthUiFrameExecutionReceipt;
use super::lane_frame_receipt::WorthUiLaneFrameReceipt;
use super::lane_rows;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSteadyFrameCounterBoundary;

#[derive(Clone, Debug)]
pub struct WorthUiSteadyFrameCounterReceiptBuilder {
    basis: super::WorthUiFrameExecutionBasis,
    active_plan_digest: u64,
    diagnostic_policy: WorthUiSteadyFrameDiagnosticPolicy,
    capture_richness: WorthUiCounterCaptureRichness,
    counters: WorthUiSteadyFrameCounters,
    lane_receipts: Vec<WorthUiLaneFrameReceipt>,
    construction_denial: Option<WorthUiSteadyFrameCounterDenialReason>,
    frame_path_report_materialization_count: u64,
}

impl WorthUiSteadyFrameCounterBoundary {
    pub fn for_execution_basis(
        basis: super::WorthUiFrameExecutionBasis,
    ) -> WorthUiSteadyFrameCounterReceiptBuilder {
        WorthUiSteadyFrameCounterReceiptBuilder::new(basis)
    }

    #[cfg(test)]
    pub fn for_active_plan(active_plan_digest: u64) -> WorthUiSteadyFrameCounterReceiptBuilder {
        WorthUiSteadyFrameCounterReceiptBuilder::new(super::WorthUiFrameExecutionBasis::new(
            0,
            0,
            active_plan_digest,
            0,
        ))
    }
}

impl WorthUiSteadyFrameCounterReceiptBuilder {
    pub(crate) fn new(basis: super::WorthUiFrameExecutionBasis) -> Self {
        Self {
            active_plan_digest: basis.active_plan(),
            basis,
            diagnostic_policy: WorthUiSteadyFrameDiagnosticPolicy::Minimal,
            capture_richness: WorthUiCounterCaptureRichness::Standard,
            counters: WorthUiSteadyFrameCounters::default(),
            lane_receipts: Vec::new(),
            construction_denial: None,
            frame_path_report_materialization_count: 0,
        }
    }

    pub fn record_ordinary_lane_frame(mut self, receipt: WorthUiOrdinaryLaneFrameReceipt) -> Self {
        self.counters.record_ordinary(receipt.counters());
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_ordinary(
            self.active_plan_digest,
            receipt,
        ));
        self
    }

    pub fn record_virtualized_data_frame(
        mut self,
        receipt: WorthUiVirtualizedDataFrameReceipt,
    ) -> Self {
        self.counters.record_virtualized_data(receipt.counters());
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_virtualized_data(
            self.active_plan_digest,
            receipt,
        ));
        self
    }

    pub fn record_canvas_spatial_frame(
        mut self,
        receipt: WorthUiCanvasSpatialFrameReceipt,
    ) -> Self {
        self.counters.record_canvas_spatial(receipt.counters());
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_canvas_spatial(
            self.active_plan_digest,
            receipt,
        ));
        self
    }

    pub fn record_realtime_overlay_frame(mut self, receipt: WorthUiRealtimeFrameReceipt) -> Self {
        self.counters.record_realtime_overlay(receipt.counters());
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_realtime_overlay(
            self.active_plan_digest,
            receipt,
        ));
        self
    }

    #[cfg(test)]
    pub(crate) fn record_ordinary_counters_for_test(
        mut self,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Self {
        self.counters.record_ordinary(counters);
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_ordinary_counters_for_test(
            self.active_plan_digest,
            counters,
        ));
        self
    }

    #[cfg(test)]
    pub(crate) fn record_realtime_counters_for_test(
        mut self,
        counters: WorthUiRealtimeLaneCounters,
    ) -> Self {
        self.counters.record_realtime_overlay(counters);
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_realtime_counters_for_test(
            self.active_plan_digest,
            counters,
        ));
        self
    }

    #[cfg(test)]
    pub(crate) fn record_virtualized_counters_for_test(
        mut self,
        counters: WorthUiVirtualizedDataCounters,
    ) -> Self {
        self.counters.record_virtualized_data(counters);
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_virtualized_counters_for_test(
            self.active_plan_digest,
            counters,
        ));
        self
    }

    #[cfg(test)]
    pub(crate) fn record_canvas_counters_for_test(
        mut self,
        counters: WorthUiCanvasSpatialCounters,
    ) -> Self {
        self.counters.record_canvas_spatial(counters);
        self.push_lane_receipt(WorthUiLaneFrameReceipt::from_canvas_counters_for_test(
            self.active_plan_digest,
            counters,
        ));
        self
    }

    #[cfg(test)]
    pub(crate) fn record_frame_path_report_materialization_for_test(mut self) -> Self {
        self.frame_path_report_materialization_count += 1;
        self
    }

    #[cfg(test)]
    pub(crate) fn record_lane_packet_for_test(
        mut self,
        kind: WorthUiLaneFrameReceiptKind,
        packet: WorthUiMeasurementCounterPacket,
    ) -> Self {
        self.lane_receipts
            .push(WorthUiLaneFrameReceipt::from_packet_for_test(kind, packet));
        self
    }

    #[cfg(test)]
    pub(crate) fn replace_last_work_scope_for_test(
        mut self,
        work_scope: crate::runtime::WorthUiFrameWorkScope,
    ) -> Self {
        self.lane_receipts
            .last_mut()
            .expect("test must record a real lane receipt before replacing its scope")
            .replace_work_scope_for_test(work_scope);
        self
    }

    pub fn seal(mut self) -> Result<WorthUiFrameExecutionReceipt, WorthUiSteadyFrameCounterDenial> {
        if let Some(reason) = self.construction_denial {
            return Err(WorthUiSteadyFrameCounterDenial::new(reason));
        }
        if self.lane_receipts.is_empty() {
            return Err(WorthUiSteadyFrameCounterDenial::new(
                WorthUiSteadyFrameCounterDenialReason::EmptySteadyFrameReceipt,
            ));
        }
        self.lane_receipts.sort_by_key(|receipt| receipt.kind());
        if self.has_duplicate_lane_receipts() {
            return Err(WorthUiSteadyFrameCounterDenial::new(
                WorthUiSteadyFrameCounterDenialReason::DuplicateLaneFrameReceipt,
            ));
        }
        for receipt in &self.lane_receipts {
            counter_schema::validate_packet_schema(receipt.packet())?;
        }
        if self.has_forbidden_frame_path_work() || self.has_forbidden_lane_packet_work() {
            return Err(WorthUiSteadyFrameCounterDenial::new(
                WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork,
            ));
        }
        if self.has_forbidden_diagnostic_materialization() {
            return Err(WorthUiSteadyFrameCounterDenial::new(
                WorthUiSteadyFrameCounterDenialReason::DiagnosticMaterializationOnMinimalPolicy,
            ));
        }
        let aggregate_packet = self.aggregate_packet()?;
        counter_schema::validate_packet_schema(&aggregate_packet)?;
        for receipt in &self.lane_receipts {
            self.validate_lane_packet_matches_counters(receipt)?;
        }
        Ok(WorthUiFrameExecutionReceipt::new(
            self.basis,
            self.active_plan_digest,
            self.diagnostic_policy,
            self.counters,
            aggregate_packet,
            self.lane_receipts,
        ))
    }

    fn push_lane_receipt(
        &mut self,
        receipt: Result<WorthUiLaneFrameReceipt, WorthUiSteadyFrameCounterDenial>,
    ) {
        match receipt {
            Ok(receipt) => self.lane_receipts.push(receipt),
            Err(denial) => self.construction_denial = Some(denial.reason()),
        }
    }

    fn aggregate_packet(
        &self,
    ) -> Result<WorthUiMeasurementCounterPacket, WorthUiSteadyFrameCounterDenial> {
        let family = WorthUiRuntimeCounterFamily::SteadyFrameRendering;
        let mut builder = family
            .at_boundary(family.allowed_boundary())
            .with_capture_richness(self.capture_richness)
            .with_active_plan_digest(self.active_plan_digest);
        for row in lane_rows::aggregate_rows(self.counters) {
            builder = builder.record(row);
        }
        builder.seal().map_err(|denial| {
            WorthUiSteadyFrameCounterDenial::new(
                WorthUiSteadyFrameCounterDenialReason::MeasurementCertification(denial),
            )
        })
    }

    fn has_duplicate_lane_receipts(&self) -> bool {
        self.lane_receipts
            .windows(2)
            .any(|window| window[0].kind() == window[1].kind())
    }

    fn has_forbidden_frame_path_work(&self) -> bool {
        self.counters.total_forbidden_source_or_registry_work() > 0
            || self.counters.ordinary().artifact_tree_scan_count() > 0
            || self.counters.ordinary().full_plan_scan_count() > 0
            || self
                .counters
                .virtualized_data()
                .full_collection_scan_count()
                > 0
            || self
                .counters
                .canvas_spatial()
                .domain_geometry_truth_read_count()
                > 0
            || self
                .counters
                .canvas_spatial()
                .renderer_internal_read_count()
                > 0
            || self
                .counters
                .realtime_overlay()
                .ordinary_layout_pass_count()
                > 0
    }

    fn has_forbidden_lane_packet_work(&self) -> bool {
        self.lane_receipts.iter().any(|receipt| {
            receipt.packet().counters().iter().any(|counter| {
                matches!(
                    counter.name(),
                    counter_schema::ORDINARY_ARTIFACT_TREE_SCAN_COUNT
                        | counter_schema::ORDINARY_FULL_PLAN_SCAN_COUNT
                        | counter_schema::VIRTUALIZED_FULL_COLLECTION_SCAN_COUNT
                        | counter_schema::CANVAS_DOMAIN_GEOMETRY_TRUTH_READS
                        | counter_schema::CANVAS_RENDERER_INTERNAL_READS
                        | counter_schema::REALTIME_ORDINARY_LAYOUT_PASSES
                        | counter_schema::ORDINARY_SOURCE_PARSE_COUNT
                        | counter_schema::ORDINARY_REGISTRY_LOOKUP_COUNT
                        | counter_schema::REALTIME_SOURCE_PARSE_COUNT
                        | counter_schema::REALTIME_REGISTRY_LOOKUP_COUNT
                ) && counter.value() > 0
            })
        })
    }

    fn validate_lane_packet_matches_counters(
        &self,
        receipt: &WorthUiLaneFrameReceipt,
    ) -> Result<(), WorthUiSteadyFrameCounterDenial> {
        let (expected_family, mut expected_rows) = match receipt.kind() {
            crate::runtime::WorthUiLaneFrameReceiptKind::Ordinary => (
                WorthUiRuntimeCounterFamily::OrdinaryLaneExecution,
                lane_rows::ordinary_rows(self.counters.ordinary()),
            ),
            crate::runtime::WorthUiLaneFrameReceiptKind::VirtualizedData => (
                WorthUiRuntimeCounterFamily::VirtualizedDataExecution,
                lane_rows::virtualized_rows(self.counters.virtualized_data()),
            ),
            crate::runtime::WorthUiLaneFrameReceiptKind::CanvasSpatial => (
                WorthUiRuntimeCounterFamily::CanvasSpatialExecution,
                lane_rows::canvas_rows(self.counters.canvas_spatial()),
            ),
            crate::runtime::WorthUiLaneFrameReceiptKind::RealtimeOverlay => (
                WorthUiRuntimeCounterFamily::RealtimeOverlayExecution,
                lane_rows::realtime_rows(self.counters.realtime_overlay()),
            ),
        };
        expected_rows.sort();
        let packet = receipt.packet();
        if packet.family() != expected_family
            || packet.active_plan_digest() != self.active_plan_digest
            || packet.counters() != expected_rows.as_slice()
        {
            return Err(WorthUiSteadyFrameCounterDenial::new(
                WorthUiSteadyFrameCounterDenialReason::LaneFrameReceiptMismatch,
            ));
        }
        Ok(())
    }

    fn has_forbidden_diagnostic_materialization(&self) -> bool {
        !self
            .diagnostic_policy
            .allows_frame_path_report_materialization()
            && (self.counters.total_diagnostic_materialization_count() > 0
                || self.frame_path_report_materialization_count > 0)
    }
}
