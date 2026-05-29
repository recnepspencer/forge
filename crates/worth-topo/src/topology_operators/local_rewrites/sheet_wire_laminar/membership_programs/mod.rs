mod face_inner_loop_program;
mod shared;
mod shell_membership_program;
mod wire_membership_program;

use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryInspection};

use super::membership_admission::supports_admitted_shell_or_wire_create_program;
use super::shell_face_rehome_support::{
    parse_shell_face_rehome_program, resolve_single_face_two_face_shell_split_program,
};
use super::wire_rehome_support::{parse_wire_rehome_program, resolve_wire_split_program};
use crate::projection::runtime_boundary::query_runtime::{
    load_post_write_materialized_topology, TopologyQueryBindingIndex,
};
use crate::topology_operators::application::{
    TopologyOperatorExecution, TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::local_rewrites::boundary_wiring::supports_admitted_relation_create_program;
use crate::topology_operators::{
    NamingEditContinuityMatrix, TopologyEditApplicationMode, TopologyEditContract,
    TopologyEditDigest, TopologyEditFamily,
};

pub(crate) fn supports_composed_membership_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> bool {
    supports_admitted_relation_create_program(contracts)
        || supports_admitted_shell_or_wire_create_program(bindings, contracts)
}

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn apply_composed_membership_program(
        &mut self,
        mode: TopologyEditApplicationMode,
        families: Vec<TopologyEditFamily>,
        topology_edit_digest: TopologyEditDigest,
        naming_continuity_matrix: NamingEditContinuityMatrix,
        naming_report: crate::topology_operators::TopologyEditNamingReport,
        contracts: &[TopologyEditContract],
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        let receipt = if supports_admitted_relation_create_program(contracts) {
            self.compose_face_inner_loop_program(contracts, bindings)?
        } else if let Some(program) = parse_shell_face_rehome_program(contracts) {
            self.compose_shell_rehome_program(program, contracts, bindings)?
        } else if let Some(program) =
            resolve_single_face_two_face_shell_split_program(bindings, contracts)
        {
            self.compose_shell_split_program(program, bindings)?
        } else if let Some(program) = parse_wire_rehome_program(contracts) {
            self.compose_wire_rehome_program(program, contracts, bindings)?
        } else if let Some(program) = resolve_wire_split_program(bindings, contracts) {
            self.compose_wire_split_program(program, bindings)?
        } else {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(vec![
                TopologyEditFamily::AttachBoundaryMembership,
                TopologyEditFamily::AttachShellOrWireMembership,
            ]));
        };
        self.finish_composed_membership_execution(
            mode,
            families,
            topology_edit_digest,
            naming_continuity_matrix,
            naming_report,
            receipt,
        )
    }

    fn finish_composed_membership_execution(
        &mut self,
        mode: TopologyEditApplicationMode,
        families: Vec<TopologyEditFamily>,
        topology_edit_digest: TopologyEditDigest,
        naming_continuity_matrix: NamingEditContinuityMatrix,
        naming_report: crate::topology_operators::TopologyEditNamingReport,
        receipt: ForgeQueryBatchWriteReceipt,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        let inspection = match self.workspace.inspect(&receipt)? {
            ForgeQueryInspection::BatchWriteReceipt(inspection) => inspection,
            _ => return Err(TopologyOperatorExecutionError::UnexpectedInspectionFamily),
        };
        let materialized = load_post_write_materialized_topology(self.workspace, self.assembly)?;
        Ok(TopologyOperatorExecution {
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
