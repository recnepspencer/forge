//! Host observation lane transitions: freeze → observe → normalize → admit.

use worth_ui_host_contract::{
    UiHostObservation, UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity,
    WorthUiHostCapabilityReport, WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{UiCurrentMeasurementResult, UiMeasurementResult};

use super::{
    invalidate_stale_host_measurement_evidence, normalize_host_measurement_evidence,
    request_host_measurement, UiHostMeasurementAssumptionProfile, UiHostMeasurementEvidenceDenial,
    UiHostMeasurementNeed, UiHostMeasurementNormalizationContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementFreshnessWitness {
    current_generation: UiEvidenceAuthorityGeneration,
    current_profile: UiHostMeasurementAssumptionProfile,
}

impl UiHostMeasurementFreshnessWitness {
    pub fn new(
        current_generation: UiEvidenceAuthorityGeneration,
        current_profile: UiHostMeasurementAssumptionProfile,
    ) -> Self {
        Self {
            current_generation,
            current_profile,
        }
    }

    pub(crate) fn current_generation(self) -> UiEvidenceAuthorityGeneration {
        self.current_generation
    }

    pub(crate) fn current_profile(self) -> UiHostMeasurementAssumptionProfile {
        self.current_profile
    }
}

pub(crate) fn observe_host_measurement<A: WorthUiMeasurementHostAdapter>(
    adapter: &A,
    identity: UiMeasurementRequestIdentity,
    evidence_family: UiMeasurementEvidenceFamily,
    need: UiHostMeasurementNeed,
    capability_report: &WorthUiHostCapabilityReport,
) -> Result<UiHostObservation, UiHostMeasurementEvidenceDenial> {
    request_host_measurement(adapter, identity, evidence_family, need, capability_report)
        .map_err(UiHostMeasurementEvidenceDenial::Execution)
}

pub(crate) fn normalize_host_observation(
    observation: UiHostObservation,
    evidence_generation: UiEvidenceAuthorityGeneration,
    normalization_context: UiHostMeasurementNormalizationContext,
) -> Result<UiMeasurementResult, UiHostMeasurementEvidenceDenial> {
    normalize_host_measurement_evidence(observation, evidence_generation, normalization_context)
        .map_err(UiHostMeasurementEvidenceDenial::Normalization)
}

pub(crate) fn construct_freshness_witness(
    evidence_generation: UiEvidenceAuthorityGeneration,
    assumption_profile: UiHostMeasurementAssumptionProfile,
) -> UiHostMeasurementFreshnessWitness {
    UiHostMeasurementFreshnessWitness::new(evidence_generation, assumption_profile)
}

pub(crate) fn admit_fresh_host_evidence<'a>(
    result: &'a UiMeasurementResult,
    freshness_witness: UiHostMeasurementFreshnessWitness,
) -> Result<UiCurrentMeasurementResult<'a>, UiHostMeasurementEvidenceDenial> {
    invalidate_stale_host_measurement_evidence(
        result,
        freshness_witness.current_generation(),
        freshness_witness.current_profile(),
    )
    .map_err(UiHostMeasurementEvidenceDenial::Stale)?;
    Ok(UiCurrentMeasurementResult::new(result))
}