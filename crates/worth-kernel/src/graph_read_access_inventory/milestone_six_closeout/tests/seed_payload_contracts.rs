use super::super::WorthGraphReadAccessMilestoneSixCloseout;
use super::current_inventory_closeout;
use crate::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessScopeBinding, WorthGraphReadAccessScopeExpectation,
    WorthGraphReadAccessScopeFamily, WorthGraphReadAccessScopeKind,
};
use crate::graph_read_access_inventory::WorthGraphReadAccessInventoryRowContext;

#[test]
fn milestone_seven_declaration_candidates_carry_full_handoff_payload() {
    let closeout = WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(
        current_inventory_closeout(),
    )
    .expect("current inventory should produce final Milestone 6 closeout");
    let seed = closeout.milestone_seven_seed();

    for candidate in seed.declaration_candidates() {
        assert_declaration_candidate_inventory_disposition(candidate.inventory_row_context());
        assert_selected_obligation_scope_payload(candidate.inventory_row_context().scope_binding());
        assert!(!candidate.read_family_target().as_str().is_empty());
        assert!(!candidate.touched_authority_input().is_empty());
        assert!(!candidate
            .requirement_vocabulary()
            .requirement_kinds()
            .is_empty());
        assert!(!candidate.milestone_seven_lowering_target().is_empty());
    }
}

#[test]
fn milestone_seven_capability_gaps_carry_query_gap_handoff_payload() {
    let closeout = WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(
        current_inventory_closeout(),
    )
    .expect("current inventory should produce final Milestone 6 closeout");
    let seed = closeout.milestone_seven_seed();

    for gap in seed.capability_gaps() {
        assert_capability_gap_inventory_disposition(gap.inventory_row_context());
        assert_planar_boolean_capability_gap_scope_payload(
            gap.inventory_row_context().scope_binding(),
        );
        assert!(!gap.missing_capability().as_str().is_empty());
        assert!(gap.must_not_exceed_count() > 0);
        assert!(!gap.blocker().is_empty());
        assert!(!gap.removal_trigger().is_empty());
        assert!(!format!("{:?}", gap.expected_denial().denial_kind()).is_empty());
        assert!(!format!("{:?}", gap.expected_denial().suggested_posture()).is_empty());
    }
}

#[test]
fn milestone_seven_deletion_items_carry_deletion_only_residue_payload() {
    let closeout = WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(
        current_inventory_closeout(),
    )
    .expect("current inventory should produce final Milestone 6 closeout");
    let seed = closeout.milestone_seven_seed();

    for deletion in seed.deletion_items() {
        assert_deletion_only_inventory_disposition(deletion.inventory_row_context());
        assert_deletion_only_residue_scope_payload(
            deletion.inventory_row_context().scope_binding(),
        );
        assert!(!deletion.deletion_trigger().is_empty());
    }
}

fn assert_declaration_candidate_inventory_disposition(
    context: &WorthGraphReadAccessInventoryRowContext,
) {
    assert_eq!(
        context.classification(),
        WorthGraphReadAccessClassification::QueryDeclarationCandidate
    );
    assert_eq!(
        context.milestone_seven_disposition(),
        WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate
    );
    assert_eq!(
        context.deletion_action(),
        WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration
    );
    assert_ne!(
        context.cost_posture(),
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow
    );
    assert_ne!(context.identity().source_path(), OLD_GRAPH_READ_PATH);
    assert!(!context.identity().current_caller().is_empty());
}

fn assert_capability_gap_inventory_disposition(context: &WorthGraphReadAccessInventoryRowContext) {
    assert_eq!(
        context.classification(),
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap
    );
    assert_eq!(
        context.milestone_seven_disposition(),
        WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap
    );
    assert_eq!(
        context.deletion_action(),
        WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists
    );
    assert_ne!(
        context.cost_posture(),
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow
    );
    assert_ne!(context.identity().source_path(), OLD_GRAPH_READ_PATH);
}

fn assert_deletion_only_inventory_disposition(context: &WorthGraphReadAccessInventoryRowContext) {
    assert_eq!(
        context.classification(),
        WorthGraphReadAccessClassification::DeletionTarget
    );
    assert_eq!(
        context.milestone_seven_disposition(),
        WorthGraphReadAccessMilestoneSevenDisposition::DeletionOnly
    );
    assert_eq!(
        context.cost_posture(),
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow
    );
    assert_eq!(context.identity().source_path(), OLD_GRAPH_READ_PATH);
}

fn assert_selected_obligation_scope_payload(scope: &WorthGraphReadAccessScopeBinding) {
    assert_eq!(
        scope.scope_expectation(),
        WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput
    );
    assert!(scope.selected_obligation_index().is_some());
    assert!(scope.authority_digest().is_some());
    assert!(scope.touch_descriptor_digest().is_some());
    assert!(scope.execution_proof_digest().is_some());
    if scope.scope_kind() == WorthGraphReadAccessScopeKind::SelectedObligation {
        assert!(scope.selected_registration_digest().is_some());
    }
}

fn assert_planar_boolean_capability_gap_scope_payload(scope: &WorthGraphReadAccessScopeBinding) {
    assert_eq!(
        scope.scope_family(),
        WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation
    );
    assert_eq!(
        scope.scope_expectation(),
        WorthGraphReadAccessScopeExpectation::QueryAccessRequirementCandidateInput
    );
    assert!(scope.selected_obligation_index().is_some());
    assert!(scope.authority_digest().is_some());
    assert!(scope.touch_descriptor_digest().is_some());
    assert!(scope.execution_proof_digest().is_some());
}

fn assert_deletion_only_residue_scope_payload(scope: &WorthGraphReadAccessScopeBinding) {
    assert_eq!(
        scope.scope_kind(),
        WorthGraphReadAccessScopeKind::DeletedGraphReadSource
    );
    assert_eq!(
        scope.scope_expectation(),
        WorthGraphReadAccessScopeExpectation::DeletionOnlyResidue
    );
    assert!(scope.adoption_manifest_digest().is_some());
}

const OLD_GRAPH_READ_PATH: &str = "crates/worth-kernel/src/query_adoption/graph_read_access";
