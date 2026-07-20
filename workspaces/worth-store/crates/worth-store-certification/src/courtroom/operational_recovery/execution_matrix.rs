use std::collections::BTreeSet;

use super::S10OperationalScenarioKind;
use sha2::{Digest, Sha256};
use worth_store_physical_certification::{
    OperationalRecoveryCrashCutEvidence, OperationalRecoveryDriverTrace,
    OperationalRecoveryTraceJoinDenial, OperationalRecoveryYieldpoint,
    PhysicalCertificationEvidenceBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10ScenarioExecutionMatrixDenial {
    Empty,
    ScenarioMismatch,
    PhysicalProfileMismatch,
    FixtureMismatch,
    DuplicateScheduleTranscript,
    PhysicalEvidenceIncomplete,
    DriverEvidenceIncomplete,
    DriverTrace(OperationalRecoveryTraceJoinDenial),
    ControlledDefectDidNotFail,
    ControlledDefectMissingFault,
    ControlledDefectScenarioMismatch,
    ControlledDefectFixtureMismatch,
    DuplicateControlledDefect,
    DuplicateCrashCut(OperationalRecoveryYieldpoint),
    CrashCutOperationMismatch(OperationalRecoveryYieldpoint),
}

#[derive(Debug, Clone)]
pub struct S10ScenarioExecutionMatrix {
    scenario_kind: Option<S10OperationalScenarioKind>,
    physical_runs: Vec<PhysicalCertificationEvidenceBundle>,
    controlled_defects: Vec<PhysicalCertificationEvidenceBundle>,
    driver_trace: OperationalRecoveryDriverTrace,
    crash_reopen_coverage: Vec<OperationalRecoveryCrashCutEvidence>,
    matrix_identity: [u8; 32],
}

impl S10ScenarioExecutionMatrix {
    pub fn join(
        physical_runs: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        driver_traces: impl IntoIterator<Item = OperationalRecoveryDriverTrace>,
    ) -> Result<Self, S10ScenarioExecutionMatrixDenial> {
        Self::join_with_controlled_defects(physical_runs, driver_traces, [])
    }

    pub fn join_with_controlled_defects(
        physical_runs: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        driver_traces: impl IntoIterator<Item = OperationalRecoveryDriverTrace>,
        controlled_defects: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
    ) -> Result<Self, S10ScenarioExecutionMatrixDenial> {
        Self::join_for_optional_scenario(None, physical_runs, driver_traces, controlled_defects, [])
    }

    pub fn join_for_scenario(
        scenario_kind: S10OperationalScenarioKind,
        physical_runs: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        driver_traces: impl IntoIterator<Item = OperationalRecoveryDriverTrace>,
        controlled_defects: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
    ) -> Result<Self, S10ScenarioExecutionMatrixDenial> {
        Self::join_for_optional_scenario(
            Some(scenario_kind),
            physical_runs,
            driver_traces,
            controlled_defects,
            [],
        )
    }

    pub fn join_for_scenario_with_crash_coverage(
        scenario_kind: S10OperationalScenarioKind,
        physical_runs: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        driver_traces: impl IntoIterator<Item = OperationalRecoveryDriverTrace>,
        controlled_defects: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        crash_reopen_coverage: impl IntoIterator<Item = OperationalRecoveryCrashCutEvidence>,
    ) -> Result<Self, S10ScenarioExecutionMatrixDenial> {
        Self::join_for_optional_scenario(
            Some(scenario_kind),
            physical_runs,
            driver_traces,
            controlled_defects,
            crash_reopen_coverage,
        )
    }

    fn join_for_optional_scenario(
        scenario_kind: Option<S10OperationalScenarioKind>,
        physical_runs: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        driver_traces: impl IntoIterator<Item = OperationalRecoveryDriverTrace>,
        controlled_defects: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        crash_reopen_coverage: impl IntoIterator<Item = OperationalRecoveryCrashCutEvidence>,
    ) -> Result<Self, S10ScenarioExecutionMatrixDenial> {
        let physical_runs = physical_runs.into_iter().collect::<Vec<_>>();
        let controlled_defects = controlled_defects.into_iter().collect::<Vec<_>>();
        let Some(first_physical) = physical_runs.first() else {
            return Err(S10ScenarioExecutionMatrixDenial::Empty);
        };
        let scenario = *first_physical.primary().scenario_digest();
        let profile = first_physical.replay().plan().profile();
        let fixture = first_physical
            .replay()
            .fixture_manifest()
            .evidence_identity();
        let mut transcripts = BTreeSet::new();
        for physical in &physical_runs {
            let primary = physical.primary();
            if *primary.scenario_digest() != scenario {
                return Err(S10ScenarioExecutionMatrixDenial::ScenarioMismatch);
            }
            if physical.replay().plan().profile() != profile {
                return Err(S10ScenarioExecutionMatrixDenial::PhysicalProfileMismatch);
            }
            if physical.replay().fixture_manifest().evidence_identity() != fixture {
                return Err(S10ScenarioExecutionMatrixDenial::FixtureMismatch);
            }
            if primary.oracle_verdict_count() == 0 || primary.counter_row_count() == 0 {
                return Err(S10ScenarioExecutionMatrixDenial::PhysicalEvidenceIncomplete);
            }
            if !transcripts.insert(*primary.transcript_digest()) {
                return Err(S10ScenarioExecutionMatrixDenial::DuplicateScheduleTranscript);
            }
        }
        let mut defect_transcripts = BTreeSet::new();
        for defect in &controlled_defects {
            if defect.failure_digest().is_none() {
                return Err(S10ScenarioExecutionMatrixDenial::ControlledDefectDidNotFail);
            }
            if defect.replay().fault_events().is_empty() {
                return Err(S10ScenarioExecutionMatrixDenial::ControlledDefectMissingFault);
            }
            if *defect.primary().scenario_digest() != scenario {
                return Err(S10ScenarioExecutionMatrixDenial::ControlledDefectScenarioMismatch);
            }
            if defect.replay().fixture_manifest().evidence_identity() != fixture {
                return Err(S10ScenarioExecutionMatrixDenial::ControlledDefectFixtureMismatch);
            }
            let identity = *defect.primary().transcript_digest();
            if transcripts.contains(&identity) || !defect_transcripts.insert(identity) {
                return Err(S10ScenarioExecutionMatrixDenial::DuplicateControlledDefect);
            }
        }
        let trace = OperationalRecoveryDriverTrace::join(driver_traces)
            .map_err(S10ScenarioExecutionMatrixDenial::DriverTrace)?;
        if trace.reached().is_empty() {
            return Err(S10ScenarioExecutionMatrixDenial::DriverEvidenceIncomplete);
        }
        let mut crash_points = BTreeSet::new();
        let mut crash_reopen_coverage = crash_reopen_coverage.into_iter().collect::<Vec<_>>();
        for evidence in &crash_reopen_coverage {
            if !crash_points.insert(evidence.yieldpoint()) {
                return Err(S10ScenarioExecutionMatrixDenial::DuplicateCrashCut(
                    evidence.yieldpoint(),
                ));
            }
            if evidence
                .operation_identities()
                .iter()
                .any(|operation| !trace.operation_identities().contains(operation))
            {
                return Err(S10ScenarioExecutionMatrixDenial::CrashCutOperationMismatch(
                    evidence.yieldpoint(),
                ));
            }
        }
        crash_reopen_coverage.sort_by_key(OperationalRecoveryCrashCutEvidence::yieldpoint);
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-scenario-execution-matrix-v1");
        digest.update([scenario_kind.map_or(0, |kind| kind as u8 + 1)]);
        digest.update(scenario);
        digest.update(fixture);
        for transcript in &transcripts {
            digest.update(transcript);
        }
        for transcript in &defect_transcripts {
            digest.update(transcript);
        }
        digest.update(trace.evidence_identity());
        for evidence in &crash_reopen_coverage {
            digest.update(evidence.evidence_identity());
        }
        Ok(Self {
            scenario_kind,
            physical_runs,
            controlled_defects,
            driver_trace: trace,
            crash_reopen_coverage,
            matrix_identity: digest.finalize().into(),
        })
    }

    pub fn physical_runs(&self) -> &[PhysicalCertificationEvidenceBundle] {
        &self.physical_runs
    }
    pub fn controlled_defects(&self) -> &[PhysicalCertificationEvidenceBundle] {
        &self.controlled_defects
    }
    pub const fn scenario_kind(&self) -> Option<S10OperationalScenarioKind> {
        self.scenario_kind
    }
    pub fn primary(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.physical_runs[0]
    }
    pub const fn driver_trace(&self) -> &OperationalRecoveryDriverTrace {
        &self.driver_trace
    }
    pub fn crash_reopen_coverage(&self) -> &[OperationalRecoveryCrashCutEvidence] {
        &self.crash_reopen_coverage
    }
    pub fn schedules_executed(&self) -> u64 {
        self.physical_runs.len() as u64
    }
    pub const fn matrix_identity(&self) -> [u8; 32] {
        self.matrix_identity
    }
}
