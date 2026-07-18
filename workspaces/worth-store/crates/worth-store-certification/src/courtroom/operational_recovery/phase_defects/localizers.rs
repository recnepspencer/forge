use sha2::{Digest, Sha256};
use worth_store_operations::ControlStoreSelectionIndeterminate;
use worth_store_physical_certification::{
    OperationalRecoveryDriverTrace, PhysicalCertificationEvidenceBundle,
};

use super::{S10PhaseDefectDenial, S10PhaseDefectLocalization, S10PhaseDefectSourceKind};
use crate::courtroom::operational_recovery::phase_invocation::control_generation_identity;
use crate::courtroom::operational_recovery::scenario_audit_binding::require_audits_from_control_history;
use crate::courtroom::operational_recovery::scenario_counter_binding::require_operation_counter_bindings;
use crate::courtroom::operational_recovery::scenario_identity::phase_19_join_identity;
use crate::courtroom::operational_recovery::scenario_trace_binding::require_production_observation_identities;
use crate::courtroom::operational_recovery::{
    S10OperationalScenarioEvidence, S10Phase, S10PhaseInvocationEvidence,
    S10ScenarioProductionEvidence,
};

mod runtime_record_omission;
pub use runtime_record_omission::localize_s10_runtime_record_omission;

pub fn localize_s10_control_selection_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
    denial: &ControlStoreSelectionIndeterminate,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let invocation = phase_invocation(scenario, 2)?;
    let members = invocation.localization_members();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-control-selection-defect-v1");
    match denial {
        ControlStoreSelectionIndeterminate::SelectedMediaUnavailable {
            media_identity_fingerprint,
        } if members.contains(media_identity_fingerprint) => {
            digest.update([1]);
            digest.update(media_identity_fingerprint);
        }
        ControlStoreSelectionIndeterminate::SelectedGenerationNotReadable {
            selected,
            observed,
        } if members.contains(&control_generation_identity(selected.get())) => {
            digest.update([2]);
            digest.update(selected.get().to_be_bytes());
            digest.update(
                observed
                    .map_or(0, |generation| generation.get())
                    .to_be_bytes(),
            );
        }
        ControlStoreSelectionIndeterminate::SelectedPrefixDigestMismatch { selected, observed }
            if members.contains(selected) || members.contains(observed) =>
        {
            digest.update([3]);
            digest.update(selected);
            digest.update(observed);
        }
        ControlStoreSelectionIndeterminate::SelectedAuthorityMismatch { selected, observed }
            if members.contains(selected) =>
        {
            digest.update([4]);
            digest.update(selected);
            digest.update(observed);
        }
        ControlStoreSelectionIndeterminate::SelectedMediaCopiesDivergent
        | ControlStoreSelectionIndeterminate::InvalidHistory(_) => {
            return Err(S10PhaseDefectDenial::ControlSelectionNotLocalizable)
        }
        _ => return Err(S10PhaseDefectDenial::ControlSelectionNotBound),
    }
    Ok(localization(
        scenario,
        invocation,
        S10PhaseDefectSourceKind::ControlSelection,
        digest.finalize().into(),
        1,
    ))
}

pub fn localize_s10_observation_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
    mutant: &PhysicalCertificationEvidenceBundle,
    defective_trace: &OperationalRecoveryDriverTrace,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let rejection = failed_physical_mutant(scenario, mutant)?;
    require_distinct_trace(scenario, defective_trace)?;
    let clean = scenario.driver_trace();
    let inspection_changed =
        defective_trace.inspection_evidence_identity() != clean.inspection_evidence_identity();
    let truth_changed =
        defective_trace.truth_evidence_identity() != clean.truth_evidence_identity();
    let phase = match (inspection_changed, truth_changed) {
        (true, false) => 3,
        (false, true) => 4,
        _ => return Err(S10PhaseDefectDenial::ObservationDefectAmbiguous),
    };
    Ok(localization(
        scenario,
        phase_invocation(scenario, phase)?,
        S10PhaseDefectSourceKind::IndependentInspection,
        rejection,
        failed_oracle_count(mutant),
    ))
}

