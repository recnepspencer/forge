use crate::runtime::WorthUiFrameExecutionReceipt;

use super::counters::WorthUiLaneFrameCostCertificationCounters;
use super::denial::{
    WorthUiLaneFrameCostCertificationDenial, WorthUiLaneFrameCostCertificationDenialReason,
};
use super::foundational_readiness::WorthUiLaneFrameCostFoundationalReadiness;
use super::frame_cost::WorthUiFrameCostCertification;
use super::lane_coverage::WorthUiLaneCertification;
use super::no_source_frame::{WorthUiBroadScanRegressionDenial, WorthUiNoSourceFrameProof};
use super::scale_variation::WorthUiLaneScaleVariationProof;
use super::scenario::WorthUiLaneFrameCostCertificationScenario;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneAndFrameCostCertification {
    scenario_name: String,
    lane_certification: WorthUiLaneCertification,
    frame_cost_certification: WorthUiFrameCostCertification,
    no_source_frame_proof: WorthUiNoSourceFrameProof,
    broad_scan_regression_denial: WorthUiBroadScanRegressionDenial,
    scale_variation_proof: WorthUiLaneScaleVariationProof,
    foundational_readiness: WorthUiLaneFrameCostFoundationalReadiness,
    counters: WorthUiLaneFrameCostCertificationCounters,
}

impl WorthUiLaneAndFrameCostCertification {
    pub(crate) fn certify(
        scenario: WorthUiLaneFrameCostCertificationScenario,
        active_plan_digest: u64,
    ) -> Result<Self, WorthUiLaneFrameCostCertificationDenial> {
        let mut counters = WorthUiLaneFrameCostCertificationCounters::default();
        if scenario.name().is_empty() {
            return Err(denial(
                WorthUiLaneFrameCostCertificationDenialReason::EmptyScenario,
                counters,
            ));
        }
        let receipt = scenario.steady_frame_receipt().ok_or_else(|| {
            denial(
                WorthUiLaneFrameCostCertificationDenialReason::MissingSteadyFrameReceipt,
                counters,
            )
        })?;
        validate_active_plan_digest(receipt, active_plan_digest, counters)?;
        validate_scale_sample_plan_digests(&scenario, active_plan_digest, counters)?;
        counters.record_lane_receipts(receipt.lane_receipts().len());

        let certified = receipt.clone().certify().map_err(|denial_reason| {
            denial(
                WorthUiLaneFrameCostCertificationDenialReason::UncertifiedFrameReceipt(
                    denial_reason.reason(),
                ),
                counters,
            )
        })?;
        counters.record_certified_frame_receipt();

        let lane_certification = WorthUiLaneCertification::certify(receipt)
            .map_err(|reason| denial(reason, counters))?;
        let no_source_frame_proof = WorthUiNoSourceFrameProof::certify(receipt.counters())
            .map_err(|reason| denial(reason, counters))?;
        let broad_scan_regression_denial =
            WorthUiBroadScanRegressionDenial::certify_absent(receipt.counters())
                .map_err(|reason| denial(reason, counters))?;
        let scale_variation_proof = WorthUiLaneScaleVariationProof::certify(
            receipt,
            scenario.virtualized_data_scale_samples(),
            scenario.realtime_scale_samples(),
        )
        .map_err(|reason| denial(reason, counters))?;
        counters.record_scale_samples(
            scenario.virtualized_data_scale_samples().len()
                + scenario.realtime_scale_samples().len(),
        );
        validate_cross_lane_parity(&scenario, active_plan_digest, counters)?;

        let foundational_readiness = WorthUiLaneFrameCostFoundationalReadiness::certify(
            &certified,
            scenario.requires_foundational_readiness(),
        )
        .map_err(|reason| denial(reason, counters))?;
        counters.record_foundational_receipts(
            foundational_readiness
                .foundational_evidence()
                .receipt_count(),
        );

        Ok(Self {
            scenario_name: scenario.name().to_owned(),
            lane_certification,
            frame_cost_certification: WorthUiFrameCostCertification::new(certified),
            no_source_frame_proof,
            broad_scan_regression_denial,
            scale_variation_proof,
            foundational_readiness,
            counters,
        })
    }

    pub fn scenario_name(&self) -> &str {
        &self.scenario_name
    }

    pub fn lane_certification(&self) -> &WorthUiLaneCertification {
        &self.lane_certification
    }

    pub fn frame_cost_certification(&self) -> &WorthUiFrameCostCertification {
        &self.frame_cost_certification
    }

    pub fn no_source_frame_proof(&self) -> WorthUiNoSourceFrameProof {
        self.no_source_frame_proof
    }

    pub fn broad_scan_regression_denial(&self) -> WorthUiBroadScanRegressionDenial {
        self.broad_scan_regression_denial
    }

    pub fn scale_variation_proof(&self) -> WorthUiLaneScaleVariationProof {
        self.scale_variation_proof
    }

    pub fn foundational_readiness(&self) -> &WorthUiLaneFrameCostFoundationalReadiness {
        &self.foundational_readiness
    }

    pub fn counters(&self) -> WorthUiLaneFrameCostCertificationCounters {
        self.counters
    }
}

fn validate_active_plan_digest(
    receipt: &WorthUiFrameExecutionReceipt,
    active_plan_digest: u64,
    counters: WorthUiLaneFrameCostCertificationCounters,
) -> Result<(), WorthUiLaneFrameCostCertificationDenial> {
    if receipt.active_plan_digest() == active_plan_digest {
        return Ok(());
    }
    Err(denial(
        WorthUiLaneFrameCostCertificationDenialReason::ActivePlanDigestMismatch {
            active_plan_digest,
            receipt_plan_digest: receipt.active_plan_digest(),
        },
        counters,
    ))
}

fn validate_scale_sample_plan_digests(
    scenario: &WorthUiLaneFrameCostCertificationScenario,
    active_plan_digest: u64,
    counters: WorthUiLaneFrameCostCertificationCounters,
) -> Result<(), WorthUiLaneFrameCostCertificationDenial> {
    for sample in scenario
        .virtualized_data_scale_samples()
        .iter()
        .chain(scenario.realtime_scale_samples())
    {
        if sample.active_plan_digest() != active_plan_digest {
            return Err(denial(
                WorthUiLaneFrameCostCertificationDenialReason::ActivePlanDigestMismatch {
                    active_plan_digest,
                    receipt_plan_digest: sample.active_plan_digest(),
                },
                counters,
            ));
        }
    }
    Ok(())
}

fn validate_cross_lane_parity(
    scenario: &WorthUiLaneFrameCostCertificationScenario,
    active_plan_digest: u64,
    counters: WorthUiLaneFrameCostCertificationCounters,
) -> Result<(), WorthUiLaneFrameCostCertificationDenial> {
    let Some(parity) = scenario.cross_lane_parity() else {
        return Err(denial(
            WorthUiLaneFrameCostCertificationDenialReason::MissingCrossLaneParity,
            counters,
        ));
    };
    if parity.active_plan_digest() == active_plan_digest {
        return Ok(());
    }
    Err(denial(
        WorthUiLaneFrameCostCertificationDenialReason::CrossLaneParityPlanDigestMismatch {
            active_plan_digest,
            parity_active_plan_digest: parity.active_plan_digest(),
        },
        counters,
    ))
}

fn denial(
    reason: WorthUiLaneFrameCostCertificationDenialReason,
    counters: WorthUiLaneFrameCostCertificationCounters,
) -> WorthUiLaneFrameCostCertificationDenial {
    WorthUiLaneFrameCostCertificationDenial::new(reason, counters)
}
