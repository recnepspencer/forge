use worth_store_physical_certification::{
    lower_physical_simulation_plan, oracle_verdict_topology, physical_scenario,
    BlockedReclaimUntilReleaseOracle, ExecutedPhysicalSimulationObservation, ForbiddenShortcutSet,
    IndependentVerifierObservation, NoJsonAuthorityOracle, NoMixedRootOracle,
    NoPrivateMutationOracle, ObservationDenial, OfflineVerifierBoundarySeam, OracleDenial,
    OracleFamilyKind, PhysicalOracleNonClaim, PhysicalOracleVerdictTopologyPosture,
    PhysicalProofOracleKind, PhysicalProofOracleVerdictKind, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioIntent, PhysicalScenarioSchedule,
    PhysicalSimulationCapabilitySet, PhysicalSimulationObserver, PhysicalSimulationPlan,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ReusablePhysicalOracleFamily, ShortcutRejectionObservation, SimulationEvidencePolicy,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

use worth_store_test_support::harness::recovery::compaction_observation as compaction_interlock_trace;

#[test]
fn executed_observation_receipt_feeds_convergent_oracle_verdicts() {
    let plan = lower_physical_isolation_plan();
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(&plan).unwrap();
    let runtime_trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(&plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap();
    let verifier_trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(&plan, &execution)
        .unwrap()
        .with_independent_verifier_observation(IndependentVerifierObservation::agreement(
            OfflineVerifierBoundarySeam::RuntimeVerifierComparison,
        ))
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap();

    let runtime_verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &runtime_trace)
        .unwrap();
    let verifier_verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &verifier_trace)
        .unwrap();

    assert_eq!(runtime_verdict.kind(), verifier_verdict.kind());
    assert_eq!(runtime_verdict.family(), verifier_verdict.family());
    assert_eq!(runtime_verdict.oracle(), verifier_verdict.oracle());
    assert_eq!(
        runtime_verdict.basis().scenario_identity(),
        verifier_verdict.basis().scenario_identity()
    );
    assert_eq!(
        runtime_verdict.basis().plan_identity(),
        verifier_verdict.basis().plan_identity()
    );
    assert_eq!(
        verifier_verdict.non_claims(),
        &[PhysicalOracleNonClaim::PhysicalIsolationCorrectness]
    );
}

#[test]
fn executed_observation_denies_plan_receipt_mismatch() {
    let plan = lower_physical_isolation_plan();
    let other_plan = lower_multifamily_plan();
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(&other_plan).unwrap();

    let denial = PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(&plan, &execution)
        .expect_err("receipt from another plan must not satisfy observer input");

    assert_eq!(denial, ObservationDenial::ExecutionReceiptPlanMismatch);
}

#[test]
fn public_scenario_composes_multiple_reusable_oracle_families() {
    let plan = lower_multifamily_plan();
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(&plan).unwrap();
    let trace =
        PhysicalSimulationObserver::independent_physical_trace()
            .observe_executed_plan(&plan, &execution)
            .unwrap()
            .with_shortcut_rejection_observation(
                ShortcutRejectionObservation::private_mutation_denied(),
            )
            .with_shortcut_rejection_observation(
                ShortcutRejectionObservation::json_authority_denied(),
            )
            .with_compaction_interlock_observation(
                compaction_interlock_trace::store_compaction_observation(),
            )
            .complete()
            .unwrap();

    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationReadinessShape));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::ForbiddenShortcutRejection));

    let s5 = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(BlockedReclaimUntilReleaseOracle)
        .judge(&plan, &trace)
        .unwrap();
    let private = ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoPrivateMutationOracle)
        .judge(&plan, &trace)
        .unwrap();
    let json = ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoJsonAuthorityOracle)
        .judge(&plan, &trace)
        .unwrap();
    let not_required = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(worth_store_physical_certification::CrashRecoversOldOrNewNeverMixedOracle)
        .judge(&plan, &trace)
        .expect_err("unrequired family cannot judge multi-family plan");

    assert_eq!(
        s5.oracle(),
        PhysicalProofOracleKind::BlockedReclaimUntilRelease
    );
    assert_eq!(private.kind(), PhysicalProofOracleVerdictKind::Satisfied);
    assert_eq!(json.kind(), PhysicalProofOracleVerdictKind::Satisfied);
    assert_eq!(
        not_required,
        OracleDenial::OracleFamilyNotRequired {
            family: OracleFamilyKind::RecoveryDogfood,
        }
    );
}

#[test]
fn verdict_topology_preserves_reserved_non_success_categories_as_explicit_debt() {
    let topology = oracle_verdict_topology();
    let proof_backed = topology
        .iter()
        .filter(|entry| entry.posture() == PhysicalOracleVerdictTopologyPosture::ProofBacked)
        .map(|entry| entry.kind())
        .collect::<Vec<_>>();
    let reserved = topology
        .iter()
        .filter(|entry| {
            entry.posture() == PhysicalOracleVerdictTopologyPosture::ReservedUntilProofProgression
        })
        .map(|entry| entry.kind())
        .collect::<Vec<_>>();

    assert_eq!(
        proof_backed,
        vec![
            PhysicalProofOracleVerdictKind::Satisfied,
            PhysicalProofOracleVerdictKind::Failed,
        ]
    );
    assert_eq!(
        reserved,
        vec![
            PhysicalProofOracleVerdictKind::Denied,
            PhysicalProofOracleVerdictKind::Deferred,
            PhysicalProofOracleVerdictKind::Stale,
            PhysicalProofOracleVerdictKind::RebindRequired,
        ]
    );
}

fn lower_physical_isolation_plan() -> PhysicalSimulationPlan {
    lower_named_physical_isolation_plan("store.physical.s45.phase7.executed-observation")
}

fn lower_named_physical_isolation_plan(name: &str) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(physical_isolation_scenario(name), complete_context()).unwrap()
}

fn lower_multifamily_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(multifamily_scenario(), complete_context()).unwrap()
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

fn physical_isolation_scenario(
    name: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(name)
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase7-execution", 7)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn multifamily_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase7.multi-family")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase7-multi-family", 7)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::shortcut_rejection_probe(
            "shortcut-probe",
        ))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(
            PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_with_shortcut_rejection(),
        )
        .certify_definition()
        .unwrap()
}
