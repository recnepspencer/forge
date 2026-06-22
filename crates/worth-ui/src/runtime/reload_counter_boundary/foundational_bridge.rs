use crate::runtime::{
    WorthUiCertifiedMeasurementPacket, WorthUiCertifiedReloadLoweringCounterReceipt,
    WorthUiComplexityContract, WorthUiFoundationalCounterBridge,
    WorthUiFoundationalCounterEvidence, WorthUiMeasurementCertificationDenial,
};

use super::denial::{WorthUiReloadCounterBoundaryDenial, WorthUiReloadCounterBoundaryDenialReason};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadLoweringFoundationalEvidence {
    evidence: Vec<WorthUiFoundationalCounterEvidence>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiReloadLoweringFoundationalBridge;

impl WorthUiReloadLoweringFoundationalBridge {
    pub fn lower(
        certified: &WorthUiCertifiedReloadLoweringCounterReceipt,
    ) -> Result<WorthUiReloadLoweringFoundationalEvidence, WorthUiReloadCounterBoundaryDenial> {
        let mut evidence = Vec::new();
        for packet in certified.receipt().packets() {
            let contract = WorthUiComplexityContract::hot_path(packet.boundary().token())
                .requires_boundary(packet.boundary())
                .requires_counter_family(packet.family())
                .foundational_boundary(packet.boundary().foundational_boundary());
            let certified_packet: WorthUiCertifiedMeasurementPacket = packet
                .clone()
                .certify_against(contract)
                .map_err(measurement_denial)?;
            let foundational =
                WorthUiFoundationalCounterBridge::lower_certified_packet(&certified_packet)
                    .map_err(foundational_denial)?;
            evidence.push(foundational);
        }
        Ok(WorthUiReloadLoweringFoundationalEvidence { evidence })
    }
}

impl WorthUiReloadLoweringFoundationalEvidence {
    pub fn evidence(&self) -> &[WorthUiFoundationalCounterEvidence] {
        &self.evidence
    }

    pub fn receipt_count(&self) -> usize {
        self.evidence.len()
    }
}

fn measurement_denial(
    denial: WorthUiMeasurementCertificationDenial,
) -> WorthUiReloadCounterBoundaryDenial {
    WorthUiReloadCounterBoundaryDenial::new(
        WorthUiReloadCounterBoundaryDenialReason::MeasurementCertification(denial),
    )
}

fn foundational_denial(
    denial: WorthUiMeasurementCertificationDenial,
) -> WorthUiReloadCounterBoundaryDenial {
    WorthUiReloadCounterBoundaryDenial::new(
        WorthUiReloadCounterBoundaryDenialReason::FoundationalLowering(denial),
    )
}
