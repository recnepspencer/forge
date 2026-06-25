use worth_spatial::facade::workload_vocabulary::{
    SpatialEvidenceQueryGapKind, SpatialEvidenceQueryGapRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectorExpressivenessGapRow {
    kind: QuerySelectorExpressivenessGapKind,
    owner: &'static str,
    needed_by: String,
    blocker: String,
    follow_on_milestone: String,
    source_gap_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectorExpressivenessGapKind {
    DeclaredMutationCollectionNotExpressed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectorExpressivenessGaps {
    rows: Vec<QuerySelectorExpressivenessGapRow>,
}

impl QuerySelectorExpressivenessGaps {
    pub(crate) fn from_spatial_gap_rows(rows: &[SpatialEvidenceQueryGapRow]) -> Self {
        let rows = rows
            .iter()
            .filter(|row| {
                row.kind() == SpatialEvidenceQueryGapKind::DeclaredMutationCollectionNotExpressed
            })
            .map(QuerySelectorExpressivenessGapRow::from_spatial_gap)
            .collect();
        Self { rows }
    }

    pub fn rows(&self) -> &[QuerySelectorExpressivenessGapRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

impl QuerySelectorExpressivenessGapRow {
    fn from_spatial_gap(row: &SpatialEvidenceQueryGapRow) -> Self {
        Self {
            kind: QuerySelectorExpressivenessGapKind::from_spatial_kind(row.kind()),
            owner: "forge-query",
            needed_by: "worth-spatial spatial evidence obligation selection".to_string(),
            blocker: format!("selector expressiveness gap: {}", row.blocker()),
            follow_on_milestone: "touched-graph-milestone-5-phase-5-query-selector-expressiveness"
                .to_string(),
            source_gap_digest: row.gap_digest().to_string(),
        }
    }

    pub fn kind(&self) -> QuerySelectorExpressivenessGapKind {
        self.kind
    }

    pub fn owner(&self) -> &'static str {
        self.owner
    }

    pub fn needed_by(&self) -> &str {
        &self.needed_by
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn follow_on_milestone(&self) -> &str {
        &self.follow_on_milestone
    }

    pub fn source_gap_digest(&self) -> &str {
        &self.source_gap_digest
    }
}

impl QuerySelectorExpressivenessGapKind {
    fn from_spatial_kind(kind: SpatialEvidenceQueryGapKind) -> Self {
        match kind {
            SpatialEvidenceQueryGapKind::DeclaredMutationCollectionNotExpressed => {
                Self::DeclaredMutationCollectionNotExpressed
            }
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredMutationCollectionNotExpressed => {
                "declared-mutation-collection-not-expressed"
            }
        }
    }
}
