use super::super::row::WorthGraphReadAccessInventoryRow;
use super::scope_binding::WorthGraphReadAccessScopeBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessScopePlanReport {
    entries: Vec<WorthGraphReadAccessScopePlanEntry>,
}

impl WorthGraphReadAccessScopePlanReport {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_rows(
        rows: &[WorthGraphReadAccessInventoryRow],
    ) -> Self {
        Self {
            entries: rows
                .iter()
                .map(WorthGraphReadAccessScopePlanEntry::from_row)
                .collect(),
        }
    }

    pub fn entries(&self) -> &[WorthGraphReadAccessScopePlanEntry] {
        &self.entries
    }

    pub fn entry_for_source_path(
        &self,
        source_path: &str,
    ) -> Option<&WorthGraphReadAccessScopePlanEntry> {
        self.entries
            .iter()
            .find(|entry| entry.source_path() == source_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessScopePlanEntry {
    source_path: String,
    scope_binding: WorthGraphReadAccessScopeBinding,
}

impl WorthGraphReadAccessScopePlanEntry {
    fn from_row(row: &WorthGraphReadAccessInventoryRow) -> Self {
        Self {
            source_path: row.source_path().to_owned(),
            scope_binding: row.scope_binding().clone(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn scope_binding(&self) -> &WorthGraphReadAccessScopeBinding {
        &self.scope_binding
    }
}
