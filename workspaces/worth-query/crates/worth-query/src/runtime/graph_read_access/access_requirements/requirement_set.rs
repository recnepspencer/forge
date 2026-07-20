use super::{
    WorthQueryGraphReadAccessRequirementCounters, WorthQueryGraphReadAccessRequirementKind,
    WorthQueryGraphReadAccessRequirementRow,
};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessRequirementSetDigest(String);

impl WorthQueryGraphReadAccessRequirementSetDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessRequirementSet {
    digest: WorthQueryGraphReadAccessRequirementSetDigest,
    read_graph_digest: String,
    access_shape_digest: String,
    selectivity_shape_digest: String,
    rows: Vec<WorthQueryGraphReadAccessRequirementRow>,
    counters: WorthQueryGraphReadAccessRequirementCounters,
}

impl WorthQueryGraphReadAccessRequirementSet {
    pub fn digest(&self) -> &WorthQueryGraphReadAccessRequirementSetDigest {
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

    pub fn rows(&self) -> &[WorthQueryGraphReadAccessRequirementRow] {
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

    pub fn counters(&self) -> &WorthQueryGraphReadAccessRequirementCounters {
        &self.counters
    }

    pub fn contains_kind(&self, kind: &WorthQueryGraphReadAccessRequirementKind) -> bool {
        self.rows.iter().any(|row| row.kind() == kind)
    }

    pub fn requires_kind(&self, kind: WorthQueryGraphReadAccessRequirementKind) -> bool {
        self.contains_kind(&kind)
    }

    pub(crate) fn new(
        read_graph_digest: impl Into<String>,
        access_shape_digest: impl Into<String>,
        selectivity_shape_digest: impl Into<String>,
        mut rows: Vec<WorthQueryGraphReadAccessRequirementRow>,
    ) -> Self {
        rows.sort_by_key(WorthQueryGraphReadAccessRequirementRow::digest_part);
        rows.dedup();
        let read_graph_digest = read_graph_digest.into();
        let access_shape_digest = access_shape_digest.into();
        let selectivity_shape_digest = selectivity_shape_digest.into();
        let counters = WorthQueryGraphReadAccessRequirementCounters::from_rows(&rows);
        let parts = requirement_set_canonical_parts(
            &read_graph_digest,
            &access_shape_digest,
            &selectivity_shape_digest,
            counters.row_count(),
            &rows,
        );
        let digest = WorthQueryGraphReadAccessRequirementSetDigest::from_parts(&parts);
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
    rows: &[WorthQueryGraphReadAccessRequirementRow],
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
