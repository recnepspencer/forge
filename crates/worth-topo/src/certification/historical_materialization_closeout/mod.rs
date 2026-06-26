use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::certification::DeterministicDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyHistoricalMaterializationArea {
    RuntimeArtifactFloors,
    RetainedTruthProjection,
    HistoricalSnapshotEntry,
}

impl TopologyHistoricalMaterializationArea {
    pub const ALL: [Self; 3] = [
        Self::RuntimeArtifactFloors,
        Self::RetainedTruthProjection,
        Self::HistoricalSnapshotEntry,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeArtifactFloors => "runtime_artifact_floors",
            Self::RetainedTruthProjection => "retained_truth_projection",
            Self::HistoricalSnapshotEntry => "historical_snapshot_entry",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyHistoricalMaterializationStatus {
    Closed,
    Gap,
}

impl TopologyHistoricalMaterializationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Gap => "gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyHistoricalMaterializationRow {
    area: TopologyHistoricalMaterializationArea,
    status: TopologyHistoricalMaterializationStatus,
    reason: String,
    designated_survivor: String,
    row_digest: DeterministicDigest,
}

impl TopologyHistoricalMaterializationRow {
    pub fn area(&self) -> TopologyHistoricalMaterializationArea {
        self.area
    }

    pub fn status(&self) -> TopologyHistoricalMaterializationStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn designated_survivor(&self) -> &str {
        &self.designated_survivor
    }

