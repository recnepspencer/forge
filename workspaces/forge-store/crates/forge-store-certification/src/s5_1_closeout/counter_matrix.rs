use forge_store_physical_certification::{
    S51SecurityScopeFailureKind, S51SecurityScopeHarnessEvidence, S51SecurityScopeHarnessSchedule,
    S51SecurityScopeReplayMutationKind,
};

use super::{S51CertificationCloseoutDenial, S51CertificationCloseoutInput};

const REQUIRED_FAILURES: [S51SecurityScopeFailureKind; 6] = [
    S51SecurityScopeFailureKind::MetadataPreserved,
    S51SecurityScopeFailureKind::PhysicalScopeDrift,
    S51SecurityScopeFailureKind::StaleKeyPosture,
    S51SecurityScopeFailureKind::WrongTenantScope,
    S51SecurityScopeFailureKind::MissingAuthenticityRequirement,
    S51SecurityScopeFailureKind::ReplayedCustodyPosture,
];

const REQUIRED_SCHEDULES: [S51SecurityScopeHarnessSchedule; 4] = [
    S51SecurityScopeHarnessSchedule::StableReadPlanAdmission,
    S51SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
    S51SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
    S51SecurityScopeHarnessSchedule::RepairReadAdmission,
];

const REQUIRED_REPLAY_MUTATIONS: [S51SecurityScopeReplayMutationKind; 3] = [
    S51SecurityScopeReplayMutationKind::ChangedTenantScope,
    S51SecurityScopeReplayMutationKind::ChangedKeyVersionPosture,
    S51SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement,
];

const WITNESSES_PER_READINESS_ACCEPTANCE: u64 = 4;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct S51CloseoutCounterMatrix {
    scenario_evidence_rows: u64,
    replay_transcripts: u64,
    scenario_lower_store_requests: u64,
    scenario_lower_store_current_authority_checks: u64,
    scenario_lower_store_witness_sets_issued: u64,
    scenario_lower_store_denials: u64,
    replay_lower_store_requests: u64,
    replay_lower_store_current_authority_checks: u64,
    replay_lower_store_witness_sets_issued: u64,
    replay_lower_store_denials: u64,
    physical_scope_drift: u64,
    stale_key_posture: u64,
    wrong_tenant_scope: u64,
    missing_authenticity_requirement: u64,
    replayed_custody_posture: u64,
    replay_wrong_tenant_scope: u64,
    replay_stale_key_posture: u64,
    replay_missing_authenticity_requirement: u64,
    replay_baseline_admissions: u64,
    replay_attempts: u64,
    replay_denials_before_logical_decode: u64,
    handoff_attempts: u64,
    handoff_admitted: u64,
}

impl S51CloseoutCounterMatrix {
    pub fn from_input(
        input: &S51CertificationCloseoutInput,
    ) -> Result<Self, S51CertificationCloseoutDenial> {
        require_scenario_families(input.scenario_evidence())?;
        require_replay_transcripts(input)?;

        let mut matrix = Self::default();
        for evidence in input.scenario_evidence() {
            crosscheck_lower_store_counters(*evidence)?;
            matrix.record_evidence(*evidence);
        }
        for transcript in input.replay_transcripts() {
            if !transcript.replays_same_physical_schedule() {
                return Err(
                    S51CertificationCloseoutDenial::ReplayTranscriptNotSamePhysicalSchedule,
                );
            }
            if !transcript.replay_rejected_before_logical_decode() {
                return Err(
                    S51CertificationCloseoutDenial::ReplayTranscriptDidNotDenyBeforeLogicalDecode,
                );
            }
            crosscheck_lower_store_counters(transcript.baseline_evidence())?;
            crosscheck_lower_store_counters(transcript.replay_evidence())?;
            matrix.record_replay_evidence(transcript.baseline_evidence());
            matrix.record_replay_evidence(transcript.replay_evidence());
            matrix.replay_transcripts += 1;
            matrix.replay_baseline_admissions += transcript.counters().baseline_admissions();
            matrix.replay_attempts += transcript.counters().replay_attempts();
            matrix.replay_denials_before_logical_decode +=
                transcript.counters().replay_denials_before_logical_decode();
        }

        let scope_counters = input.security_scope().receipt().counters();
        matrix.handoff_attempts = scope_counters.requests();
        matrix.handoff_admitted = scope_counters.witnesses_issued();
        expect(
            "s5_1.closeout.handoff.denied",
            0,
            scope_counters.denials(),
        )?;
        expect(
            "s5_1.closeout.handoff.unsupported",
            0,
            scope_counters.unsupported_postures(),
        )?;
        expect(
            "s5_1.closeout.handoff.unavailable",
            0,
            scope_counters.unavailable_postures(),
        )?;
        Ok(matrix)
    }

