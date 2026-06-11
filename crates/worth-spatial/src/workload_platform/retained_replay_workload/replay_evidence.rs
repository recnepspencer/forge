use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayEvidenceKind {
    RetainedArtifactCapture,
    HistoricalReplay,
    ProjectionConsumptionParity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayEvidenceRow {
    kind: ReplayEvidenceKind,
    evidence_identity: String,
    human_evidence: String,
}

impl ReplayEvidenceRow {
    pub(crate) fn new(
        kind: ReplayEvidenceKind,
        evidence_identity: impl Into<String>,
        human_evidence: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_evidence: human_evidence.into(),
        }
    }

    pub fn kind(&self) -> ReplayEvidenceKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_evidence(&self) -> &str {
        &self.human_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayEvidenceSet {
    rows: Vec<ReplayEvidenceRow>,
}

impl ReplayEvidenceSet {
    pub(crate) fn from_retained_replay(
        retained_artifact_identity: &str,
        historical: &RetainedPlanarHistoricalInspection,
        projection: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        Self {
            rows: vec![
                ReplayEvidenceRow::new(
                    ReplayEvidenceKind::RetainedArtifactCapture,
                    retained_artifact_identity,
                    "Replay consumed retained planar and projection-consumed artifacts.",
                ),
                ReplayEvidenceRow::new(
                    ReplayEvidenceKind::HistoricalReplay,
                    historical.historical_digest(),
                    "Replay used retained historical inspection instead of re-running extraction.",
                ),
                ReplayEvidenceRow::new(
                    ReplayEvidenceKind::ProjectionConsumptionParity,
                    projection.projection_consumption_digest(),
                    "Projection-consumed facts match the retained planar artifact digest.",
                ),
            ],
        }
    }

    pub fn rows(&self) -> &[ReplayEvidenceRow] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}
