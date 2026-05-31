use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryInspection,
};

use crate::projection::runtime_boundary::query_runtime::{
    load_post_write_materialized_topology, TopologyQueryBindingIndex,
};
use crate::topology_operators::application::{
    TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::local_rewrites::boundary_wiring::ResolvedLoopSuccessorRewire;
use crate::topology_operators::{
    NamingEditContinuityMatrix, TopologyEditAction, TopologyEditApplicationMode,
    TopologyEditContract, TopologyEditDigest, TopologyEditFamily,
};

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn execute_composed_loop_successor_program(
        &mut self,
        semantic_family_key: &'static str,
        _mode: TopologyEditApplicationMode,
        families: Vec<TopologyEditFamily>,
        topology_edit_digest: TopologyEditDigest,
        naming_continuity_matrix: NamingEditContinuityMatrix,
        naming_report: crate::topology_operators::TopologyEditNamingReport,
        contracts: Vec<TopologyEditContract>,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyOperatorExecutionError> {
        let rewires = contracts
            .iter()
            .map(|contract| match contract.action {
                TopologyEditAction::RewireLoopSuccessor {
                    relation_id,
                    kind,
                    half_edge_id,
                    successor_half_edge_id,
                } => self.resolve_loop_successor_rewire(
                    bindings,
                    relation_id,
                    kind,
                    half_edge_id,
                    successor_half_edge_id,
                ),
                _ => Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                    TopologyEditFamily::RewireLoopSuccessor,
                ])),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let receipt: ForgeQueryBatchWriteReceipt = self.workspace.compose_graph(|graph| {
            for rewire in &rewires {
                add_verified_successor_retarget(graph, rewire)?;
            }
            Ok(())
        })?;
        let inspection: ForgeQueryBatchWriteReceiptInspection =
            match self.workspace.inspect(&receipt)? {
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

fn add_verified_successor_retarget(
    graph: &mut forge_query::facade::ForgeQueryGraphCompositionBuilder,
    rewire: &ResolvedLoopSuccessorRewire,
) -> Result<(), forge_query::facade::ForgeQueryRuntimeError> {
    let relation_kind = rewire.relation_kind.kind_name().to_string();
    let relation_kind_verify = relation_kind.clone();
    let relation_kind_update = relation_kind.clone();
    let authoritative_identity = rewire.authoritative_identity.clone();
    let successor_authoritative_identity = rewire.successor_authoritative_identity.clone();
    let verify_source = rewire.source_query_identity.clone();
    let verify_target = rewire.current_target_query_identity.clone();
    let update_source = rewire.source_query_identity.clone();
    let update_target = rewire.updated_target_query_identity.clone();
    let dependency_path = rewire.dependency_path.clone();
    graph.retarget_existing_verified(
        rewire.binding.clone(),
        |verify| {
            let verify = verify
                .aspect("topology.kind", relation_kind_verify.clone())
                .aspect("topology.source_identity", verify_source)
                .aspect("topology.target_identity", verify_target);
            if let Some(path) = dependency_path.clone() {
                verify.aspect(path, relation_kind_verify.clone())
            } else {
                verify
            }
        },
        |update| {
            let update = update
                .continuity_rebind_existing_target(
                    authoritative_identity,
                    successor_authoritative_identity,
                )
                .aspect("topology.kind", relation_kind_update.clone())
                .aspect("topology.source_identity", update_source)
                .aspect("topology.target_identity", update_target);
            if let Some(path) = dependency_path {
                update.aspect(path, relation_kind_update)
            } else {
                update
            }
        },
    )
}
