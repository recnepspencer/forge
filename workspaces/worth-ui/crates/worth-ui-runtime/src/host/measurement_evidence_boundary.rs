use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::{UiCurrentMeasurementResult, UiMeasurementResult};

use super::{
    invalidate_stale_host_measurement_evidence, normalize_host_measurement_evidence,
    request_host_measurement, UiHostMeasurementAssumptionProfile, UiHostMeasurementEvidenceDenial,
    UiHostMeasurementNeed, UiHostMeasurementNormalizationContext,
};

pub fn collect_host_measurement_evidence<A: WorthUiMeasurementHostAdapter>(
    adapter: &A,
    identity: UiMeasurementRequestIdentity,
    evidence_family: UiMeasurementEvidenceFamily,
    need: UiHostMeasurementNeed,
    capability_report: &WorthUiHostCapabilityReport,
    evidence_generation: UiEvidenceAuthorityGeneration,
    normalization_context: UiHostMeasurementNormalizationContext,
) -> Result<UiMeasurementResult, UiHostMeasurementEvidenceDenial> {
    let observation =
        request_host_measurement(adapter, identity, evidence_family, need, capability_report)
            .map_err(UiHostMeasurementEvidenceDenial::Execution)?;
    let normalized = normalize_host_measurement_evidence(
        observation,
        evidence_generation,
        normalization_context,
    )
    .map_err(UiHostMeasurementEvidenceDenial::Normalization)?;

    let freshness_witness = UiHostMeasurementFreshnessWitness::new(
        evidence_generation,
        normalization_context.assumption_profile(),
    );
    admit_current_host_measurement_evidence(&normalized, freshness_witness).map_err(|denial| {
        match denial {
            UiHostMeasurementEvidenceDenial::Stale(stale) => {
                UiHostMeasurementEvidenceDenial::Stale(stale)
            }
            UiHostMeasurementEvidenceDenial::Execution(execution) => {
                UiHostMeasurementEvidenceDenial::Execution(execution)
            }
            UiHostMeasurementEvidenceDenial::Normalization(normalization) => {
                UiHostMeasurementEvidenceDenial::Normalization(normalization)
            }
        }
    })?;
    Ok(normalized)
}

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
}

pub fn admit_current_host_measurement_evidence<'a>(
    result: &'a UiMeasurementResult,
    freshness_witness: UiHostMeasurementFreshnessWitness,
) -> Result<UiCurrentMeasurementResult<'a>, UiHostMeasurementEvidenceDenial> {
    invalidate_stale_host_measurement_evidence(
        result,
        freshness_witness.current_generation,
        freshness_witness.current_profile,
    )
    .map_err(UiHostMeasurementEvidenceDenial::Stale)?;
    Ok(UiCurrentMeasurementResult::new(result))
}
