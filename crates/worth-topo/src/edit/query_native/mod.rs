mod bindings;
mod existing_truth;
mod relation_create;

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
        let unsupported = families
            .iter()
            .copied()
            .filter(|family| {
                *family != WorthTopologyEditFamily::CreateTopologyEntity
                    && *family != WorthTopologyEditFamily::DetachBoundaryMembership
                    && *family != WorthTopologyEditFamily::DetachRadialAdjacency
                    && *family != WorthTopologyEditFamily::DetachShellOrWireMembership
                    && *family != WorthTopologyEditFamily::RetireTopologyEntity
            })
            .collect::<Vec<_>>();
        if !unsupported.is_empty() {
            return Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                unsupported,
            ));
        }

        let entity_rows = self.workspace.read(self.assembly.entities());
        let relation_rows = self.workspace.read(self.assembly.relations());
        let created_entity_kinds = planned_created_entity_kinds(contracts);
        let lowered_batch = contracts.iter().try_fold(
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
        )?;

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
            WorthTopologyEditAction::RetireTopologyEntity {
                entity_id, kind, ..
            } => {
                self.lower_retire_topology_entity(builder, entity_rows, *entity_id, *kind, contract)
            }
            _ => Err(WorthTopologyQueryEditExecutionError::UnsupportedFamilies(
                vec![contract.family],
            )),
        }
    }
}

fn planned_created_entity_kinds(
    contracts: &[WorthTopologyEditContract],
) -> BTreeMap<String, WorthTopologyEntityKind> {
    contracts
        .iter()
        .filter_map(|contract| match &contract.action {
            WorthTopologyEditAction::CreateTopologyEntity {
                create_key, kind, ..
            } => Some((create_key.as_str().to_string(), *kind)),
            _ => None,
        })
        .collect()
}
