mod face_inner_loop_program;
mod shared;
mod shell_membership_program;
mod wire_membership_program;

use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryInspection};

use crate::projection::runtime_boundary::query_runtime::load_post_write_materialized_topology;
use crate::topology_operators::application::{
    TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::{
    NamingEditContinuityMatrix, TopologyEditApplicationMode, TopologyEditDigest, TopologyEditFamily,
};

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn finish_composed_membership_execution(
        &mut self,
        _mode: TopologyEditApplicationMode,
        semantic_family_key: &'static str,
        families: Vec<TopologyEditFamily>,
        topology_edit_digest: TopologyEditDigest,
        naming_continuity_matrix: NamingEditContinuityMatrix,
        naming_report: crate::topology_operators::TopologyEditNamingReport,
        receipt: ForgeQueryBatchWriteReceipt,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        let inspection = match self.workspace.inspect(&receipt)? {
            ForgeQueryInspection::BatchWriteReceipt(inspection) => inspection,
            _ => return Err(TopologyOperatorExecutionError::UnexpectedInspectionFamily),
        };
        let materialized = load_post_write_materialized_topology(self.workspace, self.surfaces)?;
        Ok(TopologyDeclaredMutationArtifact {
            semantic_family_key,
            families,
            receipt,
            inspection,
            materialized,
            topology_edit_digest,
            naming_continuity_matrix,
            naming_report,
        })
    }
}
