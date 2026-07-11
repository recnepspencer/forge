use forge_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration,
    BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest, BackgroundWorkBudgetSnapshot,
    FixedMetadataReservation,
};
use forge_store_physical_integrity::{
    ChunkIntegrityStreamingWindow, DamageClassification, ExecutedQuarantineFinding,
    PhysicalQuarantineAuthority, QuarantineSealRequest, ScrubPlanBudget,
    ScrubPlanningMemoryEnvelope,
};

use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::with_entry_seed;
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

pub(super) fn with_scrub_budget(run: impl FnOnce(ScrubPlanBudget)) {
    with_entry_seed(b"phase-13-planned-work", |seed| {
        let mut allocation = AllocationAdmission::from_declaration(allocation_envelopes());
        let mut envelopes = BackgroundEnvelopeAdmission::new();
        let work_budget = BackgroundWorkBudgetSnapshot::foreground_reserved(32, 0, 0, 32);
        let scrub_envelope = envelopes
            .admit(
                BackgroundEnvelopeRequest::scrub_planning()
                    .resident_frames(1)
                    .resident_bytes(64)
                    .pin_pages_for_bounded_step(1)
                    .allocation_bytes(64)
                    .finish(),
                work_budget,
                &mut allocation,
            )
            .unwrap();
        let streaming_envelope = envelopes
            .admit(
                BackgroundEnvelopeRequest::large_record_streaming()
                    .allocation_bytes(64)
                    .streaming_window(256, 64)
                    .finish(),
                work_budget,
                &mut allocation,
            )
            .unwrap();
        let budget = ScrubPlanBudget::new(
            seed.entry_witness(),
            ScrubPlanningMemoryEnvelope::from_admitted(scrub_envelope).unwrap(),
            ChunkIntegrityStreamingWindow::from_admitted_streaming_envelope(streaming_envelope)
                .unwrap(),
        );
        run(budget);
    })
}

pub(super) fn seal_intact_page_report(
    report: &forge_store_physical_integrity::PageIntegrityReport,
) -> forge_store_physical_integrity::QuarantineRecord {
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

fn allocation_envelopes() -> forge_store_buffer_pool::AllocationEnvelopeSet {
    AllocationEnvelopeDeclaration::declare()
        .foreground(bytes(64))
        .maintenance(bytes(64))
        .recovery(bytes(64))
        .scrub(bytes(64))
        .import_export(bytes(64))
        .streaming(bytes(64))
        .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
        .seal()
        .unwrap()
}

fn bytes(value: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(value).unwrap()
}
