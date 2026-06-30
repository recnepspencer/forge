use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairConstructionReceipt, PlanarBooleanOperandPairIdentity,
    PlanarBooleanOperation, PlanarBooleanSupportReceipt, WorkloadCatalog, WorthWorkload,
    WorthWorkloadParts,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, complete_ledger_stage_snapshot,
    complete_ledger_with_additional_rows, CertifiedWorkloadEvidenceStageSnapshot,
};
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters,
};

use super::super::support::certified_boolean_readiness_workload_receipt;

#[derive(Clone)]
pub(crate) struct BooleanHarness {
    pub(crate) pair: BuiltBooleanOperandPairRecipe,
    pub(crate) declaration: worth_kernel::workload_composition::PlanarBooleanDeclarationReceipt,
    pub(crate) route: PlanarBooleanSupportReceipt,
    pub(crate) other_route: PlanarBooleanSupportReceipt,
    pub(crate) pair_construction: PlanarBooleanOperandPairConstructionReceipt,
    pub(crate) blocker_evidence: PlanarBooleanBlockerEvidenceReceipt,
}

pub(crate) fn boolean_harness() -> BooleanHarness {
    let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
        .build()
        .expect("clean boolean operand pair should build");
    let basis = PlanarBooleanEntryBasis::bind(
        certified_boolean_readiness_workload_receipt("phase5-boolean-evidence"),
        "phase 5 boolean evidence basis",
    )
    .expect("boolean entry basis should certify");
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(basis.clone())
    .declared_by_query("phase 5 boolean evidence declaration")
    .bind()
    .expect("boolean declaration should bind");
    let route = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(basis.clone())
    .declared_by_query("phase 5 boolean evidence route")
    .inspect_support()
    .expect("boolean route should bind");
    let other_route = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(basis.clone())
    .declared_by_query("phase 5 boolean evidence other route")
    .inspect_support()
    .expect("second route should bind");
    let blocker_outcome = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::EmberFuture,
    )
    .from_basis(basis)
    .declared_by_query("phase 5 boolean blocker provenance")
    .classify_outcome()
    .expect("EMBER policy-required outcome should classify");

    BooleanHarness {
        pair_construction: pair.construction_receipt(),
        pair,
        declaration,
        route,
        other_route,
        blocker_evidence: PlanarBooleanBlockerEvidenceReceipt::from_outcome(&blocker_outcome)
            .expect("policy-required outcome should expose blocker evidence"),
    }
}

pub(crate) fn rebuild_left_workload(
    harness: &BooleanHarness,
    boolean_rows: Vec<WorkloadEvidenceRow>,
) -> WorthWorkload {
    let left = harness.pair.left().workload();
    let ledger = complete_ledger_with_additional_rows(left.evidence_ledger(), boolean_rows)
        .expect("classical authority stages should remain complete");

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
    .expect("left workload should re-compose with boolean evidence rows")
}

pub(crate) fn boolean_declaration_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    certification_only_admitted_stage_row(
        WorkloadEvidenceStage::BooleanDeclarationEntry,
        harness.declaration.query_declaration_digest(),
        WorkloadEvidenceStageCounters::boolean_declaration(),
    )
}

pub(crate) fn boolean_route_row(receipt: &PlanarBooleanSupportReceipt) -> WorkloadEvidenceRow {
    certification_only_admitted_stage_row(
        WorkloadEvidenceStage::BooleanRoutePlan,
        receipt.query_support_digest(),
        WorkloadEvidenceStageCounters::boolean_route(),
    )
}

pub(crate) fn boolean_pair_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    certification_only_admitted_stage_row(
        WorkloadEvidenceStage::BooleanOperandPairConstruction,
        harness.pair_construction.construction_digest(),
        WorkloadEvidenceStageCounters::boolean_operand_pair(),
    )
}

pub(crate) fn boolean_blocker_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    certification_only_admitted_stage_row(
        WorkloadEvidenceStage::BooleanBlockerProvenance,
        harness.blocker_evidence.blocker_digest(),
        WorkloadEvidenceStageCounters::boolean_blocker(),
    )
}

fn pair_identity(pair: &BuiltBooleanOperandPairRecipe) -> PlanarBooleanOperandPairIdentity {
    PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
        .expect("catalog pair identity should be reusable")
}

pub(crate) fn stage_row(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> CertifiedWorkloadEvidenceStageSnapshot {
    complete_ledger_stage_snapshot(ledger, stage).expect("expected evidence row")
}

pub(crate) fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-phase5-evidence".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("phase 5 evidence contract thread should spawn")
        .join()
        .expect("phase 5 evidence contract thread should finish");
}