    pub const fn scenario_evidence_rows(self) -> u64 {
        self.scenario_evidence_rows
    }

    pub const fn replay_transcripts(self) -> u64 {
        self.replay_transcripts
    }

    pub const fn lower_store_requests(self) -> u64 {
        self.scenario_lower_store_requests + self.replay_lower_store_requests
    }

    pub const fn lower_store_current_authority_checks(self) -> u64 {
        self.scenario_lower_store_current_authority_checks
            + self.replay_lower_store_current_authority_checks
    }

    pub const fn lower_store_witness_sets_issued(self) -> u64 {
        self.scenario_lower_store_witness_sets_issued + self.replay_lower_store_witness_sets_issued
    }

    pub const fn lower_store_denials(self) -> u64 {
        self.scenario_lower_store_denials + self.replay_lower_store_denials
    }

    pub const fn scenario_lower_store_requests(self) -> u64 {
        self.scenario_lower_store_requests
    }

    pub const fn replay_lower_store_requests(self) -> u64 {
        self.replay_lower_store_requests
    }

    pub const fn physical_scope_drift(self) -> u64 {
        self.physical_scope_drift
    }

    pub const fn stale_key_posture(self) -> u64 {
        self.stale_key_posture
    }

    pub const fn wrong_tenant_scope(self) -> u64 {
        self.wrong_tenant_scope
    }

    pub const fn missing_authenticity_requirement(self) -> u64 {
        self.missing_authenticity_requirement
    }

    pub const fn replayed_custody_posture(self) -> u64 {
        self.replayed_custody_posture
    }

    pub const fn replay_wrong_tenant_scope(self) -> u64 {
        self.replay_wrong_tenant_scope
    }

    pub const fn replay_stale_key_posture(self) -> u64 {
        self.replay_stale_key_posture
    }

    pub const fn replay_missing_authenticity_requirement(self) -> u64 {
        self.replay_missing_authenticity_requirement
    }

    pub const fn replay_baseline_admissions(self) -> u64 {
        self.replay_baseline_admissions
    }

    pub const fn replay_attempts(self) -> u64 {
        self.replay_attempts
    }

    pub const fn replay_denials_before_logical_decode(self) -> u64 {
        self.replay_denials_before_logical_decode
    }

    pub const fn handoff_attempts(self) -> u64 {
        self.handoff_attempts
    }

    pub const fn handoff_admitted(self) -> u64 {
        self.handoff_admitted
    }

    pub const fn consumed_lower_store_evidence_rows(self) -> u64 {
        self.scenario_evidence_rows + (self.replay_transcripts * 2)
    }

    fn record_evidence(&mut self, evidence: S51SecurityScopeHarnessEvidence) {
        let lower = evidence.lower_store_admission_counters();
        self.scenario_evidence_rows += 1;
        self.scenario_lower_store_requests += lower.requests();
        self.scenario_lower_store_current_authority_checks += lower.current_authority_checks();
        self.scenario_lower_store_witness_sets_issued +=
            exact_witness_sets_issued_after_crosscheck(evidence);
        self.scenario_lower_store_denials += lower.denials();
        self.physical_scope_drift += evidence.counters().physical_scope_drift();
        self.stale_key_posture += evidence.counters().stale_key_posture();
        self.wrong_tenant_scope += evidence.counters().wrong_tenant_scope();
        self.missing_authenticity_requirement +=
            evidence.counters().missing_authenticity_requirement();
        self.replayed_custody_posture += evidence.counters().replayed_custody_posture();
    }

