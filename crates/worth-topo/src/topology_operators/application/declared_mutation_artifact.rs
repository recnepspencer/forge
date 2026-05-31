use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::topology_operators::{
    NamingEditContinuityMatrix, TopologyEditDigest, TopologyEditFamily, TopologyEditNamingReport,
};

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct TopologyDeclaredMutationArtifact {
    pub(crate) semantic_family_key: &'static str,
    pub(crate) families: Vec<TopologyEditFamily>,
    pub(crate) receipt: ForgeQueryBatchWriteReceipt,
    pub(crate) inspection: ForgeQueryBatchWriteReceiptInspection,
    pub(crate) materialized: MaterializedTopologyView,
    pub(crate) topology_edit_digest: TopologyEditDigest,
    pub(crate) naming_continuity_matrix: NamingEditContinuityMatrix,
    pub(crate) naming_report: TopologyEditNamingReport,
}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyDeclaredMutationArtifact {
    pub(crate) fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }
}
