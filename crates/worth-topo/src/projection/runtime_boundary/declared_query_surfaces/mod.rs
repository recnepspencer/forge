//! Internal declared Query surfaces for live and computed topology reads.

pub(crate) mod derived_surfaces;
pub(crate) mod query_diagnostics;
pub(crate) mod retained_artifacts;
pub(crate) mod retained_payload;
pub(crate) mod truth_surfaces;

use std::fmt;

use forge_query::facade::{
    ForgeQueryDerivedMaterializationTarget, ForgeQueryDerivedViewHandle,
    ForgeQueryLiveArtifactTarget, ForgeQueryLiveView, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};
use serde_json::Value;
const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

const ENTITY_SURFACE: &str = ".topology.entities";
const RELATION_SURFACE: &str = ".topology.relations";
const PERSISTENT_NAME_SURFACE: &str = ".naming.persistent_names";
const MATERIALIZED_SURFACE: &str = ".topology.materialized";
const INTERPRETED_SURFACE: &str = ".topology.interpreted";
const VALIDATION_SURFACE: &str = ".topology.validation";
const DIAGNOSTICS_SURFACE: &str = ".topology.diagnostics";
const EQUIVALENCE_SURFACE: &str = ".topology.equivalence_contract";

#[derive(Debug, Clone)]
pub(crate) struct TopologyDeclaredQuerySurfaces {
    entities: ForgeQueryLiveView<Value>,
    relations: ForgeQueryLiveView<Value>,
    persistent_names: ForgeQueryLiveView<Value>,
    materialized: ForgeQueryDerivedViewHandle<Value>,
    interpreted: ForgeQueryDerivedViewHandle<Value>,
    validation: ForgeQueryDerivedViewHandle<Value>,
    diagnostics: ForgeQueryDerivedViewHandle<Value>,
    equivalence_contract: ForgeQueryDerivedViewHandle<Value>,
}

pub(crate) use derived_surfaces::{
    declare_topology_interpreted_surface, declare_topology_validation_surface,
};
#[cfg(test)]
pub(crate) use query_diagnostics::equivalence_contract_from_diagnostics_rows;
pub(crate) use query_diagnostics::{
    declare_topology_diagnostics_surface, declare_topology_equivalence_contract_surface,
};
pub(crate) use truth_surfaces::{
    declare_persistent_name_live_view, declare_topology_entity_live_view,
    declare_topology_materialized_surface, declare_topology_relation_live_view,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQuerySurfaceError {
    message: String,
}

impl TopologyQuerySurfaceError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TopologyQuerySurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for TopologyQuerySurfaceError {}

pub(crate) fn declare_topology_query_surfaces(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<TopologyDeclaredQuerySurfaces, ForgeQueryRuntimeError> {
    let entities = declare_topology_entity_live_view(workspace, ENTITY_SURFACE)?;
    let relations = declare_topology_relation_live_view(workspace, RELATION_SURFACE)?;
    let persistent_names = declare_persistent_name_live_view(workspace, PERSISTENT_NAME_SURFACE)?;
    let materialized = declare_topology_materialized_surface(
        workspace,
        MATERIALIZED_SURFACE,
        &entities,
        &relations,
    )?;
    let interpreted =
        declare_topology_interpreted_surface(workspace, INTERPRETED_SURFACE, &materialized)?;
    let validation = declare_topology_validation_surface(
        workspace,
        VALIDATION_SURFACE,
        &materialized,
        &interpreted,
    )?;
    let diagnostics = declare_topology_diagnostics_surface(
        workspace,
        DIAGNOSTICS_SURFACE,
        &materialized,
        &interpreted,
        &validation,
    )?;
    let equivalence_contract = declare_topology_equivalence_contract_surface(
        workspace,
        EQUIVALENCE_SURFACE,
        &diagnostics,
    )?;
    Ok(TopologyDeclaredQuerySurfaces {
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

impl TopologyDeclaredQuerySurfaces {
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
}

pub(crate) fn materialize_declared_query_surface_row<T>(
    workspace: &mut ForgeQueryWorkspace,
    view: &ForgeQueryDerivedViewHandle<Value>,
) -> Result<T, TopologyQuerySurfaceError>
where
    T: serde::de::DeserializeOwned,
{
    workspace
        .materialize_intent(view)
        .execute()
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?
        .single_retained_row()
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))
        .and_then(|row| retained_payload::decode_retained_payload_row(row, view.name()))
}

pub(crate) fn materialize_declared_query_surface_binding(
    workspace: &mut ForgeQueryWorkspace,
    artifact_name: impl Into<String>,
    views: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
) -> Result<forge_query::facade::ForgeQueryDerivedArtifactBinding, TopologyQuerySurfaceError> {
    workspace
        .materialize_derived_artifact_binding(artifact_name, views)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))
}

pub(crate) fn read_declared_query_surface_binding(
    workspace: &mut ForgeQueryWorkspace,
    artifact_name: impl Into<String>,
    views: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
) -> Result<forge_query::facade::ForgeQueryLiveArtifactBinding, TopologyQuerySurfaceError> {
    workspace
        .read_live_artifact_binding(artifact_name, views)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))
}

#[cfg(test)]
mod boundary_tests;
#[cfg(test)]
mod tests;
