use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::super::admission::{
    AdmittedCausalInspection, AdvisoryCausalInspection, DeniedCausalInspection,
};
use super::super::inventory::CausalEvidenceFamily;
use super::super::receipt_types::{CausalInspectionReason, CausalObservationOutcome};
use forge_runtime_bridge::facade::BridgeCausalEvidenceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryCausalTemporalAsyncExplanationKind {
    TemporalWake,
    AsyncCompletion,
    MixedCauseSuppression,
    PreviewRemask,
    ReplayDrift,
    ResumeMismatch,
    StaleCompletion,
    Generic,
}

impl QueryCausalTemporalAsyncExplanationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TemporalWake => "temporal_wake",
            Self::AsyncCompletion => "async_completion",
            Self::MixedCauseSuppression => "mixed_cause_suppression",
            Self::PreviewRemask => "preview_remask",
            Self::ReplayDrift => "replay_drift",
            Self::ResumeMismatch => "resume_mismatch",
            Self::StaleCompletion => "stale_completion",
            Self::Generic => "generic",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCausalTemporalAsyncExplanation {
    kind: QueryCausalTemporalAsyncExplanationKind,
    inspection_reason: CausalInspectionReason,
    observation_outcome: CausalObservationOutcome,
    evidence_families: Vec<CausalEvidenceFamily>,
    offline_explainable: bool,
    explanation_identity: ForgeQueryEvidenceIdentity,
}

impl QueryCausalTemporalAsyncExplanation {
    pub(in crate::runtime) fn project(
        inspection_reason: CausalInspectionReason,
        observation_outcome: CausalObservationOutcome,
        evidence_families: &[CausalEvidenceFamily],
        bridge_denial_family: Option<BridgeCausalEvidenceFamily>,
    ) -> Self {
        let mut evidence_families = evidence_families.to_vec();
        evidence_families.sort();
        evidence_families.dedup();
        let flags = EvidenceFlags::from_families(&evidence_families, bridge_denial_family);
        let kind = classify_kind(inspection_reason, &flags);
        let offline_explainable = kind != QueryCausalTemporalAsyncExplanationKind::Generic;
        let explanation_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalInspectionArtifact)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "temporal-async-explanation",
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("reason"),
                    inspection_reason.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("outcome"),
                    observation_outcome.as_str(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("family"),
                    evidence_families.iter().map(CausalEvidenceFamily::as_str),
                )
                .field_bool(ForgeQueryEvidenceTag::new("offline"), offline_explainable)
                .seal();
        Self {
            kind,
            inspection_reason,
            observation_outcome,
            evidence_families,
            offline_explainable,
            explanation_identity,
        }
    }

    pub fn kind(&self) -> QueryCausalTemporalAsyncExplanationKind {
        self.kind
    }

    pub fn inspection_reason(&self) -> CausalInspectionReason {
        self.inspection_reason
    }

    pub fn observation_outcome(&self) -> CausalObservationOutcome {
        self.observation_outcome
    }

    pub fn evidence_families(&self) -> &[CausalEvidenceFamily] {
        &self.evidence_families
    }

    pub fn offline_explainable(&self) -> bool {
        self.offline_explainable
    }

    pub fn explanation_digest(&self) -> &str {
        self.explanation_identity.as_str()
    }

    pub fn explanation_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.explanation_identity
    }
}

pub(in crate::runtime) fn project_admitted_temporal_async_explanation(
    inspection: &AdmittedCausalInspection,
) -> QueryCausalTemporalAsyncExplanation {
    QueryCausalTemporalAsyncExplanation::project(
        inspection.subject().inspection_reason(),
        inspection.subject().observation_outcome(),
        inspection.subject().resolved_evidence_families(),
        None,
    )
}

pub(in crate::runtime) fn project_advisory_temporal_async_explanation(
    inspection: &AdvisoryCausalInspection,
) -> QueryCausalTemporalAsyncExplanation {
    QueryCausalTemporalAsyncExplanation::project(
        inspection.subject().inspection_reason(),
        inspection.subject().observation_outcome(),
        inspection.subject().resolved_evidence_families(),
        None,
    )
}

pub(in crate::runtime) fn project_denied_temporal_async_explanation(
    inspection: &DeniedCausalInspection,
    bridge_denial_family: Option<BridgeCausalEvidenceFamily>,
) -> QueryCausalTemporalAsyncExplanation {
    QueryCausalTemporalAsyncExplanation::project(
        inspection.subject().inspection_reason(),
        inspection.subject().observation_outcome(),
        inspection.subject().resolved_evidence_families(),
        bridge_denial_family,
    )
}

#[derive(Clone, Copy)]
struct EvidenceFlags {
    temporal_wake: bool,
    async_completion: bool,
    replay: bool,
    preview: bool,
    stale_failure: bool,
    continuity: bool,
}

impl EvidenceFlags {
    fn from_families(
        families: &[CausalEvidenceFamily],
        bridge_denial_family: Option<BridgeCausalEvidenceFamily>,
    ) -> Self {
        let has = |family| families.iter().any(|candidate| *candidate == family);
        Self {
            temporal_wake: has(CausalEvidenceFamily::SignalInvalidation),
            async_completion: has(CausalEvidenceFamily::SignalEvaluation)
                || has(CausalEvidenceFamily::SignalForensicAvailability),
            replay: has(CausalEvidenceFamily::BridgeReplay)
                || has(CausalEvidenceFamily::SignalReplayCursor),
            preview: has(CausalEvidenceFamily::BridgePreview),
            stale_failure: has(CausalEvidenceFamily::BridgeSourceFailure)
                || bridge_denial_family == Some(BridgeCausalEvidenceFamily::BridgeSourceFailure),
            continuity: has(CausalEvidenceFamily::BridgeContinuity)
                || has(CausalEvidenceFamily::BridgeStream)
                || has(CausalEvidenceFamily::SignalReplayCursor),
        }
    }
}

fn classify_kind(
    inspection_reason: CausalInspectionReason,
    flags: &EvidenceFlags,
) -> QueryCausalTemporalAsyncExplanationKind {
    match inspection_reason {
        CausalInspectionReason::SuppressedResult
            if flags.temporal_wake && flags.async_completion =>
        {
            QueryCausalTemporalAsyncExplanationKind::MixedCauseSuppression
        }
        CausalInspectionReason::BranchPreviewResult if flags.preview => {
            QueryCausalTemporalAsyncExplanationKind::PreviewRemask
        }
        CausalInspectionReason::HistoricalReplayResult if flags.replay && flags.continuity => {
            QueryCausalTemporalAsyncExplanationKind::ResumeMismatch
        }
        CausalInspectionReason::HistoricalReplayResult if flags.replay => {
            QueryCausalTemporalAsyncExplanationKind::ReplayDrift
        }
        CausalInspectionReason::DeniedResult if flags.async_completion && flags.stale_failure => {
            QueryCausalTemporalAsyncExplanationKind::StaleCompletion
        }
        CausalInspectionReason::ChangedResult if flags.async_completion => {
            QueryCausalTemporalAsyncExplanationKind::AsyncCompletion
        }
        CausalInspectionReason::ChangedResult if flags.temporal_wake => {
            QueryCausalTemporalAsyncExplanationKind::TemporalWake
        }
        _ => QueryCausalTemporalAsyncExplanationKind::Generic,
    }
}
