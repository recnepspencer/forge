//! Canonical Forge Query-backed Worth topology assembly.
//!
//! This module is the Worth-owned workspace seam for:
//! - declaring the full topology live/computed surface family once
//! - importing authoritative relational truth through aspect-native query writes
//! - carrying persistent-name truth alongside topology truth
//! - decoding a retained derived snapshot back into Worth domain outputs

use forge_query::facade::{
    ForgeQueryAspect, ForgeQueryBatchWriteReceipt,
    ForgeQueryCollection as ForgeQueryRuntimeCollection, ForgeQueryDerivedViewHandle,
    ForgeQueryLiveView, ForgeQueryRuntime, ForgeQueryRuntimeError, ForgeQueryWorkspace,
    ForgeQueryWriteReceipt,
};
use forge_relational::facade::runtime::{EntityReadRecord, RelationReadRecord, RelationalReadView};
use serde_json::Value;
use worth_schema::facade::{
    DerivedTopologyReadBasis, WorthEntityKind, WorthNamingEntityKind, WorthNamingRelationKind,
    WorthRelationKind,
};

use super::{
    declare_worth_persistent_name_live_view, declare_worth_topology_diagnostics_surface,
    declare_worth_topology_entity_live_view, declare_worth_topology_equivalence_contract_surface,
    declare_worth_topology_interpreted_surface, declare_worth_topology_materialized_surface,
    declare_worth_topology_relation_live_view, declare_worth_topology_validation_surface,
    equivalence_contract_from_diagnostics_rows, interpreted_topology_from_materialized_rows,
    materialized::topology_relation_dependency_path, materialized_topology_from_query_rows,
    naming_attachment_report_from_query_rows, validation_report_from_query_rows,
    WorthTopologyQueryMutationEvidence, WorthTopologyQuerySurfaceError,
};
use crate::facade::{
    DerivedTopologyValidationReport, InterpretedTopologyView, MaterializedTopologyView,
    WorthDerivedEquivalenceContractReport, WorthDerivedReadDiagnostics,
    WorthNamingAttachmentReport,
};

mod authority;
mod authority_support;
mod import;
pub use self::authority::{WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError};
use self::import::{
    import_persistent_name_records, import_topology_entity_records,
    import_topology_relation_records, index_persistent_name_targets,
};

const ENTITY_SURFACE: &str = "worth.topology.entities";
const RELATION_SURFACE: &str = "worth.topology.relations";
const PERSISTENT_NAME_SURFACE: &str = "worth.naming.persistent_names";
const MATERIALIZED_SURFACE: &str = "worth.topology.materialized";
const INTERPRETED_SURFACE: &str = "worth.topology.interpreted";
const VALIDATION_SURFACE: &str = "worth.topology.validation";
const DIAGNOSTICS_SURFACE: &str = "worth.topology.diagnostics";
const EQUIVALENCE_SURFACE: &str = "worth.topology.equivalence_contract";

#[derive(Debug, Clone)]
pub struct WorthTopologyQueryAssembly {
    entities: ForgeQueryLiveView<Value>,
    relations: ForgeQueryLiveView<Value>,
    persistent_names: ForgeQueryLiveView<Value>,
    materialized: ForgeQueryDerivedViewHandle<Value>,
    interpreted: ForgeQueryDerivedViewHandle<Value>,
    validation: ForgeQueryDerivedViewHandle<Value>,
    diagnostics: ForgeQueryDerivedViewHandle<Value>,
    equivalence_contract: ForgeQueryDerivedViewHandle<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorthTopologyQuerySnapshot {
    pub naming_attachments: WorthNamingAttachmentReport,
    pub materialized: MaterializedTopologyView,
    pub interpreted: InterpretedTopologyView,
    pub validation: DerivedTopologyValidationReport,
    pub diagnostics: WorthDerivedReadDiagnostics,
    pub equivalence_contract: WorthDerivedEquivalenceContractReport,
}

#[derive(Debug)]
pub enum WorthTopologyQueryImportError {
    Runtime(ForgeQueryRuntimeError),
    UnsupportedEntityKind {
        entity_id: String,
        kind_name: String,
    },
    UnsupportedRelationKind {
        relation_id: String,
        kind_name: String,
    },
    MissingEntityMapping {
        relation_id: String,
        endpoint: &'static str,
        entity_id: String,
    },
}

impl std::fmt::Display for WorthTopologyQueryImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Runtime(error) => write!(f, "query workspace runtime error: {error}"),
            Self::UnsupportedEntityKind {
                entity_id,
                kind_name,
            } => write!(
                f,
                "query import does not support non-Worth entity kind `{kind_name}` on `{entity_id}`"
            ),
            Self::UnsupportedRelationKind {
                relation_id,
                kind_name,
            } => write!(
                f,
                "query import does not support non-Worth relation kind `{kind_name}` on `{relation_id}`"
            ),
            Self::MissingEntityMapping {
                relation_id,
                endpoint,
                entity_id,
            } => write!(
                f,
                "query import is missing imported query identity mapping for relation `{relation_id}` {endpoint} endpoint `{entity_id}`"
            ),
        }
    }
}

