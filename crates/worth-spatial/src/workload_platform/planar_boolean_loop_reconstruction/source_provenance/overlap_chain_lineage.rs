use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

use super::identity::{overlap_chain_lineage_identity, overlap_chain_lineage_map_identity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopOverlapChainLineageRow {
    lineage_identity: String,
    chain_identity: String,
    member_identities: Vec<String>,
    fragment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
}

impl PlanarBooleanLoopOverlapChainLineageRow {
    pub(crate) fn new(
        lineage_identity: String,
        chain_identity: String,
        member_identities: Vec<String>,
        fragment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    ) -> Self {
        Self {
            lineage_identity,
            chain_identity,
            member_identities,
            fragment_identities,
            source_loop_identities,
            source_edge_identities,
            boundary_roles,
        }
    }

    pub fn lineage_identity(&self) -> &str {
        &self.lineage_identity
    }

    pub fn chain_identity(&self) -> &str {
        &self.chain_identity
    }

    pub fn member_identities(&self) -> &[String] {
        &self.member_identities
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn source_edge_identities(&self) -> &[String] {
        &self.source_edge_identities
    }

    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }

    pub(crate) fn certifies_canonical_identity(&self, request_identity: &str) -> bool {
        self.lineage_identity
            == overlap_chain_lineage_identity(
                request_identity,
                self.chain_identity(),
                self.member_identities(),
                self.fragment_identities(),
                self.source_loop_identities(),
                self.source_edge_identities(),
                self.boundary_roles(),
            )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopOverlapChainLineageMap {
    lineage_map_identity: String,
    request_identity: String,
    overlap_chain_set_identity: String,
    rows: Vec<PlanarBooleanLoopOverlapChainLineageRow>,
    chain_offsets: BTreeMap<String, usize>,
}

impl PlanarBooleanLoopOverlapChainLineageMap {
    pub(crate) fn new(
        lineage_map_identity: String,
        request_identity: String,
        overlap_chain_set_identity: String,
        rows: Vec<PlanarBooleanLoopOverlapChainLineageRow>,
    ) -> Self {
        let chain_offsets = rows
            .iter()
            .enumerate()
            .map(|(offset, row)| (row.chain_identity().to_string(), offset))
            .collect();
        Self {
            lineage_map_identity,
            request_identity,
            overlap_chain_set_identity,
            rows,
            chain_offsets,
        }
    }

    pub fn lineage_map_identity(&self) -> &str {
        &self.lineage_map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn overlap_chain_set_identity(&self) -> &str {
        &self.overlap_chain_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopOverlapChainLineageRow] {
        &self.rows
    }

    pub fn lineage_for_chain_identity(
        &self,
        chain_identity: &str,
    ) -> Option<&PlanarBooleanLoopOverlapChainLineageRow> {
        self.chain_offsets
            .get(chain_identity)
            .and_then(|offset| self.rows.get(*offset))
    }

    pub(crate) fn certifies_canonical_identities(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.certifies_canonical_identity(self.request_identity()))
            && self.lineage_map_identity
                == overlap_chain_lineage_map_identity(self.request_identity(), &self.rows)
    }
}
