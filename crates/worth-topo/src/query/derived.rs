use std::fmt;

use forge_query::facade::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedView, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization, ForgeQueryRuntimeError,
    ForgeQueryWorkspace, ForgeQueryWorkspaceError,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use worth_schema::facade::{
    WorthQueryAspectPath, WorthQueryComputedDeclarationBuilder, WorthQueryDeclarationError,
};

use crate::interpretation::{interpret_topology_view, InterpretedTopologyView};
use crate::materialization::MaterializedTopologyView;
use crate::validators::{
    validate_interpreted_topology, DerivedTopologyValidationReport, WorthTopologyValidationError,
};

use super::QUERY_SURFACE_FAILURE_ROW_KEY;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyQuerySurfaceError {
    message: String,
}

impl WorthTopologyQuerySurfaceError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for WorthTopologyQuerySurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for WorthTopologyQuerySurfaceError {}

#[derive(Debug, Clone)]
pub struct WorthTopologyInterpretedMaintainer {
    materialized_view_name: String,
}

impl WorthTopologyInterpretedMaintainer {
    pub fn new(materialized_view_name: impl Into<String>) -> Self {
        Self {
            materialized_view_name: materialized_view_name.into(),
        }
    }
}

impl ForgeQueryDerivedViewMaintainer for WorthTopologyInterpretedMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &forge_query::facade::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let payload = json!({
            QUERY_SURFACE_FAILURE_ROW_KEY: format!(
                "incremental delivery reached `{}` for `{}`; whole-refresh fallback was expected",
                delta.collection,
                view.name(),
            ),
        });
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "worth-topology-interpreted-incremental-unexpected",
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        _mutation: &forge_query::facade::ForgeQueryRetainedMutationContext,
        upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let materialized_rows = upstreams
            .computed_rows(&self.materialized_view_name)
            .unwrap_or(&[]);
        let payload = match interpreted_topology_from_materialized_rows(materialized_rows) {
            Ok(interpreted) => {
                serde_json::to_value(interpreted).expect("interpreted topology view must serialize")
            }
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            "worth-topology-interpreted",
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "worth-topology-materialized-interpretation",
        ))
    }
}

#[derive(Debug, Clone)]
pub struct WorthTopologyValidationMaintainer {
    materialized_view_name: String,
    interpreted_view_name: String,
}

impl WorthTopologyValidationMaintainer {
    pub fn new(
        materialized_view_name: impl Into<String>,
        interpreted_view_name: impl Into<String>,
    ) -> Self {
        Self {
            materialized_view_name: materialized_view_name.into(),
            interpreted_view_name: interpreted_view_name.into(),
        }
    }
}

impl ForgeQueryDerivedViewMaintainer for WorthTopologyValidationMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &forge_query::facade::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let payload = json!({
            QUERY_SURFACE_FAILURE_ROW_KEY: format!(
                "incremental delivery reached `{}` for `{}`; whole-refresh fallback was expected",
                delta.collection,
                view.name(),
            ),
        });
        materialization.replace_rows([payload.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "worth-topology-validation-incremental-unexpected",
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
        )
    }

    fn refresh_from_upstreams(
        &mut self,
        view: &ForgeQueryDerivedView,
        _mutation: &forge_query::facade::ForgeQueryRetainedMutationContext,
        upstreams: &forge_query::facade::ForgeQueryRetainedUpstreamInputs,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> Option<ForgeQueryDerivedPatch> {
        let materialized_rows = upstreams
            .computed_rows(&self.materialized_view_name)
            .unwrap_or(&[]);
        let interpreted_rows = upstreams
            .computed_rows(&self.interpreted_view_name)
            .unwrap_or(&[]);
        let payload = match validation_report_from_query_rows(materialized_rows, interpreted_rows) {
            Ok(validation) => {
                serde_json::to_value(validation).expect("validation report must serialize")
            }
            Err(error) => json!({ QUERY_SURFACE_FAILURE_ROW_KEY: error.to_string() }),
        };
        materialization.replace_rows([payload.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            "worth-topology-validation",
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            payload,
            "worth-topology-interpreted-validation",
        ))
    }
}

