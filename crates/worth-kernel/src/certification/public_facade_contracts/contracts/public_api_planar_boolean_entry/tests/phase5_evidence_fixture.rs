use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairConstructionReceipt, PlanarBooleanOperandPairIdentity,
    PlanarBooleanOperation, PlanarBooleanSupportPosture, PlanarBooleanSupportReceipt,
    WorkloadCatalog, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
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
    let mut rows = left.evidence_ledger().rows().to_vec();
    rows.extend(boolean_rows);
    let ledger = WorkloadEvidenceLedger::from_rows(rows)
        .expect("boolean ledger rows should stay inspectable")
        .certify_complete()
        .expect("classical authority stages should remain complete");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger: ledger,
    })
    .expect("left workload should re-compose with boolean evidence rows")
}

pub(crate) fn boolean_declaration_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(&harness.declaration)
}

pub(crate) fn boolean_route_row<T: BooleanEvidenceRowAuthority>(
    receipt: &T,
) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt)
}

pub(crate) fn boolean_pair_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(&harness.pair_construction)
}

pub(crate) fn boolean_blocker_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(&harness.blocker_evidence)
}

pub(crate) struct CounterlessBooleanRouteEvidence {
    digest: String,
    support: WorkloadEvidenceSupport,
}

impl CounterlessBooleanRouteEvidence {
    pub(crate) fn new(route: &PlanarBooleanSupportReceipt) -> Self {
        let support = match route.posture() {
            PlanarBooleanSupportPosture::Admitted => WorkloadEvidenceSupport::Admitted,
            PlanarBooleanSupportPosture::VisibleNotAdmitted => WorkloadEvidenceSupport::Unsupported,
        };
        Self {
            digest: route.query_support_digest().to_string(),
            support,
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessBooleanRouteEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::RoutePlan
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::default()
    }
}

impl BooleanEvidenceRowAuthority for CounterlessBooleanRouteEvidence {}

pub(crate) struct SupportMismatchedBooleanRouteEvidence {
    digest: String,
}

impl SupportMismatchedBooleanRouteEvidence {
    pub(crate) fn new(route: &PlanarBooleanSupportReceipt) -> Self {
        Self {
            digest: route.query_support_digest().to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for SupportMismatchedBooleanRouteEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::RoutePlan
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Unsupported
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_route()
    }
}

impl BooleanEvidenceRowAuthority for SupportMismatchedBooleanRouteEvidence {}

fn pair_identity(pair: &BuiltBooleanOperandPairRecipe) -> PlanarBooleanOperandPairIdentity {
    PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
        .expect("catalog pair identity should be reusable")
}

pub(crate) fn stage_row(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> &WorkloadEvidenceRow {
    ledger.row_for_stage(stage).expect("expected evidence row")
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
