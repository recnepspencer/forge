use std::num::NonZeroU64;

use worth_store::physical_runtime::ScrubPhysicalAllocation;
use worth_store_physical_integrity::{
    DamageClassification, ExecutedQuarantineFinding, PhysicalQuarantineAuthority,
    QuarantineSealRequest, ScrubPlanPolicy,
};
use worth_store_test_support::harness::physical_residency::PhysicalResidencyStoreWorld;

use crate::{
    PhysicalProofOracleKind, PhysicalScenarioDefinition, PhysicalStoryStep, PhysicalSubstrateLane,
};

pub(super) fn planned_work_scenario_definition() -> crate::PhysicalScenarioDefinition {
    PhysicalScenarioDefinition::story("phase_13_planned_work_scenario")
        .physical_substrate_lane(PhysicalSubstrateLane::HappyAuthority)
        .proves_law("scenario plans remain planned work before execution")
        .step(PhysicalStoryStep::GivenCleanPhysicalStore)
        .step(PhysicalStoryStep::WhenAuthoritativeRecordIsAppended)
        .step(PhysicalStoryStep::ThenRecordLocatesByPhysicalReference)
        .requires_oracle(PhysicalProofOracleKind::ScenarioPlanOwnsStrategy)
        .define()
        .unwrap()
}

pub(super) fn with_scrub_plan_authority(
    run: impl FnOnce(ScrubPhysicalAllocation<'_>, ScrubPlanPolicy),
) {
    let world = PhysicalResidencyStoreWorld::initialize("planned-scrub-evidence").unwrap();
    let allocation = world
        .serving()
        .physical_allocations()
        .admit_scrub(NonZeroU64::new(64).unwrap())
        .unwrap();
    run(
        allocation,
        ScrubPlanPolicy::bounded(
            NonZeroU64::new(64).unwrap(),
            NonZeroU64::new(1).unwrap(),
        ),
    );
    assert!(!world.close().residency().requires_inspection());
}

pub(super) fn seal_intact_page_report(
    report: &worth_store_physical_integrity::PageIntegrityReport,
) -> worth_store_physical_integrity::QuarantineRecord {
    let finding = ExecutedQuarantineFinding::intact_page(report);
    let record =
        PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
            .unwrap();
    assert!(matches!(
        record.damage_classification(),
        DamageClassification::IntactPhysicalBoundary(_)
    ));
    record
}
