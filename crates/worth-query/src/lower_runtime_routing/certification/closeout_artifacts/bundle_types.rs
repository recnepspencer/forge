use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeCertificationLane {
    CrossingsSurface,
    BoundaryClosureSurface,
    AcceptanceEvidence,
    SyntheticTailPolicy,
    RouteParity,
    FormerSpecialistSeamClosure,
    DeferredNeighborDenial,
    DownstreamBoundaryAudit,
    ProofShapeSurface,
    CompileFailBoundary,
    Performance,
}

impl WorthQueryLowerRuntimeCertificationLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CrossingsSurface => "crossings-surface",
            Self::BoundaryClosureSurface => "boundary-closure-surface",
            Self::AcceptanceEvidence => "acceptance-evidence",
            Self::SyntheticTailPolicy => "synthetic-tail-policy",
            Self::RouteParity => "route-parity",
            Self::FormerSpecialistSeamClosure => "former-specialist-seam-closure",
            Self::DeferredNeighborDenial => "deferred-neighbor-denial",
            Self::DownstreamBoundaryAudit => "downstream-boundary-audit",
            Self::ProofShapeSurface => "proof-shape-surface",
            Self::CompileFailBoundary => "compile-fail-boundary",
            Self::Performance => "performance",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeCertificationRow {
    lane: WorthQueryLowerRuntimeCertificationLane,
    artifact_digest: String,
    detail: String,
    counter_snapshot_digest: String,
    failure_digest: Option<String>,
    row_digest: String,
}

impl WorthQueryLowerRuntimeCertificationRow {
    pub(crate) fn new(
        lane: WorthQueryLowerRuntimeCertificationLane,
        artifact_digest: impl Into<String>,
        detail: impl Into<String>,
        counter_snapshot_digest: impl Into<String>,
        failure_digest: Option<String>,
    ) -> Self {
        let artifact_digest = artifact_digest.into();
        let detail = detail.into();
        let counter_snapshot_digest = counter_snapshot_digest.into();
        let row_digest = hash_parts(&[
            "lower_runtime_certification_row_v1".to_string(),
            format!("lane:{}", lane.as_str()),
            format!("artifact:{artifact_digest}"),
            format!("detail:{detail}"),
            format!("counters:{counter_snapshot_digest}"),
            format!(
                "failure:{}",
                failure_digest.clone().unwrap_or_else(|| "none".to_string())
            ),
        ]);
        Self {
            lane,
            artifact_digest,
            detail,
            counter_snapshot_digest,
            failure_digest,
            row_digest,
        }
    }

    pub fn lane(&self) -> WorthQueryLowerRuntimeCertificationLane {
        self.lane
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeCertificationOutputDigest {
    name: &'static str,
    digest: String,
}

impl WorthQueryLowerRuntimeCertificationOutputDigest {
    pub(crate) fn new(name: &'static str, digest: impl Into<String>) -> Self {
        Self {
            name,
            digest: digest.into(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeCertificationBundle {
    rows: Vec<WorthQueryLowerRuntimeCertificationRow>,
    output_digests: Vec<WorthQueryLowerRuntimeCertificationOutputDigest>,
    certification_bundle_digest: String,
}

impl WorthQueryLowerRuntimeCertificationBundle {
    pub(crate) fn new(
        rows: Vec<WorthQueryLowerRuntimeCertificationRow>,
        output_digests: Vec<WorthQueryLowerRuntimeCertificationOutputDigest>,
    ) -> Self {
        let certification_bundle_digest = hash_parts(&[
            hash_parts(
                &rows
                    .iter()
                    .map(|row| row.row_digest().to_string())
                    .collect::<Vec<_>>(),
            ),
            hash_parts(
                &output_digests
                    .iter()
                    .map(|output| format!("{}:{}", output.name(), output.digest()))
                    .collect::<Vec<_>>(),
            ),
        ]);
        Self {
            rows,
            output_digests,
            certification_bundle_digest,
        }
    }

    pub fn rows(&self) -> &[WorthQueryLowerRuntimeCertificationRow] {
        &self.rows
    }

    pub fn output_digests(&self) -> &[WorthQueryLowerRuntimeCertificationOutputDigest] {
        &self.output_digests
    }

    pub fn certification_bundle_digest(&self) -> &str {
        &self.certification_bundle_digest
    }

    pub fn output_digest(&self, name: &str) -> Option<&str> {
        self.output_digests
            .iter()
            .find(|output| output.name() == name)
            .map(WorthQueryLowerRuntimeCertificationOutputDigest::digest)
    }
}
