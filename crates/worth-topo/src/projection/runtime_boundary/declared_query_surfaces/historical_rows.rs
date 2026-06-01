use forge_query::facade::ForgeQueryWorkspace;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde_json::Value;

use super::snapshot_rows::TopologyQuerySnapshotRows;
use super::{TopologyDeclaredQuerySurfaces, TopologyQuerySurfaceError};
use crate::derived_topology::materialized_graph::{
    TopologyMaterializer, TopologyQueryMaterializationInput,
};
use crate::facade::{
    DerivedReadDiagnostics, DerivedTopologyValidationReport, InterpretedTopologyView,
};
use crate::projection::diagnostic_surfaces::build_derived_read_diagnostics;
use crate::projection::{
    interpreted_topology_from_materialized_rows, naming_attachment_report_from_query_input,
    validation_report_from_query_rows, TopologyNamingAttachmentInput,
};

pub(super) fn historical_snapshot_rows(
    surfaces: &TopologyDeclaredQuerySurfaces,
    workspace: &mut ForgeQueryWorkspace,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<TopologyQuerySnapshotRows, TopologyQuerySurfaceError> {
    let entity_rows = workspace.read(surfaces.entities());
    let relation_rows = workspace.read(surfaces.relations());
    let persistent_name_rows = workspace.read(surfaces.persistent_names());
    let naming_attachments = naming_attachment_report_from_query_input(
        TopologyNamingAttachmentInput::new(&entity_rows, &persistent_name_rows),
    )?;

    let materialized_input =
        TopologyQueryMaterializationInput::decode(&entity_rows, &relation_rows)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let materialized = TopologyMaterializer::materialize_query_input(&materialized_input)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let materialized_rows = vec![encode_row(
        &materialized,
        "query-derived `materialized topology` row",
    )?];

    let interpreted_rows = workspace.materialize(surfaces.interpreted());
    let (interpreted, interpreted_rows) = if interpreted_rows.is_empty() {
        let interpreted = interpreted_topology_from_materialized_rows(&materialized_rows)?;
        let interpreted_rows = vec![encode_row(
            &interpreted,
            "query-derived `interpreted topology` row",
        )?];
        (interpreted, interpreted_rows)
    } else {
        let interpreted: InterpretedTopologyView =
            serde_json::from_value(interpreted_rows[0].clone()).map_err(|error| {
                TopologyQuerySurfaceError::new(format!(
                    "query-derived `interpreted topology` row failed to decode: {error}"
                ))
            })?;
        (interpreted, interpreted_rows)
    };

    let validation_rows = workspace.materialize(surfaces.validation());
    let (validation, validation_rows) = if validation_rows.is_empty() {
        let validation = validation_report_from_query_rows(&materialized_rows, &interpreted_rows)?;
        let validation_rows = vec![encode_row(
            &validation,
            "query-derived `topology validation` row",
        )?];
        (validation, validation_rows)
    } else {
        let validation: DerivedTopologyValidationReport =
            serde_json::from_value(validation_rows[0].clone()).map_err(|error| {
                TopologyQuerySurfaceError::new(format!(
                    "query-derived `topology validation` row failed to decode: {error}"
                ))
            })?;
        (validation, validation_rows)
    };

    let diagnostics_rows = workspace.materialize(surfaces.diagnostics());
    let (diagnostics, diagnostics_rows) = if diagnostics_rows.is_empty() {
        let diagnostics =
            build_derived_read_diagnostics(read_basis, &materialized, &interpreted, &validation);
        let diagnostics_rows = vec![encode_row(
            &diagnostics,
            "query-derived `derived read diagnostics` row",
        )?];
        (diagnostics, diagnostics_rows)
    } else {
        let diagnostics: DerivedReadDiagnostics =
            serde_json::from_value(diagnostics_rows[0].clone()).map_err(|error| {
                TopologyQuerySurfaceError::new(format!(
                    "query-derived `derived read diagnostics` row failed to decode: {error}"
                ))
            })?;
        (diagnostics, diagnostics_rows)
    };

    let equivalence_rows = workspace.materialize(surfaces.equivalence_contract());
    let equivalence_rows = if equivalence_rows.is_empty() {
        vec![encode_row(
            &diagnostics.equivalence_contract_report,
            "query-derived `derived equivalence contract` row",
        )?]
    } else {
        equivalence_rows
    };

    Ok(TopologyQuerySnapshotRows {
        naming_attachments,
        materialized_rows,
        interpreted_rows,
        validation_rows,
        diagnostics_rows,
        equivalence_rows,
    })
}

fn encode_row<T: serde::Serialize>(
    value: &T,
    label: &str,
) -> Result<Value, TopologyQuerySurfaceError> {
    serde_json::to_value(value).map_err(|error| {
        TopologyQuerySurfaceError::new(format!("{label} failed to encode: {error}"))
    })
}
