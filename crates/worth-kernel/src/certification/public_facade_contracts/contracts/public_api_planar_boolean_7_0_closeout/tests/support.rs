use super::super::{
    entry_support, PlanarBoolean7_0AntiTheatreProof, PlanarBoolean7_0CloseoutBundle,
    PlanarBoolean7_0EvidenceProof,
};
use worth_kernel::workload_composition::{
    PlanarBooleanBlockerEvidenceReceipt, PlanarBooleanDeclaration, PlanarBooleanDeclarationReceipt,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperation, PlanarBooleanOutcomeReceipt, WorkloadCatalog, WorthWorkload,
    WorthWorkloadParts,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceLedger, WorkloadEvidenceRow};

const KERNEL_SUMMARY_FIXTURE_IDENTITY: &str =
    "public_planar_boolean_entry_basis_rejects_kernel_summary_substitution";

#[derive(Clone)]
pub(crate) struct PlanarBoolean7_0Harness {
    pub(crate) basis: PlanarBooleanEntryBasis,
    pub(crate) declaration: PlanarBooleanDeclarationReceipt,
    pub(crate) outcome: PlanarBooleanOutcomeReceipt,
    pub(crate) pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    pub(crate) evidence_proof: PlanarBoolean7_0EvidenceProof,
    pub(crate) anti_theatre_proof: PlanarBoolean7_0AntiTheatreProof,
}

pub(crate) fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-7-0-closeout".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("phase 7 closeout thread should spawn")
        .join()
        .expect("phase 7 closeout thread should finish");
}

pub(crate) fn closeout_harness() -> PlanarBoolean7_0Harness {
    closeout_harness_named("phase7-closeout")
}

pub(crate) fn closeout_harness_named(query_scope: &'static str) -> PlanarBoolean7_0Harness {
    let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
        .declared(format!("{query_scope} catalog pair"))
        .build()
        .expect("catalog pair should build");
    let basis = PlanarBooleanEntryBasis::bind(
        entry_support::certified_boolean_readiness_workload_receipt(query_scope),
        format!("{query_scope} basis"),
    )
    .expect("entry basis should certify");
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(basis.clone())
    .declared_by_query(format!("{query_scope} declaration"))
    .bind()
    .expect("boolean declaration should bind");
    entry_support::assert_planar_boolean_query_digest(declaration.query_declaration_digest());
    let route = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(basis.clone())
    .declared_by_query(format!("{query_scope} route"))
    .inspect_support()
    .expect("boolean route should inspect");
    entry_support::assert_planar_boolean_query_digest(route.query_support_digest());
    entry_support::assert_planar_boolean_query_digest(basis.query_declaration_digest());
    let outcome = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        pair_identity(&pair),
        PlanarBooleanExecutionLane::EmberFuture,
    )
    .from_basis(basis.clone())
    .declared_by_query(format!("{query_scope} outcome"))
    .classify_outcome()
    .expect("policy-required outcome should classify");
    let blocker_evidence = PlanarBooleanBlockerEvidenceReceipt::from_outcome(&outcome)
        .expect("policy-required outcome should expose blocker evidence");
    let pair_construction = pair.construction_receipt();
    let admitted_workload = rebuild_left_workload(
        pair.left().workload(),
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&declaration),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&route),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&pair_construction),
        ],
    );
    let blocked_workload = rebuild_left_workload(
        pair.left().workload(),
        vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
            &blocker_evidence,
        )],
    );
    let evidence_proof = PlanarBoolean7_0EvidenceProof::certify(
        &admitted_workload,
        &declaration,
        &route,
        &pair_construction,
        &blocked_workload,
        &blocker_evidence,
    )
    .expect("evidence proof should certify");
    let anti_theatre_proof = PlanarBoolean7_0AntiTheatreProof::certify(
        pair.left().workload().evidence_ledger(),
        &blocked_workload,
        &blocker_evidence,
        &admitted_workload,
        &pair_construction,
        KERNEL_SUMMARY_FIXTURE_IDENTITY,
    )
    .expect("anti-theatre proof should certify");

    PlanarBoolean7_0Harness {
        basis,
        declaration,
        outcome,
        pair,
        evidence_proof,
        anti_theatre_proof,
    }
}

pub(crate) fn closeout_bundle(harness: &PlanarBoolean7_0Harness) -> PlanarBoolean7_0CloseoutBundle {
    PlanarBoolean7_0CloseoutBundle::collect()
        .with_declaration_family_proof(&harness.declaration)
        .with_entry_basis_proof(&harness.basis)
        .with_outcome_and_provenance_proof(&harness.outcome)
        .with_catalog_recipe_proof(&harness.pair)
        .with_evidence_stage_proof(&harness.evidence_proof)
        .with_anti_theatre_proof(&harness.anti_theatre_proof)
}

fn pair_identity(
    pair: &worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> worth_kernel::workload_composition::PlanarBooleanOperandPairIdentity {
    worth_kernel::workload_composition::PlanarBooleanOperandPairIdentity::new(
        pair.operand_pair_identity(),
    )
    .expect("catalog pair identity should be reusable")
}

fn rebuild_left_workload(
    workload: &WorthWorkload,
    boolean_rows: Vec<WorkloadEvidenceRow>,
) -> WorthWorkload {
    let mut rows = workload.evidence_ledger().rows().to_vec();
    rows.extend(boolean_rows);
    let ledger = WorkloadEvidenceLedger::from_rows(rows)
        .expect("boolean closeout ledger should stay inspectable")
        .certify_complete()
        .expect("classical workload stages should remain complete");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: workload.topology().clone(),
        geometry_binding: workload.geometry_binding().clone(),
        surface_support: workload.surface_support().clone(),
        projection: workload.projection().clone(),
        transform: workload.transform().clone(),
        retained_replay: workload.retained_replay().clone(),
        diagnostics: workload.diagnostics().clone(),
        response: workload.response().clone(),
        evidence_ledger: ledger,
    })
    .expect("recomposed closeout workload should certify")
}