impl std::error::Error for WorthTopologyQueryImportError {}

impl From<ForgeQueryRuntimeError> for WorthTopologyQueryImportError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Runtime(value)
    }
}

impl WorthTopologyQueryAssembly {
    pub fn declare(workspace: &mut ForgeQueryWorkspace) -> Result<Self, ForgeQueryRuntimeError> {
        let entities = declare_worth_topology_entity_live_view(workspace, ENTITY_SURFACE)?;
        let relations = declare_worth_topology_relation_live_view(workspace, RELATION_SURFACE)?;
        let persistent_names =
            declare_worth_persistent_name_live_view(workspace, PERSISTENT_NAME_SURFACE)?;
        let materialized = declare_worth_topology_materialized_surface(
            workspace,
            MATERIALIZED_SURFACE,
            &entities,
            &relations,
        )?;
        let interpreted = declare_worth_topology_interpreted_surface(
            workspace,
            INTERPRETED_SURFACE,
            &materialized,
        )?;
        let validation = declare_worth_topology_validation_surface(
            workspace,
            VALIDATION_SURFACE,
            &materialized,
            &interpreted,
        )?;
        let diagnostics = declare_worth_topology_diagnostics_surface(
            workspace,
            DIAGNOSTICS_SURFACE,
            &materialized,
            &interpreted,
            &validation,
        )?;
        let equivalence_contract = declare_worth_topology_equivalence_contract_surface(
            workspace,
            EQUIVALENCE_SURFACE,
            &diagnostics,
        )?;
        Ok(Self {
            entities,
            relations,
            persistent_names,
            materialized,
            interpreted,
            validation,
            diagnostics,
            equivalence_contract,
        })
    }

    pub fn entities(&self) -> &ForgeQueryLiveView<Value> {
        &self.entities
    }

    pub fn relations(&self) -> &ForgeQueryLiveView<Value> {
        &self.relations
    }

    pub fn persistent_names(&self) -> &ForgeQueryLiveView<Value> {
        &self.persistent_names
    }

    pub fn materialized(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.materialized
    }

    pub fn interpreted(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.interpreted
    }

    pub fn validation(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.validation
    }

    pub fn diagnostics(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.diagnostics
    }

    pub fn equivalence_contract(&self) -> &ForgeQueryDerivedViewHandle<Value> {
        &self.equivalence_contract
    }

