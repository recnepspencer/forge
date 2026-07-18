use worth_store_physical_certification::{
    OperationalRecoveryControlTransitionKind, OperationalRecoveryDriverTrace,
    OperationalRecoveryYieldpoint,
};

use super::{S10ScenarioCertificationDenial, S10ScenarioProductionEvidence};

pub(super) fn require_production_trace_binding(
    production: S10ScenarioProductionEvidence<'_>,
    trace: &OperationalRecoveryDriverTrace,
) -> Result<(), S10ScenarioCertificationDenial> {
    require_production_observation_identities(
        production,
        trace.inspection_evidence_identity(),
        trace.truth_evidence_identity(),
    )?;
    for record in production.control_records().iter().filter(|record| {
        OperationalRecoveryControlTransitionKind::from_record(record.kind()).is_some()
    }) {
        let identity = record.stable_fingerprint();
        if !trace.control_artifact_identities().contains(&identity) {
            return Err(S10ScenarioCertificationDenial::MissingDriverControlArtifact(identity));
        }
    }
    for kind in OperationalRecoveryControlTransitionKind::ALL {
        let expected = production
            .control_records()
            .iter()
            .filter(|record| kind.matches(record.kind()))
            .count();
        let before = trace
            .reached()
            .iter()
            .filter(|point| {
                **point == OperationalRecoveryYieldpoint::BeforeDurableControlTransition(kind)
            })
            .count();
        let after = trace
            .reached()
            .iter()
            .filter(|point| {
                **point == OperationalRecoveryYieldpoint::AfterDurableControlTransition(kind)
            })
            .count();
        if before < expected || after < expected {
            return Err(S10ScenarioCertificationDenial::DriverControlTransitionCountMismatch(kind));
        }
    }
    Ok(())
}

pub(super) fn require_production_observation_identities(
    production: S10ScenarioProductionEvidence<'_>,
    inspection: Option<[u8; 32]>,
    semantic_truth: Option<[u8; 32]>,
) -> Result<(), S10ScenarioCertificationDenial> {
    let truth = production.truth();
    if inspection != Some(truth.source_inspection_identity()) {
        return Err(S10ScenarioCertificationDenial::DriverInspectionEvidenceMismatch);
    }
    if semantic_truth != Some(truth.truth_evidence_identity()) {
        return Err(S10ScenarioCertificationDenial::DriverTruthEvidenceMismatch);
    }
    Ok(())
}
