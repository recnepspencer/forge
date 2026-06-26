use forge_query::facade::consumer_kit::ForgeQueryGraphReadBypassResidueRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadBypassResidueReportRow {
    class: &'static str,
    owner: String,
    introduced_in: String,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: String,
    removal_trigger: String,
    row_digest: String,
}

impl WorthGraphReadBypassResidueReportRow {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_query_row(
        row: &ForgeQueryGraphReadBypassResidueRow,
    ) -> Self {
        Self {
            class: row.class().as_str(),
            owner: row.owner().to_string(),
            introduced_in: row.introduced_in().to_string(),
            current_count: row.current_count(),
            must_not_exceed_count: row.must_not_exceed_count(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
            row_digest: row.row_digest().to_string(),
        }
    }

    pub const fn class(&self) -> &'static str {
        self.class
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn introduced_in(&self) -> &str {
        &self.introduced_in
    }

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
