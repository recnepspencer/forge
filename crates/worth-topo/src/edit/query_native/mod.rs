mod admission;
mod bindings;
mod error;
mod existing_truth;
mod graph_membership_workflow;
mod graph_successor_workflow;
mod relation_boundary;
mod relation_create;
mod relation_shell_face_rehome;
mod relation_shell_face_rehome_support;
mod relation_shell_face_split;
mod relation_shell_or_wire;
mod relation_successor;
mod relation_successor_support;
mod relation_update;
mod relation_wire_rehome;
mod relation_wire_rehome_support;

use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryEntity,
    ForgeQueryInspection, ForgeQueryMutationBatchBuilder, ForgeQueryWorkspace,
};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use crate::materialization::MaterializedTopologyView;
use crate::query::WorthTopologyQueryAssembly;

use super::types::{
    WorthTopologyEditAction, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyEditNamingReport,
};
use super::{
    WorthNamingEditContinuityMatrix, WorthTopologyEditApplicationMode, WorthTopologyEditBatch,
    WorthTopologyEditDigest,
};
use admission::{planned_created_entity_kinds, unsupported_families};
pub use error::WorthTopologyQueryEditExecutionError;
use graph_membership_workflow::supports_graph_composed_membership_workflow;
use graph_successor_workflow::supports_graph_composed_loop_successor_workflow;
use relation_shell_or_wire::supports_admitted_shell_or_wire_create_workflow;

#[derive(Debug, Clone)]
pub struct WorthTopologyQueryEditExecution {
    pub mode: WorthTopologyEditApplicationMode,
    pub families: Vec<WorthTopologyEditFamily>,
    pub receipt: ForgeQueryBatchWriteReceipt,
    pub inspection: ForgeQueryBatchWriteReceiptInspection,
    pub materialized: MaterializedTopologyView,
    pub topology_edit_digest: WorthTopologyEditDigest,
    pub naming_continuity_matrix: WorthNamingEditContinuityMatrix,
    pub naming_report: WorthTopologyEditNamingReport,
}

pub(crate) struct WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    workspace: &'workspace mut ForgeQueryWorkspace,
    assembly: &'assembly WorthTopologyQueryAssembly,
}

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(crate) fn new(
        workspace: &'workspace mut ForgeQueryWorkspace,
        assembly: &'assembly WorthTopologyQueryAssembly,
    ) -> Self {
        Self {
            workspace,
            assembly,
        }
    }

    pub(crate) fn apply(
        &mut self,
        batch: WorthTopologyEditBatch,
        mode: WorthTopologyEditApplicationMode,
    ) -> Result<WorthTopologyQueryEditExecution, WorthTopologyQueryEditExecutionError> {
        if mode != WorthTopologyEditApplicationMode::Mainline {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedMode(mode));
        }

        let topology_edit_digest = batch.topology_edit_digest();
        let naming_continuity_matrix = batch.naming_edit_continuity_matrix();
        let naming_report = batch.naming_report();
        let families = batch.families();
        let contracts = batch.contracts();
        let entity_rows = self.workspace.read(self.assembly.entities());
        let relation_rows = self.workspace.read(self.assembly.relations());
        let unsupported = unsupported_families(&entity_rows, &relation_rows, &families, contracts);
        if !unsupported.is_empty() {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                unsupported,
            ));
        }
        if supports_graph_composed_loop_successor_workflow(&entity_rows, &relation_rows, contracts)
        {
            return self.apply_graph_composed_loop_successor_workflow(
                mode,
                families,
                topology_edit_digest,
                naming_continuity_matrix,
                naming_report,
                contracts,
                &entity_rows,
                &relation_rows,
            );
        }
        if supports_graph_composed_membership_workflow(&entity_rows, &relation_rows, contracts) {
            return self.apply_graph_composed_membership_workflow(
                mode,
                families,
                topology_edit_digest,
                naming_continuity_matrix,
                naming_report,
                contracts,
                &entity_rows,
                &relation_rows,
            );
        }
        let created_entity_kinds = planned_created_entity_kinds(contracts);
        let lowered_batch = if supports_admitted_shell_or_wire_create_workflow(
            &entity_rows,
            &relation_rows,
            contracts,
        ) {
            self.lower_admitted_shell_or_wire_create_workflow(
                &entity_rows,
                &relation_rows,
                contracts,
            )?
        } else {
            contracts.iter().try_fold(
                ForgeQueryMutationBatchBuilder::new(),
                |builder, contract| {
                    self.lower_contract(
                        builder,
                        &entity_rows,
                        &relation_rows,
                        &created_entity_kinds,
                        contract,
                    )
                },
            )?
        };

        let receipt = self.workspace.batch(|_| lowered_batch)?;
        let inspection = match self.workspace.inspect(&receipt)? {
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

    fn lower_contract(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
        created_entity_kinds: &BTreeMap<String, WorthTopologyEntityKind>,
        contract: &WorthTopologyEditContract,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        match &contract.action {
            WorthTopologyEditAction::AttachBoundaryMembership {
                kind,
                owner,
                member,
                ..
            } => self.lower_attach_boundary_membership(
                builder,
                entity_rows,
                created_entity_kinds,
                *kind,
                owner,
                member,
            ),
            WorthTopologyEditAction::AttachShellOrWireMembership {
                kind,
                owner,
                member,
                ..
            } => self.lower_attach_shell_or_wire_membership(
                builder,
                entity_rows,
                created_entity_kinds,
                *kind,
                owner,
                member,
            ),
            WorthTopologyEditAction::CreateTopologyEntity {
                create_key, kind, ..
            } => Ok(builder.insert_symbolic(
                create_key.as_str(),
                "WorthTopologyEntity",
                |mutation| {
                    mutation
                        .aspect("topology.kind", kind.kind_name())
                        .aspect("topology.structure", create_key.as_str())
                        .aspect("naming.persistent_name", create_key.as_str())
                },
            )),
            WorthTopologyEditAction::DetachBoundaryMembership {
                relation_id, kind, ..
            } => self.lower_delete_existing_relation(
                builder,
                relation_rows,
                *relation_id,
                kind.relation_kind(),
                contract,
            ),
            WorthTopologyEditAction::DetachRadialAdjacency { relation_id } => self
                .lower_delete_existing_relation(
                    builder,
                    relation_rows,
                    *relation_id,
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                    contract,
                ),
            WorthTopologyEditAction::DetachShellOrWireMembership {
                relation_id, kind, ..
            } => self.lower_delete_existing_relation(
                builder,
                relation_rows,
                *relation_id,
                kind.relation_kind(),
                contract,
            ),
            WorthTopologyEditAction::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            } => self.lower_rewire_loop_endpoint(
                builder,
                entity_rows,
                relation_rows,
                *relation_id,
                *endpoint,
                *half_edge_id,
                *vertex_id,
            ),
            WorthTopologyEditAction::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            } => self.lower_rewire_loop_successor(
                builder,
                entity_rows,
                relation_rows,
                *relation_id,
                *kind,
                *half_edge_id,
                *successor_half_edge_id,
            ),
            WorthTopologyEditAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            } => self.lower_splice_radial_adjacency(
                builder,
                entity_rows,
                relation_rows,
                *relation_id,
                *half_edge_id,
                *radial_next_half_edge_id,
            ),
            WorthTopologyEditAction::RetireTopologyEntity {
                entity_id, kind, ..
            } => {
                self.lower_retire_topology_entity(builder, entity_rows, *entity_id, *kind, contract)
            }
        }
    }
}
