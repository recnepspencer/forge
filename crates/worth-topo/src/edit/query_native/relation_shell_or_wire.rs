use forge_query::facade::{ForgeQueryEntity, ForgeQueryMutationBatchBuilder};

use super::relation_shell_face_rehome_support::{
    resolve_single_face_two_face_shell_split_workflow,
    supports_owned_face_set_shell_rehome_workflow,
};
use super::relation_wire_rehome_support::{
    supports_connected_wire_split_workflow, supports_owned_half_edge_set_wire_rehome_workflow,
};
use super::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::TopologyEditContract;

pub(super) fn supports_admitted_shell_or_wire_create_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[TopologyEditContract],
) -> bool {
    supports_owned_half_edge_set_wire_rehome_workflow(entity_rows, relation_rows, contracts)
        || supports_connected_wire_split_workflow(entity_rows, relation_rows, contracts)
        || resolve_single_face_two_face_shell_split_workflow(entity_rows, relation_rows, contracts)
            .is_some()
        || supports_owned_face_set_shell_rehome_workflow(entity_rows, relation_rows, contracts)
}

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_admitted_shell_or_wire_create_workflow(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        if supports_owned_half_edge_set_wire_rehome_workflow(entity_rows, relation_rows, contracts)
        {
            self.lower_rehome_owned_half_edge_set_to_new_wire_workflow(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else if supports_connected_wire_split_workflow(entity_rows, relation_rows, contracts) {
            self.lower_split_connected_half_edge_set_to_new_wire_workflow(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else if resolve_single_face_two_face_shell_split_workflow(
            entity_rows,
            relation_rows,
            contracts,
        )
        .is_some()
        {
            self.lower_split_single_face_from_two_face_shell_to_new_shell_workflow(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else if supports_owned_face_set_shell_rehome_workflow(
            entity_rows,
            relation_rows,
            contracts,
        ) {
            self.lower_rehome_owned_face_set_to_new_shell_workflow(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else {
            Err(TopologyQueryEditExecutionError::UnsupportedFamilies(vec![
                crate::edit::TopologyEditFamily::AttachShellOrWireMembership,
            ]))
        }
    }
}
