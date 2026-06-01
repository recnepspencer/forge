pub(crate) mod admission;
pub(crate) mod bindings;
#[cfg(test)]
mod boundary_tests;
mod declaration_entry;
mod declared_mutation_artifact;
mod dependency_paths;
mod error;
mod existing_truth;
pub(crate) use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
pub(crate) use declaration_entry::mutation_payload::TopologyDeclarationMutationPayload;

use std::collections::BTreeMap;

use forge_query::facade::{ForgeQueryMutationBatchBuilder, ForgeQueryWorkspace};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::runtime_boundary::query_runtime::load_post_write_materialized_topology;

use super::mutation_records::TopologyMutationFamily;
use super::{
    TopologyDeclaredMutationActionRef, TopologyDeclaredMutationMember,
    TopologyMutationApplicationMode,
};
pub(crate) use declared_mutation_artifact::TopologyDeclaredMutationArtifact;
pub(crate) use dependency_paths::topology_relation_dependency_path;
pub use error::{
    TopologyDeclarationEntryRefusalClass, TopologyDeclarationEntryStopClass,
    TopologyMutationApplicationError,
};

pub(crate) struct TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) workspace: &'workspace mut ForgeQueryWorkspace,
    pub(crate) surfaces: &'surfaces TopologyDeclaredQuerySurfaces,
}

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn new(
        workspace: &'workspace mut ForgeQueryWorkspace,
        surfaces: &'surfaces TopologyDeclaredQuerySurfaces,
    ) -> Self {
        Self {
            workspace,
            surfaces,
        }
    }

    fn lower_mutation_member(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        member: TopologyDeclaredMutationMember<'_>,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
        match member.action_ref() {
            TopologyDeclaredMutationActionRef::AttachBoundaryMembership {
                kind,
                owner,
                member,
            } => self.lower_attach_boundary_membership(
                builder,
                bindings,
                created_entity_kinds,
                kind,
                owner,
                member,
            ),
            TopologyDeclaredMutationActionRef::AttachShellOrWireMembership {
                kind,
                owner,
                member,
            } => self.lower_attach_shell_or_wire_membership(
                builder,
                bindings,
                created_entity_kinds,
                kind,
                owner,
                member,
            ),
            TopologyDeclaredMutationActionRef::CreateTopologyEntity { create_key, kind } => Ok(
                builder.insert_symbolic(create_key, "TopologyEntity", |mutation| {
                    mutation
                        .aspect("topology.kind", kind.kind_name())
                        .aspect("topology.structure", create_key)
                        .aspect("naming.persistent_name", create_key)
                }),
            ),
            TopologyDeclaredMutationActionRef::DetachBoundaryMembership { relation_id, kind } => {
                self.lower_delete_existing_relation(
                    builder,
                    bindings,
                    relation_id,
                    kind.relation_kind(),
                    member,
                )
            }
            TopologyDeclaredMutationActionRef::DetachRadialAdjacency { relation_id } => self
                .lower_delete_existing_relation(
                    builder,
                    bindings,
                    relation_id,
                    TopologyRelationKind::HalfEdgeRadialNext,
                    member,
                ),
            TopologyDeclaredMutationActionRef::DetachShellOrWireMembership {
                relation_id,
                kind,
            } => self.lower_delete_existing_relation(
                builder,
                bindings,
                relation_id,
                kind.relation_kind(),
                member,
            ),
            TopologyDeclaredMutationActionRef::RewireLoopEndpoint {
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            } => self.lower_rewire_loop_endpoint(
                builder,
                bindings,
                relation_id,
                endpoint,
                half_edge_id,
                vertex_id,
            ),
            TopologyDeclaredMutationActionRef::RewireLoopSuccessor {
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            } => self.lower_rewire_loop_successor(
                builder,
                bindings,
                relation_id,
                kind,
                half_edge_id,
                successor_half_edge_id,
            ),
            TopologyDeclaredMutationActionRef::SpliceRadialAdjacency {
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            } => self.lower_splice_radial_adjacency(
                builder,
                bindings,
                relation_id,
                half_edge_id,
                radial_next_half_edge_id,
            ),
            TopologyDeclaredMutationActionRef::RetireTopologyEntity { entity_id, kind } => {
                self.lower_retire_topology_entity(builder, bindings, entity_id, kind, member)
            }
        }
    }
}
