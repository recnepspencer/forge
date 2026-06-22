use super::{
    ForgeQueryGraphReadAccessRequirementCounters, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadAccessRequirementRow,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessRequirementSetDigest(String);

impl ForgeQueryGraphReadAccessRequirementSetDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessRequirementSet {
    digest: ForgeQueryGraphReadAccessRequirementSetDigest,
    read_graph_digest: String,
    access_shape_digest: String,
    selectivity_shape_digest: String,
    rows: Vec<ForgeQueryGraphReadAccessRequirementRow>,
    counters: ForgeQueryGraphReadAccessRequirementCounters,
}

impl ForgeQueryGraphReadAccessRequirementSet {
    pub fn digest(&self) -> &ForgeQueryGraphReadAccessRequirementSetDigest {
        &self.digest
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn access_shape_digest(&self) -> &str {
        &self.access_shape_digest
    }

    pub fn selectivity_shape_digest(&self) -> &str {
        &self.selectivity_shape_digest
    }

    pub fn rows(&self) -> &[ForgeQueryGraphReadAccessRequirementRow] {
        &self.rows
    }

    pub fn canonical_parts(&self) -> Vec<String> {
        requirement_set_canonical_parts(
            &self.read_graph_digest,
            &self.access_shape_digest,
            &self.selectivity_shape_digest,
            self.counters.row_count(),
            &self.rows,
        )
    }

    pub fn counters(&self) -> &ForgeQueryGraphReadAccessRequirementCounters {
        &self.counters
    }

    pub fn contains_kind(&self, kind: &ForgeQueryGraphReadAccessRequirementKind) -> bool {
        self.rows.iter().any(|row| row.kind() == kind)
    }

    pub fn requires_kind(&self, kind: ForgeQueryGraphReadAccessRequirementKind) -> bool {
        self.contains_kind(&kind)
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        access_shape_digest: impl Into<String>,
        selectivity_shape_digest: impl Into<String>,
        mut rows: Vec<ForgeQueryGraphReadAccessRequirementRow>,
    ) -> Self {
        rows.sort_by_key(ForgeQueryGraphReadAccessRequirementRow::digest_part);
        rows.dedup();
        let read_graph_digest = read_graph_digest.into();
        let access_shape_digest = access_shape_digest.into();
        let selectivity_shape_digest = selectivity_shape_digest.into();
        let counters = ForgeQueryGraphReadAccessRequirementCounters::from_rows(&rows);
        let parts = requirement_set_canonical_parts(
            &read_graph_digest,
            &access_shape_digest,
            &selectivity_shape_digest,
            counters.row_count(),
            &rows,
        );
        let digest = ForgeQueryGraphReadAccessRequirementSetDigest::from_parts(&parts);
        Self {
            digest,
            read_graph_digest,
            access_shape_digest,
            selectivity_shape_digest,
            rows,
            counters,
        }
    }
}

fn requirement_set_canonical_parts(
    read_graph_digest: &str,
    access_shape_digest: &str,
    selectivity_shape_digest: &str,
    row_count: usize,
    rows: &[ForgeQueryGraphReadAccessRequirementRow],
) -> Vec<String> {
    let mut parts = vec![
        format!("read_graph:{read_graph_digest}"),
        format!("access_shape:{access_shape_digest}"),
        format!("selectivity_shape:{selectivity_shape_digest}"),
        format!("row_count:{row_count}"),
    ];
    parts.extend(rows.iter().map(|row| row.digest_part()));
    parts
}
