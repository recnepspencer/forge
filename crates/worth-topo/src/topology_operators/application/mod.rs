#[cfg(test)]
pub(crate) mod admission;
pub(crate) mod bindings;
#[cfg(test)]
mod boundary_tests;
mod declaration_entry;
mod dependency_paths;
mod error;
mod existing_truth;
pub(crate) use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
pub(crate) use declaration_entry::contract_payload::TopologyDeclarationContractPayload;

use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection,
    ForgeQueryMutationBatchBuilder, ForgeQueryWorkspace,
};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::load_post_write_materialized_topology;

use super::contracts::{
    TopologyEditAction, TopologyEditContract, TopologyEditFamily, TopologyEditNamingReport,
};
use super::{NamingEditContinuityMatrix, TopologyEditApplicationMode, TopologyEditDigest};
pub(crate) use dependency_paths::topology_relation_dependency_path;
pub use error::{
    TopologyDeclarationEntryRefusalClass, TopologyDeclarationEntryStopClass,
    TopologyOperatorExecutionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyOperatorExecutionPath {
    DeclarationEntry { semantic_family_key: &'static str },
}

#[derive(Debug, Clone)]
pub struct TopologyOperatorExecution {
    pub mode: TopologyEditApplicationMode,
    pub path: TopologyOperatorExecutionPath,
    pub families: Vec<TopologyEditFamily>,
    pub receipt: ForgeQueryBatchWriteReceipt,
    pub inspection: ForgeQueryBatchWriteReceiptInspection,
    pub materialized: MaterializedTopologyView,
    pub topology_edit_digest: TopologyEditDigest,
    pub naming_continuity_matrix: NamingEditContinuityMatrix,
    pub naming_report: TopologyEditNamingReport,
}

pub(crate) struct TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) workspace: &'workspace mut ForgeQueryWorkspace,
    pub(crate) surfaces: &'surfaces TopologyDeclaredQuerySurfaces,
}

impl<'workspace, 'surfaces> TopologyOperatorRunner<'workspace, 'surfaces> {
    pub(crate) fn new(
        workspace: &'workspace mut ForgeQueryWorkspace,
        surfaces: &'surfaces TopologyDeclaredQuerySurfaces,
    ) -> Self {
        Self {
            workspace,
            surfaces,
        }
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
