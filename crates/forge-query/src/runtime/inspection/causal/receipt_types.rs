use crate::identity::hash_parts;

use super::inventory::CausalEvidenceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryObservationReceiptFamily {
    WriteReceipt,
    IntentReceipt,
    IntentDenial,
    BranchIntentReceipt,
    PreviewOutcome,
    ReadReceipt,
    Fixture,
}

impl QueryObservationReceiptFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WriteReceipt => "write_receipt",
            Self::IntentReceipt => "intent_receipt",
            Self::IntentDenial => "intent_denial",
            Self::BranchIntentReceipt => "branch_intent_receipt",
            Self::PreviewOutcome => "preview_outcome",
            Self::ReadReceipt => "read_receipt",
            Self::Fixture => "fixture",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalObservationOutcome {
    Changed,
    Suppressed,
    Denied,
    BranchPreview,
    Replayed,
}

impl CausalObservationOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Changed => "changed",
            Self::Suppressed => "suppressed",
            Self::Denied => "denied",
            Self::BranchPreview => "branch_preview",
            Self::Replayed => "replayed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionReason {
    ChangedResult,
    SuppressedResult,
    DeniedResult,
    BranchPreviewResult,
    HistoricalReplayResult,
}

impl CausalInspectionReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChangedResult => "changed_result",
            Self::SuppressedResult => "suppressed_result",
            Self::DeniedResult => "denied_result",
            Self::BranchPreviewResult => "branch_preview_result",
            Self::HistoricalReplayResult => "historical_replay_result",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationEvidenceIdentity {
    family: CausalEvidenceFamily,
    reference_digest: String,
}

impl CausalObservationEvidenceIdentity {
    pub(in crate::runtime) fn new(
        family: CausalEvidenceFamily,
        reference_digest: impl Into<String>,
    ) -> Self {
        Self {
            family,
            reference_digest: reference_digest.into(),
        }
    }

    pub fn family(&self) -> CausalEvidenceFamily {
        self.family
    }

    pub fn reference_digest(&self) -> &str {
        &self.reference_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryObservationReceipt {
    family: QueryObservationReceiptFamily,
    observation_receipt_digest: String,
    query_digest: String,
    basis_posture: String,
    basis_digest: String,
    result_shape_context_digest: String,
    observation_target_digest: String,
    outcome: CausalObservationOutcome,
    evidence_identities: Vec<CausalObservationEvidenceIdentity>,
    receipt_digest: String,
}

impl QueryObservationReceipt {
    pub(super) fn from_parts(parts: ObservationReceiptParts) -> Self {
        let evidence_part = parts
            .evidence_identities
            .iter()
            .map(|identity| {
                format!(
                    "{}:{}",
                    identity.family().as_str(),
                    identity.reference_digest()
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let receipt_digest = hash_parts(&[
            "query_observation_receipt_v1".to_string(),
            parts.family.as_str().to_string(),
            format!("observation:{}", parts.observation_receipt_digest),
            format!("query:{}", parts.query_digest),
            format!("basis-posture:{}", parts.basis_posture),
            format!("basis:{}", parts.basis_digest),
            format!("result-shape:{}", parts.result_shape_context_digest),
            format!("target:{}", parts.observation_target_digest),
            format!("outcome:{}", parts.outcome.as_str()),
            format!("evidence:{evidence_part}"),
        ]);
        Self {
            family: parts.family,
            observation_receipt_digest: parts.observation_receipt_digest,
            query_digest: parts.query_digest,
            basis_posture: parts.basis_posture,
            basis_digest: parts.basis_digest,
            result_shape_context_digest: parts.result_shape_context_digest,
            observation_target_digest: parts.observation_target_digest,
            outcome: parts.outcome,
            evidence_identities: parts.evidence_identities,
            receipt_digest,
        }
    }

    pub fn family(&self) -> QueryObservationReceiptFamily {
        self.family
    }

    pub fn observation_receipt_digest(&self) -> &str {
        &self.observation_receipt_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_posture(&self) -> &str {
        &self.basis_posture
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_shape_context_digest(&self) -> &str {
        &self.result_shape_context_digest
    }

    pub fn observation_target_digest(&self) -> &str {
        &self.observation_target_digest
    }

    pub fn outcome(&self) -> CausalObservationOutcome {
        self.outcome
    }

    pub fn evidence_identities(&self) -> &[CausalObservationEvidenceIdentity] {
        &self.evidence_identities
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}

pub(super) struct ObservationReceiptParts {
    pub family: QueryObservationReceiptFamily,
    pub observation_receipt_digest: String,
    pub query_digest: String,
    pub basis_posture: String,
    pub basis_digest: String,
    pub result_shape_context_digest: String,
    pub observation_target_digest: String,
    pub outcome: CausalObservationOutcome,
    pub evidence_identities: Vec<CausalObservationEvidenceIdentity>,
}
