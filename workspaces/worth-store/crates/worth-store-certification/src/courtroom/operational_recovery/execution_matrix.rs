use std::collections::BTreeSet;

use sha2::{Digest, Sha256};
use worth_store_physical_certification::{
    OperationalRecoveryDriverTrace, OperationalRecoveryTraceJoinDenial,
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
}

#[derive(Debug, Clone)]
pub struct S10ScenarioExecutionMatrix {
    physical_runs: Vec<PhysicalCertificationEvidenceBundle>,
    driver_trace: OperationalRecoveryDriverTrace,
    matrix_identity: [u8; 32],
}

impl S10ScenarioExecutionMatrix {
    pub fn join(
        physical_runs: impl IntoIterator<Item = PhysicalCertificationEvidenceBundle>,
        driver_traces: impl IntoIterator<Item = OperationalRecoveryDriverTrace>,
    ) -> Result<Self, S10ScenarioExecutionMatrixDenial> {
        let physical_runs = physical_runs.into_iter().collect::<Vec<_>>();
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
        let trace = OperationalRecoveryDriverTrace::join(driver_traces)
            .map_err(S10ScenarioExecutionMatrixDenial::DriverTrace)?;
        if trace.reached().is_empty() {
            return Err(S10ScenarioExecutionMatrixDenial::DriverEvidenceIncomplete);
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-scenario-execution-matrix-v1");
        digest.update(scenario);
        digest.update(fixture);
        for transcript in &transcripts {
            digest.update(transcript);
        }
        digest.update(trace.evidence_identity());
        Ok(Self {
            physical_runs,
            driver_trace: trace,
            matrix_identity: digest.finalize().into(),
        })
    }

    pub fn physical_runs(&self) -> &[PhysicalCertificationEvidenceBundle] {
        &self.physical_runs
    }
    pub fn primary(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.physical_runs[0]
    }
    pub const fn driver_trace(&self) -> &OperationalRecoveryDriverTrace {
        &self.driver_trace
    }
    pub fn schedules_executed(&self) -> u64 {
        self.physical_runs.len() as u64
    }
    pub const fn matrix_identity(&self) -> [u8; 32] {
        self.matrix_identity
    }
}
