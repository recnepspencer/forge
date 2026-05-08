mod face_inner_loop;
mod shared;
mod shell_workflow;
mod wire_workflow;

use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryEntity, ForgeQueryInspection};

use super::relation_boundary::supports_admitted_relation_create_workflow;
use super::relation_shell_face_rehome_support::{
    parse_shell_face_rehome_workflow, resolve_single_face_two_face_shell_split_workflow,
};
use super::relation_shell_or_wire::supports_admitted_shell_or_wire_create_workflow;
use super::relation_wire_rehome_support::{
    parse_wire_rehome_workflow, resolve_wire_split_workflow,
};
use super::{TopologyQueryEditExecution, TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::{
    NamingEditContinuityMatrix, TopologyEditApplicationMode, TopologyEditContract,
    TopologyEditDigest, TopologyEditFamily,
};

pub(super) fn supports_graph_composed_membership_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[TopologyEditContract],
) -> bool {
    supports_admitted_relation_create_workflow(contracts)
        || supports_admitted_shell_or_wire_create_workflow(entity_rows, relation_rows, contracts)
}

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn apply_graph_composed_membership_workflow(
        &mut self,
        mode: TopologyEditApplicationMode,
        families: Vec<TopologyEditFamily>,
        topology_edit_digest: TopologyEditDigest,
        naming_continuity_matrix: NamingEditContinuityMatrix,
        naming_report: crate::edit::TopologyEditNamingReport,
        contracts: &[TopologyEditContract],
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<TopologyQueryEditExecution, TopologyQueryEditExecutionError> {
        let receipt = if supports_admitted_relation_create_workflow(contracts) {
            self.compose_face_inner_loop_workflow(contracts, entity_rows)?
        } else if let Some(workflow) = parse_shell_face_rehome_workflow(contracts) {
            self.compose_shell_rehome_workflow(workflow, contracts, entity_rows, relation_rows)?
        } else if let Some(workflow) =
            resolve_single_face_two_face_shell_split_workflow(entity_rows, relation_rows, contracts)
        {
            self.compose_shell_split_workflow(workflow, entity_rows, relation_rows)?
        } else if let Some(workflow) = parse_wire_rehome_workflow(contracts) {
            self.compose_wire_rehome_workflow(workflow, contracts, entity_rows, relation_rows)?
        } else if let Some(workflow) =
            resolve_wire_split_workflow(entity_rows, relation_rows, contracts)
        {
            self.compose_wire_split_workflow(workflow, entity_rows, relation_rows)?
        } else {
            return Err(TopologyQueryEditExecutionError::UnsupportedFamilies(vec![
                TopologyEditFamily::AttachBoundaryMembership,
                TopologyEditFamily::AttachShellOrWireMembership,
            ]));
        };
        self.finish_graph_membership_execution(
            mode,
            families,
            topology_edit_digest,
            naming_continuity_matrix,
            naming_report,
            receipt,
        )
    }

    fn finish_graph_membership_execution(
        &mut self,
        mode: TopologyEditApplicationMode,
        families: Vec<TopologyEditFamily>,
        topology_edit_digest: TopologyEditDigest,
        naming_continuity_matrix: NamingEditContinuityMatrix,
        naming_report: crate::edit::TopologyEditNamingReport,
        receipt: ForgeQueryBatchWriteReceipt,
    ) -> Result<TopologyQueryEditExecution, TopologyQueryEditExecutionError> {
        let inspection = match self.workspace.inspect(&receipt)? {
            ForgeQueryInspection::BatchWriteReceipt(inspection) => inspection,
            _ => return Err(TopologyQueryEditExecutionError::UnexpectedInspectionFamily),
        };
        let materialized_rows = self.workspace.materialize(self.assembly.materialized());
        let materialized =
            serde_json::from_value(materialized_rows[0].clone()).map_err(|error| {
                TopologyQueryEditExecutionError::MaterializedDecode(format!(
                    "query-derived `materialized topology` row failed to decode: {error}"
                ))
            })?;
        Ok(TopologyQueryEditExecution {
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
