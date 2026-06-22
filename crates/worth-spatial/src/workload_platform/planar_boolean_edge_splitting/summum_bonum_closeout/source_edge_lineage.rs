use std::collections::{BTreeMap, BTreeSet};

use super::input::PlanarBooleanEdgeSplitSummumBonumCloseoutInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitCloseoutLineageRow {
    source_edge_identity: String,
    carrier_identity: String,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    overlap_chain_identities: Vec<String>,
}

impl PlanarBooleanEdgeSplitCloseoutLineageRow {
    pub(crate) fn new(
        source_edge_identity: String,
        carrier_identity: String,
        fragment_identities: Vec<String>,
        split_vertex_identities: Vec<String>,
        overlap_chain_identities: Vec<String>,
    ) -> Self {
        Self {
            source_edge_identity,
            carrier_identity,
            fragment_identities,
            split_vertex_identities,
            overlap_chain_identities,
        }
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }
    pub fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }
    pub fn overlap_chain_identities(&self) -> &[String] {
        &self.overlap_chain_identities
    }
}

pub(crate) fn closeout_source_edge_lineage_rows(
    input: PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
) -> Vec<PlanarBooleanEdgeSplitCloseoutLineageRow> {
    let mut rows = BTreeMap::<(String, String), LineageAcc>::new();
    record_fragment_lineage(input, &mut rows);
    record_split_vertex_lineage(input, &mut rows);
    record_overlap_chain_lineage(input, &mut rows);
    rows.into_iter()
        .map(|((source_edge_identity, carrier_identity), row)| {
            PlanarBooleanEdgeSplitCloseoutLineageRow::new(
                source_edge_identity,
                carrier_identity,
                row.fragments.into_iter().collect(),
                row.vertices.into_iter().collect(),
                row.chains.into_iter().collect(),
            )
        })
        .collect()
}

fn record_fragment_lineage(
    input: PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
    rows: &mut BTreeMap<(String, String), LineageAcc>,
) {
    for fragment in input.fragments().fragments() {
        rows.entry((
            fragment.source_edge_identity().to_string(),
            fragment.carrier_identity().to_string(),
        ))
        .or_default()
        .fragments
        .insert(fragment.fragment_identity().to_string());
    }
}

fn record_split_vertex_lineage(
    input: PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
    rows: &mut BTreeMap<(String, String), LineageAcc>,
) {
    for vertex in input.vertices().vertices() {
        rows.entry((
            vertex.source_edge_identity().to_string(),
            vertex.carrier_identity().to_string(),
        ))
        .or_default()
        .vertices
        .insert(vertex.split_vertex_identity().to_string());
    }
}

fn record_overlap_chain_lineage(
    input: PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'_>,
    rows: &mut BTreeMap<(String, String), LineageAcc>,
) {
    for chain in input.overlap_chains().chains() {
        for member in chain.members() {
            rows.entry((
                member.source_edge_identity().to_string(),
                member.carrier_identity().to_string(),
            ))
            .or_default()
            .chains
            .insert(chain.chain_identity().to_string());
        }
    }
}

#[derive(Default)]
struct LineageAcc {
    fragments: BTreeSet<String>,
    vertices: BTreeSet<String>,
    chains: BTreeSet<String>,
}
