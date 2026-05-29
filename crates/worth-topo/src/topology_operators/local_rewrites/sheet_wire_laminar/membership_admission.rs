use forge_query::facade::ForgeQueryMutationBatchBuilder;

use super::shell_face_rehome_support::{
    resolve_single_face_two_face_shell_split_program, supports_owned_face_set_shell_rehome_program,
};
use super::wire_rehome_support::{
    supports_connected_wire_split_program, supports_owned_half_edge_set_wire_rehome_program,
};
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::TopologyEditContract;

pub(crate) fn supports_admitted_shell_or_wire_create_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> bool {
    supports_owned_half_edge_set_wire_rehome_program(bindings, contracts)
        || supports_connected_wire_split_program(bindings, contracts)
        || resolve_single_face_two_face_shell_split_program(bindings, contracts).is_some()
        || supports_owned_face_set_shell_rehome_program(bindings, contracts)
}

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn lower_admitted_shell_or_wire_create_program(
        &self,
        bindings: &TopologyQueryBindingIndex,
        contracts: &[TopologyEditContract],
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        if supports_owned_half_edge_set_wire_rehome_program(bindings, contracts) {
            self.lower_rehome_owned_half_edge_set_to_new_wire_program(bindings, contracts)
        } else if supports_connected_wire_split_program(bindings, contracts) {
            self.lower_split_connected_half_edge_set_to_new_wire_program(bindings, contracts)
        } else if resolve_single_face_two_face_shell_split_program(bindings, contracts).is_some() {
            self.lower_split_single_face_from_two_face_shell_to_new_shell_program(
                bindings, contracts,
            )
        } else if supports_owned_face_set_shell_rehome_program(bindings, contracts) {
            self.lower_rehome_owned_face_set_to_new_shell_program(bindings, contracts)
        } else {
            Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                crate::topology_operators::TopologyEditFamily::AttachShellOrWireMembership,
            ]))
        }
    }
}
