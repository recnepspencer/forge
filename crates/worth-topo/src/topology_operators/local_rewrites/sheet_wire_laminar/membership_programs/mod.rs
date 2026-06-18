mod face_inner_loop_program;
mod shared;
mod shell_membership_program;
mod shell_split_program;
mod wire_membership_program;

use forge_query::facade::ForgeQueryBatchWriteReceipt;

use crate::query_domain::TopologyQueryDomain;
use crate::topology_operators::application::{
    finalize_graph_or_batch_receipt_closeout, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
    TopologyRetainedApplicationHandoff,
};
use crate::topology_operators::{
    TopologyDeclaredMutationSequence, TopologyMutationApplicationMode,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn finish_composed_membership_execution<I>(
        &mut self,
        _mode: TopologyMutationApplicationMode,
        retained_handoff: TopologyRetainedApplicationHandoff<I>,
        semantic_family_key: &'static str,
        sequence: &TopologyDeclaredMutationSequence,
        receipt: ForgeQueryBatchWriteReceipt,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
    where
        I: forge_query::facade::ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        finalize_graph_or_batch_receipt_closeout(
            self,
            retained_handoff,
            semantic_family_key,
            sequence,
            receipt,
            crate::projection::runtime_boundary::query_runtime::TopologyQueryMutationLaneExecutionShape::GraphComposition,
        )
    }
}
