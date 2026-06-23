use super::capability_row::{
    QueryGraphReadAccessCapabilityKind, QueryGraphReadAccessCapabilityRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryGraphReadAccessCapabilityReport {
    rows: &'static [QueryGraphReadAccessCapabilityRow],
}

impl QueryGraphReadAccessCapabilityReport {
    pub(super) fn new(rows: &'static [QueryGraphReadAccessCapabilityRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[QueryGraphReadAccessCapabilityRow] {
        self.rows
    }

    pub fn contains_kind(&self, kind: QueryGraphReadAccessCapabilityKind) -> bool {
        self.rows.iter().any(|row| row.kind() == kind)
    }

    pub fn contains_query_label(&self, label: &str) -> bool {
        self.rows.iter().any(|row| row.query_label() == label)
    }

    pub fn contains_label_for_kind(
        &self,
        kind: QueryGraphReadAccessCapabilityKind,
        label: &str,
    ) -> bool {
        self.rows
            .iter()
            .any(|row| row.kind() == kind && row.query_label() == label)
    }

    pub fn count_for_kind(&self, kind: QueryGraphReadAccessCapabilityKind) -> usize {
        self.rows.iter().filter(|row| row.kind() == kind).count()
    }

    pub fn labels_for_kind(&self, kind: QueryGraphReadAccessCapabilityKind) -> Vec<&'static str> {
        self.rows
            .iter()
            .filter(|row| row.kind() == kind)
            .map(QueryGraphReadAccessCapabilityRow::query_label)
            .collect()
    }

    pub fn has_duplicate_kind_label_pairs(&self) -> bool {
        let rows = self.rows();
        rows.iter().enumerate().any(|(index, row)| {
            rows[index + 1..].iter().any(|candidate| {
                row.kind() == candidate.kind() && row.query_label() == candidate.query_label()
            })
        })
    }

    pub fn claims_execution_authority(&self) -> bool {
        self.rows
            .iter()
            .any(QueryGraphReadAccessCapabilityRow::claims_execution_authority)
    }
}
