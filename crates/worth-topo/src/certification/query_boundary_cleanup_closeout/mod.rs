use std::fs;
use std::path::{Path, PathBuf};

use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::certification::DeterministicDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyQueryBoundaryCleanupArea {
    OperatorPath,
    SnapshotSurfaces,
    ReadViewDecode,
    BasisAdapter,
    PublicFacade,
}

impl TopologyQueryBoundaryCleanupArea {
    pub const ALL: [Self; 5] = [
        Self::OperatorPath,
        Self::SnapshotSurfaces,
        Self::ReadViewDecode,
        Self::BasisAdapter,
        Self::PublicFacade,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperatorPath => "operator_path",
            Self::SnapshotSurfaces => "snapshot_surfaces",
            Self::ReadViewDecode => "read_view_decode",
            Self::BasisAdapter => "basis_adapter",
            Self::PublicFacade => "public_facade",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyQueryBoundaryCleanupStatus {
    Closed,
    Gap,
}

impl TopologyQueryBoundaryCleanupStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Gap => "gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBoundaryCleanupRow {
    area: TopologyQueryBoundaryCleanupArea,
    status: TopologyQueryBoundaryCleanupStatus,
    reason: String,
    designated_survivor: Option<String>,
    row_digest: DeterministicDigest,
}

impl TopologyQueryBoundaryCleanupRow {
    pub fn area(&self) -> TopologyQueryBoundaryCleanupArea {
        self.area
    }

    pub fn status(&self) -> TopologyQueryBoundaryCleanupStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn designated_survivor(&self) -> Option<&str> {
        self.designated_survivor.as_deref()
    }

