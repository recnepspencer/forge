use crate::workload_composition::deletion_closeout::WorthTouchedGraphConflictDeletionLedgerRow;
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use super::residue_chain::{
    WorthTouchedGraphConflictQueryGapKind, WorthTouchedGraphConflictResidueBoundaryPosture,
    WorthTouchedGraphConflictResidueRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictArchitectureAlignmentReportRow {
    family_kind: TouchedGraphParityFamilyKind,
    source_path: String,
    surface_name: String,
    owner: String,
    query_gap_kind: Option<WorthTouchedGraphConflictQueryGapKind>,
    blocker: String,
    removal_trigger: String,
    mechanically_unreachable_from_ordinary_path: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDeletionAlignmentRow {
    family_kind: TouchedGraphParityFamilyKind,
    source_path: String,
    surface_name: String,
    blocker: String,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictArchitectureAlignmentReport {
    topology_compiled_product_identity_digest: String,
    topology_equivalence_policy_identity_digest: String,
    spatial_compiled_product_identity_digest: String,
    spatial_equivalence_policy_identity_digest: String,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    displaced_legacy_authority_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    deleted_authority_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    capped_deletion_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    certification_only_fence_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
    capped_residue_rows: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
    query_gap_support_rows: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
    ordinary_second_ontology_blockers: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
    report_digest: String,
}

impl WorthTouchedGraphConflictArchitectureAlignmentReport {
    pub(crate) fn from_parts(
        topology_compiled_product_identity_digest: String,
        topology_equivalence_policy_identity_digest: String,
        spatial_compiled_product_identity_digest: String,
        spatial_equivalence_policy_identity_digest: String,
        reuse_decision_identity_digest: Option<String>,
        rebuild_denial_identity_digest: Option<String>,
        displaced_legacy_authority_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
        deleted_authority_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
        capped_deletion_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
        certification_only_fence_rows: Vec<WorthTouchedGraphConflictDeletionAlignmentRow>,
        capped_residue_rows: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
        query_gap_support_rows: Vec<WorthTouchedGraphConflictArchitectureAlignmentReportRow>,
        ordinary_second_ontology_blockers: Vec<
            WorthTouchedGraphConflictArchitectureAlignmentReportRow,
        >,
        report_digest: String,
    ) -> Self {
        Self {
            topology_compiled_product_identity_digest,
            topology_equivalence_policy_identity_digest,
            spatial_compiled_product_identity_digest,
            spatial_equivalence_policy_identity_digest,
            reuse_decision_identity_digest,
            rebuild_denial_identity_digest,
            displaced_legacy_authority_rows,
            deleted_authority_rows,
            capped_deletion_rows,
            certification_only_fence_rows,
            capped_residue_rows,
            query_gap_support_rows,
            ordinary_second_ontology_blockers,
            report_digest,
        }
    }

    pub fn topology_compiled_product_identity_digest(&self) -> &str {
        &self.topology_compiled_product_identity_digest
    }

    pub fn topology_equivalence_policy_identity_digest(&self) -> &str {
        &self.topology_equivalence_policy_identity_digest
    }

    pub fn spatial_compiled_product_identity_digest(&self) -> &str {
        &self.spatial_compiled_product_identity_digest
    }

    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        &self.spatial_equivalence_policy_identity_digest
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub fn deleted_authority_rows(&self) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.deleted_authority_rows
    }

    pub fn capped_deletion_rows(&self) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.capped_deletion_rows
    }

    pub fn certification_only_fence_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.certification_only_fence_rows
    }

    pub fn capped_residue_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictArchitectureAlignmentReportRow] {
        &self.capped_residue_rows
    }

    pub fn query_gap_support_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictArchitectureAlignmentReportRow] {
        &self.query_gap_support_rows
    }

    pub fn ordinary_second_ontology_blockers(
        &self,
    ) -> &[WorthTouchedGraphConflictArchitectureAlignmentReportRow] {
        &self.ordinary_second_ontology_blockers
    }

    pub fn displaced_legacy_authority_rows(
        &self,
    ) -> &[WorthTouchedGraphConflictDeletionAlignmentRow] {
        &self.displaced_legacy_authority_rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn milestone_fifteen_ready(&self) -> bool {
        self.ordinary_second_ontology_blockers.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn with_test_reachable_second_ontology_blocker(
        mut self,
        row: WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    ) -> Self {
        self.ordinary_second_ontology_blockers = vec![row];
        self
    }
}

impl WorthTouchedGraphConflictArchitectureAlignmentReportRow {
    pub(crate) fn from_residue_row(row: &WorthTouchedGraphConflictResidueRow) -> Self {
        Self {
            family_kind: row.family_kind(),
            source_path: row.source_path().to_string(),
            surface_name: row.surface_name().to_string(),
            owner: row.owner().to_string(),
            query_gap_kind: row.query_gap_kind(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
            mechanically_unreachable_from_ordinary_path: row.boundary_posture()
                != WorthTouchedGraphConflictResidueBoundaryPosture::CoveredOrdinaryConsumerDependency,
        }
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub const fn query_gap_kind(&self) -> Option<WorthTouchedGraphConflictQueryGapKind> {
        self.query_gap_kind
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn mechanically_unreachable_from_ordinary_path(&self) -> bool {
        self.mechanically_unreachable_from_ordinary_path
    }

    #[cfg(test)]
    pub(crate) fn hostile_second_ontology_blocker(
        family_kind: TouchedGraphParityFamilyKind,
        source_path: impl Into<String>,
        surface_name: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self {
            family_kind,
            source_path: source_path.into(),
            surface_name: surface_name.into(),
            owner: owner.into(),
            query_gap_kind: None,
            blocker: blocker.into(),
            removal_trigger: removal_trigger.into(),
            mechanically_unreachable_from_ordinary_path: false,
        }
    }
}

impl WorthTouchedGraphConflictDeletionAlignmentRow {
    pub(crate) fn from_deletion_row(row: &WorthTouchedGraphConflictDeletionLedgerRow) -> Self {
        Self {
            family_kind: row.family_kind(),
            source_path: row.source_path().to_string(),
            surface_name: row.surface_name().to_string(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
        }
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}
