#[path = "../../../support/recovery/coverage_support/coverage_support.rs"]
mod coverage_support;

use forge_store_physical_certification::{
    reject_edited_matrix_row, reject_manual_coverage_prose, reject_unchecked_maturity_claim,
    CoverageGapDenial, CoverageSurfaceKind, FaultDeliveryAttempt, HarnessCoverageStage,
    OracleFamilyKind, PhysicalCoverageRegistry, PhysicalDriverKind, PhysicalHarnessReadinessReport,
    PhysicalMutationCoverageEvidence,
    reject_missing_physical_isolation_correctness_non_claim, PhysicalIsolationCorrectnessNonClaimEvidence,
    PhysicalIsolationHarnessReadinessDenial,
};
use forge_store_test_support::admitted_developer_smoke_driver_contracts;

#[test]
fn matrix_denies_missing_registration_evidence() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(replay.oracle_verdicts())
        .unwrap()
        .register_counter_receipt(replay.counter_receipt())
        .unwrap()
        .register_transcript(&replay)
        .unwrap()
        .generate_matrix()
        .unwrap_err();

    assert_eq!(
        denial,
        CoverageGapDenial::MissingRegistrationEvidence {
            surface: CoverageSurfaceKind::Scenario
        }
    );
}

#[test]
fn registry_denies_schedule_without_prior_plan_registration() {
    let plan = coverage_support::lowered_plan();
    let schedule = coverage_support::schedule(&plan);
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_schedule(&schedule)
        .unwrap_err();

    assert_eq!(
        denial,
        CoverageGapDenial::MissingPlanBeforeDependentSurface {
            surface: CoverageSurfaceKind::YieldpointSchedule
        }
    );
}

#[test]
fn registry_denies_plan_that_does_not_match_registered_scenario() {
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&coverage_support::shortcut_plan())
        .unwrap_err();

    assert_eq!(denial, CoverageGapDenial::PlanScenarioIdentityMismatch);
}

#[test]
fn registry_denies_scenario_that_does_not_match_registered_plan() {
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_plan(&coverage_support::shortcut_plan())
        .unwrap()
        .register_scenario(&coverage_support::scenario())
        .unwrap_err();

    assert_eq!(denial, CoverageGapDenial::PlanScenarioIdentityMismatch);
}

#[test]
fn registry_denies_duplicate_scenario_registration_before_rows_can_go_stale() {
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_scenario(&coverage_support::scenario())
        .unwrap_err();

    assert_eq!(
        denial,
        CoverageGapDenial::DuplicateRegistrationEvidence {
            surface: CoverageSurfaceKind::Scenario
        }
    );
}

#[test]
fn registry_denies_plan_replacement_before_rows_can_go_stale() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_plan(&coverage_support::lowered_ci_plan())
        .unwrap_err();

    assert_eq!(
        denial,
        CoverageGapDenial::DuplicateRegistrationEvidence {
            surface: CoverageSurfaceKind::Plan
        }
    );
}

#[test]
fn registry_denies_driver_contracts_that_do_not_match_plan() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let unrelated_contracts = admitted_developer_smoke_driver_contracts()
        .unwrap()
        .without(PhysicalDriverKind::IoPressureBoundary);
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(&unrelated_contracts)
        .unwrap_err();

    assert_eq!(denial, CoverageGapDenial::DriverContractPlanMismatch);
}

#[test]
fn registry_denies_missing_plan_required_oracle_family() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let incomplete_verdicts = replay
        .oracle_verdicts()
        .iter()
        .filter(|verdict| verdict.family() != OracleFamilyKind::TranscriptReplayEvidence)
        .cloned()
        .collect::<Vec<_>>();
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(&incomplete_verdicts)
        .unwrap_err();

    assert_eq!(denial, CoverageGapDenial::MissingRequiredOracleVerdict);
}

#[test]
fn non_generated_coverage_and_maturity_claims_are_denied() {
    assert_eq!(
        reject_manual_coverage_prose().unwrap_err(),
        CoverageGapDenial::ManualCoverageProseDenied
    );
    assert_eq!(
        reject_edited_matrix_row().unwrap_err(),
        CoverageGapDenial::EditedMatrixRowDenied
    );
    assert_eq!(
        reject_unchecked_maturity_claim().unwrap_err(),
        CoverageGapDenial::UncheckedMaturityClaimDenied
    );
}