    pub fn row_digest(&self) -> &DeterministicDigest {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyHistoricalMaterializationCloseoutReport {
    rows: Vec<TopologyHistoricalMaterializationRow>,
    phase_seven_ready: bool,
    closeout_digest: DeterministicDigest,
}

impl TopologyHistoricalMaterializationCloseoutReport {
    pub fn rows(&self) -> &[TopologyHistoricalMaterializationRow] {
        &self.rows
    }

    pub fn phase_seven_ready(&self) -> bool {
        self.phase_seven_ready
    }

    pub fn status(
        &self,
        area: TopologyHistoricalMaterializationArea,
    ) -> TopologyHistoricalMaterializationStatus {
        self.rows
            .iter()
            .find(|row| row.area() == area)
            .map(TopologyHistoricalMaterializationRow::status)
            .unwrap_or_else(|| {
                panic!("historical materialization closeout rows should cover every area")
            })
    }

    pub fn closeout_digest(&self) -> &DeterministicDigest {
        &self.closeout_digest
    }
}

pub fn certify_topology_historical_materialization_closeout(
) -> Result<TopologyHistoricalMaterializationCloseoutReport, TopologyCertificationError> {
    let rows = vec![
        certify_runtime_artifact_floors_row()?,
        certify_retained_truth_projection_row()?,
        certify_historical_snapshot_entry_row()?,
    ];
    let phase_seven_ready = rows
        .iter()
        .all(|row| row.status() == TopologyHistoricalMaterializationStatus::Closed);
    let closeout_digest = digest_rows(rows.iter().map(|row| {
        format!(
            "{}:{}:{}:{}:{}",
            row.area().as_str(),
            row.status().as_str(),
            row.reason(),
            row.designated_survivor(),
            row.row_digest().digest_hex
        )
    }));
    let report = TopologyHistoricalMaterializationCloseoutReport {
        rows,
        phase_seven_ready,
        closeout_digest,
    };
    if !report.phase_seven_ready() {
        return Err(TopologyCertificationError::ReadView(
            "worth-topo historical materialization closeout contains gap rows".to_string(),
        ));
    }
    Ok(report)
}

fn certify_runtime_artifact_floors_row(
) -> Result<TopologyHistoricalMaterializationRow, TopologyCertificationError> {
    let derived_binding =
        source_text("..\\forge-query\\src\\runtime\\surface\\derived_artifact_binding.rs")?;
    let retained_scalar_facts =
        source_text("..\\forge-query\\src\\runtime\\surface\\retained_scalar_facts.rs")?;
    let retained_scalar_alignment =
        source_text("..\\forge-query\\src\\runtime\\surface\\retained_scalar_alignment.rs")?;
    let materialization_intents = source_text(
        "..\\forge-query\\src\\runtime\\runtime_inspection_materialization_intents.rs",
    )?;
    let workspace_live_queries =
        source_text("..\\forge-query\\src\\runtime\\workspace_live_queries.rs")?;

    ensure(derived_binding.contains("pub fn materialization<T>("))?;
    ensure(derived_binding.contains("single_retained_row"))?;
    ensure(retained_scalar_facts.contains("pub fn consume_scalar_fields"))?;
    ensure(retained_scalar_alignment.contains("retained scalar"))?;
    ensure(materialization_intents.contains("pub fn materialize_derived_artifact_binding"))?;
    ensure(workspace_live_queries.contains("pub fn read_live_artifact_binding"))?;

    closed_row(
        TopologyHistoricalMaterializationArea::RuntimeArtifactFloors,
        "Query runtime owns the retained and live historical artifact floors through exact materialize-and-bind, read-and-bind, retained row access, retained scalar evidence, and native retained payload decode seams",
        "crates/forge-query/src/runtime",
    )
}

fn certify_retained_truth_projection_row(
) -> Result<TopologyHistoricalMaterializationRow, TopologyCertificationError> {
    let retained_artifacts = source_text(
        "src/projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs",
    )?;

    ensure(retained_artifacts.contains("TopologyHistoricalTruthArtifact"))?;
    ensure(retained_artifacts.contains("TopologyHistoricalDerivedSurfaceSnapshot"))?;
    ensure(retained_artifacts.contains("pub(crate) fn materialized(&self)"))?;
    ensure(retained_artifacts.contains("pub(crate) fn diagnostics(&self)"))?;
    ensure(retained_artifacts.contains("materialize_declared_query_surface_binding("))?;
    ensure(retained_artifacts.contains("decode_bundle_row("))?;
    ensure(retained_artifacts.contains("single_retained_row()"))?;
    ensure(retained_artifacts.contains("retained_payload::decode_retained_payload_row("))?;
    ensure(
        retained_artifacts
            .contains("diagnostics.equivalence_contract_report != equivalence_contract"),
    )?;
    ensure(!retained_artifacts.contains("workspace.materialize("))?;
    ensure(!retained_artifacts.contains("workspace.read("))?;
    ensure(!retained_artifacts.contains("stage_topology_read_from_view("))?;
    ensure(!retained_artifacts.contains("TopologyQueryMaterializationInput::decode("))?;
    ensure(!retained_artifacts.contains("equivalence_contract_from_diagnostics_rows"))?;
    ensure(!retained_artifacts.contains("verify_scalar_alignment("))?;
    ensure(!retained_artifacts.contains("decode_row_triple("))?;
    ensure(!retained_artifacts.contains("decode_row_pair("))?;

    closed_row(
        TopologyHistoricalMaterializationArea::RetainedTruthProjection,
        "the surviving production retained-artifact seam is now a thin topology projection over Query-owned retained artifact floors and native retained payload rows, not local historical truth reconstruction from staged read authority or legacy paired/triple row decoding",
        "src/projection/runtime_boundary/declared_query_surfaces/retained_artifacts.rs",
    )
}

fn certify_historical_snapshot_entry_row(
) -> Result<TopologyHistoricalMaterializationRow, TopologyCertificationError> {
    let declared_query_surfaces_mod =
        source_text("src/projection/runtime_boundary/declared_query_surfaces/mod.rs")?;
    let read_basis_runtime = source_text("src/certification/support/read_basis_query_runtime.rs")?;
    let derived_snapshot =
        source_text("src/certification/support/historical_query_snapshot/derived_snapshot.rs")?;
    let full_snapshot =
        source_text("src/certification/support/historical_query_snapshot/full_snapshot.rs")?;

    ensure(!declared_query_surfaces_mod.contains("historical_snapshot"))?;
    ensure(read_basis_runtime.contains("historical_derived_surface_snapshot("))?;
    ensure(derived_snapshot.contains("runtime.historical_derived_surface_snapshot()"))?;
    ensure(derived_snapshot.contains("runtime.historical_equivalence_read_basis_facts()"))?;
    ensure(!derived_snapshot.contains("materialize_derived_artifact_bundle("))?;
    ensure(full_snapshot.contains("read_declared_query_surface_binding("))?;
    ensure(!full_snapshot.contains("workspace.read(surfaces.entities())"))?;
    ensure(!full_snapshot.contains("workspace.read(surfaces.persistent_names())"))?;
    ensure(!full_snapshot.contains("stage_topology_read_from_view("))?;

    closed_row(
        TopologyHistoricalMaterializationArea::HistoricalSnapshotEntry,
        "certification historical snapshot callers now cross one shared read-basis query-runtime seam and Query-owned retained/live artifact bindings instead of reopening staged read, declaration, materialization, or live-read archaeology inline",
        "src/certification/support/read_basis_query_runtime.rs",
    )
}

fn source_text(path: &str) -> Result<String, TopologyCertificationError> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read_to_string(&path)
        .map_err(|error| TopologyCertificationError::ReadView(format!("{path:?}: {error}")))
}

fn ensure(condition: bool) -> Result<(), TopologyCertificationError> {
    if condition {
        Ok(())
    } else {
        Err(TopologyCertificationError::ReadView(
            "historical materialization closeout assertion failed".to_string(),
        ))
    }
}

fn closed_row(
    area: TopologyHistoricalMaterializationArea,
    reason: impl Into<String>,
    designated_survivor: impl Into<String>,
) -> Result<TopologyHistoricalMaterializationRow, TopologyCertificationError> {
    let reason = reason.into();
    let designated_survivor = designated_survivor.into();
    let row_digest = digest_rows(
        vec![format!(
            "{}:{}:{}",
            area.as_str(),
            reason,
            designated_survivor
        )]
        .into_iter(),
    );
    Ok(TopologyHistoricalMaterializationRow {
        area,
        status: TopologyHistoricalMaterializationStatus::Closed,
        reason,
        designated_survivor,
        row_digest,
    })
}

#[cfg(test)]
mod tests;
