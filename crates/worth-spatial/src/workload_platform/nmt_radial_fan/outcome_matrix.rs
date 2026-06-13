use super::denial::NmtRadialFanDenial;
use super::receipt::NmtRadialFanReceipt;
use crate::workload_platform::user_response::WorthUserOutcome;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NmtRadialFanOutcomeKind {
    Admitted,
    UnsupportedInput,
    DirtyInput,
    IntegrityMismatch,
    Denied,
    PredicateUncertain,
    MissingEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtRadialFanOutcomeRow {
    kind: NmtRadialFanOutcomeKind,
    human_reason: String,
    evidence_identity: String,
}

impl NmtRadialFanOutcomeRow {
    pub fn admitted(receipt: &NmtRadialFanReceipt) -> Self {
        Self {
            kind: NmtRadialFanOutcomeKind::Admitted,
            human_reason: format!(
                "Open radial fan kept {} posture with {} incident faces and {} non-manifold edge.",
                receipt.topology_posture_label(),
                receipt.counters().incident_face_count(),
                receipt.counters().non_manifold_edge_count()
            ),
            evidence_identity: receipt.fan_digest().to_string(),
        }
    }

    pub fn from_denial(denial: &NmtRadialFanDenial) -> Self {
        let human_reason = denial.human_reason();
        Self {
            kind: denial_kind(denial),
            evidence_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "nmt-radial-fan-denial-outcome".to_string(),
                    denial_kind(denial).as_str().to_string(),
                    human_reason.clone(),
                ],
            ),
            human_reason,
        }
    }

    pub fn from_user_outcome(kind: NmtRadialFanOutcomeKind, outcome: &WorthUserOutcome) -> Self {
        Self {
            kind,
            human_reason: outcome.human_response().summary().to_string(),
            evidence_identity: outcome.evidence().digest().to_string(),
        }
    }

    pub fn kind(&self) -> NmtRadialFanOutcomeKind {
        self.kind
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
}

impl NmtRadialFanOutcomeKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::UnsupportedInput => "unsupported-input",
            Self::DirtyInput => "dirty-input",
            Self::IntegrityMismatch => "integrity-mismatch",
            Self::Denied => "denied",
            Self::PredicateUncertain => "predicate-uncertain",
            Self::MissingEvidence => "missing-evidence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtRadialFanOutcomeMatrix {
    rows: Vec<NmtRadialFanOutcomeRow>,
}

impl NmtRadialFanOutcomeMatrix {
    pub fn from_rows(rows: Vec<NmtRadialFanOutcomeRow>) -> Result<Self, NmtRadialFanDenial> {
        let matrix = Self { rows };
        matrix.require(NmtRadialFanOutcomeKind::Admitted)?;
        matrix.require(NmtRadialFanOutcomeKind::UnsupportedInput)?;
        matrix.require(NmtRadialFanOutcomeKind::DirtyInput)?;
        matrix.require(NmtRadialFanOutcomeKind::IntegrityMismatch)?;
        matrix.require(NmtRadialFanOutcomeKind::Denied)?;
        matrix.require(NmtRadialFanOutcomeKind::PredicateUncertain)?;
        matrix.require(NmtRadialFanOutcomeKind::MissingEvidence)?;
        if matrix
            .rows
            .iter()
            .any(|row| row.human_reason.trim().is_empty() || !row.human_reason.contains(' '))
        {
            return Err(NmtRadialFanDenial::DirtyInput {
                reason: "outcome matrix requires human-readable reasons".to_string(),
            });
        }
        Ok(matrix)
    }

    pub fn rows(&self) -> &[NmtRadialFanOutcomeRow] {
        &self.rows
    }

    pub fn row_for_kind(&self, kind: NmtRadialFanOutcomeKind) -> Option<&NmtRadialFanOutcomeRow> {
        self.rows.iter().find(|row| row.kind == kind)
    }

    fn require(&self, kind: NmtRadialFanOutcomeKind) -> Result<(), NmtRadialFanDenial> {
        self.row_for_kind(kind)
            .map(|_| ())
            .ok_or(NmtRadialFanDenial::DirtyInput {
                reason: format!("outcome matrix is missing {kind:?} branch"),
            })
    }
}

fn denial_kind(denial: &NmtRadialFanDenial) -> NmtRadialFanOutcomeKind {
    match denial {
        NmtRadialFanDenial::ClosedManifoldLaunderingAttempt { .. } => {
            NmtRadialFanOutcomeKind::IntegrityMismatch
        }
        NmtRadialFanDenial::UnsupportedSurfaceFamily { .. } => {
            NmtRadialFanOutcomeKind::UnsupportedInput
        }
        NmtRadialFanDenial::DirtyInput { .. } => NmtRadialFanOutcomeKind::DirtyInput,
        NmtRadialFanDenial::MismatchedTopologyConstructionReceipt
        | NmtRadialFanDenial::MismatchedProjectionReceipt
        | NmtRadialFanDenial::MismatchedTransformReceipt
        | NmtRadialFanDenial::MismatchedRetainedReplayReceipt => {
            NmtRadialFanOutcomeKind::IntegrityMismatch
        }
        NmtRadialFanDenial::LabelOnlyMotion => NmtRadialFanOutcomeKind::Denied,
        NmtRadialFanDenial::PredicateUncertain { .. } => {
            NmtRadialFanOutcomeKind::PredicateUncertain
        }
        NmtRadialFanDenial::MissingOpenBoundaryEvidence
        | NmtRadialFanDenial::MissingRadialAdjacencyEvidence
        | NmtRadialFanDenial::MissingReceiptBackedStage(_)
        | NmtRadialFanDenial::MissingTopologyEvidence
        | NmtRadialFanDenial::MissingProjectionEvidence
        | NmtRadialFanDenial::MissingTransformEvidence
        | NmtRadialFanDenial::MissingRetainedReplayEvidence => {
            NmtRadialFanOutcomeKind::MissingEvidence
        }
        _ => NmtRadialFanOutcomeKind::DirtyInput,
    }
}
