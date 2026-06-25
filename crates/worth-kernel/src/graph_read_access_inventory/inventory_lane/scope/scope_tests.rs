use super::super::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessInventoryErrorKind,
    WorthGraphReadAccessInventoryRow, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessOwner,
};
use super::{reject_read_access_plan_scope_substitution, WorthGraphReadAccessScopeBinding};
use crate::graph_read_access_inventory::inventory_lane::WorthGraphReadAccessScopeSubstitutionRole;

#[test]
fn selected_obligation_seed_cannot_be_relabelled_as_read_access_plan() {
    for claimed_surface in [
        WorthGraphReadAccessScopeSubstitutionRole::GraphReadDeclaration,
        WorthGraphReadAccessScopeSubstitutionRole::AdmittedAccessPlan,
        WorthGraphReadAccessScopeSubstitutionRole::ReadAccessReceipt,
        WorthGraphReadAccessScopeSubstitutionRole::NoNPlusOneExecutionProof,
    ] {
        let error = reject_read_access_plan_scope_substitution(claimed_surface)
            .expect_err("selected obligation scope must not accept downstream read labels");
        assert_eq!(
            error.kind(),
            WorthGraphReadAccessInventoryErrorKind::SelectedObligationRelabelledAsReadAccessPlan
        );
    }

    reject_read_access_plan_scope_substitution(
        WorthGraphReadAccessScopeSubstitutionRole::SelectedObligationScope,
    )
    .expect("selected obligation scope label should remain admissible");
}

#[test]
fn inventory_rows_reject_scope_bindings_from_the_wrong_architectural_class() {
    let error = declaration_candidate_row()
        .scope_binding(deleted_graph_read_source_scope(
            "crates/worth-topo/src/projection/read_views/domain",
        ))
        .build()
        .expect_err("declaration candidates cannot use deleted-source residue as scope evidence");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::ScopeClassificationMismatch
    );
}

#[test]
fn inventory_rows_reject_scope_bindings_from_a_different_source_path() {
    let error = declaration_candidate_row()
        .scope_binding(deleted_graph_read_source_scope(
            "crates/worth-kernel/src/query_adoption/graph_read_access",
        ))
        .build()
        .expect_err("scope binding must name the same source path as the admitted row");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::ScopeSourcePathMismatch
    );
}

fn declaration_candidate_row() -> super::super::WorthGraphReadAccessInventoryRowBuilder {
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
}

fn deleted_graph_read_source_scope(source_path: &str) -> WorthGraphReadAccessScopeBinding {
    WorthGraphReadAccessScopeBinding::deleted_graph_read_source(source_path, "adoption-a").unwrap()
}