    /// Imports one authoritative read view through aspect-native query writes.
    ///
    /// The returned receipt aggregates the sequential import session into one
    /// inspectable batch artifact, even though later writes depend on the
    /// identities minted by earlier writes.
    pub fn import_read_view(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        read_view: &RelationalReadView,
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<ForgeQueryBatchWriteReceipt, WorthTopologyQueryImportError> {
        let evidence = WorthTopologyQueryMutationEvidence::from_read_basis(read_basis);
        let (entity_map, mut receipts) =
            import_topology_entity_records(workspace, read_view.entities(), &evidence)?;
        let naming_targets = index_persistent_name_targets(read_view.relations())?;
        receipts.extend(import_persistent_name_records(
            workspace,
            read_view.entities(),
            &entity_map,
            &naming_targets,
            &evidence,
        )?);
        receipts.extend(import_topology_relation_records(
            workspace,
            read_view.relations(),
            &entity_map,
            &evidence,
        )?);
        ForgeQueryBatchWriteReceipt::from_write_receipts(receipts).map_err(Into::into)
    }

    pub fn snapshot(
        &self,
        workspace: &mut ForgeQueryWorkspace,
    ) -> Result<WorthTopologyQuerySnapshot, WorthTopologyQuerySurfaceError> {
        let entity_rows = workspace.read(&self.entities);
        let relation_rows = workspace.read(&self.relations);
        let persistent_name_rows = workspace.read(&self.persistent_names);
        let naming_attachments =
            naming_attachment_report_from_query_rows(&entity_rows, &persistent_name_rows)?;
        let materialized = materialized_topology_from_query_rows(&entity_rows, &relation_rows)
            .map_err(|error| WorthTopologyQuerySurfaceError::new(error.to_string()))?;
        let materialized_rows = workspace.materialize(&self.materialized);
        let interpreted_rows = workspace.materialize(&self.interpreted);
        let _validation_rows = workspace.materialize(&self.validation);
        let diagnostics_rows = workspace.materialize(&self.diagnostics);
        let equivalence_rows = workspace.materialize(&self.equivalence_contract);
        let interpreted = interpreted_topology_from_materialized_rows(&materialized_rows)?;
        let validation = validation_report_from_query_rows(&materialized_rows, &interpreted_rows)?;
        let diagnostics: WorthDerivedReadDiagnostics =
            serde_json::from_value(diagnostics_rows[0].clone()).map_err(|error| {
                WorthTopologyQuerySurfaceError::new(format!(
                    "query-derived `derived read diagnostics` row failed to decode: {error}"
                ))
            })?;
        let equivalence_contract = equivalence_contract_from_diagnostics_rows(&diagnostics_rows)?;
        let decoded_equivalence: WorthDerivedEquivalenceContractReport =
            serde_json::from_value(equivalence_rows[0].clone()).map_err(|error| {
                WorthTopologyQuerySurfaceError::new(format!(
                    "query-derived `derived equivalence contract` row failed to decode: {error}"
                ))
            })?;
        if equivalence_contract != decoded_equivalence {
            return Err(WorthTopologyQuerySurfaceError::new(
                "query-derived equivalence contract row and diagnostics-carried equivalence contract diverged",
            ));
        }
        Ok(WorthTopologyQuerySnapshot {
            naming_attachments,
            materialized,
            interpreted,
            validation,
            diagnostics,
            equivalence_contract,
        })
    }
}

pub fn worth_topology_query_workspace(
    name: impl Into<String>,
) -> Result<ForgeQueryWorkspace, ForgeQueryRuntimeError> {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([
            ForgeQueryRuntimeCollection::new(
                "WorthTopologyEntity",
                [
                    ForgeQueryAspect::new("identity.id", "identity.id"),
                    ForgeQueryAspect::new("topology.kind", "topology.kind"),
                    ForgeQueryAspect::new("lineage.provenance", "lineage.provenance"),
                    ForgeQueryAspect::new("topology.structure", "topology.structure"),
                    ForgeQueryAspect::new("naming.persistent_name", "naming.persistent_name"),
                ],
            ),
            ForgeQueryRuntimeCollection::new(
                "WorthTopologyRelation",
                [
                    ForgeQueryAspect::new("identity.id", "identity.id"),
                    ForgeQueryAspect::new("topology.kind", "topology.kind"),
                    ForgeQueryAspect::new("lineage.provenance", "lineage.provenance"),
                    ForgeQueryAspect::new("topology.ownership", "topology.ownership"),
                    ForgeQueryAspect::new("topology.boundary", "topology.boundary"),
                    ForgeQueryAspect::new("topology.radial", "topology.radial"),
                    ForgeQueryAspect::new("topology.source_identity", "topology.source_identity"),
                    ForgeQueryAspect::new("topology.target_identity", "topology.target_identity"),
                ],
            ),
            ForgeQueryRuntimeCollection::new(
                "WorthPersistentName",
                [
                    ForgeQueryAspect::new("identity.id", "identity.id"),
                    ForgeQueryAspect::new("topology.kind", "topology.kind"),
                    ForgeQueryAspect::new("lineage.provenance", "lineage.provenance"),
                    ForgeQueryAspect::new("naming.persistent_name", "naming.persistent_name"),
                    ForgeQueryAspect::new("naming.target_identity", "naming.target_identity"),
                ],
            ),
        ])
        .build()?
        .workspace(name)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod upsert_assertion_tests;
