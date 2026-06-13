use worth_kernel::workload_composition::{
    PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanDeclaration, PlanarBooleanEntryBasis,
    PlanarBooleanExecutionLane, PlanarBooleanFamily, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError, WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::support::certified_boolean_readiness_workload_receipt;

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

#[derive(Clone)]
struct BooleanHarness {
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    declaration: worth_kernel::workload_composition::PlanarBooleanDeclarationReceipt,
    route: worth_kernel::workload_composition::PlanarBooleanSupportReceipt,
    other_route: worth_kernel::workload_composition::PlanarBooleanSupportReceipt,
    pair_construction:
        worth_kernel::workload_composition::PlanarBooleanOperandPairConstructionReceipt,
    blocker_evidence: worth_kernel::workload_composition::PlanarBooleanBlockerEvidenceReceipt,
}

fn boolean_harness() -> BooleanHarness {
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

fn rebuild_left_workload(
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

fn boolean_declaration_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(&harness.declaration)
}

fn boolean_route_row(
    receipt: &impl worth_spatial::facade::workload_vocabulary::BooleanEvidenceReceipt,
) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt)
}

fn boolean_pair_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(&harness.pair_construction)
}

fn boolean_blocker_row(harness: &BooleanHarness) -> WorkloadEvidenceRow {
    WorkloadEvidenceRow::from_boolean_evidence_receipt(&harness.blocker_evidence)
}

struct CounterlessBooleanRouteEvidence {
    digest: String,
    support: WorkloadEvidenceSupport,
}

impl CounterlessBooleanRouteEvidence {
    fn new(route: &worth_kernel::workload_composition::PlanarBooleanSupportReceipt) -> Self {
        let support = match route.posture() {
            worth_kernel::workload_composition::PlanarBooleanSupportPosture::Admitted => {
                WorkloadEvidenceSupport::Admitted
            }
            worth_kernel::workload_composition::PlanarBooleanSupportPosture::VisibleNotAdmitted => {
                WorkloadEvidenceSupport::Unsupported
            }
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

struct SupportMismatchedBooleanRouteEvidence {
    digest: String,
}

impl SupportMismatchedBooleanRouteEvidence {
    fn new(route: &worth_kernel::workload_composition::PlanarBooleanSupportReceipt) -> Self {
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

fn pair_identity(
    pair: &worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> worth_kernel::workload_composition::PlanarBooleanOperandPairIdentity {
    worth_kernel::workload_composition::PlanarBooleanOperandPairIdentity::new(
        pair.operand_pair_identity(),
    )
    .expect("catalog pair identity should be reusable")
}

fn stage_row(
    ledger: &worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> &worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow {
    ledger.row_for_stage(stage).expect("expected evidence row")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-phase5-evidence".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("phase 5 evidence contract thread should spawn")
        .join()
        .expect("phase 5 evidence contract thread should finish");
}
