#[path = "../../../support/recovery/closeout/fixture.rs"]
mod closeout_fixture;
#[path = "receipt_assertions.rs"]
mod receipt_assertions;
#[path = "../../../support/physical_isolation/interleaving_harness_support/interleaving_harness_support.rs"]
mod s5_interleaving_harness_support;
#[allow(dead_code)]
#[path = "../../../support/security/security_scope_simulation_harness/support.rs"]
mod support;

use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue,
    FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceMaterializationProfile, InternedString, ScalarAspectType,
};
use forge_proof::{TransitionOutcome, TransitionOutcome::Success};
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_certification::{
    certify_security_scope_closeout, S51CertificationCloseoutDenial, S51CertificationCloseoutInput,
    S51CertificationEvidencePolicy, S51CloseoutFoundationalLane,
};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_physical_certification::{
    SecurityScopeHarnessEvidence, SecurityScopeHarnessScenario, SecurityScopeHarnessSchedule,
    SecurityScopeReplayMutationKind,
};
use forge_store_readiness::PhysicalFoundationEvidenceField;
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionCounterSnapshot, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};
use forge_store_test_support::{
    execute_security_scope_harness_replay_with_physical_replay,
    execute_security_scope_harness_scenario, security_scope_drift_scenario,
    security_scope_metadata_preservation_scenarios, security_scope_missing_authenticity_scenario,
    security_scope_replayed_custody_scenario, security_scope_stale_key_scenario,
    security_scope_wrong_tenant_scenario,
};
use support::{physical_replay_for_scenario, replay_scenario};

