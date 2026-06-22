use crate::certification::error::TopologyCertificationError;

use super::super::support::{closed_row, collect_rs_sources_recursive, ensure, source_text};
use super::super::TopologyQueryBoundaryCleanupArea;

const DIRECT_VALIDATION_PATTERNS: [&str; 8] = [
    "validate_interpreted_topology(",
    "TopologyValidator::derived_validation_report",
    "TopologyValidator::materialized_validation_report",
    "ownership::validate(",
    "loop_wiring::validate(",
    "radial_rings::validate(",
    "shell_closure::validate(",
    "vertex_disks::validate(",
];
const DIRECT_VALIDATION_SURVIVOR: &str =
    "src/projection/diagnostic_surfaces/derived_read_diagnostics.rs";

pub(crate) fn certify_derived_validation_rehome_row(
) -> Result<super::super::TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let diagnostic_surface =
        source_text("src/projection/diagnostic_surfaces/derived_read_diagnostics.rs")?;
    let read_stage = source_text("src/projection/runtime_boundary/read_stage.rs")?;
    let computed_views =
        source_text("src/projection/runtime_boundary/declared_query_surfaces/derived_surfaces/computed_views.rs")?;
    let operator_closeout = source_text("src/certification/topology_operator_closeout/shared.rs")?;
    let covered_sources = covered_validation_rehome_sources()?;

    ensure(diagnostic_surface.contains("derive_topology_validation_report"))?;
    ensure(
        diagnostic_surface.contains("validate_interpreted_topology(materialized, interpreted)"),
    )?;
    ensure(diagnostic_surface.contains("DerivedValidationExecutionReport"))?;
    ensure(diagnostic_surface.contains("execution_count: 1"))?;
    ensure(prohibited_direct_validation_hits(&covered_sources).is_empty())?;
    for source in [&read_stage, &computed_views, &operator_closeout] {
        ensure(source.contains("derive_topology_validation_report"))?;
    }

    closed_row(
        TopologyQueryBoundaryCleanupArea::DerivedValidationRehome,
        "covered topology read, computed-view, and operator-closeout paths derive validation through the derived-read diagnostic surface instead of re-running local validator dispatch",
        Some("src/projection/diagnostic_surfaces/derived_read_diagnostics.rs"),
        [
            "src/projection/diagnostic_surfaces/derived_read_diagnostics.rs",
            "src/projection/runtime_boundary/read_stage.rs",
            "src/projection/runtime_boundary/declared_query_surfaces/derived_surfaces/computed_views.rs",
            "src/certification/topology_operator_closeout/shared.rs",
        ],
    )
}

fn covered_validation_rehome_sources() -> Result<Vec<(String, String)>, TopologyCertificationError>
{
    let mut sources = collect_rs_sources_recursive("src/projection/runtime_boundary")?;
    sources.extend(collect_rs_sources_recursive(
        "src/projection/diagnostic_surfaces",
    )?);
    sources.extend(collect_rs_sources_recursive(
        "src/certification/topology_operator_closeout",
    )?);
    Ok(sources)
}

fn prohibited_direct_validation_hits(sources: &[(String, String)]) -> Vec<String> {
    sources
        .iter()
        .filter(|(path, _)| path != DIRECT_VALIDATION_SURVIVOR)
        .flat_map(|(path, source)| {
            DIRECT_VALIDATION_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{path}:{pattern}"))
        })
        .collect()
}
