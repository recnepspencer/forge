use std::collections::BTreeSet;

use crate::identity::hash_parts;

use super::receipt_types::{
    CausalInspectionReason, CausalObservationOutcome, QueryObservationReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalObservationMissingReferencePosture {
    Complete,
    MissingOptional,
    MissingRequiredDenied,
}

impl CausalObservationMissingReferencePosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::MissingOptional => "missing_optional",
            Self::MissingRequiredDenied => "missing_required_denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationAnchorDigest {
    digest: String,
}

impl CausalObservationAnchorDigest {
    fn new(digest: String) -> Self {
        Self { digest }
    }

    pub fn as_str(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationAnchorCounters {
    source_receipt_family_count: usize,
    reference_family_count: usize,
    missing_reference_posture: CausalObservationMissingReferencePosture,
    anchor_digest_width: usize,
    runtime_graph_scan_count: usize,
    diagnostics_retention_scan_count: usize,
    counter_snapshot: String,
}

impl CausalObservationAnchorCounters {
    fn new(
        reference_family_count: usize,
        missing_reference_posture: CausalObservationMissingReferencePosture,
        anchor_digest_width: usize,
    ) -> Self {
        let counter_snapshot = hash_parts(&[
            "causal_observation_anchor_counters_v1".to_string(),
            "source_receipt_family_count:1".to_string(),
            format!("reference_family_count:{reference_family_count}"),
            format!(
                "missing_reference_posture:{}",
                missing_reference_posture.as_str()
            ),
            format!("anchor_digest_width:{anchor_digest_width}"),
            "runtime_graph_scan_count:0".to_string(),
            "diagnostics_retention_scan_count:0".to_string(),
        ]);
        Self {
            source_receipt_family_count: 1,
            reference_family_count,
            missing_reference_posture,
            anchor_digest_width,
            runtime_graph_scan_count: 0,
            diagnostics_retention_scan_count: 0,
            counter_snapshot,
        }
    }

    pub fn source_receipt_family_count(&self) -> usize {
        self.source_receipt_family_count
    }

    pub fn reference_family_count(&self) -> usize {
        self.reference_family_count
    }

    pub fn missing_reference_posture(&self) -> CausalObservationMissingReferencePosture {
        self.missing_reference_posture
    }

    pub fn anchor_digest_width(&self) -> usize {
        self.anchor_digest_width
    }

    pub fn runtime_graph_scan_count(&self) -> usize {
        self.runtime_graph_scan_count
    }

    pub fn diagnostics_retention_scan_count(&self) -> usize {
        self.diagnostics_retention_scan_count
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalObservationAnchorErrorKind {
    MissingObservationReceipt,
    MissingRequiredEvidenceReference,
    InspectionReasonOutcomeMismatch,
}

impl CausalObservationAnchorErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingObservationReceipt => "missing_observation_receipt",
            Self::MissingRequiredEvidenceReference => "missing_required_evidence_reference",
            Self::InspectionReasonOutcomeMismatch => "inspection_reason_outcome_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationAnchorError {
    kind: CausalObservationAnchorErrorKind,
    message: &'static str,
    failure_digest: String,
}

impl CausalObservationAnchorError {
    fn new(
        kind: CausalObservationAnchorErrorKind,
        message: &'static str,
        evidence: &[String],
    ) -> Self {
        let mut parts = vec![
            "causal_observation_anchor_error_v1".to_string(),
            kind.as_str().to_string(),
            message.to_string(),
        ];
        parts.extend(evidence.iter().cloned());
        Self {
            kind,
            message,
            failure_digest: hash_parts(&parts),
        }
    }

    pub fn kind(&self) -> CausalObservationAnchorErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationAnchor {
    observation_receipt: QueryObservationReceipt,
    inspection_reason: CausalInspectionReason,
    lower_runtime_evidence_family_count: usize,
    missing_reference_posture: CausalObservationMissingReferencePosture,
    anchor_digest: CausalObservationAnchorDigest,
    counters: CausalObservationAnchorCounters,
}

impl CausalObservationAnchor {
    pub fn observation_receipt(&self) -> &QueryObservationReceipt {
        &self.observation_receipt
    }

    pub fn inspection_reason(&self) -> CausalInspectionReason {
        self.inspection_reason
    }

    pub fn lower_runtime_evidence_family_count(&self) -> usize {
        self.lower_runtime_evidence_family_count
    }

    pub fn missing_reference_posture(&self) -> CausalObservationMissingReferencePosture {
        self.missing_reference_posture
    }

    pub fn anchor_digest(&self) -> &CausalObservationAnchorDigest {
        &self.anchor_digest
    }

    pub fn counters(&self) -> &CausalObservationAnchorCounters {
        &self.counters
    }
}

pub fn anchor_causal_observation(
    observation_receipt: QueryObservationReceipt,
    inspection_reason: CausalInspectionReason,
) -> Result<CausalObservationAnchor, CausalObservationAnchorError> {
    if !inspection_reason_matches_observation_outcome(&inspection_reason, &observation_receipt) {
        return Err(CausalObservationAnchorError::new(
            CausalObservationAnchorErrorKind::InspectionReasonOutcomeMismatch,
            "causal observation anchor reason must match the observed Query outcome",
            &[
                format!("reason:{}", inspection_reason.as_str()),
                format!("outcome:{}", observation_receipt.outcome().as_str()),
            ],
        ));
    }
    if observation_receipt.observation_receipt_digest().is_empty() {
        return Err(CausalObservationAnchorError::new(
            CausalObservationAnchorErrorKind::MissingObservationReceipt,
            "causal observation anchors require one canonical Query observation receipt",
            &[format!("family:{}", observation_receipt.family().as_str())],
        ));
    }
    if observation_receipt.evidence_identities().is_empty() {
        return Err(CausalObservationAnchorError::new(
            CausalObservationAnchorErrorKind::MissingRequiredEvidenceReference,
            "causal observation anchors require at least one lower-runtime or Query evidence identity carried by the source receipt",
            &[format!(
                "observation:{}",
                observation_receipt.observation_receipt_digest()
            )],
        ));
    }
    if let Some(missing_identity) = observation_receipt
        .evidence_identities()
        .iter()
        .find(|identity| identity.reference_digest().is_empty())
    {
        return Err(CausalObservationAnchorError::new(
            CausalObservationAnchorErrorKind::MissingRequiredEvidenceReference,
            "causal observation anchors require non-empty evidence reference digests carried by the source receipt",
            &[
                format!(
                    "observation:{}",
                    observation_receipt.observation_receipt_digest()
                ),
                format!("family:{}", missing_identity.family().as_str()),
            ],
        ));
    }

    let unique_families = observation_receipt
        .evidence_identities()
        .iter()
        .map(|identity| identity.family())
        .collect::<BTreeSet<_>>();
    let evidence_part = observation_receipt
        .evidence_identities()
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
    let anchor_digest_value = hash_parts(&[
        "causal_observation_anchor_v1".to_string(),
        format!("reason:{}", inspection_reason.as_str()),
        format!("receipt:{}", observation_receipt.receipt_digest()),
        format!(
            "observation:{}",
            observation_receipt.observation_receipt_digest()
        ),
        format!("query:{}", observation_receipt.query_digest()),
        format!("basis-posture:{}", observation_receipt.basis_posture()),
        format!("basis:{}", observation_receipt.basis_digest()),
        format!(
            "result-shape:{}",
            observation_receipt.result_shape_context_digest()
        ),
        format!("target:{}", observation_receipt.observation_target_digest()),
        format!("outcome:{}", observation_receipt.outcome().as_str()),
        format!("evidence:{evidence_part}"),
    ]);
    let missing_reference_posture = CausalObservationMissingReferencePosture::Complete;
    let counters = CausalObservationAnchorCounters::new(
        unique_families.len(),
        missing_reference_posture,
        anchor_digest_value.len(),
    );

    Ok(CausalObservationAnchor {
        observation_receipt,
        inspection_reason,
        lower_runtime_evidence_family_count: unique_families.len(),
        missing_reference_posture,
        anchor_digest: CausalObservationAnchorDigest::new(anchor_digest_value),
        counters,
    })
}

fn inspection_reason_matches_observation_outcome(
    inspection_reason: &CausalInspectionReason,
    observation_receipt: &QueryObservationReceipt,
) -> bool {
    matches!(
        (inspection_reason, observation_receipt.outcome()),
        (
            CausalInspectionReason::ChangedResult,
            CausalObservationOutcome::Changed
        ) | (
            CausalInspectionReason::SuppressedResult,
            CausalObservationOutcome::Suppressed
        ) | (
            CausalInspectionReason::DeniedResult,
            CausalObservationOutcome::Denied
        ) | (
            CausalInspectionReason::BranchPreviewResult,
            CausalObservationOutcome::BranchPreview
        ) | (
            CausalInspectionReason::HistoricalReplayResult,
            CausalObservationOutcome::Replayed
        )
    )
}
