use crate::runtime::{WorthUiMeasurementCounterPacket, WorthUiSteadyFrameCounters};

use super::counter_schema;
use super::denial::{WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason};
use super::diagnostic_policy::WorthUiSteadyFrameDiagnosticPolicy;
use super::lane_frame_receipt::WorthUiLaneFrameReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFrameExecutionReceipt {
    active_plan_digest: u64,
    diagnostic_policy: WorthUiSteadyFrameDiagnosticPolicy,
    counters: WorthUiSteadyFrameCounters,
    aggregate_packet: WorthUiMeasurementCounterPacket,
    lane_receipts: Vec<WorthUiLaneFrameReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCertifiedFrameExecutionReceipt {
    receipt: WorthUiFrameExecutionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRenderCostReceipt {
    active_plan_digest: u64,
    packets: Vec<WorthUiMeasurementCounterPacket>,
}

impl WorthUiFrameExecutionReceipt {
    pub(crate) fn new(
        active_plan_digest: u64,
        diagnostic_policy: WorthUiSteadyFrameDiagnosticPolicy,
        counters: WorthUiSteadyFrameCounters,
        aggregate_packet: WorthUiMeasurementCounterPacket,
        lane_receipts: Vec<WorthUiLaneFrameReceipt>,
    ) -> Self {
        Self {
            active_plan_digest,
            diagnostic_policy,
            counters,
            aggregate_packet,
            lane_receipts,
        }
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn diagnostic_policy(&self) -> WorthUiSteadyFrameDiagnosticPolicy {
        self.diagnostic_policy
    }

    pub fn counters(&self) -> WorthUiSteadyFrameCounters {
        self.counters
    }

    pub fn aggregate_packet(&self) -> &WorthUiMeasurementCounterPacket {
        &self.aggregate_packet
    }

    pub fn lane_receipts(&self) -> &[WorthUiLaneFrameReceipt] {
        &self.lane_receipts
    }

    pub fn certify(
        self,
    ) -> Result<WorthUiCertifiedFrameExecutionReceipt, WorthUiSteadyFrameCounterDenial> {
        validate_frame_receipt(&self)?;
        Ok(WorthUiCertifiedFrameExecutionReceipt { receipt: self })
    }

    pub fn render_cost_receipt(&self) -> WorthUiRenderCostReceipt {
        let mut packets = Vec::with_capacity(self.lane_receipts.len() + 1);
        packets.push(self.aggregate_packet.clone());
        packets.extend(
            self.lane_receipts
                .iter()
                .map(|receipt| receipt.packet().clone()),
        );
        WorthUiRenderCostReceipt {
            active_plan_digest: self.active_plan_digest,
            packets,
        }
    }
}

impl WorthUiCertifiedFrameExecutionReceipt {
    pub fn receipt(&self) -> &WorthUiFrameExecutionReceipt {
        &self.receipt
    }
}

impl WorthUiRenderCostReceipt {
    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn packets(&self) -> &[WorthUiMeasurementCounterPacket] {
        &self.packets
    }
}

fn validate_frame_receipt(
    receipt: &WorthUiFrameExecutionReceipt,
) -> Result<(), WorthUiSteadyFrameCounterDenial> {
    counter_schema::validate_packet_schema(&receipt.aggregate_packet)?;
    for lane_receipt in &receipt.lane_receipts {
        counter_schema::validate_packet_schema(lane_receipt.packet())?;
    }
    if receipt
        .lane_receipts
        .iter()
        .any(|lane| lane.packet().active_plan_digest() != receipt.active_plan_digest)
        || receipt.aggregate_packet.active_plan_digest() != receipt.active_plan_digest
    {
        return Err(WorthUiSteadyFrameCounterDenial::new(
            WorthUiSteadyFrameCounterDenialReason::MeasurementCertification(
                crate::runtime::WorthUiMeasurementCertificationDenial::BoundaryNotRequiredByContract,
            ),
        ));
    }
    Ok(())
}
