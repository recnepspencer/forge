use worth_kernel::workload_composition::{WorkloadCompositionError, WorkloadStageRequirement};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceRow, WorkloadEvidenceStage};

#[path = "phase5_evidence_fixture.rs"]
mod fixture;

use fixture::{
    boolean_blocker_row, boolean_declaration_row, boolean_harness, boolean_pair_row,
    boolean_route_row, rebuild_left_workload, run_with_large_stack, stage_row,
    CounterlessBooleanRouteEvidence, SupportMismatchedBooleanRouteEvidence,
};

#[test]
fn boolean_evidence_ledger_rejects_missing_or_mismatched_boolean_stage_rows() {
    run_with_large_stack(|| {
        let harness = boolean_harness();

        let missing = rebuild_left_workload(&harness, vec![boolean_route_row(&harness.route)]);
        assert_eq!(
            missing
                .require_boolean_declaration_entry(&harness.declaration)
                .expect_err("missing declaration entry row must fail"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );

        let mismatched = rebuild_left_workload(
            &harness,
            vec![
                boolean_declaration_row(&harness),
                boolean_route_row(&harness.other_route),
            ],
        );
        assert_eq!(
            mismatched
                .require_boolean_route_plan(&harness.route)
                .expect_err("mismatched route-plan digest must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
                WorkloadEvidenceStage::BooleanRoutePlan
            )
        );

        let manual = rebuild_left_workload(
            &harness,
            vec![
                WorkloadEvidenceRow::new(
                    WorkloadEvidenceStage::BooleanDeclarationEntry,
                    harness.declaration.query_declaration_digest(),
                ),
                boolean_route_row(&harness.route),
            ],
        );
        assert_eq!(
            manual
                .require_boolean_declaration_entry(&harness.declaration)
                .expect_err("manual boolean declaration row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );

        let counterless = rebuild_left_workload(
            &harness,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &CounterlessBooleanRouteEvidence::new(&harness.route),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_route_plan(&harness.route)
                .expect_err("counterless boolean route row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanRoutePlan
            )
        );

        let unsupported = rebuild_left_workload(
            &harness,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &SupportMismatchedBooleanRouteEvidence::new(&harness.route),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_route_plan(&harness.route)
                .expect_err("support-mismatched boolean route row must fail"),
            WorkloadCompositionError::UnsupportedStage(WorkloadStageRequirement::BooleanRoutePlan)
        );
    });
}

#[test]
fn boolean_stage_counters_count_real_receipt_backed_boolean_rows_only() {
    run_with_large_stack(|| {
        let harness = boolean_harness();
        let workload = rebuild_left_workload(
            &harness,
            vec![
                boolean_declaration_row(&harness),
                boolean_route_row(&harness.route),
                boolean_pair_row(&harness),
                boolean_blocker_row(&harness),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split"),
            ],
        );
        let ledger = workload.evidence_ledger();

        assert_eq!(ledger.counters().rows(), 13);
        assert_eq!(ledger.counters().boolean_rows(), 4);
        assert_eq!(
            stage_row(ledger, WorkloadEvidenceStage::BooleanDeclarationEntry)
                .counters()
                .boolean_declaration_count(),
            1
        );
        assert_eq!(
            stage_row(ledger, WorkloadEvidenceStage::BooleanRoutePlan)
                .counters()
                .boolean_route_count(),
            1
        );
        assert_eq!(
            stage_row(
                ledger,
                WorkloadEvidenceStage::BooleanOperandPairConstruction
            )
            .counters()
            .boolean_operand_pair_count(),
            1
        );
        assert_eq!(
            stage_row(ledger, WorkloadEvidenceStage::BooleanBlockerProvenance)
                .counters()
                .boolean_blocker_count(),
            1
        );
        assert_eq!(
            stage_row(ledger, WorkloadEvidenceStage::BooleanSplit)
                .counters()
                .total_receipt_backed_counters(),
            0
        );
    });
}

#[test]
fn worth_workload_cannot_compose_boolean_operands_without_required_boolean_evidence() {
    run_with_large_stack(|| {
        let harness = boolean_harness();
        let bare = harness.pair.left().workload().clone();
        assert_eq!(
            bare.require_boolean_declaration_entry(&harness.declaration)
                .expect_err("bare workload must reject missing boolean declaration evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanDeclarationEntry
            )
        );
        assert_eq!(
            bare.require_boolean_route_plan(&harness.route)
                .expect_err("bare workload must reject missing boolean route evidence"),
            WorkloadCompositionError::MissingEvidenceStage(WorkloadEvidenceStage::BooleanRoutePlan)
        );
        assert_eq!(
            bare.require_boolean_operand_pair_construction(&harness.pair_construction)
                .expect_err("bare workload must reject missing operand-pair evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanOperandPairConstruction
            )
        );

        let admitted = rebuild_left_workload(
            &harness,
            vec![
                boolean_declaration_row(&harness),
                boolean_route_row(&harness.route),
                boolean_pair_row(&harness),
            ],
        );
        admitted
            .require_boolean_declaration_entry(&harness.declaration)
            .expect("real declaration evidence should pass");
        admitted
            .require_boolean_route_plan(&harness.route)
            .expect("real route-plan evidence should pass");
        admitted
            .require_boolean_operand_pair_construction(&harness.pair_construction)
            .expect("real operand-pair evidence should pass");

        let blocker = rebuild_left_workload(&harness, vec![boolean_blocker_row(&harness)]);
        blocker
            .require_boolean_blocker_provenance(&harness.blocker_evidence)
            .expect("real blocker provenance should pass");
    });
}
