mod admission;
mod bindings;
mod existing_truth;
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
    ForgeQueryInspection, ForgeQueryMutationBatchBuilder, ForgeQueryRuntimeError,
    ForgeQueryWorkspace, ForgeQueryWorkspaceError,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use crate::materialization::MaterializedTopologyView;
use crate::query::{WorthTopologyQueryAssembly, WorthTopologyQuerySurfaceError};

use super::types::{
    WorthTopologyEditAction, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyEditNamingReport,
};
use super::{WorthTopologyEditApplicationMode, WorthTopologyEditBatch};
use admission::{planned_created_entity_kinds, unsupported_families};
use relation_shell_or_wire::supports_admitted_shell_or_wire_create_workflow;

#[derive(Debug, Clone)]
pub struct WorthTopologyQueryEditExecution {
    pub mode: WorthTopologyEditApplicationMode,
    pub families: Vec<WorthTopologyEditFamily>,
    pub receipt: ForgeQueryBatchWriteReceipt,
    pub inspection: ForgeQueryBatchWriteReceiptInspection,
    pub materialized: MaterializedTopologyView,
    pub naming_report: WorthTopologyEditNamingReport,
}

#[derive(Debug)]
pub enum WorthTopologyQueryEditExecutionError {
    UnsupportedMode(WorthTopologyEditApplicationMode),
    UnsupportedFamilies(Vec<WorthTopologyEditFamily>),
    MissingCreatedEntityReference(String),
    MissingExistingEntityBinding(EntityId),
    MissingExistingRelationBinding(RelationId),
    CreatedEntityKindMismatch {
        create_key: String,
        expected: WorthTopologyEntityKind,
        actual: WorthTopologyEntityKind,
    },
    ExistingEntityKindMismatch {
        entity_id: EntityId,
        expected: WorthTopologyEntityKind,
        actual: WorthTopologyEntityKind,
    },
    ExistingRelationKindMismatch {
        relation_id: RelationId,
        expected: WorthTopologyRelationKind,
        actual: WorthTopologyRelationKind,
    },
    ExistingRelationSourceMismatch {
        relation_id: RelationId,
        expected_source_entity_id: EntityId,
        actual_source_identity: String,
    },
    ExistingEntityOutgoingRelationCountMismatch {
        entity_id: EntityId,
        relation_kind: WorthTopologyRelationKind,
        expected: usize,
        actual: usize,
    },
    ExistingEntityIncomingRelationCountMismatch {
        entity_id: EntityId,
        relation_kind: WorthTopologyRelationKind,
        expected: usize,
        actual: usize,
    },
    ExistingHalfEdgesNotOnSameEdge {
        relation_id: RelationId,
        source_half_edge_id: EntityId,
        target_half_edge_id: EntityId,
        source_edge_identity: String,
        target_edge_identity: String,
    },
    ExistingHalfEdgesNotOnSameLoop {
        relation_id: RelationId,
        source_half_edge_id: EntityId,
        target_half_edge_id: EntityId,
        source_loop_identity: String,
        target_loop_identity: String,
    },
    Query(ForgeQueryRuntimeError),
    Surface(WorthTopologyQuerySurfaceError),
    MaterializedDecode(String),
    UnexpectedInspectionFamily,
}

impl std::fmt::Display for WorthTopologyQueryEditExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMode(mode) => write!(
                f,
                "worth topology query edit execution does not admit mode `{mode:?}` yet"
            ),
            Self::UnsupportedFamilies(families) => write!(
                f,
                "worth topology query edit execution does not admit families `{families:?}` yet"
            ),
            Self::MissingCreatedEntityReference(create_key) => write!(
                f,
                "worth topology query edit execution is missing same-batch created entity `{create_key}`"
            ),
            Self::MissingExistingEntityBinding(entity_id) => write!(
                f,
                "worth topology query edit execution is missing live query binding for authoritative entity `{entity_id:?}`"
            ),
            Self::MissingExistingRelationBinding(relation_id) => write!(
                f,
                "worth topology query edit execution is missing live query binding for authoritative relation `{relation_id:?}`"
            ),
            Self::CreatedEntityKindMismatch {
                create_key,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected created entity `{create_key}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingEntityKindMismatch {
                entity_id,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative entity `{entity_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationKindMismatch {
                relation_id,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative relation `{relation_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationSourceMismatch {
                relation_id,
                expected_source_entity_id,
                actual_source_identity,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative relation `{relation_id:?}` to originate from halfedge `{expected_source_entity_id:?}`, found query source identity `{actual_source_identity}`"
            ),
            Self::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative entity `{entity_id:?}` to have exactly {expected} outgoing `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "worth topology query edit execution expected authoritative entity `{entity_id:?}` to have exactly {expected} incoming `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingHalfEdgesNotOnSameEdge {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_edge_identity,
                target_edge_identity,
            } => write!(
                f,
                "worth topology query edit execution expected radial splice relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same edge, found source edge `{source_edge_identity}` and target edge `{target_edge_identity}`"
            ),
            Self::ExistingHalfEdgesNotOnSameLoop {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_loop_identity,
                target_loop_identity,
            } => write!(
                f,
                "worth topology query edit execution expected loop-successor relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same loop, found source loop `{source_loop_identity}` and target loop `{target_loop_identity}`"
            ),
            Self::Query(error) => write!(f, "{error}"),
            Self::Surface(error) => write!(f, "{error}"),
            Self::MaterializedDecode(message) => write!(f, "{message}"),
            Self::UnexpectedInspectionFamily => write!(
                f,
                "worth topology query edit execution expected batch-write receipt inspection"
            ),
        }
    }
}

impl std::error::Error for WorthTopologyQueryEditExecutionError {}

impl From<ForgeQueryRuntimeError> for WorthTopologyQueryEditExecutionError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Query(value)
    }
}

impl From<ForgeQueryWorkspaceError> for WorthTopologyQueryEditExecutionError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Query(ForgeQueryRuntimeError::Workspace(value))
    }
}

impl From<WorthTopologyQuerySurfaceError> for WorthTopologyQueryEditExecutionError {
    fn from(value: WorthTopologyQuerySurfaceError) -> Self {
        Self::Surface(value)
    }
}

pub struct WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    workspace: &'workspace mut ForgeQueryWorkspace,
    assembly: &'assembly WorthTopologyQueryAssembly,
}

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub fn new(
        workspace: &'workspace mut ForgeQueryWorkspace,
        assembly: &'assembly WorthTopologyQueryAssembly,
    ) -> Self {
        Self {
            workspace,
            assembly,
        }
    }

    pub fn apply(
        &mut self,
        batch: WorthTopologyEditBatch,
        mode: WorthTopologyEditApplicationMode,
    ) -> Result<WorthTopologyQueryEditExecution, WorthTopologyQueryEditExecutionError> {
        if mode != WorthTopologyEditApplicationMode::Mainline {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedMode(mode));
        }

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