#[test]
fn readiness_denies_missing_non_claim() {
    assert_eq!(
        reject_missing_physical_isolation_correctness_non_claim().unwrap_err(),
        PhysicalIsolationHarnessReadinessDenial::MissingPhysicalIsolationCorrectnessNonClaim
    );
}

#[test]
fn missing_mutation_result_blocks_ci_maturity() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(replay.oracle_verdicts())
        .unwrap()
        .register_counter_receipt(replay.counter_receipt())
        .unwrap()
        .register_transcript(&replay)
        .unwrap()
        .generate_matrix()
        .unwrap_err();

    assert_eq!(
        denial,
        CoverageGapDenial::MissingRegistrationEvidence {
            surface: CoverageSurfaceKind::MutationResult
        }
    );
}

#[test]
fn mutation_coverage_requires_replay_observed_private_mutation_denial() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle_without_mutation_denial(&plan);
    let denial = PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
        HarnessCoverageStage::SimulationAdmission,
        &replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap_err();

    assert_eq!(denial, CoverageGapDenial::MissingMutationResult);
}

#[test]
fn mutation_coverage_from_another_plan_is_denied() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let ci_plan = coverage_support::lowered_ci_plan();
    let ci_replay = coverage_support::replay_bundle(&ci_plan);
    let mutation = PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
        HarnessCoverageStage::SimulationAdmission,
        &ci_replay,
        FaultDeliveryAttempt::private_mutation(),
    )
    .unwrap();

    let denial = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::SimulationAdmission)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(replay.oracle_verdicts())
        .unwrap()
        .register_counter_receipt(replay.counter_receipt())
        .unwrap()
        .register_transcript(&replay)
        .unwrap()
        .register_mutation_result(&mutation)
        .unwrap_err();

    assert_eq!(denial, CoverageGapDenial::MutationPlanIdentityMismatch);
}

#[test]
fn wrong_sequence_maturity_cannot_admit_physical_isolation_readiness() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let maturity = PhysicalCoverageRegistry::for_sequence(HarnessCoverageStage::Recovery)
        .register_scenario(&coverage_support::scenario())
        .unwrap()
        .register_plan(&plan)
        .unwrap()
        .register_schedule(replay.schedule())
        .unwrap()
        .register_actor_set()
        .unwrap()
        .register_driver_contracts(plan.driver_contracts())
        .unwrap()
        .register_oracle_verdicts(replay.oracle_verdicts())
        .unwrap()
        .register_counter_receipt(replay.counter_receipt())
        .unwrap()
        .register_transcript(&replay)
        .unwrap()
        .register_mutation_result(
            &PhysicalMutationCoverageEvidence::from_replay_private_mutation_denial(
                HarnessCoverageStage::Recovery,
                &replay,
                FaultDeliveryAttempt::private_mutation(),
            )
            .unwrap(),
        )
        .unwrap()
        .generate_matrix()
        .unwrap()
        .derive_maturity();

    let denial = maturity
        .admit_physical_isolation_simulation_harness_readiness(
            PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        PhysicalIsolationHarnessReadinessDenial::WrongSequenceMaturityEvidence
    );
}

#[test]
fn non_ci_profile_maturity_cannot_admit_physical_isolation_readiness() {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let maturity = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap()
        .derive_maturity();

    let denial = maturity
        .admit_physical_isolation_simulation_harness_readiness(
            PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        PhysicalIsolationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence
    );
}

#[test]
fn missing_coverage_gap_materializes_foundational_named_gap_report() {
    let denial = CoverageGapDenial::MissingRegistrationEvidence {
        surface: CoverageSurfaceKind::MutationResult,
    };
    let report =
        PhysicalHarnessReadinessReport::from_coverage_gap(
            HarnessCoverageStage::SimulationAdmission,
            &denial,
        );

    assert_eq!(report.sequence(), HarnessCoverageStage::SimulationAdmission);
    assert_eq!(report.named_gaps().len(), 1);
    assert_eq!(report.support_report().support_rows().count(), 1);
}
