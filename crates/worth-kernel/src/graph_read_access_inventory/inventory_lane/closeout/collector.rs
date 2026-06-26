use std::collections::BTreeSet;

use super::super::bypass_audit::certify_graph_read_bypass_adoption;
use super::super::coverage::WorthGraphReadAccessCoverageGuardReport;
use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::row::{
    WorthGraphReadAccessInventoryRow, WorthGraphReadAccessInventoryRowBuilder,
};
use super::super::seed::WorthGraphReadAccessInventorySeed;
use super::summary::WorthGraphReadAccessInventoryCloseout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessInventoryCollector {
    seed: WorthGraphReadAccessInventorySeed,
    guard_report: Option<WorthGraphReadAccessCoverageGuardReport>,
    rows: Vec<WorthGraphReadAccessInventoryRow>,
    row_identities: BTreeSet<WorthGraphReadAccessInventoryRowIdentity>,
}

impl WorthGraphReadAccessInventoryCollector {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_seed(
        seed: WorthGraphReadAccessInventorySeed,
    ) -> Self {
        Self {
            seed,
            guard_report: None,
            rows: Vec::new(),
            row_identities: BTreeSet::new(),
        }
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn with_guard_report(
        mut self,
        guard_report: WorthGraphReadAccessCoverageGuardReport,
    ) -> Self {
        self.guard_report = Some(guard_report);
        self
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) const fn seed(
        &self,
    ) -> &WorthGraphReadAccessInventorySeed {
        &self.seed
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn admit_row(
        mut self,
        builder: WorthGraphReadAccessInventoryRowBuilder,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        let row = builder.build()?;
        let identity = WorthGraphReadAccessInventoryRowIdentity::from_row(&row);
        if !self.row_identities.insert(identity) {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::DuplicateInventoryRowIdentity,
            ));
        }
        self.rows.push(row);
        Ok(self)
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn closeout(
        self,
    ) -> Result<WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryError> {
        let guard_report = self.guard_report.ok_or_else(|| {
            WorthGraphReadAccessInventoryError::new(
                WorthGraphReadAccessInventoryErrorKind::MissingCoverageGuardReport,
            )
        })?;
        let bypass_adoption_report = certify_graph_read_bypass_adoption(&self.rows)?;
        WorthGraphReadAccessInventoryCloseout::from_admitted_rows(
            self.seed,
            guard_report,
            bypass_adoption_report,
            self.rows,
        )
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorthGraphReadAccessInventoryRowIdentity {
    source_path: String,
    owner: &'static str,
    current_caller: String,
}

impl WorthGraphReadAccessInventoryRowIdentity {
    fn from_row(row: &WorthGraphReadAccessInventoryRow) -> Self {
        Self {
            source_path: row.source_path().to_string(),
            owner: row.owner().as_str(),
            current_caller: row.current_caller().to_string(),
        }
    }
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
