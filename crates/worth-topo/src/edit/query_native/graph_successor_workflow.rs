use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryEntity,
    ForgeQueryInspection,
};

use super::relation_successor::supports_admitted_loop_successor_workflow;
use super::relation_update::ResolvedLoopSuccessorRewire;
use super::{
    WorthTopologyQueryEditExecution, WorthTopologyQueryEditExecutionError,
    WorthTopologyQueryEditRunner,
};
use crate::edit::{
    WorthNamingEditContinuityMatrix, WorthTopologyEditAction, WorthTopologyEditApplicationMode,
    WorthTopologyEditContract, WorthTopologyEditDigest, WorthTopologyEditFamily,
};
use crate::materialization::MaterializedTopologyView;

pub(super) fn supports_graph_composed_loop_successor_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[WorthTopologyEditContract],
) -> bool {
    supports_admitted_loop_successor_workflow(entity_rows, relation_rows, contracts)
}

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn apply_graph_composed_loop_successor_workflow(
        &mut self,
        mode: WorthTopologyEditApplicationMode,
        families: Vec<WorthTopologyEditFamily>,
        topology_edit_digest: WorthTopologyEditDigest,
        naming_continuity_matrix: WorthNamingEditContinuityMatrix,
        naming_report: crate::edit::WorthTopologyEditNamingReport,
        contracts: &[WorthTopologyEditContract],
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<WorthTopologyQueryEditExecution, WorthTopologyQueryEditExecutionError> {
        let rewires = contracts
            .iter()
            .map(|contract| match contract.action {
                WorthTopologyEditAction::RewireLoopSuccessor {
                    relation_id,
                    kind,
                    half_edge_id,
                    successor_half_edge_id,
                } => self.resolve_loop_successor_rewire(
                    entity_rows,
                    relation_rows,
                    relation_id,
                    kind,
                    half_edge_id,
                    successor_half_edge_id,
                ),
                _ => Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                    vec![WorthTopologyEditFamily::RewireLoopSuccessor],
                )),
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
                _ => return Err(WorthTopologyQueryEditExecutionError::UnexpectedInspectionFamily),
            };
        let materialized_rows = self.workspace.materialize(self.assembly.materialized());
        let materialized: MaterializedTopologyView =
            serde_json::from_value(materialized_rows[0].clone()).map_err(|error| {
                WorthTopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "query-derived `materialized topology` row failed to decode: {error}"
                ))
            })?;
        Ok(WorthTopologyQueryEditExecution {
            mode,
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
