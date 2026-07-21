use crate::runtime::{
    WorthUiCertifiedFrameExecutionReceipt, WorthUiCertifiedMeasurementPacket,
    WorthUiComplexityContract, WorthUiFoundationalCounterBridge,
    WorthUiFoundationalCounterEvidence, WorthUiMeasurementCertificationDenial,
};

use super::denial::{WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSteadyFrameFoundationalEvidence {
    evidence: Vec<WorthUiFoundationalCounterEvidence>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSteadyFrameFoundationalBridge;

impl WorthUiSteadyFrameFoundationalBridge {
    pub fn lower_counter_receipts(
        certified: &WorthUiCertifiedFrameExecutionReceipt,
    ) -> Result<WorthUiSteadyFrameFoundationalEvidence, WorthUiSteadyFrameCounterDenial> {
        let mut evidence = Vec::new();
        evidence.push(lower_packet(certified.receipt().aggregate_packet())?);
        for lane_receipt in certified.receipt().lane_receipts() {
            evidence.push(lower_packet(lane_receipt.packet())?);
        }
        Ok(WorthUiSteadyFrameFoundationalEvidence { evidence })
    }
}

impl WorthUiSteadyFrameFoundationalEvidence {
    pub fn evidence(&self) -> &[WorthUiFoundationalCounterEvidence] {
        &self.evidence
    }

    pub fn receipt_count(&self) -> usize {
        self.evidence.len()
    }
}

fn lower_packet(
    packet: &crate::runtime::WorthUiMeasurementCounterPacket,
) -> Result<WorthUiFoundationalCounterEvidence, WorthUiSteadyFrameCounterDenial> {
    let contract = WorthUiComplexityContract::hot_path(packet.boundary().token())
        .requires_boundary(packet.boundary())
        .requires_counter_family(packet.family())
        .foundational_boundary(packet.boundary().foundational_boundary());
    let certified_packet: WorthUiCertifiedMeasurementPacket = packet
        .clone()
        .certify_against(contract)
        .map_err(measurement_denial)?;
    WorthUiFoundationalCounterBridge::lower_certified_packet(&certified_packet)
        .map_err(foundational_denial)
}

fn measurement_denial(
    denial: WorthUiMeasurementCertificationDenial,
) -> WorthUiSteadyFrameCounterDenial {
    WorthUiSteadyFrameCounterDenial::new(
        WorthUiSteadyFrameCounterDenialReason::MeasurementCertification(denial),
    )
}

fn foundational_denial(
    denial: WorthUiMeasurementCertificationDenial,
) -> WorthUiSteadyFrameCounterDenial {
    WorthUiSteadyFrameCounterDenial::new(
        WorthUiSteadyFrameCounterDenialReason::FoundationalLowering(denial),
    )
}
