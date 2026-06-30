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
use serde_json::{json, Value};
const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";
const QUERY_SURFACE_FAILURE_KIND_KEY: &str = "query_surface_error_kind";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TopologyQuerySurfaceErrorKind {
    Generic,
    MissingHistoricalReadBasisMetadata,
    RefreshMetadataDecodeFailed,
    RetainedPayloadDecodeFailed,
    CompiledProductAdmissionDenied,
    CompiledProductFamilySelectionFailed,
    CompiledProductIdentityLoweringFailed,
    UnregisteredValidationReport,
    UnsupportedTouchedAspect,
}

impl TopologyQuerySurfaceErrorKind {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::MissingHistoricalReadBasisMetadata => "missing_historical_read_basis_metadata",
            Self::RefreshMetadataDecodeFailed => "refresh_metadata_decode_failed",
            Self::RetainedPayloadDecodeFailed => "retained_payload_decode_failed",
            Self::CompiledProductAdmissionDenied => "compiled_product_admission_denied",
            Self::CompiledProductFamilySelectionFailed => {
                "compiled_product_family_selection_failed"
            }
            Self::CompiledProductIdentityLoweringFailed => {
                "compiled_product_identity_lowering_failed"
            }
            Self::UnregisteredValidationReport => "unregistered_validation_report",
            Self::UnsupportedTouchedAspect => "unsupported_touched_aspect",
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "generic" => Self::Generic,
            "missing_historical_read_basis_metadata" => Self::MissingHistoricalReadBasisMetadata,
            "refresh_metadata_decode_failed" => Self::RefreshMetadataDecodeFailed,
            "retained_payload_decode_failed" => Self::RetainedPayloadDecodeFailed,
            "compiled_product_admission_denied" => Self::CompiledProductAdmissionDenied,
            "compiled_product_family_selection_failed" => {
                Self::CompiledProductFamilySelectionFailed
            }
            "compiled_product_identity_lowering_failed" => {
                Self::CompiledProductIdentityLoweringFailed
            }
            "unregistered_validation_report" => Self::UnregisteredValidationReport,
            "unsupported_touched_aspect" => Self::UnsupportedTouchedAspect,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQuerySurfaceError {
    kind: TopologyQuerySurfaceErrorKind,
    message: String,
}

impl TopologyQuerySurfaceError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self::with_kind(TopologyQuerySurfaceErrorKind::Generic, message)
    }

    pub(crate) fn with_kind(
        kind: TopologyQuerySurfaceErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub(crate) fn failure_payload(&self) -> Value {
        json!({
            QUERY_SURFACE_FAILURE_ROW_KEY: self.message,
            QUERY_SURFACE_FAILURE_KIND_KEY: self.kind.as_str(),
        })
    }
}

impl fmt::Display for TopologyQuerySurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for TopologyQuerySurfaceError {}

pub(crate) fn decode_query_surface_failure_payload(
    payload: &Value,
    surface_name: &str,
) -> Option<TopologyQuerySurfaceError> {
    let message = payload.get(QUERY_SURFACE_FAILURE_ROW_KEY)?.as_str()?;
    let kind = payload
        .get(QUERY_SURFACE_FAILURE_KIND_KEY)
        .and_then(Value::as_str)
        .and_then(TopologyQuerySurfaceErrorKind::from_str)
        .unwrap_or(TopologyQuerySurfaceErrorKind::RetainedPayloadDecodeFailed);
    Some(TopologyQuerySurfaceError::with_kind(
        kind,
        format!("retained surface `{surface_name}` declared failure payload: {message}"),
    ))
}

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
