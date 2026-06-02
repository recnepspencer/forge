use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::certification::DeterministicDigest;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyCommittedArtifactAlignmentArea {
    QueryAftermathFloor,
    AcceptedMutationProjection,
    LiveArtifactContract,
}

impl TopologyCommittedArtifactAlignmentArea {
    pub const ALL: [Self; 3] = [
        Self::QueryAftermathFloor,
        Self::AcceptedMutationProjection,
        Self::LiveArtifactContract,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryAftermathFloor => "query_aftermath_floor",
            Self::AcceptedMutationProjection => "accepted_mutation_projection",
            Self::LiveArtifactContract => "live_artifact_contract",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyCommittedArtifactAlignmentStatus {
    Closed,
    Gap,
}

impl TopologyCommittedArtifactAlignmentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Gap => "gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyCommittedArtifactAlignmentRow {
    area: TopologyCommittedArtifactAlignmentArea,
    status: TopologyCommittedArtifactAlignmentStatus,
    reason: String,
    designated_survivor: String,
    row_digest: DeterministicDigest,
}

impl TopologyCommittedArtifactAlignmentRow {
    pub fn area(&self) -> TopologyCommittedArtifactAlignmentArea {
        self.area
    }

    pub fn status(&self) -> TopologyCommittedArtifactAlignmentStatus {
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
pub struct TopologyCommittedArtifactAlignmentCloseoutReport {
    rows: Vec<TopologyCommittedArtifactAlignmentRow>,
    phase_nine_ready: bool,
    closeout_digest: DeterministicDigest,
}

impl TopologyCommittedArtifactAlignmentCloseoutReport {
    pub fn rows(&self) -> &[TopologyCommittedArtifactAlignmentRow] {
        &self.rows
    }

    pub fn phase_nine_ready(&self) -> bool {
        self.phase_nine_ready
    }

    pub fn status(
        &self,
        area: TopologyCommittedArtifactAlignmentArea,
    ) -> TopologyCommittedArtifactAlignmentStatus {
        self.rows
            .iter()
            .find(|row| row.area() == area)
            .map(TopologyCommittedArtifactAlignmentRow::status)
            .unwrap_or_else(|| {
                panic!("committed artifact alignment closeout rows should cover every area")
            })
    }

    pub fn closeout_digest(&self) -> &DeterministicDigest {
        &self.closeout_digest
    }
}

pub fn certify_topology_committed_artifact_alignment_closeout(
) -> Result<TopologyCommittedArtifactAlignmentCloseoutReport, TopologyCertificationError> {
    let rows = vec![
        certify_query_aftermath_floor_row()?,
        certify_accepted_mutation_projection_row()?,
        certify_live_artifact_contract_row()?,
    ];
    let phase_nine_ready = rows
        .iter()
        .all(|row| row.status() == TopologyCommittedArtifactAlignmentStatus::Closed);
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
    let report = TopologyCommittedArtifactAlignmentCloseoutReport {
        rows,
        phase_nine_ready,
        closeout_digest,
    };
    if !report.phase_nine_ready() {
        return Err(TopologyCertificationError::ReadView(
            "worth-topo committed artifact alignment closeout contains gap rows".to_string(),
        ));
    }
    Ok(report)
}

fn certify_query_aftermath_floor_row(
) -> Result<TopologyCommittedArtifactAlignmentRow, TopologyCertificationError> {
    let materialization_intents = source_text(
        "..\\forge-query\\src\\runtime\\runtime_inspection_materialization_intents.rs",
    )?;
    let operator_post_write =
        source_text("src/projection/runtime_boundary/query_runtime/operator_post_write.rs")?;
    let application_mod = source_text("src/topology_operators/application/mod.rs")?;

    ensure(materialization_intents.contains("pub fn materialize_batch_write_artifact_binding"))?;
    ensure(operator_post_write.contains("TopologyPostWriteQueryArtifact"))?;
    ensure(operator_post_write.contains("materialize_batch_write_artifact_binding("))?;
    ensure(!operator_post_write.contains("workspace.inspect("))?;
    ensure(!operator_post_write.contains("workspace.materialize("))?;
    ensure(
        application_mod
            .contains("let post_write_query_artifact = TopologyPostWriteQueryArtifact::build("),
    )?;

    closed_row(
        TopologyCommittedArtifactAlignmentArea::QueryAftermathFloor,
        "Query runtime owns the post-write retained artifact floor and topo crosses that seam before any topology-specific committed-artifact projection",
        "src/projection/runtime_boundary/query_runtime/operator_post_write.rs",
    )
}

fn certify_accepted_mutation_projection_row(
) -> Result<TopologyCommittedArtifactAlignmentRow, TopologyCertificationError> {
    let declared_mutation_artifact =
        source_text("src/topology_operators/application/declared_mutation_artifact.rs")?;
    let accepted_mutation_projection = source_text(
        "src/topology_operators/application/declared_mutation_artifact/accepted_mutation_projection.rs",
    )?;
    let replay_step_rows =
        source_text("src/certification/topology_operator_closeout/replay_step_rows.rs")?;
    let declaration_runtime = source_text("src/certification/support/declaration_runtime.rs")?;

    ensure(
        declared_mutation_artifact
            .contains("accepted_mutation_projection: TopologyAcceptedMutationProjection"),
    )?;
    ensure(declared_mutation_artifact.contains(
        "pub(crate) fn accepted_mutation_projection(&self) -> &TopologyAcceptedMutationProjection",
    ))?;
    ensure(!declared_mutation_artifact.contains("declared_mutation_synopsis:"))?;
    ensure(
        !declared_mutation_artifact.contains("accepted_query_contribution_semantic_projection:"),
    )?;
    ensure(accepted_mutation_projection.contains("semantic_family_key: &'static str"))?;
    ensure(
        accepted_mutation_projection
            .contains("naming_mutation_continuity_matrix: NamingMutationContinuityMatrix"),
    )?;
    ensure(accepted_mutation_projection.contains("fallback_explanation_detail: &'static str"))?;
    ensure(replay_step_rows.contains("accepted_mutation_projection()"))?;
    ensure(!replay_step_rows.contains("declared_mutation_synopsis()"))?;
    ensure(!replay_step_rows.contains("accepted_query_contribution_semantic_projection()"))?;
    ensure(declaration_runtime.contains("artifact.accepted_mutation_projection()"))?;

    closed_row(
        TopologyCommittedArtifactAlignmentArea::AcceptedMutationProjection,
        "the surviving topology-owned committed-artifact meaning now lives on one accepted mutation projection seam instead of split declaration synopsis and retained semantic projection lanes",
        "src/topology_operators/application/declared_mutation_artifact/accepted_mutation_projection.rs",
    )
}

fn certify_live_artifact_contract_row(
) -> Result<TopologyCommittedArtifactAlignmentRow, TopologyCertificationError> {
    let declared_mutation_artifact =
        source_text("src/topology_operators/application/declared_mutation_artifact.rs")?;
    let boundary_tests = source_text("src/topology_operators/application/boundary_tests.rs")?;
    let migration_plan =
        source_text("..\\..\\_docs\\worth_topo\\worth-topo-query-native-migration-plan.md")?;

    ensure(
        declared_mutation_artifact
            .contains("post_write_query_artifact: TopologyPostWriteQueryArtifact"),
    )?;
    ensure(
        declared_mutation_artifact
            .contains("accepted_mutation_projection: TopologyAcceptedMutationProjection"),
    )?;
    ensure(declared_mutation_artifact.contains("#[cfg(test)]\n    pub(crate) fn receipt("))?;
    ensure(declared_mutation_artifact.contains("#[cfg(test)]\n    pub(crate) fn inspection("))?;
    ensure(boundary_tests.contains(
        "declared mutation artifact should not keep separate live synopsis and semantic-projection accessors once one accepted mutation projection seam exists",
    ))?;
    ensure(migration_plan.contains("Phase 9 accepted mutation projection slice:"))?;
    ensure(
        migration_plan
            .contains("resolves to one accepted mutation projection plus topology-specific"),
    )?;

    closed_row(
        TopologyCommittedArtifactAlignmentArea::LiveArtifactContract,
        "the surviving live committed-artifact contract is one accepted mutation projection plus topology-specific materialized aftermath, while generic Query aftermath proof access stays test-only",
        "src/topology_operators/application/declared_mutation_artifact.rs",
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
            "committed artifact alignment closeout assertion failed".to_string(),
        ))
    }
}

fn closed_row(
    area: TopologyCommittedArtifactAlignmentArea,
    reason: impl Into<String>,
    designated_survivor: impl Into<String>,
) -> Result<TopologyCommittedArtifactAlignmentRow, TopologyCertificationError> {
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
    Ok(TopologyCommittedArtifactAlignmentRow {
        area,
        status: TopologyCommittedArtifactAlignmentStatus::Closed,
        reason,
        designated_survivor,
        row_digest,
    })
}
