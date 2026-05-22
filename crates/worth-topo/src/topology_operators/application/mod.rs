mod admission;
pub(crate) mod bindings;
#[cfg(test)]
mod boundary_tests;
mod dependency_paths;
mod error;
mod existing_truth;

use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryInspection,
    ForgeQueryMutationBatchBuilder, ForgeQueryWorkspace,
};
use schema::facade::{TopologyEntityKind, TopologyRelationKind};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    load_post_write_materialized_topology, TopologyQueryBindingIndex, TopologyRuntimeSupport,
};

use super::contracts::{
    TopologyEditAction, TopologyEditContract, TopologyEditFamily, TopologyEditNamingReport,
};
use super::local_rewrites::boundary_wiring::supports_composed_loop_successor_program;
use super::local_rewrites::sheet_wire_laminar::{
    supports_admitted_shell_or_wire_create_program, supports_composed_membership_program,
};
use super::{
    NamingEditContinuityMatrix, TopologyEditApplicationMode, TopologyEditBatch, TopologyEditDigest,
};
use admission::{planned_created_entity_kinds, unsupported_families};
pub(crate) use dependency_paths::topology_relation_dependency_path;
pub use error::TopologyOperatorExecutionError;

#[derive(Debug, Clone)]
pub struct TopologyOperatorExecution {
    pub mode: TopologyEditApplicationMode,
    pub families: Vec<TopologyEditFamily>,
    pub receipt: ForgeQueryBatchWriteReceipt,
    pub inspection: ForgeQueryBatchWriteReceiptInspection,
    pub materialized: MaterializedTopologyView,
    pub topology_edit_digest: TopologyEditDigest,
    pub naming_continuity_matrix: NamingEditContinuityMatrix,
    pub naming_report: TopologyEditNamingReport,
}

pub(crate) struct TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) workspace: &'workspace mut ForgeQueryWorkspace,
    pub(crate) assembly: &'assembly TopologyQueryAssembly,
}

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn new(
        workspace: &'workspace mut ForgeQueryWorkspace,
        assembly: &'assembly TopologyQueryAssembly,
    ) -> Self {
        Self {
            workspace,
            assembly,
        }
    }

    pub(crate) fn apply(
        &mut self,
        batch: TopologyEditBatch,
        mode: TopologyEditApplicationMode,
    ) -> Result<TopologyOperatorExecution, TopologyOperatorExecutionError> {
        if mode != TopologyEditApplicationMode::Mainline {
            return Err(TopologyOperatorExecutionError::UnsupportedMode(mode));
        }

        let topology_edit_digest = batch.topology_edit_digest();
        let naming_continuity_matrix = batch.naming_edit_continuity_matrix();
        let naming_report = batch.naming_report();
        let families = batch.families();
        let contracts = batch.contracts();
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let entity_rows = self.workspace.read(self.assembly.entities());
        let relation_rows = self.workspace.read(self.assembly.relations());
        let bindings = TopologyQueryBindingIndex::from_query_rows(&entity_rows, &relation_rows)?;
        let unsupported = unsupported_families(&support, &bindings, &families, contracts);
        if !unsupported.is_empty() {
            return Err(TopologyOperatorExecutionError::UnsupportedFamilies(
                unsupported,
            ));
        }
        if supports_composed_loop_successor_program(&bindings, contracts) {
            return self.apply_composed_loop_successor_program(
                mode,
                families,
                topology_edit_digest,
                naming_continuity_matrix,
                naming_report,
                contracts,
                &bindings,
            );
        }
        if supports_composed_membership_program(&bindings, contracts) {
            return self.apply_composed_membership_program(
                mode,
                families,
                topology_edit_digest,
                naming_continuity_matrix,
                naming_report,
                contracts,
                &bindings,
            );
        }
        let created_entity_kinds = planned_created_entity_kinds(contracts);
        let lowered_batch = if supports_admitted_shell_or_wire_create_program(&bindings, contracts)
        {
            self.lower_admitted_shell_or_wire_create_program(&bindings, contracts)?
        } else {
            contracts.iter().try_fold(
                ForgeQueryMutationBatchBuilder::new(),
                |builder, contract| {
                    self.lower_contract(builder, &bindings, &created_entity_kinds, contract)
                },
            )?
        };

        let receipt = self.workspace.batch(|_| lowered_batch)?;
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

    fn lower_contract(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        contract: &TopologyEditContract,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        match &contract.action {
            TopologyEditAction::AttachBoundaryMembership {
                kind,
                owner,
                member,
                ..
            } => self.lower_attach_boundary_membership(
                builder,
                bindings,
                created_entity_kinds,
                *kind,
                owner,
                member,
            ),
            TopologyEditAction::AttachShellOrWireMembership {
                kind,
                owner,
                member,
                ..
            } => self.lower_attach_shell_or_wire_membership(
                builder,
                bindings,
                created_entity_kinds,
                *kind,
                owner,
                member,
            ),
            TopologyEditAction::CreateTopologyEntity {
                create_key, kind, ..
            } => Ok(
                builder.insert_symbolic(create_key.as_str(), "TopologyEntity", |mutation| {
                    mutation
                        .aspect("topology.kind", kind.kind_name())
                        .aspect("topology.structure", create_key.as_str())
                        .aspect("naming.persistent_name", create_key.as_str())
                }),
            ),
            TopologyEditAction::DetachBoundaryMembership {
                relation_id, kind, ..
            } => self.lower_delete_existing_relation(
                builder,
                bindings,
                *relation_id,
                kind.relation_kind(),
                contract,
            ),
            TopologyEditAction::DetachRadialAdjacency { relation_id } => self
                .lower_delete_existing_relation(
                    builder,
                    bindings,
                    *relation_id,
                    TopologyRelationKind::HalfEdgeRadialNext,
                    contract,
                ),
            TopologyEditAction::DetachShellOrWireMembership {
                relation_id, kind, ..
            } => self.lower_delete_existing_relation(
                builder,
                bindings,
                *relation_id,
                kind.relation_kind(),
                contract,
            ),
            TopologyEditAction::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            } => self.lower_rewire_loop_endpoint(
                builder,
                bindings,
                *relation_id,
                *endpoint,
                *half_edge_id,
                *vertex_id,
            ),
            TopologyEditAction::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            } => self.lower_rewire_loop_successor(
                builder,
                bindings,
                *relation_id,
                *kind,
                *half_edge_id,
                *successor_half_edge_id,
            ),
            TopologyEditAction::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            } => self.lower_splice_radial_adjacency(
                builder,
                bindings,
                *relation_id,
                *half_edge_id,
                *radial_next_half_edge_id,
            ),
            TopologyEditAction::RetireTopologyEntity {
                entity_id, kind, ..
            } => self.lower_retire_topology_entity(builder, bindings, *entity_id, *kind, contract),
        }
    }
}
