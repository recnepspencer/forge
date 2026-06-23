use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection,
    ForgeQueryBatchWriteRetainedArtifact, ForgeQueryWorkspace,
};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::TopologyQueryMutationLaneExecutionShape;
use crate::topology_operators::application::TopologyMutationApplicationError;

#[derive(Debug, Clone)]
pub(crate) struct TopologyPostWriteQueryArtifact {
    materialized: MaterializedTopologyView,
    retained_artifact: ForgeQueryBatchWriteRetainedArtifact,
    #[cfg(test)]
    execution_shape: TopologyQueryMutationLaneExecutionShape,
}

impl TopologyPostWriteQueryArtifact {
    pub(crate) fn build(
        workspace: &mut ForgeQueryWorkspace,
        surfaces: &TopologyDeclaredQuerySurfaces,
        receipt: ForgeQueryBatchWriteReceipt,
        execution_shape: TopologyQueryMutationLaneExecutionShape,
    ) -> Result<Self, TopologyMutationApplicationError> {
        let retained_artifact = workspace.materialize_batch_write_artifact_binding(
            &receipt,
            "worth_topology.post_write_materialized",
            [surfaces.materialized().into()],
        )?;
        let materialized = retained_artifact
            .retained_artifact()
            .decode_single_row(surfaces.materialized())
            .map_err(|error| {
                TopologyMutationApplicationError::MaterializedDecode(format!(
                    "query-derived `materialized topology` row failed to decode: {error}"
                ))
            })?;
        #[cfg(not(test))]
        let _ = execution_shape;
        Ok(Self {
            materialized,
            retained_artifact,
            #[cfg(test)]
            execution_shape,
        })
    }

    #[cfg(test)]
    pub(crate) fn receipt(&self) -> &ForgeQueryBatchWriteReceipt {
        self.retained_artifact.receipt()
    }

    pub(crate) fn inspection(&self) -> &ForgeQueryBatchWriteReceiptInspection {
        self.retained_artifact.inspection()
    }

    pub(crate) fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    #[cfg(test)]
    pub(crate) fn execution_shape(&self) -> TopologyQueryMutationLaneExecutionShape {
        self.execution_shape
    }

    pub(crate) fn into_materialized(self) -> MaterializedTopologyView {
        self.materialized
    }
}
