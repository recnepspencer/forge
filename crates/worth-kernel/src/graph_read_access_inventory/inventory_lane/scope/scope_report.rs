use super::super::row::WorthGraphReadAccessInventoryRow;
use super::scope_kind::WorthGraphReadAccessScopeKind;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthGraphReadAccessScopeReport {
    scoped_row_count: usize,
    selected_obligation_scoped_count: usize,
    touched_authority_scoped_count: usize,
    touch_descriptor_scoped_count: usize,
    topology_read_proof_scoped_count: usize,
    spatial_continuation_scoped_count: usize,
    deleted_graph_read_source_scoped_count: usize,
    certification_only_scoped_count: usize,
    out_of_scope_count: usize,
}

impl WorthGraphReadAccessScopeReport {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_rows(
        rows: &[WorthGraphReadAccessInventoryRow],
    ) -> Self {
        Self {
            scoped_row_count: rows.len(),
            selected_obligation_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::SelectedObligation,
            ),
            touched_authority_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::TouchedAuthorityDigest,
            ),
            touch_descriptor_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::TouchDescriptorDigest,
            ),
            topology_read_proof_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::TopologyReadProof,
            ),
            spatial_continuation_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::SpatialContinuationProof,
            ),
            deleted_graph_read_source_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::DeletedGraphReadSource,
            ),
            certification_only_scoped_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::CertificationOnlyBoundary,
            ),
            out_of_scope_count: count_scope(
                rows,
                WorthGraphReadAccessScopeKind::OutOfScopeNonGraphRead,
            ),
        }
    }

    pub const fn scoped_row_count(&self) -> usize {
        self.scoped_row_count
    }

    pub const fn selected_obligation_scoped_count(&self) -> usize {
        self.selected_obligation_scoped_count
    }

    pub const fn touched_authority_scoped_count(&self) -> usize {
        self.touched_authority_scoped_count
    }

    pub const fn touch_descriptor_scoped_count(&self) -> usize {
        self.touch_descriptor_scoped_count
    }

    pub const fn topology_read_proof_scoped_count(&self) -> usize {
        self.topology_read_proof_scoped_count
    }

    pub const fn spatial_continuation_scoped_count(&self) -> usize {
        self.spatial_continuation_scoped_count
    }

    pub const fn deleted_graph_read_source_scoped_count(&self) -> usize {
        self.deleted_graph_read_source_scoped_count
    }

    pub const fn certification_only_scoped_count(&self) -> usize {
        self.certification_only_scoped_count
    }

    pub const fn out_of_scope_count(&self) -> usize {
        self.out_of_scope_count
    }

    pub const fn unscoped_row_count(&self) -> usize {
        0
    }
}

fn count_scope(
    rows: &[WorthGraphReadAccessInventoryRow],
    scope_kind: WorthGraphReadAccessScopeKind,
) -> usize {
    rows.iter()
        .filter(|row| row.scope_binding().scope_kind() == scope_kind)
        .count()
}
