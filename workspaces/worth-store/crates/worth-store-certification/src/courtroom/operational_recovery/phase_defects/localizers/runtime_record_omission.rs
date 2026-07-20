use sha2::{Digest, Sha256};

use super::{localization, phase_invocation};
use crate::courtroom::operational_recovery::phase_invocation::require_runtime_phase_artifact;
use crate::courtroom::operational_recovery::{
    S10OperationalScenarioEvidence, S10PhaseDefectDenial, S10PhaseDefectLocalization,
    S10PhaseDefectSourceKind, S10PhaseInvocationEvidence, S10ScenarioProductionEvidence,
};

pub fn localize_s10_runtime_record_omission(
    scenario: &S10OperationalScenarioEvidence,
    production: S10ScenarioProductionEvidence<'_>,
    phase: u8,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let invocation = phase_invocation(scenario, phase)?;
    let mut retained = production.control_records().to_vec();
    let mut removed = Vec::new();
    let candidates = phase_record_candidates(invocation, &retained);
    for identity in candidates {
        let Some(index) = retained
            .iter()
            .position(|record| record.stable_fingerprint() == identity)
        else {
            continue;
        };
        removed.push(retained.remove(index).stable_fingerprint());
        if require_runtime_phase_artifact(scenario.program().kind(), phase, &retained).is_err() {
            let mut digest = Sha256::new();
            digest.update(b"worth-store-s10-runtime-record-omission-v1");
            digest.update([phase]);
            for identity in &removed {
                digest.update(identity);
            }
            return Ok(localization(
                scenario,
                invocation,
                S10PhaseDefectSourceKind::RuntimeArtifactOmission,
                digest.finalize().into(),
                removed.len() as u64,
            ));
        }
    }
    Err(S10PhaseDefectDenial::RuntimeArtifactDefectMissing)
}

fn phase_record_candidates(
    invocation: &S10PhaseInvocationEvidence,
    records: &[worth_store_operations::OperationalControlRecord],
) -> Vec<[u8; 32]> {
    let mut candidates = invocation.localization_members().to_vec();
    for record in records {
        let identity = record.stable_fingerprint();
        if !candidates.contains(&identity) {
            candidates.push(identity);
        }
    }
    candidates
}