#[test]
fn phase_11_certifies_closeout_from_lower_store_evidence_and_counter_receipts() {
    let scenario_evidence = closeout_scenario_evidence();
    let replay_transcripts = closeout_replay_transcripts();
    let security_scope = security_foundation_scope();

    let closeout = certify_security_scope_closeout(
        S51CertificationCloseoutInput::from_replay_and_security_scope(
            scenario_evidence,
            replay_transcripts,
            security_scope,
            S51CertificationEvidencePolicy::counter_backed_foundational(),
        ),
    )
    .expect("phase 11 closeout must certify from lower Store evidence");

    assert!(closeout
        .boundary_evidence()
        .is_foundational_boundary_evidence());
    assert!(closeout.performance_receipts().all_counter_backed());
    assert!(closeout
        .api_adoption()
        .uses_required_s5_1_foundational_lanes());
    receipt_assertions::assert_exact_counter_backed_receipt_rows(closeout.performance_receipts());
    assert_eq!(closeout.readiness_construction_attempts(), 0);

    let counters = closeout.counter_matrix();
    assert_eq!(counters.scenario_evidence_rows(), 9);
    assert_eq!(counters.replay_transcripts(), 12);
    assert_eq!(counters.scenario_lower_store_requests(), 9);
    assert_eq!(counters.replay_lower_store_requests(), 24);
    assert_eq!(counters.lower_store_requests(), 33);
    assert_eq!(counters.lower_store_current_authority_checks(), 33);
    assert_eq!(counters.lower_store_witness_sets_issued(), 16);
    assert_eq!(counters.physical_scope_drift(), 1);
    assert_eq!(counters.stale_key_posture(), 1);
    assert_eq!(counters.wrong_tenant_scope(), 1);
    assert_eq!(counters.missing_authenticity_requirement(), 1);
    assert_eq!(counters.replayed_custody_posture(), 1);
    assert_eq!(counters.replay_wrong_tenant_scope(), 4);
    assert_eq!(counters.replay_stale_key_posture(), 4);
    assert_eq!(counters.replay_missing_authenticity_requirement(), 4);
    assert_eq!(counters.replay_baseline_admissions(), 12);
    assert_eq!(counters.replay_attempts(), 12);
    assert_eq!(counters.replay_denials_before_logical_decode(), 12);
    assert_eq!(counters.handoff_attempts(), 1);
    assert_eq!(counters.handoff_admitted(), 1);
    assert_eq!(counters.consumed_lower_store_evidence_rows(), 33);

    let boundary_package = closeout.boundary_evidence().package();
    assert_eq!(
        boundary_package.native_aspect_evidence_rows(),
        counters.consumed_lower_store_evidence_rows()
    );
    assert_eq!(
        boundary_package.receipt_counter_family_count() as usize,
        closeout
            .performance_receipts()
            .counter_backed_receipt()
            .counter_rows()
            .len()
    );
    assert_eq!(
        boundary_package.attachment_bundle().target_kind(),
        FoundationalBoundaryEvidenceAttachmentTargetKind::BoundaryArtifact
    );
    assert!(boundary_package.attachment_bundle().provenance().is_some());
    assert!(boundary_package.attachment_bundle().receipt().is_some());
    assert_eq!(
        boundary_package
            .materialized_bundle()
            .materialization_profile(),
        FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
    );
    assert!(!boundary_package
        .performance_canonical_basis()
        .payload()
        .entries()
        .is_empty());
    assert!(!boundary_package
        .boundary_canonical_basis()
        .payload()
        .entries()
        .is_empty());
    assert!(boundary_package.covers_lane(S51CloseoutFoundationalLane::NativeAspectValues));
    assert!(boundary_package.covers_lane(S51CloseoutFoundationalLane::Canonicalization));
    assert!(boundary_package.covers_lane(S51CloseoutFoundationalLane::BoundaryArtifact));
    assert!(boundary_package.covers_lane(S51CloseoutFoundationalLane::BoundaryEvidence));
    assert!(boundary_package.covers_lane(S51CloseoutFoundationalLane::Profile));
    assert!(boundary_package.covers_lane(S51CloseoutFoundationalLane::CounterBackedPerformance));
    assert!(boundary_package
        .carries_field(PhysicalFoundationEvidenceField::FoundationalBoundaryEvidenceBundle));
    assert!(boundary_package.carries_field(
        PhysicalFoundationEvidenceField::FoundationalCounterBackedPerformanceReceipt
    ));

    let rows = closeout.performance_receipts().rows();
    assert_eq!(
        rows.scenario_evidence_rows(),
        counters.scenario_evidence_rows()
    );
    assert_eq!(rows.replay_transcripts(), counters.replay_transcripts());
    assert_eq!(rows.lower_store_requests(), counters.lower_store_requests());
    assert_eq!(
        rows.lower_store_current_authority_checks(),
        counters.lower_store_current_authority_checks()
    );
    assert_eq!(
        rows.lower_store_witness_sets_issued(),
        counters.lower_store_witness_sets_issued()
    );
    assert_eq!(rows.lower_store_denials(), counters.lower_store_denials());
    assert_eq!(rows.physical_scope_drift(), counters.physical_scope_drift());
    assert_eq!(rows.stale_key_posture(), counters.stale_key_posture());
    assert_eq!(rows.wrong_tenant_scope(), counters.wrong_tenant_scope());
    assert_eq!(
        rows.missing_authenticity_requirement(),
        counters.missing_authenticity_requirement()
    );
    assert_eq!(
        rows.replayed_custody_posture(),
        counters.replayed_custody_posture()
    );
    assert_eq!(
        rows.replay_wrong_tenant_scope(),
        counters.replay_wrong_tenant_scope()
    );
    assert_eq!(
        rows.replay_stale_key_posture(),
        counters.replay_stale_key_posture()
    );
    assert_eq!(
        rows.replay_missing_authenticity_requirement(),
        counters.replay_missing_authenticity_requirement()
    );
    assert_eq!(
        rows.replay_baseline_admissions(),
        counters.replay_baseline_admissions()
    );
    assert_eq!(rows.replay_attempts(), counters.replay_attempts());
    assert_eq!(
        rows.replay_denials_before_logical_decode(),
        counters.replay_denials_before_logical_decode()
    );
    assert_eq!(rows.handoff_admitted(), counters.handoff_admitted());
    assert_eq!(
        closeout
            .performance_receipts()
            .counter_backed_receipt()
            .counter_rows()
            .len(),
        forge_store_certification::S51CloseoutPerformanceReceipts::required_counter_names().len()
    );
}

