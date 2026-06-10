#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRebindingContinuityKind {
    QueryContinuation,
    CorrespondenceOnly,
    KernelSummary,
}

impl PlanarRebindingContinuityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryContinuation => "query-continuation",
            Self::CorrespondenceOnly => "correspondence-only",
            Self::KernelSummary => "kernel-summary",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarRebindingContinuityEvidence {
    kind: PlanarRebindingContinuityKind,
    continuity_digest: String,
    neighborhood_replacement_digest: String,
}

impl PlanarRebindingContinuityEvidence {
    pub fn from_query_continuation(
        continuity_digest: impl Into<String>,
        neighborhood_replacement_digest: impl Into<String>,
    ) -> Self {
        Self {
            kind: PlanarRebindingContinuityKind::QueryContinuation,
            continuity_digest: continuity_digest.into(),
            neighborhood_replacement_digest: neighborhood_replacement_digest.into(),
        }
    }

    pub fn correspondence_only(continuity_digest: impl Into<String>) -> Self {
        Self {
            kind: PlanarRebindingContinuityKind::CorrespondenceOnly,
            continuity_digest: continuity_digest.into(),
            neighborhood_replacement_digest: String::new(),
        }
    }

    pub fn kernel_summary(continuity_digest: impl Into<String>) -> Self {
        Self {
            kind: PlanarRebindingContinuityKind::KernelSummary,
            continuity_digest: continuity_digest.into(),
            neighborhood_replacement_digest: String::new(),
        }
    }

    pub fn kind(&self) -> PlanarRebindingContinuityKind {
        self.kind
    }

    pub fn continuity_digest(&self) -> &str {
        &self.continuity_digest
    }

    pub fn neighborhood_replacement_digest(&self) -> &str {
        &self.neighborhood_replacement_digest
    }
}
