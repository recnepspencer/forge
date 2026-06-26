use super::{
    reject_fabricated_graph_read_receipt_proof, reject_local_support_row_graph_read_proof,
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessInventoryCollector,
    WorthGraphReadAccessInventoryErrorKind, WorthGraphReadAccessInventoryRow,
    WorthGraphReadAccessInventorySeed, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessOutOfScopeReason, WorthGraphReadAccessOwner,
    WorthGraphReadAccessResidueGrowthPolicy, WorthGraphReadAccessScopeBinding,
};

#[test]
fn inventory_closeout_requires_coverage_guard_report() {
    let error = WorthGraphReadAccessInventoryCollector::from_seed(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .admit_row(declaration_candidate_row())
    .unwrap()
    .closeout()
    .expect_err("inventory closeout must carry graph-read coverage guard evidence");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::MissingCoverageGuardReport
    );
}

#[test]
fn inventory_closeout_rejects_duplicate_admitted_row_identity() {
    let error = WorthGraphReadAccessInventoryCollector::from_seed(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .admit_row(declaration_candidate_row())
    .unwrap()
    .admit_row(declaration_candidate_row())
    .expect_err("duplicate source-owner-caller rows cannot close inventory");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::DuplicateInventoryRowIdentity
    );
}

#[test]
fn fabricated_receipts_and_local_support_rows_cannot_be_proof_inputs() {
    assert_eq!(
        reject_fabricated_graph_read_receipt_proof("fabricated-read-receipt")
            .expect_err("fabricated receipts must not satisfy inventory proof")
            .kind(),
        WorthGraphReadAccessInventoryErrorKind::FabricatedReceiptProofDenied
    );
    assert_eq!(
        reject_local_support_row_graph_read_proof("local-support-row")
            .expect_err("local support rows must not satisfy graph-read proof")
            .kind(),
        WorthGraphReadAccessInventoryErrorKind::LocalSupportRowProofDenied
    );
}

#[test]
fn out_of_scope_graph_read_rows_must_explain_non_graph_read_boundary() {
    assert_row_error(
        out_of_scope_row_without_reason(),
        WorthGraphReadAccessInventoryErrorKind::MissingOutOfScopeReason,
    );
    assert_row_error(
        out_of_scope_row_with_graph_cost_posture(),
        WorthGraphReadAccessInventoryErrorKind::OutOfScopeCostPostureMismatch,
    );
    assert_row_error(
        declaration_candidate_row()
            .out_of_scope_reason(WorthGraphReadAccessOutOfScopeReason::DocumentationOnly),
        WorthGraphReadAccessInventoryErrorKind::OutOfScopeReasonOnGraphReadClassification,
    );
}

#[test]
fn residue_growth_policy_requires_explicit_cap_update() {
    let policy = WorthGraphReadAccessResidueGrowthPolicy::admit(1, 1, 1)
        .expect("stable capped residue should be admitted");
    assert_eq!(policy.current_count(), 1);
    assert_eq!(policy.must_not_exceed_count(), 1);
    assert_eq!(policy.previous_must_not_exceed_count(), 1);

    let growth_error = WorthGraphReadAccessResidueGrowthPolicy::admit(2, 2, 1)
        .expect_err("residue growth requires an explicit cap update path");
    assert_eq!(
        growth_error.kind(),
        WorthGraphReadAccessInventoryErrorKind::ResidueGrowthRequiresCapUpdate
    );

    let cap_error = WorthGraphReadAccessResidueGrowthPolicy::admit(3, 2, 2)
        .expect_err("current residue count cannot exceed cap");
    assert_eq!(
        cap_error.kind(),
        WorthGraphReadAccessInventoryErrorKind::ResidueCountExceedsCap
    );
}

fn declaration_candidate_row() -> super::WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-topo/src/projection/read_views/domain")
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller("TopologyReadGraphAccessProof")
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(declaration_scope())
}

fn out_of_scope_row_without_reason() -> super::WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-kernel/src/docs_closeout")
        .owner(WorthGraphReadAccessOwner::WorthKernel)
        .current_caller("docs closeout")
        .classification(WorthGraphReadAccessClassification::OutOfScopeNonGraphRead)
        .cost_posture(WorthGraphReadAccessCostPosture::NoGraphTraversal)
        .deletion_action(WorthGraphReadAccessDeletionAction::OutOfScopeNoGraphRead)
        .milestone_seven_disposition(WorthGraphReadAccessMilestoneSevenDisposition::OutOfScope)
        .scope_binding(out_of_scope_scope())
}

fn out_of_scope_row_with_graph_cost_posture() -> super::WorthGraphReadAccessInventoryRowBuilder {
    out_of_scope_row_without_reason()
        .cost_posture(WorthGraphReadAccessCostPosture::BroadScan)
        .out_of_scope_reason(WorthGraphReadAccessOutOfScopeReason::NonGraphReadCloseout)
}

fn assert_row_error(
    builder: super::WorthGraphReadAccessInventoryRowBuilder,
    expected: WorthGraphReadAccessInventoryErrorKind,
) {
    let error = builder
        .build()
        .expect_err("row builder should reject invalid proof shape");
    assert_eq!(error.kind(), expected);
}

fn declaration_scope() -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::topology_read_proof(
        "crates/worth-topo/src/projection/read_views/domain",
        0,
        "authority-a",
        "touch-a",
        "execution-a",
    )
    .unwrap()
}

fn out_of_scope_scope() -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::out_of_scope_non_graph_read(
        "crates/worth-kernel/src/docs_closeout",
        "non-graph-read-boundary",
    )
    .unwrap()
}