pub fn localize_s10_observation_join_omission(
    scenario: &S10OperationalScenarioEvidence,
    production: S10ScenarioProductionEvidence<'_>,
    phase: u8,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let invocation = phase_invocation(scenario, phase)?;
    let trace = scenario.driver_trace();
    let (inspection, semantic_truth, omitted) = match phase {
        3 => (
            None,
            trace.truth_evidence_identity(),
            production.truth().source_inspection_identity(),
        ),
        4 => (
            trace.inspection_evidence_identity(),
            None,
            production.truth().truth_evidence_identity(),
        ),
        _ => return Err(S10PhaseDefectDenial::PhaseNotInvoked),
    };
    if require_production_observation_identities(production, inspection, semantic_truth).is_ok() {
        return Err(S10PhaseDefectDenial::ObservationDefectAmbiguous);
    }
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-observation-join-omission-v1");
    digest.update([phase]);
    digest.update(omitted);
    Ok(localization(
        scenario,
        invocation,
        S10PhaseDefectSourceKind::IndependentInspection,
        digest.finalize().into(),
        1,
    ))
}

pub fn localize_s10_runtime_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
    mutant: &PhysicalCertificationEvidenceBundle,
    defective_trace: &OperationalRecoveryDriverTrace,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let rejection = failed_physical_mutant(scenario, mutant)?;
    require_distinct_trace(scenario, defective_trace)?;
    let clean_artifacts = scenario.driver_trace().control_artifact_identities();
    let defective_artifacts = defective_trace.control_artifact_identities();
    let affected = scenario
        .phase_invocations()
        .iter()
        .filter(|invocation| (5..=14).contains(&invocation.phase().number()))
        .filter(|invocation| !invocation.localization_members().is_empty())
        .filter(|invocation| {
            invocation
                .localization_members()
                .iter()
                .all(|identity| clean_artifacts.contains(identity))
                && invocation
                    .localization_members()
                    .iter()
                    .any(|identity| !defective_artifacts.contains(identity))
        })
        .collect::<Vec<_>>();
    let invocation = match affected.as_slice() {
        [] => return Err(S10PhaseDefectDenial::RuntimeArtifactDefectMissing),
        [invocation] => *invocation,
        _ => return Err(S10PhaseDefectDenial::RuntimeArtifactDefectAmbiguous),
    };
    Ok(localization(
        scenario,
        invocation,
        S10PhaseDefectSourceKind::RuntimeArtifactOmission,
        rejection,
        failed_oracle_count(mutant),
    ))
}

pub fn localize_s10_audit_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
    production: S10ScenarioProductionEvidence<'_>,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    require_audits_from_control_history(production, scenario.audits())
        .map_err(|_| S10PhaseDefectDenial::ProductionAuditMismatch)?;
    let mut omitted = scenario.audits().to_vec();
    let removed = omitted
        .pop()
        .ok_or(S10PhaseDefectDenial::AuditEvidenceEmpty)?;
    if require_audits_from_control_history(production, &omitted).is_ok() {
        return Err(S10PhaseDefectDenial::AuditOmissionWasAccepted);
    }
    Ok(localization(
        scenario,
        phase_invocation(scenario, 15)?,
        S10PhaseDefectSourceKind::AuditOmission,
        removed.terminal_record_identity(),
        1,
    ))
}

pub fn localize_s10_harness_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
    mutant: &PhysicalCertificationEvidenceBundle,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let rejection = failed_physical_mutant(scenario, mutant)?;
    Ok(localization(
        scenario,
        phase_invocation(scenario, 16)?,
        S10PhaseDefectSourceKind::PhysicalHarness,
        rejection,
        failed_oracle_count(mutant),
    ))
}

pub fn localize_s10_harness_join_omission(
    scenario: &S10OperationalScenarioEvidence,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let denial = crate::courtroom::operational_recovery::S10ScenarioExecutionMatrix::join(
        Vec::<PhysicalCertificationEvidenceBundle>::new(),
        [scenario.driver_trace().clone()],
    )
    .expect_err("a scenario matrix without physical executions must fail closed");
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-harness-join-omission-v1");
    digest.update(format!("{denial:?}").as_bytes());
    digest.update(scenario.execution_matrix().matrix_identity());
    Ok(localization(
        scenario,
        phase_invocation(scenario, 16)?,
        S10PhaseDefectSourceKind::PhysicalHarness,
        digest.finalize().into(),
        1,
    ))
}

pub fn localize_s10_formal_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let suite = scenario.mutation_sensitivity();
    Ok(localization(
        scenario,
        phase_invocation(scenario, 17)?,
        S10PhaseDefectSourceKind::FormalMutation,
        suite.suite_identity(),
        suite.receipts().len() as u64,
    ))
}

