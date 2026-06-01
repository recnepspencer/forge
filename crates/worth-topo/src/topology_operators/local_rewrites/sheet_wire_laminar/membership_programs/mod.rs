mod face_inner_loop_program;
mod shared;
mod shell_membership_program;
mod wire_membership_program;

use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryInspection};

use crate::projection::runtime_boundary::query_runtime::load_post_write_materialized_topology;
use crate::topology_operators::application::{
    TopologyDeclaredMutationArtifact, TopologyMutationApplicationError,
    TopologyMutationApplicationRunner,
};
use crate::topology_operators::{
    TopologyDeclaredMutationSequence, TopologyMutationApplicationMode,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn finish_composed_membership_execution(
        &mut self,
        _mode: TopologyMutationApplicationMode,
        semantic_family_key: &'static str,
        sequence: &TopologyDeclaredMutationSequence,
        receipt: ForgeQueryBatchWriteReceipt,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> {
        let inspection = match self.workspace.inspect(&receipt)? {
            ForgeQueryInspection::BatchWriteReceipt(inspection) => inspection,
            _ => return Err(TopologyMutationApplicationError::UnexpectedInspectionFamily),
        };
        let materialized = load_post_write_materialized_topology(self.workspace, self.surfaces)?;
        Ok(TopologyDeclaredMutationArtifact::from_receipt(
            semantic_family_key,
            sequence,
            receipt,
            inspection,
            materialized,
        ))
    }
}