    pub fn row_digest(&self) -> &DeterministicDigest {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBoundaryCleanupCloseoutReport {
    rows: Vec<TopologyQueryBoundaryCleanupRow>,
    cleanup_complete: bool,
    closeout_digest: DeterministicDigest,
}

impl TopologyQueryBoundaryCleanupCloseoutReport {
    pub fn rows(&self) -> &[TopologyQueryBoundaryCleanupRow] {
        &self.rows
    }

    pub fn cleanup_complete(&self) -> bool {
        self.cleanup_complete
    }

    pub fn status(
        &self,
        area: TopologyQueryBoundaryCleanupArea,
    ) -> TopologyQueryBoundaryCleanupStatus {
        self.rows
            .iter()
            .find(|row| row.area() == area)
            .map(TopologyQueryBoundaryCleanupRow::status)
            .unwrap_or_else(|| panic!("cleanup closeout rows should cover every declared area"))
    }

    pub fn closeout_digest(&self) -> &DeterministicDigest {
        &self.closeout_digest
    }
}

pub fn certify_topology_query_boundary_cleanup_closeout(
) -> Result<TopologyQueryBoundaryCleanupCloseoutReport, TopologyCertificationError> {
    let rows = vec![
        certify_operator_path_row()?,
        certify_snapshot_surfaces_row()?,
        certify_read_view_decode_row()?,
        certify_basis_adapter_row()?,
        certify_public_facade_row()?,
    ];
    let cleanup_complete = rows
        .iter()
        .all(|row| row.status() == TopologyQueryBoundaryCleanupStatus::Closed);
    let closeout_digest = digest_rows(rows.iter().map(|row| {
        format!(
            "{}:{}:{}:{}:{}",
            row.area().as_str(),
            row.status().as_str(),
            row.reason(),
            row.designated_survivor().unwrap_or("none"),
            row.row_digest().digest_hex
        )
    }));
    let report = TopologyQueryBoundaryCleanupCloseoutReport {
        rows,
        cleanup_complete,
        closeout_digest,
    };
    if !report.cleanup_complete() {
        return Err(TopologyCertificationError::ReadView(
            "worth-topo query boundary cleanup closeout contains gap rows".to_string(),
        ));
    }
    Ok(report)
}

fn certify_operator_path_row() -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError>
{
    let sources = [
        source_text("src/topology_operators/application/mod.rs")?,
        source_text("src/topology_operators/application/admission.rs")?,
        source_text("src/topology_operators/application/bindings.rs")?,
        source_text("src/topology_operators/application/existing_truth.rs")?,
    ];
    ensure_all(&sources, |source| !source.contains(".payload"))?;
    ensure_all(&sources[..2], |source| {
        !source.contains("workspace.materialize(") && !source.contains("serde_json::from_value")
    })?;
    closed_row(
        TopologyQueryBoundaryCleanupArea::OperatorPath,
        "operator path depends on typed binding facts and shared post-write consumption instead of raw row archaeology",
        Some("src/projection/runtime_boundary/query_runtime/operator_bindings.rs"),
        ["src/topology_operators/application/mod.rs", "src/topology_operators/application/bindings.rs"],
    )
}

fn certify_snapshot_surfaces_row(
) -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let declared_query_surfaces_mod =
        source_text("src/projection/runtime_boundary/declared_query_surfaces/mod.rs")?;
    let historical_rows =
        source_text("src/projection/runtime_boundary/declared_query_surfaces/historical_rows.rs")?;
    let materialized_graph = source_text("src/derived_topology/materialized_graph/mod.rs")?;
    let persistent_naming = source_text("src/projection/truth_surfaces/persistent_naming.rs")?;

    ensure(
        !declared_query_surfaces_mod.contains("workspace.read(&self.entities)")
            && !declared_query_surfaces_mod.contains("workspace.materialize(&self.materialized)")
            && !declared_query_surfaces_mod.contains("naming_attachment_report_from_query"),
    )?;
    ensure(historical_rows.contains("TopologyQueryMaterializationInput::decode"))?;
    ensure(historical_rows.contains("TopologyNamingAttachmentInput::new"))?;
    ensure(!materialized_graph.contains("materialize_from_query_rows"))?;
    ensure(!persistent_naming.contains("naming_attachment_report_from_query_rows"))?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::SnapshotSurfaces,
        "snapshot and historical fallback ownership is concentrated in declared-query-surfaces boundary seams with typed naming and materialization ingress",
        Some("src/projection/runtime_boundary/declared_query_surfaces/historical_rows.rs"),
        [
            "src/projection/runtime_boundary/declared_query_surfaces/mod.rs",
            "src/projection/runtime_boundary/declared_query_surfaces/historical_rows.rs",
            "src/derived_topology/materialized_graph/query_input.rs",
        ],
    )
}

fn certify_read_view_decode_row(
) -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let sources = domain_view_sources()?;
    ensure_all(&sources, |source| !source.contains("ForgeQueryEntity"))?;
    ensure_all(&sources, |source| !source.contains("RetainedTopologyRows"))?;
    ensure_all(&sources, |source| !source.contains("serde_json::Value"))?;
    ensure_all(&sources, |source| !source.contains(".payload"))?;
    ensure_all(&sources, |source| !source.contains("get(\"relations\")"))?;
    ensure_all(&sources, |source| {
        !source.contains("get(\"relation_identities\")")
    })?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::ReadViewDecode,
        "public read views consume typed neighborhood facts and no longer traverse retained query payload maps directly",
        Some("src/projection/runtime_boundary/read_execution/neighborhood_decode/mod.rs"),
        [
            "src/projection/read_views/domain/views/adjacency.rs",
            "src/projection/read_views/domain/views/local_rewire.rs",
            "src/projection/read_views/domain/views/loop_cycle.rs",
        ],
    )
}

fn certify_basis_adapter_row() -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError>
{
    let basis_context =
        source_text("src/projection/runtime_boundary/read_execution/basis_context.rs")?;
    let family_execution =
        source_text("src/projection/runtime_boundary/read_execution/family_execution.rs")?;
    let contracts = source_text("src/projection/runtime_boundary/query_runtime/contracts.rs")?;
    let query_runtime_sources =
        collect_rs_sources("src/projection/runtime_boundary/query_runtime")?;

    ensure(!basis_context.contains("public_api_contract()"))?;
    ensure(!basis_context.contains("TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE"))?;
    ensure(family_execution.contains("TopologyReadBasisExecutionMode::for_workspace"))?;
    ensure(contracts.contains("workspace_requires_historical_basis_context"))?;

    let public_api_contract_mentions = query_runtime_sources
        .iter()
        .filter(|(_, source)| source.contains("public_api_contract()"))
        .map(|(path, _)| path.clone())
        .collect::<Vec<_>>();
    ensure(
        public_api_contract_mentions
            == vec!["src/projection/runtime_boundary/query_runtime/contracts.rs".to_string()],
    )?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::BasisAdapter,
        "historical basis posture is isolated to one documented runtime contract seam and no longer leaks broad contract archaeology into read execution",
        Some("src/projection/runtime_boundary/query_runtime/contracts.rs"),
        [
            "src/projection/runtime_boundary/read_execution/basis_context.rs",
            "src/projection/runtime_boundary/read_execution/family_execution.rs",
            "src/projection/runtime_boundary/query_runtime/contracts.rs",
        ],
    )
}

