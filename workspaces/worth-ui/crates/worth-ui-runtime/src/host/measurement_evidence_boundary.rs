use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::evidence::UiMeasurementResult;

use super::{
    admit_fresh_host_evidence, construct_freshness_witness, normalize_host_observation,
    observe_host_measurement, UiHostMeasurementEvidenceDenial, UiHostMeasurementFreshnessWitness,
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
    let observation = observe_host_measurement(
        adapter,
        identity,
        evidence_family,
        need,
        capability_report,
    )?;
    let normalized = normalize_host_observation(
        observation,
        evidence_generation,
        normalization_context,
    )?;
    let freshness_witness = construct_freshness_witness(
        evidence_generation,
        normalization_context.assumption_profile(),
    );
    admit_fresh_host_evidence(&normalized, freshness_witness)?;
    Ok(normalized)
}

pub fn admit_current_host_measurement_evidence<'a>(
    result: &'a UiMeasurementResult,
    freshness_witness: UiHostMeasurementFreshnessWitness,
) -> Result<crate::evidence::UiCurrentMeasurementResult<'a>, UiHostMeasurementEvidenceDenial> {
    admit_fresh_host_evidence(result, freshness_witness)
}