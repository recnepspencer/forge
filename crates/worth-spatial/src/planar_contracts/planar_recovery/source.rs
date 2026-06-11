#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoverySourceKind {
    ProjectionBasisDenial,
    RetainedOrProjectionBasisDenial,
    DirtyPlanarInput,
    UnboundedPlanarClass,
    KernelSummary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarRecoverySource {
    kind: PlanarRecoverySourceKind,
    source_digest: String,
    original_outcome_class: String,
    source_family: String,
}

impl PlanarRecoverySource {
    pub fn from_projection_denial(source_digest: impl Into<String>) -> Self {
        Self::typed_denial(
            PlanarRecoverySourceKind::ProjectionBasisDenial,
            source_digest,
            "denied:projection_basis",
            "planar.projection",
        )
    }

    pub fn from_retained_or_projection_basis_denial(source_digest: impl Into<String>) -> Self {
        Self::typed_denial(
            PlanarRecoverySourceKind::RetainedOrProjectionBasisDenial,
            source_digest,
            "denied:retained_or_projection_basis",
            "planar.retained_projection",
        )
    }

    pub fn dirty_input(source_digest: impl Into<String>) -> Self {
        Self {
            kind: PlanarRecoverySourceKind::DirtyPlanarInput,
            source_digest: source_digest.into(),
            original_outcome_class: "dirty_planar_input".to_string(),
            source_family: "planar.clean_fail.dirty_input".to_string(),
        }
    }

    pub fn unbounded_or_open(source_digest: impl Into<String>) -> Self {
        Self {
            kind: PlanarRecoverySourceKind::UnboundedPlanarClass,
            source_digest: source_digest.into(),
            original_outcome_class: "unsupported:unbounded_or_open".to_string(),
            source_family: "planar.clean_fail.unbounded_or_open".to_string(),
        }
    }

    pub fn from_kernel_summary(summary: impl Into<String>) -> Self {
        Self {
            kind: PlanarRecoverySourceKind::KernelSummary,
            source_digest: summary.into(),
            original_outcome_class: "summary_only".to_string(),
            source_family: "kernel.summary".to_string(),
        }
    }

    pub fn kind(&self) -> PlanarRecoverySourceKind {
        self.kind
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn original_outcome_class(&self) -> &str {
        &self.original_outcome_class
    }

    pub fn source_family(&self) -> &str {
        &self.source_family
    }

    fn typed_denial(
        kind: PlanarRecoverySourceKind,
        source_digest: impl Into<String>,
        original_outcome_class: impl Into<String>,
        source_family: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_digest: source_digest.into(),
            original_outcome_class: original_outcome_class.into(),
            source_family: source_family.into(),
        }
    }
}
