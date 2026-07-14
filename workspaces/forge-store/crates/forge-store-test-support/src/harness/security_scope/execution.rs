use forge_proof::TransitionOutcome;
use forge_store_physical_certification::{
    SecurityScopeFailureKind, SecurityScopeHarnessEvidence, SecurityScopeHarnessObservation,
    SecurityScopeHarnessReplayTranscript, SecurityScopeHarnessScenario,
    SecurityScopeHarnessSchedule, SecurityScopePhysicalReplayDenial,
    SecurityScopePhysicalReplayEvidence, SecurityScopeReplayMutationKind,
};
use forge_store_security::{
    evaluate_deserialized_security_scope_readmission, evaluate_store_security_scope_admission,
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreRawSecurityScopeDeclaration,
    StoreSecurityScopeAdmissionCounterSnapshot, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

use super::inputs::SecurityScopeNativeHarnessFixture;

#[derive(Debug, PartialEq, Eq)]
pub struct SecurityScopeHarnessExecution {
    evidence: SecurityScopeHarnessEvidence,
    security_scope: Option<StoreAdmittedSecurityScope>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityScopeHarnessReplayExecution {
    transcript: SecurityScopeHarnessReplayTranscript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityScopeFixtureAuthority {
    Current,
    Foreign,
}

pub fn admit_security_scope_fixture(
    authority: SecurityScopeFixtureAuthority,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    custody_posture: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    if authority == SecurityScopeFixtureAuthority::Current {
        let physical_witness =
            forge_store_security::admitted_tenant_page_security_scope_for_layout_partition_test()
                .identity()
                .physical_witness();
        let identity =
            forge_store_security::StoreSecurityScopeIdentity::from_physical_security_scope(
                physical_witness,
                key_scope,
                StoreKeyVersionPosture::Current,
                tenant_scope,
                authenticity_requirement,
                custody_posture,
            );
        return forge_store_security::admitted_security_scope_for_identity_for_test(identity);
    }

    let fixture = SecurityScopeNativeHarnessFixture::new();
    let current = fixture.drifted_authority();
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        current,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity_requirement,
        custody_posture,
        expectation,
    );
    match evaluate_store_security_scope_admission(request).into_outcome() {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("security-scope fixture input must admit: {outcome:?}"),
    }
}

pub fn execute_security_scope_harness_scenario(
    scenario: SecurityScopeHarnessScenario,
) -> SecurityScopeHarnessExecution {
    let fixture = SecurityScopeNativeHarnessFixture::new();
    execute_scenario_with_fixture(&fixture, scenario)
}

pub fn execute_security_scope_harness_replay_with_physical_replay(
    schedule: SecurityScopeHarnessSchedule,
    mutation: SecurityScopeReplayMutationKind,
    baseline_physical_replay: SecurityScopePhysicalReplayEvidence,
    replay_physical_replay: SecurityScopePhysicalReplayEvidence,
) -> Result<SecurityScopeHarnessReplayExecution, SecurityScopePhysicalReplayDenial> {
    let fixture = SecurityScopeNativeHarnessFixture::new();
    let baseline = execute_scenario_with_fixture(
        &fixture,
        SecurityScopeHarnessScenario::metadata_preserved(schedule),
    );
    let replay = execute_scenario_with_fixture(&fixture, replay_scenario(schedule, mutation));
    Ok(SecurityScopeHarnessReplayExecution {
        transcript: SecurityScopeHarnessReplayTranscript::from_physical_replay(
            mutation,
            baseline_physical_replay,
            replay_physical_replay,
            baseline.evidence(),
            replay.evidence(),
        )?,
    })
}

impl SecurityScopeHarnessExecution {
    pub const fn evidence(&self) -> SecurityScopeHarnessEvidence {
        self.evidence
    }

    pub const fn accepted_security_scope(&self) -> Option<&StoreAdmittedSecurityScope> {
        self.security_scope.as_ref()
    }
}

impl SecurityScopeHarnessReplayExecution {
    pub const fn transcript(&self) -> &SecurityScopeHarnessReplayTranscript {
        &self.transcript
    }
}

fn execute_scenario_with_fixture(
    fixture: &SecurityScopeNativeHarnessFixture,
    scenario: SecurityScopeHarnessScenario,
) -> SecurityScopeHarnessExecution {
    if scenario.failure_kind() == SecurityScopeFailureKind::MissingAuthenticityRequirement {
        let readmission = evaluate_deserialized_security_scope_readmission(
            fixture.current_authority(),
            missing_authenticity_declaration(&fixture),
            StoreSecurityScopeAdmissionExpectation::platform_page_envelope(),
        );
        let lower_store_counters = readmission.counters();
        let denial = readmission
            .into_result()
            .expect_err("missing authenticity must deny during Store readmission");
        let observation = SecurityScopeHarnessObservation::denied(scenario, denial);
        return SecurityScopeHarnessExecution {
            evidence: evidence_from_observation(observation, lower_store_counters),
            security_scope: None,
        };
    }

    let request = request_for_scenario(fixture, scenario);
    let admission = evaluate_store_security_scope_admission(request);
    let lower_store_counters = admission.counters();
    match admission.into_outcome() {
        TransitionOutcome::Success(admitted) => {
            let observation = SecurityScopeHarnessObservation::admitted(scenario);
            SecurityScopeHarnessExecution {
                evidence: evidence_from_observation(observation, lower_store_counters),
                security_scope: Some(admitted),
            }
        }
        TransitionOutcome::Denied(denial) => denial_execution(
            SecurityScopeHarnessObservation::denied(scenario, denial),
            lower_store_counters,
        ),
        TransitionOutcome::Stale(stale) => denial_execution(
            SecurityScopeHarnessObservation::stale(scenario, stale),
            lower_store_counters,
        ),
        TransitionOutcome::RebindRequired(rebind) => denial_execution(
            SecurityScopeHarnessObservation::rebind_required(scenario, rebind),
            lower_store_counters,
        ),
        TransitionOutcome::Failed(failure) => denial_execution(
            SecurityScopeHarnessObservation::failed(scenario, failure),
            lower_store_counters,
        ),
        TransitionOutcome::Deferred(_) => denial_execution(
            SecurityScopeHarnessObservation::failed(
                scenario,
                forge_store_security::StoreSecurityScopeAdmissionFailure::PhysicalAuthorityDrift,
            ),
            lower_store_counters,
        ),
    }
}

fn denial_execution(
    observation: SecurityScopeHarnessObservation,
    lower_store_counters: StoreSecurityScopeAdmissionCounterSnapshot,
) -> SecurityScopeHarnessExecution {
    SecurityScopeHarnessExecution {
        evidence: evidence_from_observation(observation, lower_store_counters),
        security_scope: None,
    }
}

fn evidence_from_observation(
    observation: SecurityScopeHarnessObservation,
    lower_store_counters: StoreSecurityScopeAdmissionCounterSnapshot,
) -> SecurityScopeHarnessEvidence {
    SecurityScopeHarnessEvidence::from_observation_and_store_counters(
        observation,
        lower_store_counters,
    )
}

const fn replay_scenario(
    schedule: SecurityScopeHarnessSchedule,
    mutation: SecurityScopeReplayMutationKind,
) -> SecurityScopeHarnessScenario {
    match mutation {
        SecurityScopeReplayMutationKind::ChangedTenantScope => {
            SecurityScopeHarnessScenario::wrong_tenant_scope(schedule)
        }
        SecurityScopeReplayMutationKind::ChangedKeyVersionPosture => {
            SecurityScopeHarnessScenario::stale_key_posture(schedule)
        }
        SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement => {
            SecurityScopeHarnessScenario::missing_authenticity_requirement(schedule)
        }
    }
}

fn request_for_scenario<'a>(
    fixture: &'a SecurityScopeNativeHarnessFixture,
    scenario: SecurityScopeHarnessScenario,
) -> StoreSecurityScopeAdmissionRequest<'a> {
    let current = fixture.current_authority();
    let physical_witness = match scenario.failure_kind() {
        SecurityScopeFailureKind::PhysicalScopeDrift => {
            fixture.drifted_authority().physical_witness()
        }
        _ => current.physical_witness(),
    };
    let key_version_posture = match scenario.failure_kind() {
        SecurityScopeFailureKind::StaleKeyPosture => StoreKeyVersionPosture::Stale,
        _ => StoreKeyVersionPosture::Current,
    };
    let tenant_scope = match scenario.failure_kind() {
        SecurityScopeFailureKind::WrongTenantScope => StoreTenantScope::StoreInternal,
        _ => StoreTenantScope::TenantPhysicalBoundary,
    };
    let authenticity_requirement = match scenario.failure_kind() {
        SecurityScopeFailureKind::MissingAuthenticityRequirement => None,
        _ => Some(platform_authenticity_requirement()),
    };
    let custody_posture = Some(StoreCustodyPosture::InternalStoreCustody);
    let declaration = match scenario.failure_kind() {
        SecurityScopeFailureKind::ReplayedCustodyPosture => {
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
    fixture: &SecurityScopeNativeHarnessFixture,
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
