use forge_query::facade::consumer_kit::ForgeQueryGraphObligationResidueManifest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBroadSelectorResidueRow {
    class: String,
    owner: String,
    introduced_in: String,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: String,
    removal_trigger: String,
    decision: String,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBroadSelectorResidueRows {
    rows: Vec<QueryBroadSelectorResidueRow>,
}

impl QueryBroadSelectorResidueRows {
    pub(crate) fn from_residue_manifest(
        manifest: &ForgeQueryGraphObligationResidueManifest,
    ) -> Self {
        let rows = manifest
            .rows()
            .iter()
            .filter(|row| is_broad_selector_residue_class(row.class()))
            .map(|row| QueryBroadSelectorResidueRow {
                class: row.class().to_string(),
                owner: row.owner().to_string(),
                introduced_in: row.introduced_in().to_string(),
                current_count: row.current_count(),
                must_not_exceed_count: row.must_not_exceed_count(),
                blocker: row.blocker().to_string(),
                removal_trigger: row.removal_trigger().to_string(),
                decision: row.decision().to_string(),
                row_digest: row.row_digest().to_string(),
            })
            .collect();
        Self { rows }
    }

    pub fn rows(&self) -> &[QueryBroadSelectorResidueRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl QueryBroadSelectorResidueRow {
    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn introduced_in(&self) -> &str {
        &self.introduced_in
    }

    pub fn current_count(&self) -> usize {
        self.current_count
    }

    pub fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn decision(&self) -> &str {
        &self.decision
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn is_broad_selector_residue_class(class: &str) -> bool {
    BROAD_SELECTOR_RESIDUE_CLASSES.contains(&class)
}

const BROAD_SELECTOR_RESIDUE_CLASSES: &[&str] = &[
    "worth-spatial-broad-collection-selector",
    "worth-spatial-lifecycle-only-selector",
    "worth-kernel-broad-collection-selector",
    "worth-kernel-lifecycle-only-selector",
    "forge-query-broad-collection-selector",
    "forge-query-lifecycle-only-selector",
];

#[cfg(test)]
mod tests {
    use forge_query::facade::consumer_kit::{
        ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationResidueRow,
    };

    use super::QueryBroadSelectorResidueRows;

    #[test]
    fn broad_selector_residue_uses_declared_classes_not_blocker_text() {
        let manifest = ForgeQueryGraphObligationResidueManifest::capped([
            residue_row(
                "worth-spatial-broad-collection-selector",
                "broad collection selector remains capped",
            ),
            residue_row(
                "worth-spatial-unrelated-residue",
                "mentions broad in prose but is not a selector residue class",
            ),
        ])
        .expect("residue manifest");

        let rows = QueryBroadSelectorResidueRows::from_residue_manifest(&manifest);

        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows.rows()[0].class(),
            "worth-spatial-broad-collection-selector"
        );
        assert_eq!(rows.rows()[0].introduced_in(), "phase5-selector-precision");
        assert_eq!(rows.rows()[0].decision(), "temporary capped residue");
        assert!(!rows.rows()[0].row_digest().is_empty());
    }

    fn residue_row(class: &str, blocker: &str) -> ForgeQueryGraphObligationResidueRow {
        ForgeQueryGraphObligationResidueRow::explicit(
            class,
            "worth-spatial",
            "phase5-selector-precision",
            1,
            1,
            blocker,
            "replace with Query-owned precise selectors",
            "temporary capped residue",
        )
        .expect("residue row")
    }
}
