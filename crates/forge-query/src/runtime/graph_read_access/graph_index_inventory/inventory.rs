use super::ForgeQueryGraphIndexSupportRow;
use crate::identity::hash_parts;
use crate::runtime::ForgeQueryGraphReadAccessRequirementKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphIndexInventory {
    digest: String,
    rows: Vec<ForgeQueryGraphIndexSupportRow>,
}

impl ForgeQueryGraphIndexInventory {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn rows(&self) -> &[ForgeQueryGraphIndexSupportRow] {
        &self.rows
    }

    pub fn row_for_requirement_kind(
        &self,
        requirement_kind: &ForgeQueryGraphReadAccessRequirementKind,
    ) -> Option<&ForgeQueryGraphIndexSupportRow> {
        self.rows
            .iter()
            .find(|row| row.requirement_kind() == requirement_kind)
    }

    pub fn rows_for_requirement_kind(
        &self,
        requirement_kind: &ForgeQueryGraphReadAccessRequirementKind,
    ) -> Vec<&ForgeQueryGraphIndexSupportRow> {
        self.rows
            .iter()
            .filter(|row| row.requirement_kind() == requirement_kind)
            .collect()
    }

    pub(crate) fn from_current_runtime_support() -> Self {
        Self::from_rows(
            ForgeQueryGraphReadAccessRequirementKind::all()
                .iter()
                .cloned()
                .map(ForgeQueryGraphIndexSupportRow::for_requirement_kind)
                .collect(),
        )
    }

    pub(crate) fn from_rows(mut rows: Vec<ForgeQueryGraphIndexSupportRow>) -> Self {
        rows.sort_by_key(|row| {
            (
                row.requirement_kind().as_str().to_string(),
                row.digest().to_string(),
            )
        });
        rows.dedup_by_key(|row| row.digest().to_string());
        let digest = hash_parts(
            &std::iter::once("forge_query_graph_index_inventory_v1".to_string())
                .chain(rows.iter().map(ForgeQueryGraphIndexSupportRow::digest_part))
                .collect::<Vec<_>>(),
        );
        Self { digest, rows }
    }
}

pub fn forge_query_graph_index_inventory() -> ForgeQueryGraphIndexInventory {
    ForgeQueryGraphIndexInventory::from_current_runtime_support()
}
