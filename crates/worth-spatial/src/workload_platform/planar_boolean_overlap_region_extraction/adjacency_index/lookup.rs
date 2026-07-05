use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanLoopIslandOverlapParticipationRow, PlanarBooleanLoopOverlapParticipationRow,
    PlanarBooleanOverlapChainRegionLineageRow,
};

pub(crate) struct ValidatedOverlapAdjacencyLookup<'a> {
    loop_rows_by_identity: BTreeMap<&'a str, &'a PlanarBooleanLoopOverlapParticipationRow>,
    island_rows_by_identity: BTreeMap<&'a str, &'a PlanarBooleanLoopIslandOverlapParticipationRow>,
    lineage_rows_by_identity: BTreeMap<&'a str, &'a PlanarBooleanOverlapChainRegionLineageRow>,
    lineage_rows_by_chain: BTreeMap<&'a str, Vec<&'a PlanarBooleanOverlapChainRegionLineageRow>>,
}

pub(crate) struct OverlapAdjacencyNeighborhoodComponent<'a> {
    chain_identity: &'a str,
    lineage_rows: Vec<&'a PlanarBooleanOverlapChainRegionLineageRow>,
    participating_loop_identities: Vec<String>,
    participating_island_identities: Vec<String>,
}

impl<'a> ValidatedOverlapAdjacencyLookup<'a> {
    pub(crate) fn new() -> Self {
        Self {
            loop_rows_by_identity: BTreeMap::new(),
            island_rows_by_identity: BTreeMap::new(),
            lineage_rows_by_identity: BTreeMap::new(),
            lineage_rows_by_chain: BTreeMap::new(),
        }
    }

    pub(crate) fn insert_loop_row(
        &mut self,
        row: &'a PlanarBooleanLoopOverlapParticipationRow,
    ) -> bool {
        self.loop_rows_by_identity
            .insert(row.canonical_loop_identity(), row)
            .is_none()
    }

    pub(crate) fn insert_island_row(
        &mut self,
        row: &'a PlanarBooleanLoopIslandOverlapParticipationRow,
    ) -> bool {
        self.island_rows_by_identity
            .insert(row.island_identity(), row)
            .is_none()
    }

    pub(crate) fn insert_lineage_row(
        &mut self,
        row: &'a PlanarBooleanOverlapChainRegionLineageRow,
    ) -> bool {
        self.lineage_rows_by_chain
            .entry(row.chain_identity())
            .or_default()
            .push(row);
        self.lineage_rows_by_identity
            .insert(row.lineage_identity(), row)
            .is_none()
    }

    pub(crate) fn loop_row(
        &self,
        canonical_loop_identity: &str,
    ) -> Option<&'a PlanarBooleanLoopOverlapParticipationRow> {
        self.loop_rows_by_identity
            .get(canonical_loop_identity)
            .copied()
    }

    pub(crate) fn island_row(
        &self,
        island_identity: &str,
    ) -> Option<&'a PlanarBooleanLoopIslandOverlapParticipationRow> {
        self.island_rows_by_identity.get(island_identity).copied()
    }

    pub(crate) fn has_lineage_identity(&self, lineage_identity: &str) -> bool {
        self.lineage_rows_by_identity.contains_key(lineage_identity)
    }

    pub(crate) fn neighborhood_components(&self) -> Vec<OverlapAdjacencyNeighborhoodComponent<'a>> {
        self.lineage_rows_by_chain
            .iter()
            .map(
                |(chain_identity, lineage_rows)| OverlapAdjacencyNeighborhoodComponent {
                    chain_identity,
                    lineage_rows: lineage_rows.clone(),
                    participating_loop_identities: sorted_unique(
                        lineage_rows
                            .iter()
                            .flat_map(|row| row.participating_loop_identities().iter().cloned()),
                    ),
                    participating_island_identities: sorted_unique(
                        lineage_rows
                            .iter()
                            .flat_map(|row| row.participating_island_identities().iter().cloned()),
                    ),
                },
            )
            .collect()
    }
}

impl<'a> OverlapAdjacencyNeighborhoodComponent<'a> {
    pub(crate) fn chain_identity(&self) -> &str {
        self.chain_identity
    }

    pub(crate) fn lineage_rows(&self) -> &[&'a PlanarBooleanOverlapChainRegionLineageRow] {
        &self.lineage_rows
    }

    pub(crate) fn participating_loop_identities(&self) -> &[String] {
        &self.participating_loop_identities
    }

    pub(crate) fn participating_island_identities(&self) -> &[String] {
        &self.participating_island_identities
    }

    pub(crate) fn connectivity_identity(&self) -> String {
        format!(
            "{}:{}:{}",
            self.chain_identity,
            self.participating_loop_identities.join("|"),
            self.participating_island_identities.join("|"),
        )
    }
}

pub(crate) fn sorted_unique(values: impl Iterator<Item = String>) -> Vec<String> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}
