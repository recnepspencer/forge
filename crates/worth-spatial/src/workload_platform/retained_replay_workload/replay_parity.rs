use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParityKind {
    LiveRetainedReplayedProjectionMatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityRow {
    kind: ReplayParityKind,
    parity_identity: String,
    human_parity: String,
}

impl ReplayParityRow {
    pub(crate) fn new(
        kind: ReplayParityKind,
        parity_identity: impl Into<String>,
        human_parity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            parity_identity: parity_identity.into(),
            human_parity: human_parity.into(),
        }
    }

    pub fn kind(&self) -> ReplayParityKind {
        self.kind
    }

    pub fn parity_identity(&self) -> &str {
        &self.parity_identity
    }

    pub fn human_parity(&self) -> &str {
        &self.human_parity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityReport {
    rows: Vec<ReplayParityRow>,
}

impl ReplayParityReport {
    pub(crate) fn from_retained_projection_match(
        historical: &RetainedPlanarHistoricalInspection,
        projection: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        Self {
            rows: vec![ReplayParityRow::new(
                ReplayParityKind::LiveRetainedReplayedProjectionMatch,
                format!(
                    "replay-parity:{}:{}",
                    historical.retained_fact_digest(),
                    projection.projection_consumption_digest()
                ),
                "Live retained facts, retained replay, and projection-consumed facts agree.",
            )],
        }
    }

    pub fn rows(&self) -> &[ReplayParityRow] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}
