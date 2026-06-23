use std::collections::{BTreeMap, BTreeSet};

use super::counters::PlanarBooleanSplitEdgeChainLedgerCounters;
use super::denial::{
    PlanarBooleanSplitEdgeChainLedgerDenial, PlanarBooleanSplitEdgeChainLedgerDenialKind,
};
use super::input::PlanarBooleanSplitEdgeChainLedgerInput;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitDecisionRow, PlanarBooleanSplitEdgeFragment,
    PlanarBooleanSplitPersistentNameRow, PlanarBooleanSplitVertexIdentityRow,
};

type EdgeKey = (String, String);

pub(crate) struct PlanarBooleanSplitEdgeChainProductIndex<'a> {
    vertices_by_edge: BTreeMap<EdgeKey, Vec<&'a PlanarBooleanSplitVertexIdentityRow>>,
    fragments_by_edge: BTreeMap<EdgeKey, Vec<&'a PlanarBooleanSplitEdgeFragment>>,
    overlap_chains_by_edge: BTreeMap<EdgeKey, Vec<String>>,
    names_by_artifact: BTreeMap<String, &'a PlanarBooleanSplitPersistentNameRow>,
    decisions_by_artifact: BTreeMap<String, Vec<&'a PlanarBooleanSplitDecisionRow>>,
    fragment_coverage_by_edge: BTreeMap<EdgeKey, Vec<String>>,
    overlap_coverage_by_edge: BTreeMap<EdgeKey, Vec<String>>,
}

impl<'a> PlanarBooleanSplitEdgeChainProductIndex<'a> {
    pub(crate) fn build(
        input: &'a PlanarBooleanSplitEdgeChainLedgerInput<'a>,
        counters: &mut PlanarBooleanSplitEdgeChainLedgerCounters,
    ) -> Result<Self, PlanarBooleanSplitEdgeChainLedgerDenial> {
        let mut vertices_by_edge = BTreeMap::<EdgeKey, Vec<_>>::new();
        for vertex in input.split_vertices().vertices() {
            vertices_by_edge
                .entry(edge_key(
                    vertex.source_edge_identity(),
                    vertex.carrier_identity(),
                ))
                .or_default()
                .push(vertex);
        }

        let mut fragments_by_edge = BTreeMap::<EdgeKey, Vec<_>>::new();
        for fragment in input.split_fragments().fragments() {
            counters.consumed_fragment();
            fragments_by_edge
                .entry(edge_key(
                    fragment.source_edge_identity(),
                    fragment.carrier_identity(),
                ))
                .or_default()
                .push(fragment);
        }

        let mut overlap_chains_by_edge = BTreeMap::<EdgeKey, Vec<String>>::new();
        for chain in input.overlap_chains().chains() {
            counters.consumed_overlap_chain();
            for member in chain.members() {
                overlap_chains_by_edge
                    .entry(edge_key(
                        member.source_edge_identity(),
                        member.carrier_identity(),
                    ))
                    .or_default()
                    .push(chain.chain_identity().to_string());
            }
        }
        dedup_values(&mut overlap_chains_by_edge);

        let mut names_by_artifact = BTreeMap::new();
        for row in input.split_persistent_names().persistent_name_rows() {
            counters.bound_persistent_name();
            names_by_artifact.insert(row.artifact_identity().to_string(), row);
        }

        let mut decisions_by_artifact = BTreeMap::<String, Vec<_>>::new();
        for row in input.split_decision_log().receipt().decision_rows() {
            counters.bound_decision();
            decisions_by_artifact
                .entry(row.affected_artifact_identity().to_string())
                .or_default()
                .push(row);
        }

        let mut fragment_coverage_by_edge = BTreeMap::<EdgeKey, Vec<String>>::new();
        for row in input.split_chain_validation().fragment_coverage_rows() {
            fragment_coverage_by_edge
                .entry(edge_key(row.source_edge_identity(), row.carrier_identity()))
                .or_default()
                .push(row.row_identity().to_string());
        }

        let mut overlap_coverage_by_edge = BTreeMap::<EdgeKey, Vec<String>>::new();
        for row in input.split_chain_validation().overlap_coverage_rows() {
            overlap_coverage_by_edge
                .entry(edge_key(row.source_edge_identity(), row.carrier_identity()))
                .or_default()
                .push(row.row_identity().to_string());
        }

        reject_missing_artifact_bindings(
            input,
            &names_by_artifact,
            &decisions_by_artifact,
            counters,
        )?;

        Ok(Self {
            vertices_by_edge,
            fragments_by_edge,
            overlap_chains_by_edge,
            names_by_artifact,
            decisions_by_artifact,
            fragment_coverage_by_edge,
            overlap_coverage_by_edge,
        })
    }