pub fn worth_topology_interpreted_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, WorthQueryDeclarationError> {
    WorthQueryComputedDeclarationBuilder::new(surface_name)
        .reads([
            WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
            WorthQueryAspectPath::TOPOLOGY_OWNERSHIP,
            WorthQueryAspectPath::TOPOLOGY_BOUNDARY,
            WorthQueryAspectPath::TOPOLOGY_RADIAL,
        ])
        .produces([WorthQueryAspectPath::DIAGNOSTICS_INTERPRETATIONS])
        .whole_refresh_fallback()
        .build()
}

pub fn worth_topology_validation_computed_declaration(
    surface_name: impl Into<String>,
) -> Result<ForgeQueryDerivedView, WorthQueryDeclarationError> {
    WorthQueryComputedDeclarationBuilder::new(surface_name)
        .reads([
            WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
            WorthQueryAspectPath::TOPOLOGY_OWNERSHIP,
            WorthQueryAspectPath::TOPOLOGY_BOUNDARY,
            WorthQueryAspectPath::TOPOLOGY_RADIAL,
            WorthQueryAspectPath::DIAGNOSTICS_INTERPRETATIONS,
        ])
        .produces([WorthQueryAspectPath::DIAGNOSTICS_DECISIONS])
        .whole_refresh_fallback()
        .build()
}

pub fn declare_worth_topology_interpreted_surface<T, M>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = worth_topology_interpreted_computed_declaration(surface_name)
        .map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?
        .depends_on_derived_name(materialized_view.name());
    workspace.computed_view(
        view,
        WorthTopologyInterpretedMaintainer::new(materialized_view.name()),
    )
}

pub fn declare_worth_topology_validation_surface<T, M, I>(
    workspace: &mut ForgeQueryWorkspace,
    surface_name: impl Into<String>,
    materialized_view: &ForgeQueryDerivedViewHandle<M>,
    interpreted_view: &ForgeQueryDerivedViewHandle<I>,
) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
    let surface_name = surface_name.into();
    let view = worth_topology_validation_computed_declaration(surface_name)
        .map_err(|error| {
            ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
        })?
        .depends_on_derived_name(materialized_view.name())
        .depends_on_derived_name(interpreted_view.name());
    workspace.computed_view(
        view,
        WorthTopologyValidationMaintainer::new(materialized_view.name(), interpreted_view.name()),
    )
}

pub fn interpreted_topology_from_materialized_rows(
    materialized_rows: &[Value],
) -> Result<InterpretedTopologyView, WorthTopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_single_computed_row(materialized_rows, "materialized topology")?;
    Ok(interpret_topology_view(&materialized))
}

pub fn validation_report_from_query_rows(
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
) -> Result<DerivedTopologyValidationReport, WorthTopologyQuerySurfaceError> {
    let materialized: MaterializedTopologyView =
        decode_single_computed_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_single_computed_row(interpreted_rows, "interpreted topology")?;
    validate_interpreted_topology(&materialized, &interpreted).map_err(validation_surface_error)
}

pub(super) fn decode_single_computed_row<T>(
    rows: &[Value],
    surface_label: &str,
) -> Result<T, WorthTopologyQuerySurfaceError>
where
    T: DeserializeOwned,
{
    match rows {
        [] => Err(WorthTopologyQuerySurfaceError::new(format!(
            "query-derived `{surface_label}` surface has no retained row to decode"
        ))),
        [row] => serde_json::from_value(row.clone()).map_err(|error| {
            WorthTopologyQuerySurfaceError::new(format!(
                "query-derived `{surface_label}` row failed to decode: {error}"
            ))
        }),
        _ => Err(WorthTopologyQuerySurfaceError::new(format!(
            "query-derived `{surface_label}` surface expected one retained row, found {}",
            rows.len()
        ))),
    }
}

fn validation_surface_error(error: WorthTopologyValidationError) -> WorthTopologyQuerySurfaceError {
    WorthTopologyQuerySurfaceError::new(format!("query-derived validation refresh failed: {error}"))
}
