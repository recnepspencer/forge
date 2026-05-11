use forge_query::facade::{ForgeQueryEntity, ForgeQueryMutationBatchBuilder};

use super::shell_face_rehome_support::{
    resolve_single_face_two_face_shell_split_program, supports_owned_face_set_shell_rehome_program,
};
use super::wire_rehome_support::{
    supports_connected_wire_split_program, supports_owned_half_edge_set_wire_rehome_program,
};
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::TopologyEditContract;

pub(crate) fn supports_admitted_shell_or_wire_create_program(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[TopologyEditContract],
) -> bool {
    supports_owned_half_edge_set_wire_rehome_program(entity_rows, relation_rows, contracts)
        || supports_connected_wire_split_program(entity_rows, relation_rows, contracts)
        || resolve_single_face_two_face_shell_split_program(entity_rows, relation_rows, contracts)
            .is_some()
        || supports_owned_face_set_shell_rehome_program(entity_rows, relation_rows, contracts)
}

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn lower_admitted_shell_or_wire_create_program(
        &self,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        if supports_owned_half_edge_set_wire_rehome_program(entity_rows, relation_rows, contracts) {
            self.lower_rehome_owned_half_edge_set_to_new_wire_program(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else if supports_connected_wire_split_program(entity_rows, relation_rows, contracts) {
            self.lower_split_connected_half_edge_set_to_new_wire_program(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else if resolve_single_face_two_face_shell_split_program(
            entity_rows,
            relation_rows,
            contracts,
        )
        .is_some()
        {
            self.lower_split_single_face_from_two_face_shell_to_new_shell_program(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else if supports_owned_face_set_shell_rehome_program(
            entity_rows,
            relation_rows,
            contracts,
        ) {
            self.lower_rehome_owned_face_set_to_new_shell_program(
                entity_rows,
                relation_rows,
                contracts,
            )
        } else {
            Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                crate::topology_operators::TopologyEditFamily::AttachShellOrWireMembership,
            ]))
        }
    }
}