fn certify_public_facade_row() -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError>
{
    let facade = source_text("src/facade.rs")?;
    let compile_fail_contracts =
        source_text("src/certification/public_facade_contracts/compile_fail_contracts.rs")?;

    ensure(!facade.contains("from_query_rows"))?;
    ensure(!facade.contains("TopologyReadSessionState"))?;
    ensure(facade.contains("TopologyConfiguredDomainReadSession"))?;
    ensure(facade.contains("TopologyRuntimeSupport"))?;
    ensure(compile_fail_contracts.contains("public_query_row_helpers_not_exported.rs"))?;
    ensure(compile_fail_contracts.contains("public_query_row_materializer_not_exported.rs"))?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::PublicFacade,
        "public facade foregrounds typed topology-facing seams and compile-fail proof rejects the old row-shaped exports",
        Some("src/certification/public_facade_contracts/compile_fail/public_query_row_helpers_not_exported.rs"),
        [
            "src/facade.rs",
            "src/certification/public_facade_contracts/compile_fail_contracts.rs",
        ],
    )
}

fn closed_row(
    area: TopologyQueryBoundaryCleanupArea,
    reason: &str,
    designated_survivor: Option<&str>,
    evidence_paths: impl IntoIterator<Item = &'static str>,
) -> Result<TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let survivor = designated_survivor.map(str::to_string);
    let evidence = evidence_paths
        .into_iter()
        .map(|path| source_text(path).map(|source| format!("path:{path}\n{source}")))
        .collect::<Result<Vec<_>, _>>()?;
    let row_digest = digest_rows(evidence.into_iter());
    Ok(TopologyQueryBoundaryCleanupRow {
        area,
        status: TopologyQueryBoundaryCleanupStatus::Closed,
        reason: reason.to_string(),
        designated_survivor: survivor,
        row_digest,
    })
}

fn ensure(condition: bool) -> Result<(), TopologyCertificationError> {
    if condition {
        Ok(())
    } else {
        Err(TopologyCertificationError::ReadView(
            "worth-topo query boundary cleanup closeout structural proof failed".to_string(),
        ))
    }
}

fn ensure_all(
    sources: &[String],
    predicate: impl Fn(&str) -> bool,
) -> Result<(), TopologyCertificationError> {
    ensure(sources.iter().all(|source| predicate(source)))
}

fn source_text(relative: &str) -> Result<String, TopologyCertificationError> {
    fs::read_to_string(workspace_path(relative))
        .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))
}

fn domain_view_sources() -> Result<Vec<String>, TopologyCertificationError> {
    let mut sources = collect_rs_sources("src/projection/read_views/domain/views")?;
    sources.retain(|(path, _)| !path.ends_with("boundary_tests.rs") && !path.ends_with("mod.rs"));
    Ok(sources.into_iter().map(|(_, source)| source).collect())
}

fn collect_rs_sources(relative: &str) -> Result<Vec<(String, String)>, TopologyCertificationError> {
    let dir = workspace_path(relative);
    let mut sources = fs::read_dir(&dir)
        .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .map(|path| {
            let source = fs::read_to_string(&path)
                .map_err(|error| TopologyCertificationError::ReadView(error.to_string()))?;
            let relative = path
                .strip_prefix(workspace_root())
                .expect("source file should live inside workspace")
                .to_string_lossy()
                .replace('\\', "/");
            Ok((relative, source))
        })
        .collect::<Result<Vec<_>, TopologyCertificationError>>()?;
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(sources)
}

fn workspace_path(relative: &str) -> PathBuf {
    workspace_root().join(relative)
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(test)]
mod tests;
