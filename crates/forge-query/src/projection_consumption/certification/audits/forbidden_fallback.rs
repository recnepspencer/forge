use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionForbiddenFallbackSeam {
    ConsumeScalarFields,
    DecodeRowPair,
    DecodeRowTriple,
    VerifyScalarAlignment,
    ReadLiveArtifactBundle,
    BindLiveArtifact,
    ReadLiveArtifactBinding,
}

impl ProjectionConsumptionForbiddenFallbackSeam {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConsumeScalarFields => "consume_scalar_fields",
            Self::DecodeRowPair => "decode_row_pair",
            Self::DecodeRowTriple => "decode_row_triple",
            Self::VerifyScalarAlignment => "verify_scalar_alignment",
            Self::ReadLiveArtifactBundle => "read_live_artifact_bundle",
            Self::BindLiveArtifact => "bind_live_artifact",
            Self::ReadLiveArtifactBinding => "read_live_artifact_binding",
        }
    }

    fn needle(&self) -> &'static str {
        match self {
            Self::ConsumeScalarFields => "consume_scalar_fields",
            Self::DecodeRowPair => "decode_row_pair",
            Self::DecodeRowTriple => "decode_row_triple",
            Self::VerifyScalarAlignment => "verify_scalar_alignment",
            Self::ReadLiveArtifactBundle => "read_live_artifact_bundle",
            Self::BindLiveArtifact => "bind_live_artifact",
            Self::ReadLiveArtifactBinding => "read_live_artifact_binding",
        }
    }

    fn all() -> &'static [Self] {
        &[
            Self::ConsumeScalarFields,
            Self::DecodeRowPair,
            Self::DecodeRowTriple,
            Self::VerifyScalarAlignment,
            Self::ReadLiveArtifactBundle,
            Self::BindLiveArtifact,
            Self::ReadLiveArtifactBinding,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionOrdinaryPathSurface {
    CommonPathDx,
    RetainedLiveHostileTests,
    RetainedLivePhaseTwelveTests,
    CommonReadGolden,
    CommonWriteGolden,
    CommonQueryContextGolden,
    RetainedLiveGolden,
}

impl ProjectionConsumptionOrdinaryPathSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CommonPathDx => "common_path_dx",
            Self::RetainedLiveHostileTests => "retained_live_hostile_tests",
            Self::RetainedLivePhaseTwelveTests => "retained_live_phase_twelve_tests",
            Self::CommonReadGolden => "common_read_golden",
            Self::CommonWriteGolden => "common_write_golden",
            Self::CommonQueryContextGolden => "common_query_context_golden",
            Self::RetainedLiveGolden => "retained_live_golden",
        }
    }

    fn source(&self) -> &'static str {
        match self {
            Self::CommonPathDx => include_str!("../../dx.rs"),
            Self::RetainedLiveHostileTests => {
                include_str!("../../tests/retained_live/phase_eleven.rs")
            }
            Self::RetainedLivePhaseTwelveTests => {
                include_str!("../../tests/retained_live/phase_twelve.rs")
            }
            Self::CommonReadGolden => include_str!(
                "../../../../tests/ui/projection_consumption/golden/projection_consumption_common_read_golden_path_compiles.rs"
            ),
            Self::CommonWriteGolden => include_str!(
                "../../../../tests/ui/projection_consumption/golden/projection_consumption_common_write_golden_path_compiles.rs"
            ),
            Self::CommonQueryContextGolden => include_str!(
                "../../../../tests/ui/projection_consumption/golden/projection_consumption_common_query_context_golden_path_compiles.rs"
            ),
            Self::RetainedLiveGolden => include_str!(
                "../../../../tests/ui/projection_consumption/golden/projection_consumption_retained_live_ordinary_path_compiles.rs"
            ),
        }
    }

    fn all() -> &'static [Self] {
        &[
            Self::CommonPathDx,
            Self::RetainedLiveHostileTests,
            Self::RetainedLivePhaseTwelveTests,
            Self::CommonReadGolden,
            Self::CommonWriteGolden,
            Self::CommonQueryContextGolden,
            Self::RetainedLiveGolden,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionForbiddenFallbackAuditRow {
    ordinary_surface: ProjectionConsumptionOrdinaryPathSurface,
    forbidden_seam: ProjectionConsumptionForbiddenFallbackSeam,
    occurrence_count: usize,
    row_digest: String,
}

impl ProjectionConsumptionForbiddenFallbackAuditRow {
    pub fn ordinary_surface(&self) -> ProjectionConsumptionOrdinaryPathSurface {
        self.ordinary_surface
    }

    pub fn forbidden_seam(&self) -> ProjectionConsumptionForbiddenFallbackSeam {
        self.forbidden_seam
    }

    pub fn occurrence_count(&self) -> usize {
        self.occurrence_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionForbiddenFallbackAudit {
    rows: Vec<ProjectionConsumptionForbiddenFallbackAuditRow>,
    total_occurrence_count: usize,
    audit_digest: String,
}

impl ProjectionConsumptionForbiddenFallbackAudit {
    pub fn rows(&self) -> &[ProjectionConsumptionForbiddenFallbackAuditRow] {
        &self.rows
    }

    pub fn total_occurrence_count(&self) -> usize {
        self.total_occurrence_count
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}

pub fn projection_consumption_forbidden_fallback_audit(
) -> ProjectionConsumptionForbiddenFallbackAudit {
    let rows = ProjectionConsumptionOrdinaryPathSurface::all()
        .iter()
        .copied()
        .flat_map(rows_for_surface)
        .collect::<Vec<_>>();
    let total_occurrence_count = rows.iter().map(|row| row.occurrence_count()).sum();
    let audit_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(format!(
                "total_occurrence_count:{total_occurrence_count}"
            )))
            .collect::<Vec<_>>(),
    );
    ProjectionConsumptionForbiddenFallbackAudit {
        rows,
        total_occurrence_count,
        audit_digest,
    }
}

fn rows_for_surface(
    ordinary_surface: ProjectionConsumptionOrdinaryPathSurface,
) -> Vec<ProjectionConsumptionForbiddenFallbackAuditRow> {
    ProjectionConsumptionForbiddenFallbackSeam::all()
        .iter()
        .copied()
        .map(|forbidden_seam| row(ordinary_surface, forbidden_seam))
        .collect()
}

fn row(
    ordinary_surface: ProjectionConsumptionOrdinaryPathSurface,
    forbidden_seam: ProjectionConsumptionForbiddenFallbackSeam,
) -> ProjectionConsumptionForbiddenFallbackAuditRow {
    let occurrence_count = ordinary_surface
        .source()
        .matches(forbidden_seam.needle())
        .count();
    let row_digest = hash_parts(&[
        "projection_consumption_forbidden_fallback_row_v1".to_string(),
        format!("surface:{}", ordinary_surface.as_str()),
        format!("seam:{}", forbidden_seam.as_str()),
        format!("occurrences:{occurrence_count}"),
    ]);
    ProjectionConsumptionForbiddenFallbackAuditRow {
        ordinary_surface,
        forbidden_seam,
        occurrence_count,
        row_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_fallback_audit_stays_zero_for_ordinary_path_sources() {
        let audit = projection_consumption_forbidden_fallback_audit();
        assert_eq!(
            audit.rows().len(),
            ProjectionConsumptionOrdinaryPathSurface::all().len()
                * ProjectionConsumptionForbiddenFallbackSeam::all().len()
        );
        assert_eq!(audit.total_occurrence_count(), 0);
        assert!(audit.rows().iter().all(|row| row.occurrence_count() == 0));
        assert!(!audit.audit_digest().is_empty());
    }
}
