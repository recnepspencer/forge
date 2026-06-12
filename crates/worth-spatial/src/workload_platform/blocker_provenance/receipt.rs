use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::dirty_planar_clean_fail::DirtyPlanarCleanFailError;
use crate::workload_platform::open_planar_posture::OpenPlanarPostureError;
use crate::workload_platform::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

use super::{
    denial::{WorkloadBlockerProvenanceDenial, WorkloadBlockerProvenanceDenialKind},
    source::{WorkloadBlockerBoundaryKind, WorkloadBlockerSourceKind},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadBlockerProvenance {
    source_kind: WorkloadBlockerSourceKind,
    boundary_kind: WorkloadBlockerBoundaryKind,
    source_identity: String,
    boundary_identity: String,
    human_reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadBlockerProvenanceReceipt {
    provenance_digest: String,
    source_kind: WorkloadBlockerSourceKind,
    boundary_kind: WorkloadBlockerBoundaryKind,
    source_identity: String,
    boundary_identity: String,
    human_reason: String,
}

impl WorkloadBlockerProvenance {
    pub fn dirty_kind_mismatch(error: &DirtyPlanarCleanFailError) -> Self {
        let human_reason = error.human_reason();
        let (source_identity, boundary_identity) = match error {
            DirtyPlanarCleanFailError::MismatchedDirtyKind { topology, boundary } => (
                format!("dirty-topology-kind:{topology:?}"),
                format!("clean-fail-boundary-kind:{boundary:?}"),
            ),
            _ => (
                format!("dirty-topology-error:{error:?}"),
                format!("clean-fail-boundary-error:{error:?}"),
            ),
        };
        Self {
            source_kind: WorkloadBlockerSourceKind::DirtyTopology,
            boundary_kind: WorkloadBlockerBoundaryKind::CleanFailBoundary,
            source_identity,
            boundary_identity,
            human_reason,
        }
    }

    pub fn unsupported_surface_open_topology_mismatch_with_identities(
        error: &OpenPlanarPostureError,
        open_topology_identity: impl Into<String>,
        unsupported_surface_identity: impl Into<String>,
    ) -> Self {
        let human_reason = error.human_reason();
        let (source_identity, boundary_identity) = match error {
            OpenPlanarPostureError::UnsupportedSurfaceDidNotConsumeOpenTopology => (
                open_topology_identity.into(),
                unsupported_surface_identity.into(),
            ),
            _ => (
                format!("open-topology-unexpected-error:{error:?}"),
                format!("unsupported-surface-unexpected-error:{error:?}"),
            ),
        };
        Self {
            source_kind: WorkloadBlockerSourceKind::OpenTopology,
            boundary_kind: WorkloadBlockerBoundaryKind::UnsupportedSurface,
            source_identity,
            boundary_identity,
            human_reason,
        }
    }

    pub fn certify(
        self,
        outcome: &WorthUserOutcome,
    ) -> Result<WorkloadBlockerProvenanceReceipt, WorkloadBlockerProvenanceDenial> {
        let cause_kind = outcome.cause().map(|cause| cause.kind());
        if outcome.kind() != WorthUserOutcomeKind::IntegrityMismatch
            || cause_kind != Some(WorthUserOutcomeCauseKind::IntegrityMismatch)
        {
            return Err(WorkloadBlockerProvenanceDenial::new(
                WorkloadBlockerProvenanceDenialKind::OutcomeDidNotReportIntegrityMismatch,
                format!(
                    "{} provenance must report integrity mismatch before closeout",
                    self.source_kind.human_name()
                ),
            ));
        }
        let summary = outcome.human_response().summary();
        if !reason_mentions_boundary(summary, &self.human_reason) {
            return Err(WorkloadBlockerProvenanceDenial::new(
                WorkloadBlockerProvenanceDenialKind::OutcomeDidNotExplainBoundary,
                format!(
                    "{} provenance response must explain the {} mismatch",
                    self.source_kind.human_name(),
                    self.boundary_kind.human_name()
                ),
            ));
        }
        Ok(WorkloadBlockerProvenanceReceipt {
            provenance_digest: provenance_digest(
                self.source_kind,
                self.boundary_kind,
                &self.source_identity,
                &self.boundary_identity,
                summary,
            ),
            source_kind: self.source_kind,
            boundary_kind: self.boundary_kind,
            source_identity: self.source_identity,
            boundary_identity: self.boundary_identity,
            human_reason: summary.to_string(),
        })
    }
}

impl WorkloadBlockerProvenanceReceipt {
    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }

    pub fn source_kind(&self) -> WorkloadBlockerSourceKind {
        self.source_kind
    }

    pub fn boundary_kind(&self) -> WorkloadBlockerBoundaryKind {
        self.boundary_kind
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn boundary_identity(&self) -> &str {
        &self.boundary_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

fn reason_mentions_boundary(summary: &str, human_reason: &str) -> bool {
    summary == human_reason
        || human_reason
            .split_whitespace()
            .filter(|word| word.len() > 4)
            .take(3)
            .all(|word| summary.contains(word))
}

fn provenance_digest(
    source_kind: WorkloadBlockerSourceKind,
    boundary_kind: WorkloadBlockerBoundaryKind,
    source_identity: &str,
    boundary_identity: &str,
    human_reason: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "workload-blocker-provenance".to_string(),
            format!("source-kind:{source_kind:?}"),
            format!("boundary-kind:{boundary_kind:?}"),
            format!("source:{source_identity}"),
            format!("boundary:{boundary_identity}"),
            format!("reason:{human_reason}"),
        ],
    )
}
