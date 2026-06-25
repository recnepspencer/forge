use super::super::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessInventoryRow,
    WorthGraphReadAccessInventoryRowBuilder, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessOutOfScopeReason, WorthGraphReadAccessOwner,
};
use super::residue_rows::capped_residue_row;
use super::scope_bindings::{
    branch_declaration_scope_for_tests, certification_scope, declaration_scope,
    declaration_scope_for_tests, deleted_source_scope, future_receipt_scope_for_tests,
    out_of_scope_binding, preview_declaration_scope_for_tests, spatial_declaration_scope_for_tests,
    spatial_scope,
};

pub(crate) fn declaration_candidate_row() -> WorthGraphReadAccessInventoryRowBuilder {
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

pub(crate) fn declaration_candidate_row_with_scope_for_tests(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessInventoryRow {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path(source_path)
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller(current_caller)
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(declaration_scope_for_tests(source_path, authority_digest))
        .build()
        .expect("test declaration candidate row should satisfy inventory contract")
}

pub(crate) fn spatial_declaration_candidate_row_for_tests(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessInventoryRow {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path(source_path)
        .owner(WorthGraphReadAccessOwner::WorthSpatial)
        .current_caller(current_caller)
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::FrontierOrVisitedSet)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(spatial_declaration_scope_for_tests(
            source_path,
            authority_digest,
        ))
        .build()
        .expect("test spatial declaration candidate row should satisfy inventory contract")
}

pub(crate) fn preview_declaration_candidate_row_for_tests(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessInventoryRow {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path(source_path)
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller(current_caller)
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(preview_declaration_scope_for_tests(
            source_path,
            authority_digest,
        ))
        .build()
        .expect("test preview declaration candidate row should satisfy inventory contract")
}

pub(crate) fn branch_declaration_candidate_row_for_tests(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessInventoryRow {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path(source_path)
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller(current_caller)
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(branch_declaration_scope_for_tests(
            source_path,
            authority_digest,
        ))
        .build()
        .expect("test branch declaration candidate row should satisfy inventory contract")
}

pub(crate) fn future_receipt_declaration_candidate_row_for_tests(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
) -> WorthGraphReadAccessInventoryRow {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path(source_path)
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller(current_caller)
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(future_receipt_scope_for_tests(
            source_path,
            authority_digest,
        ))
        .build()
        .expect("test future receipt declaration row should satisfy inventory contract")
}

pub(crate) fn declaration_candidate_row_without_owner() -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-topo/src/projection/read_views/domain")
        .current_caller("TopologyReadGraphAccessProof")
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(declaration_scope())
}

pub(crate) fn declaration_candidate_row_without_cost_posture(
) -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-topo/src/projection/read_views/domain")
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller("TopologyReadGraphAccessProof")
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(declaration_scope())
}

pub(crate) fn declaration_candidate_row_without_deletion_action(
) -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-topo/src/projection/read_views/domain")
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller("TopologyReadGraphAccessProof")
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        )
        .scope_binding(declaration_scope())
}

pub(crate) fn declaration_candidate_row_without_disposition(
) -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-topo/src/projection/read_views/domain")
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller("TopologyReadGraphAccessProof")
        .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
        .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
        .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
        .scope_binding(declaration_scope())
}

pub(crate) fn deletion_target_row() -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-kernel/src/query_adoption/graph_read_access")
        .owner(WorthGraphReadAccessOwner::WorthKernel)
        .current_caller("deleted graph-read adoption scaffolding")
        .classification(WorthGraphReadAccessClassification::DeletionTarget)
        .cost_posture(WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow)
        .deletion_action(WorthGraphReadAccessDeletionAction::DeleteAfterConsumerCutover)
        .milestone_seven_disposition(WorthGraphReadAccessMilestoneSevenDisposition::DeletionOnly)
        .scope_binding(deleted_source_scope(
            "crates/worth-kernel/src/query_adoption/graph_read_access",
        ))
}

pub(crate) fn capped_residue_inventory_row() -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-kernel/src/query_adoption/graph_read_access")
        .owner(WorthGraphReadAccessOwner::WorthKernel)
        .current_caller("deleted graph-read adoption scaffolding")
        .classification(WorthGraphReadAccessClassification::CappedResidue)
        .cost_posture(WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow)
        .deletion_action(WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists)
        .milestone_seven_disposition(WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap)
        .scope_binding(deleted_source_scope(
            "crates/worth-kernel/src/query_adoption/graph_read_access",
        ))
        .capped_residue(capped_residue_row().build().unwrap())
}

pub(crate) fn certification_only_row() -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-topo/src/projection/read_views/domain/read_proof")
        .owner(WorthGraphReadAccessOwner::WorthTopo)
        .current_caller("TopologyReadGraphAccessProof")
        .classification(WorthGraphReadAccessClassification::CertificationOnlySupport)
        .cost_posture(WorthGraphReadAccessCostPosture::BoundedTouchedRegion)
        .deletion_action(WorthGraphReadAccessDeletionAction::KeepCertificationOnly)
        .milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::CertificationOnly,
        )
        .scope_binding(certification_scope(
            "crates/worth-topo/src/projection/read_views/domain/read_proof",
        ))
}

pub(crate) fn capability_gap_row() -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-spatial/src/workload_platform/planar_boolean_events")
        .owner(WorthGraphReadAccessOwner::WorthSpatial)
        .current_caller("PlanarBooleanFragmentContinuationIndex")
        .classification(WorthGraphReadAccessClassification::QueryAccessCapabilityGap)
        .cost_posture(WorthGraphReadAccessCostPosture::FrontierOrVisitedSet)
        .deletion_action(WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists)
        .milestone_seven_disposition(WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap)
        .scope_binding(spatial_scope(
            "crates/worth-spatial/src/workload_platform/planar_boolean_events",
        ))
}

pub(crate) fn out_of_scope_row() -> WorthGraphReadAccessInventoryRowBuilder {
    WorthGraphReadAccessInventoryRow::builder()
        .source_path("crates/worth-kernel/src/docs_closeout")
        .owner(WorthGraphReadAccessOwner::WorthKernel)
        .current_caller("docs closeout")
        .classification(WorthGraphReadAccessClassification::OutOfScopeNonGraphRead)
        .cost_posture(WorthGraphReadAccessCostPosture::NoGraphTraversal)
        .deletion_action(WorthGraphReadAccessDeletionAction::OutOfScopeNoGraphRead)
        .milestone_seven_disposition(WorthGraphReadAccessMilestoneSevenDisposition::OutOfScope)
        .out_of_scope_reason(WorthGraphReadAccessOutOfScopeReason::NonGraphReadCloseout)
        .scope_binding(out_of_scope_binding(
            "crates/worth-kernel/src/docs_closeout",
        ))
}