    pub(crate) fn edge_keys(&self) -> Vec<EdgeKey> {
        self.fragments_by_edge.keys().cloned().collect()
    }

    pub(crate) fn vertex_identities(&self, key: &EdgeKey) -> Vec<String> {
        self.vertices_by_edge
            .get(key)
            .into_iter()
            .flatten()
            .map(|row| row.split_vertex_identity().to_string())
            .collect()
    }

    pub(crate) fn fragment_identities(&self, key: &EdgeKey) -> Vec<String> {
        self.fragments_by_edge
            .get(key)
            .into_iter()
            .flatten()
            .map(|row| row.fragment_identity().to_string())
            .collect()
    }

    pub(crate) fn overlap_chain_identities(&self, key: &EdgeKey) -> Vec<String> {
        self.overlap_chains_by_edge
            .get(key)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn name_row_identities(&self, artifact_identities: &[String]) -> Vec<String> {
        artifact_identities
            .iter()
            .filter_map(|identity| self.names_by_artifact.get(identity))
            .map(|row| row.row_identity().to_string())
            .collect()
    }

    pub(crate) fn decision_identities(&self, artifact_identities: &[String]) -> Vec<String> {
        artifact_identities
            .iter()
            .filter_map(|identity| self.decisions_by_artifact.get(identity))
            .flatten()
            .map(|row| row.decision_identity().to_string())
            .collect()
    }

    pub(crate) fn fragment_coverage_identities(&self, key: &EdgeKey) -> Vec<String> {
        self.fragment_coverage_by_edge
            .get(key)
            .cloned()
            .unwrap_or_default()
    }

    pub(crate) fn overlap_coverage_identities(&self, key: &EdgeKey) -> Vec<String> {
        self.overlap_coverage_by_edge
            .get(key)
            .cloned()
            .unwrap_or_default()
    }
}

fn reject_missing_artifact_bindings(
    input: &PlanarBooleanSplitEdgeChainLedgerInput<'_>,
    names_by_artifact: &BTreeMap<String, &PlanarBooleanSplitPersistentNameRow>,
    decisions_by_artifact: &BTreeMap<String, Vec<&PlanarBooleanSplitDecisionRow>>,
    counters: &mut PlanarBooleanSplitEdgeChainLedgerCounters,
) -> Result<(), PlanarBooleanSplitEdgeChainLedgerDenial> {
    let mut required = BTreeSet::new();
    for fragment in input.split_fragments().fragments() {
        required.insert(fragment.fragment_identity().to_string());
    }
    for vertex in input.split_vertices().vertices() {
        required.insert(vertex.split_vertex_identity().to_string());
    }
    for chain in input.overlap_chains().chains() {
        required.insert(chain.chain_identity().to_string());
    }

    for identity in required {
        if !names_by_artifact.contains_key(&identity) {
            counters.rejected_missing_persistent_name();
            return Err(PlanarBooleanSplitEdgeChainLedgerDenial::new(
                PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingPersistentNameBinding,
                identity,
                *counters,
                "split ledger requires persistent-name rows for every split artifact",
            ));
        }
        if !decisions_by_artifact.contains_key(&identity) {
            counters.rejected_missing_decision_log();
            return Err(PlanarBooleanSplitEdgeChainLedgerDenial::new(
                PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingDecisionBinding,
                identity,
                *counters,
                "split ledger requires decision-log rows for every split artifact",
            ));
        }
    }
    Ok(())
}

fn edge_key(source_edge_identity: &str, carrier_identity: &str) -> EdgeKey {
    (
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
    )
}

fn dedup_values(map: &mut BTreeMap<EdgeKey, Vec<String>>) {
    for values in map.values_mut() {
        values.sort();
        values.dedup();
    }
}
