pub(crate) mod admission;
pub(crate) mod bindings;
#[cfg(test)]
mod boundary_tests;
mod declaration_entry;
mod declared_mutation_artifact;
mod dependency_paths;
mod error;
mod error_display;
mod existing_truth;
pub(crate) use crate::projection::runtime_boundary::query_runtime::TopologyPostWriteQueryArtifact;
pub(crate) use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
pub(crate) use crate::projection::runtime_boundary::query_runtime::TopologyQueryMutationLaneExecutionShape;
pub(crate) use declaration_entry::mutation_payload::TopologyDeclarationMutationPayload;

use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryDeclarationInput, ForgeQueryMutationBatchBuilder, ForgeQueryWorkspace,
};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::query_domain::TopologyQueryDomain;

use super::mutation_records::TopologyMutationFamily;
use super::{
    TopologyDeclaredMutationActionRef, TopologyDeclaredMutationMember,
    TopologyMutationApplicationMode,
};
pub(crate) use declaration_entry::TopologyRetainedApplicationHandoff;
pub(crate) use declared_mutation_artifact::TopologyDeclaredMutationArtifact;
#[cfg(test)]
pub(crate) use declared_mutation_artifact::TopologyOperatorApplicationQueryAnchor;
pub(crate) use dependency_paths::topology_relation_dependency_path;
pub(crate) use error::{TopologyDeclarationEntryStopClass, TopologyMutationApplicationError};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum TopologyMutationApplicationOutcome {
    Applied(TopologyDeclaredMutationArtifact),
    Stopped(TopologyMutationApplicationStop),
    Failed(TopologyMutationApplicationError),
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TopologyMutationApplicationStop {
    error: TopologyMutationApplicationError,
    recovery: Option<forge_query::facade::ForgeQueryRecoveryBrief>,
}

#[allow(dead_code)]
impl TopologyMutationApplicationOutcome {
    pub(crate) fn from_result(
        result: Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>,
    ) -> Self {
        match result {
            Ok(artifact) => Self::Applied(artifact),
            Err(error) if error.is_declaration_entry_stop() => {
                Self::Stopped(TopologyMutationApplicationStop::from_error(error))
            }
            Err(error) => Self::Failed(error),
        }
    }
}

#[allow(dead_code)]
impl TopologyMutationApplicationStop {
    fn from_error(error: TopologyMutationApplicationError) -> Self {
        let recovery = error.declaration_entry_recovery_brief().cloned();
        Self { error, recovery }
    }

    pub(crate) fn stop_class(&self) -> Option<TopologyDeclarationEntryStopClass> {
        self.error.declaration_entry_stop_class()
    }

    pub(crate) fn recovery(&self) -> Option<&forge_query::facade::ForgeQueryRecoveryBrief> {
        self.recovery.as_ref()
    }
}

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

pub(crate) fn finalize_batch_write_closeout<I>(
    runner: &mut TopologyMutationApplicationRunner<'_, '_>,
    retained_handoff: TopologyRetainedApplicationHandoff<I>,
    lowered_mutations: ForgeQueryMutationBatchBuilder,
    semantic_family_key: &'static str,
    sequence: &super::TopologyDeclaredMutationSequence,
) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    let receipt = runner.workspace.batch(|_| lowered_mutations)?;
    finalize_graph_or_batch_receipt_closeout(
        runner,
        retained_handoff,
        semantic_family_key,
        sequence,
        receipt,
        TopologyQueryMutationLaneExecutionShape::ScalarMutation,
    )
}

pub(crate) fn finalize_graph_or_batch_receipt_closeout<I>(
    runner: &mut TopologyMutationApplicationRunner<'_, '_>,
    retained_handoff: TopologyRetainedApplicationHandoff<I>,
    semantic_family_key: &'static str,
    sequence: &super::TopologyDeclaredMutationSequence,
    receipt: forge_query::facade::ForgeQueryBatchWriteReceipt,
    execution_shape: TopologyQueryMutationLaneExecutionShape,
) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
where
    I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
{
    let post_write_query_artifact = TopologyPostWriteQueryArtifact::build(
        runner.workspace,
        runner.surfaces,
        receipt,
        execution_shape,
    )?;
    TopologyDeclaredMutationArtifact::from_receipt(
        semantic_family_key,
        &retained_handoff,
        sequence,
        post_write_query_artifact,
    )
}
