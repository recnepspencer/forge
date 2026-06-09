use forge_foundational::facade::{
    foundational_boundary_artifact_milestone4_readiness_report,
    foundational_boundary_evidence_milestone7_readiness_report,
    foundational_performance_milestone8_readiness_report,
    FoundationalBoundaryArtifactCompileFailBoundary,
    FoundationalBoundaryEvidenceCompileFailBoundary, FoundationalPerformanceCompileFailBoundary,
};

use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveForbiddenFallbackSurface {
    OrdinaryRuntimeBackedReadBootstrap,
    OrdinaryRuntimeBackedReadBootstrapSupport,
}

impl MilestoneNineFiveForbiddenFallbackSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OrdinaryRuntimeBackedReadBootstrap => "ordinary_runtime_backed_read_bootstrap",
            Self::OrdinaryRuntimeBackedReadBootstrapSupport => {
                "ordinary_runtime_backed_read_bootstrap_support"
            }
        }
    }

    fn source(&self) -> &'static str {
        match self {
            Self::OrdinaryRuntimeBackedReadBootstrap => {
                include_str!("../../../tests/runtime_backed_read_bootstrap.rs")
            }
            Self::OrdinaryRuntimeBackedReadBootstrapSupport => {
                include_str!("../../../tests/support/public_bridge_runtime/common_bootstrap.rs")
            }
        }
    }

    fn all() -> &'static [Self] {
        &[
            Self::OrdinaryRuntimeBackedReadBootstrap,
            Self::OrdinaryRuntimeBackedReadBootstrapSupport,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneNineFiveForbiddenFallbackNeedle {
    ReadLiveArtifactBinding,
    ReadLiveArtifactBundle,
    BridgeBackedRuntimeBuilder,
}

impl MilestoneNineFiveForbiddenFallbackNeedle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadLiveArtifactBinding => "read_live_artifact_binding",
            Self::ReadLiveArtifactBundle => "read_live_artifact_bundle",
            Self::BridgeBackedRuntimeBuilder => "bridge_backed_runtime_builder",
        }
    }

    fn needle(&self) -> &'static str {
        self.as_str()
    }

    fn all() -> &'static [Self] {
        &[
            Self::ReadLiveArtifactBinding,
            Self::ReadLiveArtifactBundle,
            Self::BridgeBackedRuntimeBuilder,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineFiveForbiddenFallbackRow {
    surface: MilestoneNineFiveForbiddenFallbackSurface,
    forbidden_needle: MilestoneNineFiveForbiddenFallbackNeedle,
    occurrence_count: usize,
    row_digest: String,
}

impl MilestoneNineFiveForbiddenFallbackRow {
    pub fn surface(&self) -> MilestoneNineFiveForbiddenFallbackSurface {
        self.surface
    }

    pub fn forbidden_needle(&self) -> MilestoneNineFiveForbiddenFallbackNeedle {
        self.forbidden_needle
    }

    pub fn occurrence_count(&self) -> usize {
        self.occurrence_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNineFiveForbiddenFallbackCloseoutReport {
    rows: Vec<MilestoneNineFiveForbiddenFallbackRow>,
    total_occurrence_count: usize,
    boundary_artifact_compile_fail_boundaries: Vec<FoundationalBoundaryArtifactCompileFailBoundary>,
    boundary_evidence_compile_fail_boundaries: Vec<FoundationalBoundaryEvidenceCompileFailBoundary>,
    performance_compile_fail_boundaries: Vec<FoundationalPerformanceCompileFailBoundary>,
    report_digest: String,
}

impl MilestoneNineFiveForbiddenFallbackCloseoutReport {
    pub fn rows(&self) -> &[MilestoneNineFiveForbiddenFallbackRow] {
        &self.rows
    }

    pub fn total_occurrence_count(&self) -> usize {
        self.total_occurrence_count
    }

    pub fn boundary_artifact_compile_fail_boundaries(
        &self,
    ) -> &[FoundationalBoundaryArtifactCompileFailBoundary] {
        &self.boundary_artifact_compile_fail_boundaries
    }

    pub fn boundary_evidence_compile_fail_boundaries(
        &self,
    ) -> &[FoundationalBoundaryEvidenceCompileFailBoundary] {
        &self.boundary_evidence_compile_fail_boundaries
    }

    pub fn performance_compile_fail_boundaries(
        &self,
    ) -> &[FoundationalPerformanceCompileFailBoundary] {
        &self.performance_compile_fail_boundaries
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn milestone_nine_five_forbidden_fallback_closeout_report(
) -> MilestoneNineFiveForbiddenFallbackCloseoutReport {
    let boundary_artifact = foundational_boundary_artifact_milestone4_readiness_report();
    let boundary_evidence = foundational_boundary_evidence_milestone7_readiness_report();
    let performance = foundational_performance_milestone8_readiness_report();
    let rows = MilestoneNineFiveForbiddenFallbackSurface::all()
        .iter()
        .copied()
        .flat_map(rows_for_surface)
        .collect::<Vec<_>>();
    let total_occurrence_count = rows.iter().map(|row| row.occurrence_count()).sum();
    let boundary_artifact_compile_fail_boundaries =
        boundary_artifact.compile_fail_boundaries().to_vec();
    let boundary_evidence_compile_fail_boundaries =
        boundary_evidence.compile_fail_boundaries().to_vec();
    let performance_compile_fail_boundaries = performance.compile_fail_boundaries().to_vec();
    let report_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(format!(
                "total_occurrence_count:{total_occurrence_count}"
            )))
            .chain(std::iter::once(format!(
                "boundary_artifact_compile_fail_boundaries:{}",
                boundary_artifact_compile_fail_boundaries.len()
            )))
            .chain(std::iter::once(format!(
                "boundary_evidence_compile_fail_boundaries:{}",
                boundary_evidence_compile_fail_boundaries.len()
            )))
            .chain(std::iter::once(format!(
                "performance_compile_fail_boundaries:{}",
                performance_compile_fail_boundaries.len()
            )))
            .collect::<Vec<_>>(),
    );

    MilestoneNineFiveForbiddenFallbackCloseoutReport {
        rows,
        total_occurrence_count,
        boundary_artifact_compile_fail_boundaries,
        boundary_evidence_compile_fail_boundaries,
        performance_compile_fail_boundaries,
        report_digest,
    }
}

fn rows_for_surface(
    surface: MilestoneNineFiveForbiddenFallbackSurface,
) -> Vec<MilestoneNineFiveForbiddenFallbackRow> {
    MilestoneNineFiveForbiddenFallbackNeedle::all()
        .iter()
        .copied()
        .map(|forbidden_needle| row(surface, forbidden_needle))
        .collect()
}

fn row(
    surface: MilestoneNineFiveForbiddenFallbackSurface,
    forbidden_needle: MilestoneNineFiveForbiddenFallbackNeedle,
) -> MilestoneNineFiveForbiddenFallbackRow {
    let occurrence_count = surface.source().matches(forbidden_needle.needle()).count();
    let row_digest = hash_parts(&[
        "milestone_nine_five_forbidden_fallback_closeout_row_v1".to_string(),
        format!("surface:{}", surface.as_str()),
        format!("forbidden_needle:{}", forbidden_needle.as_str()),
        format!("occurrences:{occurrence_count}"),
    ]);

    MilestoneNineFiveForbiddenFallbackRow {
        surface,
        forbidden_needle,
        occurrence_count,
        row_digest,
    }
}

#[cfg(test)]
mod tests;