#[test]
fn phase_11_rejects_harness_evidence_with_mismatched_lower_store_counters() {
    let mut scenario_evidence = closeout_scenario_evidence();
    let first_evidence = scenario_evidence[0];
    scenario_evidence[0] = SecurityScopeHarnessEvidence::from_observation_and_store_counters(
        first_evidence.observation(),
        StoreSecurityScopeAdmissionCounterSnapshot::default(),
    );

    let denial = certify_security_scope_closeout(
        S51CertificationCloseoutInput::from_replay_and_security_scope(
            scenario_evidence,
            closeout_replay_transcripts(),
            security_foundation_scope(),
            S51CertificationEvidencePolicy::counter_backed_foundational(),
        ),
    )
    .expect_err("closeout must reject harness evidence whose lower Store counters do not match");

    assert_eq!(
        denial,
        S51CertificationCloseoutDenial::CounterMismatch {
            counter: "store.security_scope.requests",
            expected: 1,
            observed: 0,
        }
    );
}

fn closeout_scenario_evidence(
) -> Vec<forge_store_physical_certification::SecurityScopeHarnessEvidence> {
    let mut evidence = Vec::new();
    for scenario in security_scope_metadata_preservation_scenarios() {
        evidence.push(execute_security_scope_harness_scenario(scenario).evidence());
    }
    for scenario in [
        security_scope_drift_scenario(),
        security_scope_stale_key_scenario(),
        security_scope_wrong_tenant_scenario(),
        security_scope_missing_authenticity_scenario(),
        security_scope_replayed_custody_scenario(),
    ] {
        evidence.push(execute_security_scope_harness_scenario(scenario).evidence());
    }
    evidence
}

fn closeout_replay_transcripts(
) -> Vec<forge_store_physical_certification::SecurityScopeHarnessReplayTranscript> {
    let mut transcripts = Vec::new();
    for schedule in [
        SecurityScopeHarnessSchedule::StableReadPlanAdmission,
        SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
        SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
        SecurityScopeHarnessSchedule::RepairReadAdmission,
    ] {
        for mutation in [
            SecurityScopeReplayMutationKind::ChangedTenantScope,
            SecurityScopeReplayMutationKind::ChangedKeyVersionPosture,
            SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement,
        ] {
            let execution = execute_security_scope_harness_replay_with_physical_replay(
                schedule,
                mutation,
                physical_replay_for_scenario(SecurityScopeHarnessScenario::metadata_preserved(
                    schedule,
                )),
                physical_replay_for_scenario(replay_scenario(schedule, mutation)),
            )
            .expect("phase 10 replay evidence must bind to physical replay");
            transcripts.push(execution.transcript().clone());
        }
    }
    transcripts
}

fn security_foundation_scope() -> StoreAdmittedSecurityScope {
    let authority = current_authority("store.s51.phase11.closeout");
    admitted_security_scope(
        &authority,
        StoreKeyScope::SecurityLifecycleFoundation,
        StoreTenantScope::SecurityLifecycleFoundation,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn admitted_security_scope(
    authority: &StoreCurrentAuthorityWitness,
    key_scope: StoreKeyScope,
    tenant_scope: StoreTenantScope,
    authenticity: StoreAuthenticityRequirement,
    custody: StoreCustodyPosture,
) -> StoreAdmittedSecurityScope {
    let expectation =
        StoreSecurityScopeAdmissionExpectation::new(key_scope, tenant_scope, authenticity, custody);
    let request = StoreSecurityScopeAdmissionRequest::new(
        authority,
        key_scope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        custody,
        expectation,
    );

    match admit_store_security_scope(request) {
        Success(admitted) => admitted,
        outcome => panic!("security scope should admit: {outcome:?}"),
    }
}

fn current_authority(label: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(label, "current"))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> forge_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}
