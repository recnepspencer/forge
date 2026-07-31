use super::WorthQueryGraphIndexSupportRow;
use crate::admission_digest::hash_parts;
use crate::graph_read_access::WorthQueryGraphReadAccessRequirementKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphIndexInventory {
    digest: String,
    rows: Vec<WorthQueryGraphIndexSupportRow>,
}

impl WorthQueryGraphIndexInventory {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn rows(&self) -> &[WorthQueryGraphIndexSupportRow] {
        &self.rows
    }

    pub fn row_for_requirement_kind(
        &self,
        requirement_kind: &WorthQueryGraphReadAccessRequirementKind,
    ) -> Option<&WorthQueryGraphIndexSupportRow> {
        self.rows
            .iter()
            .find(|row| row.requirement_kind() == requirement_kind)
    }

    pub fn rows_for_requirement_kind(
        &self,
        requirement_kind: &WorthQueryGraphReadAccessRequirementKind,
    ) -> Vec<&WorthQueryGraphIndexSupportRow> {
        self.rows
            .iter()
            .filter(|row| row.requirement_kind() == requirement_kind)
            .collect()
    }

    pub fn from_current_runtime_support() -> Self {
        Self::from_rows(
            WorthQueryGraphReadAccessRequirementKind::all()
                .iter()
                .cloned()
                .map(WorthQueryGraphIndexSupportRow::for_requirement_kind)
                .collect(),
        )
    }

    pub fn from_rows(mut rows: Vec<WorthQueryGraphIndexSupportRow>) -> Self {
        rows.sort_by_key(|row| {
            (
                row.requirement_kind().as_str().to_string(),
                row.digest().to_string(),
            )
        });
        rows.dedup_by_key(|row| row.digest().to_string());
        let digest = hash_parts(
            &std::iter::once("worth_query_graph_index_inventory_v1".to_string())
                .chain(rows.iter().map(WorthQueryGraphIndexSupportRow::digest_part))
                .collect::<Vec<_>>(),
        );
        Self { digest, rows }
    }
}

pub fn worth_query_graph_index_inventory() -> WorthQueryGraphIndexInventory {
    WorthQueryGraphIndexInventory::from_current_runtime_support()
}
