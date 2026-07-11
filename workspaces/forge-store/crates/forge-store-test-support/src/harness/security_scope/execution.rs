use forge_proof::TransitionOutcome;
use forge_store_physical_certification::{
    S51SecurityScopeFailureKind, S51SecurityScopeHarnessEvidence,
    S51SecurityScopeHarnessObservation, S51SecurityScopeHarnessReplayTranscript,
    S51SecurityScopeHarnessScenario, S51SecurityScopeHarnessSchedule,
    S51SecurityScopePhysicalReplayDenial, S51SecurityScopePhysicalReplayEvidence,
    S51SecurityScopeReplayMutationKind,
};
use forge_store_security::{
    evaluate_deserialized_security_scope_readmission, evaluate_store_security_scope_admission,
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionCounterSnapshot, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use super::inputs::S51SecurityScopeNativeHarnessFixture;

#[derive(Debug, PartialEq, Eq)]
pub struct S51SecurityScopeHarnessExecution {
    evidence: S51SecurityScopeHarnessEvidence,
    security_scope: Option<StoreAdmittedSecurityScope>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S51SecurityScopeHarnessReplayExecution {
    transcript: S51SecurityScopeHarnessReplayTranscript,
}

pub fn execute_s5_1_security_scope_harness_scenario(
    scenario: S51SecurityScopeHarnessScenario,
) -> S51SecurityScopeHarnessExecution {
    let fixture = S51SecurityScopeNativeHarnessFixture::new();
    execute_scenario_with_fixture(&fixture, scenario)
}

pub fn execute_s5_1_security_scope_harness_replay_with_physical_replay(
    schedule: S51SecurityScopeHarnessSchedule,
    mutation: S51SecurityScopeReplayMutationKind,
    baseline_physical_replay: S51SecurityScopePhysicalReplayEvidence,
    replay_physical_replay: S51SecurityScopePhysicalReplayEvidence,
) -> Result<S51SecurityScopeHarnessReplayExecution, S51SecurityScopePhysicalReplayDenial> {
    let fixture = S51SecurityScopeNativeHarnessFixture::new();
    let baseline = execute_scenario_with_fixture(
        &fixture,
        S51SecurityScopeHarnessScenario::metadata_preserved(schedule),
    );
    let replay = execute_scenario_with_fixture(&fixture, replay_scenario(schedule, mutation));
    Ok(S51SecurityScopeHarnessReplayExecution {
        transcript: S51SecurityScopeHarnessReplayTranscript::from_physical_replay(
            mutation,
            baseline_physical_replay,
            replay_physical_replay,
            baseline.evidence(),
            replay.evidence(),
        )?,
    })
}

impl S51SecurityScopeHarnessExecution {
    pub const fn evidence(&self) -> S51SecurityScopeHarnessEvidence {
        self.evidence
    }

    pub const fn accepted_security_scope(&self) -> Option<&StoreAdmittedSecurityScope> {
        self.security_scope.as_ref()
    }
}

impl S51SecurityScopeHarnessReplayExecution {
    pub const fn transcript(&self) -> &S51SecurityScopeHarnessReplayTranscript {
        &self.transcript
    }
}

fn execute_scenario_with_fixture(
    fixture: &S51SecurityScopeNativeHarnessFixture,
    scenario: S51SecurityScopeHarnessScenario,
) -> S51SecurityScopeHarnessExecution {
    if scenario.failure_kind() == S51SecurityScopeFailureKind::MissingAuthenticityRequirement {
        let readmission = evaluate_deserialized_security_scope_readmission(
            fixture.current_authority(),
            missing_authenticity_declaration(&fixture),
            StoreSecurityScopeAdmissionExpectation::platform_page_envelope(),
        );
        let lower_store_counters = readmission.counters();
        let denial = readmission
            .into_result()
            .expect_err("missing authenticity must deny during Store readmission");
        let observation = S51SecurityScopeHarnessObservation::denied(scenario, denial);
        return S51SecurityScopeHarnessExecution {
            evidence: evidence_from_observation(observation, lower_store_counters),
            security_scope: None,
        };
    }

    let request = request_for_scenario(fixture, scenario);
    let admission = evaluate_store_security_scope_admission(request);
    let lower_store_counters = admission.counters();
    match admission.into_outcome() {
        TransitionOutcome::Success(admitted) => {
            let observation = S51SecurityScopeHarnessObservation::admitted(scenario);
            S51SecurityScopeHarnessExecution {
                evidence: evidence_from_observation(observation, lower_store_counters),
                security_scope: Some(admitted),
            }
        }
        TransitionOutcome::Denied(denial) => denial_execution(
            S51SecurityScopeHarnessObservation::denied(scenario, denial),
            lower_store_counters,
        ),
        TransitionOutcome::Stale(stale) => denial_execution(
            S51SecurityScopeHarnessObservation::stale(scenario, stale),
            lower_store_counters,
        ),
        TransitionOutcome::RebindRequired(rebind) => denial_execution(
            S51SecurityScopeHarnessObservation::rebind_required(scenario, rebind),
            lower_store_counters,
        ),
        TransitionOutcome::Failed(failure) => denial_execution(
            S51SecurityScopeHarnessObservation::failed(scenario, failure),
            lower_store_counters,
        ),
        TransitionOutcome::Deferred(_) => denial_execution(
            S51SecurityScopeHarnessObservation::failed(
                scenario,
                forge_store_security::StoreSecurityScopeAdmissionFailure::PhysicalAuthorityDrift,
            ),
            lower_store_counters,
        ),
    }
}

fn denial_execution(
    observation: S51SecurityScopeHarnessObservation,
    lower_store_counters: StoreSecurityScopeAdmissionCounterSnapshot,
) -> S51SecurityScopeHarnessExecution {
    S51SecurityScopeHarnessExecution {
        evidence: evidence_from_observation(observation, lower_store_counters),
        security_scope: None,
    }
}

fn evidence_from_observation(
    observation: S51SecurityScopeHarnessObservation,
    lower_store_counters: StoreSecurityScopeAdmissionCounterSnapshot,
) -> S51SecurityScopeHarnessEvidence {
    S51SecurityScopeHarnessEvidence::from_observation_and_store_counters(
        observation,
        lower_store_counters,
    )
}

const fn replay_scenario(
    schedule: S51SecurityScopeHarnessSchedule,
    mutation: S51SecurityScopeReplayMutationKind,
) -> S51SecurityScopeHarnessScenario {
    match mutation {
        S51SecurityScopeReplayMutationKind::ChangedTenantScope => {
            S51SecurityScopeHarnessScenario::wrong_tenant_scope(schedule)
        }
        S51SecurityScopeReplayMutationKind::ChangedKeyVersionPosture => {
            S51SecurityScopeHarnessScenario::stale_key_posture(schedule)
        }
        S51SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement => {
            S51SecurityScopeHarnessScenario::missing_authenticity_requirement(schedule)
        }
    }
}

fn request_for_scenario<'a>(
    fixture: &'a S51SecurityScopeNativeHarnessFixture,
    scenario: S51SecurityScopeHarnessScenario,
) -> StoreSecurityScopeAdmissionRequest<'a> {
    let current = fixture.current_authority();
    let physical_witness = match scenario.failure_kind() {
        S51SecurityScopeFailureKind::PhysicalScopeDrift => {
            fixture.drifted_authority().physical_witness()
        }
        _ => current.physical_witness(),
    };
    let key_version_posture = match scenario.failure_kind() {
        S51SecurityScopeFailureKind::StaleKeyPosture => StoreKeyVersionPosture::Stale,
        _ => StoreKeyVersionPosture::Current,
    };
    let tenant_scope = match scenario.failure_kind() {
        S51SecurityScopeFailureKind::WrongTenantScope => StoreTenantScope::StoreInternal,
        _ => StoreTenantScope::TenantPhysicalBoundary,
    };
    let authenticity_requirement = match scenario.failure_kind() {
        S51SecurityScopeFailureKind::MissingAuthenticityRequirement => None,
        _ => Some(platform_authenticity_requirement()),
    };
    let custody_posture = Some(StoreCustodyPosture::InternalStoreCustody);
    let declaration = match scenario.failure_kind() {
        S51SecurityScopeFailureKind::ReplayedCustodyPosture => {
            StoreRawSecurityScopeDeclaration::replayed_admission_evidence(
                physical_witness,
                StoreKeyScope::PageEnvelope,
                key_version_posture,
                tenant_scope,
                authenticity_requirement,
                custody_posture,
            )
        }
        _ => StoreRawSecurityScopeDeclaration::native(
            physical_witness,
            StoreKeyScope::PageEnvelope,
            key_version_posture,
            tenant_scope,
            authenticity_requirement.expect("native scenarios carry authenticity"),
            custody_posture.expect("native scenarios carry custody"),
        ),
    };
    StoreSecurityScopeAdmissionRequest::from_raw_declaration(
        current,
        declaration,
        StoreSecurityScopeAdmissionExpectation::platform_page_envelope(),
    )
}

const fn platform_authenticity_requirement() -> StoreAuthenticityRequirement {
    StoreAuthenticityRequirement::required(StoreAuthenticityRequirementClass::AuthenticatedFrame)
}

fn missing_authenticity_declaration(
    fixture: &S51SecurityScopeNativeHarnessFixture,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        fixture.current_authority().physical_witness(),
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::TenantPhysicalBoundary,
        None,
        Some(StoreCustodyPosture::InternalStoreCustody),
    )
}
