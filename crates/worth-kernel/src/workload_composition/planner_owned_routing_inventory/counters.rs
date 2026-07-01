use super::classification::{
    PlannerOwnedRoutingDisposition, PlannerOwnedRoutingLifecycleRole, PlannerOwnedRoutingOwner,
};
use super::row::PlannerOwnedRoutingInventoryRow;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlannerOwnedRoutingInventoryCounters {
    total_rows: usize,
    ordinary_rows: usize,
    certification_only_rows: usize,
    migrate_rows: usize,
    delete_rows: usize,
    cap_rows: usize,
    query_gap_rows: usize,
    kernel_rows: usize,
    topo_rows: usize,
    spatial_rows: usize,
    query_rows: usize,
    prior_proof_consumers: usize,
    family_route_products: usize,
    selected_route_consumers: usize,
    public_proof_projections: usize,
    diagnostic_projections: usize,
    forbidden_legacy_explainers: usize,
}

impl PlannerOwnedRoutingInventoryCounters {
    pub(super) fn from_rows(rows: &[PlannerOwnedRoutingInventoryRow]) -> Self {
        let mut counters = Self::default();
        counters.total_rows = rows.len();
        for row in rows {
            counters.observe(row);
        }
        counters
    }

    fn observe(&mut self, row: &PlannerOwnedRoutingInventoryRow) {
        if row.ordinary_path() {
            self.ordinary_rows += 1;
        }
        if row.certification_only() {
            self.certification_only_rows += 1;
        }

        match row.disposition() {
            PlannerOwnedRoutingDisposition::Migrate => self.migrate_rows += 1,
            PlannerOwnedRoutingDisposition::Delete => self.delete_rows += 1,
            PlannerOwnedRoutingDisposition::Cap => self.cap_rows += 1,
            PlannerOwnedRoutingDisposition::QueryGap => self.query_gap_rows += 1,
        }

        match row.owner() {
            PlannerOwnedRoutingOwner::WorthKernel => self.kernel_rows += 1,
            PlannerOwnedRoutingOwner::WorthTopo => self.topo_rows += 1,
            PlannerOwnedRoutingOwner::WorthSpatial => self.spatial_rows += 1,
            PlannerOwnedRoutingOwner::ForgeQuery => self.query_rows += 1,
        }

        match row.lifecycle_role() {
            PlannerOwnedRoutingLifecycleRole::PriorProofInputConsumer => {
                self.prior_proof_consumers += 1;
            }
            PlannerOwnedRoutingLifecycleRole::FamilyRouteProduct => {
                self.family_route_products += 1;
            }
            PlannerOwnedRoutingLifecycleRole::SelectedRouteConsumer => {
                self.selected_route_consumers += 1;
            }
            PlannerOwnedRoutingLifecycleRole::PublicProofProjection => {
                self.public_proof_projections += 1;
            }
            PlannerOwnedRoutingLifecycleRole::DerivedDiagnosticProjection => {
                self.diagnostic_projections += 1;
            }
            PlannerOwnedRoutingLifecycleRole::ForbiddenLegacyExplainer => {
                self.forbidden_legacy_explainers += 1;
            }
        }
    }

    pub const fn total_rows(&self) -> usize {
        self.total_rows
    }
}
