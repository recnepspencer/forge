use worth_kernel::workload_composition::{
    PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanDeclaration, PlanarBooleanEntryBasis,
    PlanarBooleanExecutionLane, PlanarBooleanFamily, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError,
};
use worth_spatial::certification::workload_evidence::{
    complete_ledger_with_additional_rows, ledger_with_manual_stage_substitution,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::support::certified_boolean_readiness_workload_receipt;

#[test]
fn boolean_public_contract_rejects_raw_topology_or_summary_based_entry() {
    run_with_large_stack(|| {
        let built = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase6 raw topology boundary")
            .build()
            .expect("catalog pair should build");

        let error = ledger_with_manual_stage_substitution(
            built.left().workload().evidence_ledger(),
            WorkloadEvidenceStage::Topology,
        )
        .expect("manual topology ledger should stay inspectable")
        .certify_complete()
        .expect_err("manual topology substitution must fail before boolean entry can use it");

        assert_eq!(
            error,
            WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Topology)
        );
        assert!(error
            .human_reason()
            .contains("hand-filled topology evidence"));
    });
}

#[test]
fn boolean_public_contract_rejects_hand_filled_evidence_and_missing_provenance() {
    run_with_large_stack(|| {
        let harness = blocker_harness();

        let missing = rebuilt_workload(&harness, Vec::new());
        assert_eq!(
            missing
                .require_boolean_blocker_provenance(&harness.blocker_evidence)
                .expect_err("missing blocker provenance row must fail"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanBlockerProvenance
            )
        );

        let manual = rebuilt_workload(
            &harness,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanBlockerProvenance,
                harness.blocker_evidence.blocker_digest(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_blocker_provenance(&harness.blocker_evidence)
                .expect_err("manual blocker row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanBlockerProvenance
            )
        );

        assert!(
            PlanarBooleanBlockerEvidenceReceipt::from_outcome(&harness.admitted_outcome).is_none(),
            "admitted outcomes must not mint blocker evidence"
        );
    });
}

#[derive(Clone)]
struct BlockerHarness {
    workload: worth_kernel::workload_composition::WorthWorkload,
    blocker_evidence: PlanarBooleanBlockerEvidenceReceipt,
    admitted_outcome: worth_kernel::workload_composition::PlanarBooleanOutcomeReceipt,
}

fn blocker_harness() -> BlockerHarness {
    let built = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
        .declared("phase6 blocker provenance fence")
        .build()
        .expect("catalog pair should build");
    let basis = PlanarBooleanEntryBasis::bind(
        certified_boolean_readiness_workload_receipt("phase6-blocker-basis"),
        "phase 6 blocker provenance fence",
    )
    .expect("boolean basis should certify");
    let admitted_outcome = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        worth_kernel::workload_composition::PlanarBooleanOperandPairIdentity::new(
            built.operand_pair_identity(),
        )
        .expect("operand pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(basis.clone())
    .declared_by_query("phase6 admitted blocker guard")
    .classify_outcome()
    .expect("B-rep declaration should admit");
    let blocked_outcome = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        worth_kernel::workload_composition::PlanarBooleanOperandPairIdentity::new(
            built.operand_pair_identity(),
        )
        .expect("operand pair identity should certify"),
        PlanarBooleanExecutionLane::EmberFuture,
    )
    .from_basis(basis)
    .declared_by_query("phase6 blocked provenance guard")
    .classify_outcome()
    .expect("EMBER declaration should classify");

    BlockerHarness {
        workload: built.left().workload().clone(),
        blocker_evidence: PlanarBooleanBlockerEvidenceReceipt::from_outcome(&blocked_outcome)
            .expect("policy-required outcome should carry blocker evidence"),
        admitted_outcome,
    }
}

fn rebuilt_workload(
    harness: &BlockerHarness,
    boolean_rows: Vec<WorkloadEvidenceRow>,
) -> worth_kernel::workload_composition::WorthWorkload {
    let ledger =
        complete_ledger_with_additional_rows(harness.workload.evidence_ledger(), boolean_rows)
            .expect("classical stages should remain complete");

    worth_kernel::workload_composition::WorthWorkload::compose(
        worth_kernel::workload_composition::WorthWorkloadParts {
            topology: harness.workload.topology().clone(),
            geometry_binding: harness.workload.geometry_binding().clone(),
            surface_support: harness.workload.surface_support().clone(),
            projection: harness.workload.projection().clone(),
            transform: harness.workload.transform().clone(),
            retained_replay: harness.workload.retained_replay().clone(),
            batch_admission_execution: harness.workload.batch_admission_execution().cloned(),
            diagnostics: harness.workload.diagnostics().clone(),
            response: harness.workload.response().clone(),
            evidence_ledger: ledger,
        },
    )
    .expect("recomposed boolean fence workload should certify")
}

fn run_with_large_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-phase6-entry-fences".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("spawn large-stack planar boolean anti-theatre test")
        .join()
        .expect("join large-stack planar boolean anti-theatre test");
}
