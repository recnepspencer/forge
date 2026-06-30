use worth_kernel::workload_composition::{
    WorkloadCatalog, WorkloadCompositionError, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, complete_ledger_with_additional_rows,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[test]
fn boolean_catalog_and_entry_surfaces_require_workload_backed_construction() {
    run_with_large_stack(|| {
        let harness = catalog_harness();

        assert_eq!(
            harness.pair.support().posture(),
            worth_kernel::workload_composition::WorkloadCatalogSupportPosture::Admitted
        );
        assert!(!harness
            .pair
            .declaration()
            .query_declaration_digest()
            .trim()
            .is_empty());
        assert_eq!(
            harness.pair_construction.operand_pair_identity(),
            harness.pair.operand_pair_identity()
        );

        let bare = harness.pair.left().workload().clone();
        assert_eq!(
            bare.require_boolean_operand_pair_construction(&harness.pair_construction)
                .expect_err("bare workload must not impersonate catalog pair construction"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanOperandPairConstruction
            )
        );

        let manual = rebuilt_left_workload(
            &harness,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanOperandPairConstruction,
                harness.pair_construction.construction_digest(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_operand_pair_construction(&harness.pair_construction)
                .expect_err("manual pair-construction row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanOperandPairConstruction
            )
        );

        let real = rebuilt_left_workload(
            &harness,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanOperandPairConstruction,
                harness.pair_construction.construction_digest(),
                WorkloadEvidenceStageCounters::boolean_operand_pair(),
            )],
        );
        real.require_boolean_operand_pair_construction(&harness.pair_construction)
            .expect("catalog-built pair construction receipt should pass");
    });
}

#[derive(Clone)]
struct CatalogHarness {
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    pair_construction:
        worth_kernel::workload_composition::PlanarBooleanOperandPairConstructionReceipt,
}

fn catalog_harness() -> CatalogHarness {
    let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
        .declared("phase6 workload-backed construction")
        .build()
        .expect("catalog pair should build");

    CatalogHarness {
        pair_construction: pair.construction_receipt(),
        pair,
    }
}

fn rebuilt_left_workload(
    harness: &CatalogHarness,
    boolean_rows: Vec<WorkloadEvidenceRow>,
) -> WorthWorkload {
    let left = harness.pair.left().workload();
    let ledger = complete_ledger_with_additional_rows(left.evidence_ledger(), boolean_rows)
        .expect("classical stages should remain complete");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger: ledger,
    })
    .expect("recomposed catalog fence workload should certify")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("boolean-catalog-anti-theatre".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("boolean catalog anti-theatre thread should spawn")
        .join()
        .expect("boolean catalog anti-theatre thread should finish");
}