    fn record_replay_evidence(&mut self, evidence: S51SecurityScopeHarnessEvidence) {
        let lower = evidence.lower_store_admission_counters();
        self.replay_lower_store_requests += lower.requests();
        self.replay_lower_store_current_authority_checks += lower.current_authority_checks();
        self.replay_lower_store_witness_sets_issued +=
            exact_witness_sets_issued_after_crosscheck(evidence);
        self.replay_lower_store_denials += lower.denials();
        self.replay_wrong_tenant_scope += evidence.counters().wrong_tenant_scope();
        self.replay_stale_key_posture += evidence.counters().stale_key_posture();
        self.replay_missing_authenticity_requirement +=
            evidence.counters().missing_authenticity_requirement();
    }
}

fn require_scenario_families(
    evidence: &[S51SecurityScopeHarnessEvidence],
) -> Result<(), S51CertificationCloseoutDenial> {
    for failure in REQUIRED_FAILURES {
        if !evidence
            .iter()
            .any(|row| row.scenario().failure_kind() == failure)
        {
            return Err(S51CertificationCloseoutDenial::MissingScenarioEvidence(
                failure,
            ));
        }
    }
    Ok(())
}

fn require_replay_transcripts(
    input: &S51CertificationCloseoutInput,
) -> Result<(), S51CertificationCloseoutDenial> {
    for schedule in REQUIRED_SCHEDULES {
        for mutation in REQUIRED_REPLAY_MUTATIONS {
            if !input.replay_transcripts().iter().any(|transcript| {
                transcript.schedule() == schedule && transcript.mutation() == mutation
            }) {
                return Err(S51CertificationCloseoutDenial::MissingReplayTranscript {
                    schedule,
                    mutation,
                });
            }
        }
    }
    Ok(())
}

fn crosscheck_lower_store_counters(
    evidence: S51SecurityScopeHarnessEvidence,
) -> Result<(), S51CertificationCloseoutDenial> {
    let lower = evidence.lower_store_admission_counters();
    expect("store.security_scope.requests", 1, lower.requests())?;
    expect(
        "store.security_scope.current_authority_checks",
        1,
        lower.current_authority_checks(),
    )?;
    expect(
        "store.security_scope.witnesses_issued",
        expected_lower_store_witnesses_issued(evidence),
        lower.witnesses_issued(),
    )?;
    expect(
        "store.security_scope.physical_scope_drift",
        evidence.counters().physical_scope_drift(),
        lower.wrong_physical_scopes(),
    )?;
    expect(
        "store.security_scope.wrong_tenant_scope",
        evidence.counters().wrong_tenant_scope(),
        lower.wrong_tenant_scopes(),
    )?;
    expect(
        "store.security_scope.missing_authenticity_requirement",
        evidence.counters().missing_authenticity_requirement(),
        lower.missing_authenticity_requirements(),
    )?;
    expect(
        "store.security_scope.stale_key_posture",
        evidence.counters().stale_key_posture(),
        lower.stale_key_postures(),
    )?;
    expect(
        "store.security_scope.replayed_custody_posture",
        evidence.counters().replayed_custody_posture(),
        lower.replayed_admission_evidence() + lower.readmission_required(),
    )
}

fn expected_lower_store_witnesses_issued(evidence: S51SecurityScopeHarnessEvidence) -> u64 {
    evidence.counters().readiness_acceptances() * WITNESSES_PER_READINESS_ACCEPTANCE
}

fn exact_witness_sets_issued_after_crosscheck(evidence: S51SecurityScopeHarnessEvidence) -> u64 {
    evidence.counters().readiness_acceptances()
}

fn expect(
    counter: &'static str,
    expected: u64,
    observed: u64,
) -> Result<(), S51CertificationCloseoutDenial> {
    if expected == observed {
        Ok(())
    } else {
        Err(S51CertificationCloseoutDenial::CounterMismatch {
            counter,
            expected,
            observed,
        })
    }
}
