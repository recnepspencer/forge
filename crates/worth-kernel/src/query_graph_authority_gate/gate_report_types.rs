use super::types::{
    WorthGraphAuthorityDeletionLedgerRow, WorthGraphAuthorityDiscoveryRecord,
    WorthGraphAuthorityInventoryRow, WorthLowerAuthorityPromotionGuardPlan,
};
use super::{WorthTouchedGraphAuthorityDeletionLedgerRow, WorthTouchedGraphAuthorityInventoryRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityGateCounters {
    pub(crate) inventory_rows: usize,
    pub(crate) deletion_ledger_rows: usize,
    pub(crate) touched_graph_inventory_rows: usize,
    pub(crate) touched_graph_deletion_ledger_rows: usize,
    pub(crate) discovery_records: usize,
    pub(crate) lower_authority_guard_plans: usize,
    pub(crate) audited_sources: usize,
    pub(crate) graph_obligation_attempted_bucket_lookups: usize,
    pub(crate) graph_obligation_selected_rows: usize,
    pub(crate) graph_obligation_denied_rows: usize,
    pub(crate) graph_obligation_residue_rows: usize,
    pub(crate) graph_obligation_registration_full_scans: usize,
}

impl WorthGraphAuthorityGateCounters {
    pub const fn inventory_rows(&self) -> usize {
        self.inventory_rows
    }

    pub const fn deletion_ledger_rows(&self) -> usize {
        self.deletion_ledger_rows
    }

    pub const fn touched_graph_inventory_rows(&self) -> usize {
        self.touched_graph_inventory_rows
    }

    pub const fn touched_graph_deletion_ledger_rows(&self) -> usize {
        self.touched_graph_deletion_ledger_rows
    }

    pub const fn discovery_records(&self) -> usize {
        self.discovery_records
    }

    pub const fn lower_authority_guard_plans(&self) -> usize {
        self.lower_authority_guard_plans
    }

    pub const fn audited_sources(&self) -> usize {
        self.audited_sources
    }

    pub const fn graph_obligation_attempted_bucket_lookups(&self) -> usize {
        self.graph_obligation_attempted_bucket_lookups
    }

    pub const fn graph_obligation_selected_rows(&self) -> usize {
        self.graph_obligation_selected_rows
    }

    pub const fn graph_obligation_denied_rows(&self) -> usize {
        self.graph_obligation_denied_rows
    }

    pub const fn graph_obligation_residue_rows(&self) -> usize {
        self.graph_obligation_residue_rows
    }

    pub const fn graph_obligation_registration_full_scans(&self) -> usize {
        self.graph_obligation_registration_full_scans
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphAuthorityGateReport {
    pub(crate) inventory: Vec<WorthGraphAuthorityInventoryRow>,
    pub(crate) deletion_ledger: Vec<WorthGraphAuthorityDeletionLedgerRow>,
    pub(crate) touched_graph_inventory: Vec<WorthTouchedGraphAuthorityInventoryRow>,
    pub(crate) touched_graph_deletion_ledger: Vec<WorthTouchedGraphAuthorityDeletionLedgerRow>,
    pub(crate) discovery_records: Vec<WorthGraphAuthorityDiscoveryRecord>,
    pub(crate) lower_authority_guard_plan: Vec<WorthLowerAuthorityPromotionGuardPlan>,
    pub(crate) counters: WorthGraphAuthorityGateCounters,
}

impl WorthGraphAuthorityGateReport {
    pub fn inventory(&self) -> &[WorthGraphAuthorityInventoryRow] {
        &self.inventory
    }

    pub fn deletion_ledger(&self) -> &[WorthGraphAuthorityDeletionLedgerRow] {
        &self.deletion_ledger
    }

    pub fn touched_graph_inventory(&self) -> &[WorthTouchedGraphAuthorityInventoryRow] {
        &self.touched_graph_inventory
    }

    pub fn touched_graph_deletion_ledger(&self) -> &[WorthTouchedGraphAuthorityDeletionLedgerRow] {
        &self.touched_graph_deletion_ledger
    }

    pub fn discovery_records(&self) -> &[WorthGraphAuthorityDiscoveryRecord] {
        &self.discovery_records
    }

    pub fn lower_authority_guard_plan(&self) -> &[WorthLowerAuthorityPromotionGuardPlan] {
        &self.lower_authority_guard_plan
    }

    pub const fn counters(&self) -> &WorthGraphAuthorityGateCounters {
        &self.counters
    }
}