pub fn localize_s10_counter_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let mut omitted = scenario.counters().to_vec();
    let removed = omitted
        .pop()
        .ok_or(S10PhaseDefectDenial::CounterEvidenceEmpty)?;
    if require_operation_counter_bindings(scenario.refinement(), scenario.driver_trace(), &omitted)
        .is_ok()
    {
        return Err(S10PhaseDefectDenial::CounterOmissionWasAccepted);
    }
    Ok(localization(
        scenario,
        phase_invocation(scenario, 18)?,
        S10PhaseDefectSourceKind::CounterOmission,
        removed.session().fingerprint(),
        1,
    ))
}

pub fn localize_s10_closeout_join_phase_defect(
    scenario: &S10OperationalScenarioEvidence,
) -> Result<S10PhaseDefectLocalization, S10PhaseDefectDenial> {
    let components = [15_u8, 16, 17, 18].map(|phase| {
        phase_invocation(scenario, phase)
            .map(S10PhaseInvocationEvidence::production_artifact_identity)
    });
    let components: [[u8; 32]; 4] = components
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .expect("four phase components");
    let mut omitted = components.map(Some);
    omitted[2] = None;
    let denial = phase_19_join_identity(scenario.program(), omitted)
        .expect_err("a missing Phase 19 component must deny the join");
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-closeout-join-controlled-omission-v1");
    digest.update((denial.0 as u64).to_be_bytes());
    digest.update(components[2]);
    Ok(localization(
        scenario,
        phase_invocation(scenario, 19)?,
        S10PhaseDefectSourceKind::CloseoutJoinOmission,
        digest.finalize().into(),
        1,
    ))
}

pub(super) fn phase_invocation(
    scenario: &S10OperationalScenarioEvidence,
    phase: u8,
) -> Result<&S10PhaseInvocationEvidence, S10PhaseDefectDenial> {
    scenario
        .phase_invocations()
        .iter()
        .find(|invocation| invocation.phase() == S10Phase(phase))
        .ok_or(S10PhaseDefectDenial::PhaseNotInvoked)
}

fn failed_physical_mutant(
    scenario: &S10OperationalScenarioEvidence,
    mutant: &PhysicalCertificationEvidenceBundle,
) -> Result<[u8; 32], S10PhaseDefectDenial> {
    let failure = mutant
        .failure_digest()
        .ok_or(S10PhaseDefectDenial::MutantDidNotFailIndependentOracle)?;
    if mutant.replay().fault_events().is_empty() {
        return Err(S10PhaseDefectDenial::MutantDeliveredNoFault);
    }
    if mutant.primary().scenario_digest() != scenario.physical().primary().scenario_digest() {
        return Err(S10PhaseDefectDenial::MutantScenarioMismatch);
    }
    let rejection = *failure.transcript_digest();
    if scenario
        .execution_matrix()
        .physical_runs()
        .iter()
        .any(|run| *run.primary().transcript_digest() == rejection)
    {
        return Err(S10PhaseDefectDenial::MutantReusedCleanTranscript);
    }
    Ok(rejection)
}

fn require_distinct_trace(
    scenario: &S10OperationalScenarioEvidence,
    defective: &OperationalRecoveryDriverTrace,
) -> Result<(), S10PhaseDefectDenial> {
    if defective.evidence_identity() == scenario.driver_trace().evidence_identity() {
        return Err(S10PhaseDefectDenial::DefectiveTraceReusedCleanTrace);
    }
    Ok(())
}

fn failed_oracle_count(mutant: &PhysicalCertificationEvidenceBundle) -> u64 {
    mutant
        .failure_digest()
        .map_or(0, |failure| failure.failed_oracle_count() as u64)
}

pub(super) fn localization(
    scenario: &S10OperationalScenarioEvidence,
    invocation: &S10PhaseInvocationEvidence,
    source_kind: S10PhaseDefectSourceKind,
    rejection_identity: [u8; 32],
    failed_check_count: u64,
) -> S10PhaseDefectLocalization {
    let phase = invocation.phase();
    let phase_artifact_identity = invocation.production_artifact_identity();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-defect-localization-v2");
    digest.update([phase.number(), source_kind as u8]);
    digest.update(scenario.evidence_identity());
    digest.update(phase_artifact_identity);
    digest.update(rejection_identity);
    digest.update(failed_check_count.to_be_bytes());
    S10PhaseDefectLocalization {
        phase,
        source_kind,
        scenario_identity: scenario.evidence_identity(),
        phase_artifact_identity,
        rejection_identity,
        failed_check_count,
        localization_identity: digest.finalize().into(),
    }
}
