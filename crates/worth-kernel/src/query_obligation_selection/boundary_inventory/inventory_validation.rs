use super::inventory_record::{
    QuerySelectionAuthorityPosture, QuerySelectionBoundaryInventory,
    QuerySelectionBoundaryInventoryRow, QuerySelectionDeletionAction, QuerySelectionProofStrength,
    QuerySelectionSurfaceClassification,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectionInventoryFinding {
    surface: &'static str,
    source_path: &'static str,
    kind: QuerySelectionInventoryFindingKind,
}

impl QuerySelectionInventoryFinding {
    pub fn surface(&self) -> &'static str {
        self.surface
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn kind(&self) -> QuerySelectionInventoryFindingKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum QuerySelectionInventoryFindingKind {
    MissingSourcePath,
    MissingSurfaceName,
    MissingCurrentCaller,
    MissingCapForCappedResidue,
    MissingBlockerForCappedResidue,
    MissingRemovalTriggerForCappedResidue,
    InMemorySelectionPromotedAsExecutionAuthority,
    SupportSurfaceMarkedAsSelectedAuthority,
    LocalCeremonyMarkedAsSelectedAuthority,
    PublicLocalCeremonyExport,
    MigrationSurfaceWithoutExplicitDeletionAction,
}

pub fn validate_query_selection_boundary_inventory(
    inventory: &QuerySelectionBoundaryInventory,
) -> Vec<QuerySelectionInventoryFinding> {
    inventory
        .rows()
        .iter()
        .flat_map(validate_row)
        .collect::<Vec<_>>()
}

fn validate_row(row: &QuerySelectionBoundaryInventoryRow) -> Vec<QuerySelectionInventoryFinding> {
    let mut findings = Vec::new();
    if row.source_path().is_empty() {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MissingSourcePath,
        ));
    }
    if row.surface().is_empty() {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MissingSurfaceName,
        ));
    }
    if row.current_caller().is_empty() {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MissingCurrentCaller,
        ));
    }
    if row.classification() == QuerySelectionSurfaceClassification::CappedResidue {
        require_capped_residue_fields(row, &mut findings);
    }
    if row.proof_strength() == QuerySelectionProofStrength::InMemorySelection
        && row.authority_posture().is_selected_obligation_proof()
    {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::InMemorySelectionPromotedAsExecutionAuthority,
        ));
    }
    if row.authority_posture().is_support_only()
        && row.classification() == QuerySelectionSurfaceClassification::QueryOwnedSelection
    {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::SupportSurfaceMarkedAsSelectedAuthority,
        ));
    }
    if row.authority_posture() == QuerySelectionAuthorityPosture::LocalCeremonyAudit
        && row.classification() == QuerySelectionSurfaceClassification::QueryOwnedSelection
    {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::LocalCeremonyMarkedAsSelectedAuthority,
        ));
    }
    if row.exported_facade_path().is_some()
        && row.authority_posture() == QuerySelectionAuthorityPosture::LocalCeremonyAudit
        && row.classification() != QuerySelectionSurfaceClassification::CertificationOnlySupport
    {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::PublicLocalCeremonyExport,
        ));
    }
    if migration_or_deletion_class(row.classification())
        && !explicit_migration_action(row.deletion_action())
    {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MigrationSurfaceWithoutExplicitDeletionAction,
        ));
    }
    findings
}

fn require_capped_residue_fields(
    row: &QuerySelectionBoundaryInventoryRow,
    findings: &mut Vec<QuerySelectionInventoryFinding>,
) {
    if row.cap().is_none() {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MissingCapForCappedResidue,
        ));
    }
    if row.blocker().is_none() {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MissingBlockerForCappedResidue,
        ));
    }
    if row.removal_trigger().is_none() {
        findings.push(finding(
            row,
            QuerySelectionInventoryFindingKind::MissingRemovalTriggerForCappedResidue,
        ));
    }
}

fn migration_or_deletion_class(classification: QuerySelectionSurfaceClassification) -> bool {
    matches!(
        classification,
        QuerySelectionSurfaceClassification::MigrationProjection
            | QuerySelectionSurfaceClassification::DeletionTarget
            | QuerySelectionSurfaceClassification::CappedResidue
            | QuerySelectionSurfaceClassification::QueryGap
    )
}

fn explicit_migration_action(action: QuerySelectionDeletionAction) -> bool {
    matches!(
        action,
        QuerySelectionDeletionAction::MigrateToParallelSelectionSubstrate
            | QuerySelectionDeletionAction::CollapseToQueryOwnedSelection
            | QuerySelectionDeletionAction::DeleteAfterVerticalLane
            | QuerySelectionDeletionAction::CappedResidue
            | QuerySelectionDeletionAction::QueryGapBlocksMigration
    )
}

fn finding(
    row: &QuerySelectionBoundaryInventoryRow,
    kind: QuerySelectionInventoryFindingKind,
) -> QuerySelectionInventoryFinding {
    QuerySelectionInventoryFinding {
        surface: row.surface(),
        source_path: row.source_path(),
        kind,
    }
}
