use super::admission::{admit_causal_inspection, CausalInspectionProofFlow};
use super::admission_trace::CausalDecisionTraceIndex;
use super::anchor::{anchor_causal_observation, CausalObservationAnchorError};
use super::inventory::CausalEvidenceFamily;
use super::materialization::{
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
};
use super::receipt_types::{
    CausalInspectionReason, CausalObservationOutcome, QueryObservationReceipt,
};
use super::reference::{
    CausalEvidenceReference, CausalEvidenceReferenceResolution,
    CausalEvidenceReferenceResolutionDenial, CausalEvidenceReferenceSet,
};
use super::reference_resolution::resolve_causal_evidence_references;
use super::request::{
    causal_inspection_target, request_causal_inspection, CausalInspectionExplanationFamily,
    CausalInspectionRequest, CausalInspectionRequestError, CausalInspectionRichness,
};

/// Intent-first entrypoint for inspecting why an existing Query observation happened.
///
/// The builder is a convenience layer over the proof primitives. It still anchors the
/// observation, resolves evidence references, builds a Query inspection request, and
/// admits that request before any runtime bridge envelope is assembled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspection {
    receipt: QueryObservationReceipt,
    reason: Option<CausalInspectionReason>,
    explanation_family: CausalInspectionExplanationFamily,
    richness: CausalInspectionRichness,
    requested_evidence_families: Vec<CausalEvidenceFamily>,
    redaction_policy: CausalInspectionRedactionPolicy,
    materialization_policy: CausalInspectionMaterializationPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionSupportPosture {
    Admitted,
    Advisory,
    Denied,
}

impl CausalInspectionSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CausalInspectionPlanError {
    Anchor(CausalObservationAnchorError),
    MissingEvidence(CausalEvidenceReferenceResolutionDenial),
    Request(CausalInspectionRequestError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionPlanErrorKind {
    Anchor,
    MissingEvidence,
    Request,
}

impl CausalInspectionPlanErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Anchor => "anchor",
            Self::MissingEvidence => "missing_evidence",
            Self::Request => "request",
        }
    }
}

impl CausalInspectionPlanError {
    pub fn kind(&self) -> CausalInspectionPlanErrorKind {
        match self {
            Self::Anchor(_) => CausalInspectionPlanErrorKind::Anchor,
            Self::MissingEvidence(_) => CausalInspectionPlanErrorKind::MissingEvidence,
            Self::Request(_) => CausalInspectionPlanErrorKind::Request,
        }
    }

    pub fn failure_digest(&self) -> &str {
        match self {
            Self::Anchor(error) => error.failure_digest(),
            Self::MissingEvidence(denial) => denial.failure_digest(),
            Self::Request(error) => error.failure_digest(),
        }
    }
}

impl CausalInspection {
    pub fn for_observation(receipt: QueryObservationReceipt) -> Self {
        Self {
            receipt,
            reason: None,
            explanation_family: CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation,
            richness: CausalInspectionRichness::ReferenceOnly,
            requested_evidence_families: Vec::new(),
            redaction_policy: CausalInspectionRedactionPolicy::PreserveDetail,
            materialization_policy:
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
        }
    }

    pub fn why_changed(self) -> Self {
        self.because(CausalInspectionReason::ChangedResult)
    }

    pub fn why_suppressed(self) -> Self {
        self.because(CausalInspectionReason::SuppressedResult)
    }

    pub fn why_denied(self) -> Self {
        self.because(CausalInspectionReason::DeniedResult)
    }

    pub fn why_replayed(self) -> Self {
        self.because(CausalInspectionReason::HistoricalReplayResult)
    }

    pub fn why_previewed(self) -> Self {
        self.because(CausalInspectionReason::BranchPreviewResult)
    }

    pub fn why_temporal_wake(self) -> Self {
        self.because(CausalInspectionReason::ChangedResult)
            .evidence_families([
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
                CausalEvidenceFamily::SignalInvalidation,
            ])
    }

    pub fn why_async_completion(self) -> Self {
        self.because(CausalInspectionReason::ChangedResult)
            .evidence_families([
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
                CausalEvidenceFamily::SignalEvaluation,
            ])
    }

    pub fn why_remasked(self) -> Self {
        self.because(CausalInspectionReason::BranchPreviewResult)
            .evidence_families([
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
                CausalEvidenceFamily::BridgePreview,
            ])
    }

    pub fn why_resume_mismatch(self) -> Self {
        self.because(CausalInspectionReason::HistoricalReplayResult)
            .evidence_families([
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
                CausalEvidenceFamily::BridgeReplay,
                CausalEvidenceFamily::SignalReplayCursor,
                CausalEvidenceFamily::BridgeContinuity,
            ])
    }

    pub fn because(mut self, reason: CausalInspectionReason) -> Self {
        self.reason = Some(reason);
        self
    }

    pub fn reference_only(mut self) -> Self {
        self.richness = CausalInspectionRichness::ReferenceOnly;
        self
    }

    pub fn materialized_detail(mut self) -> Self {
        self.richness = CausalInspectionRichness::MaterializedDetail;
        self
    }

    pub fn redaction(mut self, policy: CausalInspectionRedactionPolicy) -> Self {
        self.redaction_policy = policy;
        self
    }

    pub fn materialization(mut self, policy: CausalInspectionMaterializationPolicy) -> Self {
        self.materialization_policy = policy;
        self
    }

    pub fn evidence_families<I>(mut self, families: I) -> Self
    where
        I: IntoIterator<Item = CausalEvidenceFamily>,
    {
        self.requested_evidence_families = families.into_iter().collect();
        self
    }

    pub fn include_all_retained_evidence(mut self) -> Self {
        self.requested_evidence_families.clear();
        self
    }

    pub fn durable_archive(mut self) -> Self {
        self.explanation_family = CausalInspectionExplanationFamily::DurableCausalArchive;
        self
    }

    pub fn store_backed_replay(mut self) -> Self {
        self.explanation_family =
            CausalInspectionExplanationFamily::StoreBackedReplayReconstruction;
        self
    }

    pub fn plan(self) -> Result<CausalInspectionPlan, CausalInspectionPlanError> {
        let reason = self
            .reason
            .unwrap_or_else(|| reason_for_outcome(self.receipt.outcome()));
        let anchor = anchor_causal_observation(self.receipt, reason)
            .map_err(CausalInspectionPlanError::Anchor)?;
        let target = causal_inspection_target(
            anchor.observation_receipt().observation_target_digest(),
            anchor.observation_receipt().result_shape_context_digest(),
        )
        .map_err(CausalInspectionPlanError::Request)?;
        let resolution =
            resolve_causal_evidence_references(anchor, &self.requested_evidence_families);
        let reference_set = match resolution {
            CausalEvidenceReferenceResolution::Resolved { reference_set, .. } => reference_set,
            CausalEvidenceReferenceResolution::MissingRequiredEvidence { denial, .. } => {
                return Err(CausalInspectionPlanError::MissingEvidence(denial));
            }
        };
        let request = request_causal_inspection(
            reference_set.clone(),
            target,
            self.explanation_family,
            self.richness,
            &self.requested_evidence_families,
        )
        .map_err(CausalInspectionPlanError::Request)?;
        let admission = admit_causal_inspection(request.clone());
        Ok(CausalInspectionPlan {
            reference_set,
            request,
            admission,
            redaction_policy: self.redaction_policy,
            materialization_policy: self.materialization_policy,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionPlan {
    pub(super) reference_set: CausalEvidenceReferenceSet,
    pub(super) request: CausalInspectionRequest,
    pub(super) admission: CausalInspectionProofFlow,
    pub(super) redaction_policy: CausalInspectionRedactionPolicy,
    pub(super) materialization_policy: CausalInspectionMaterializationPolicy,
}

impl CausalInspectionPlan {
    pub(crate) fn from_resolved_request(
        reference_set: CausalEvidenceReferenceSet,
        request: CausalInspectionRequest,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        let admission = admit_causal_inspection(request.clone());
        Self {
            reference_set,
            request,
            admission,
            redaction_policy,
            materialization_policy,
        }
    }

    pub fn support_posture(&self) -> CausalInspectionSupportPosture {
        match &self.admission {
            CausalInspectionProofFlow::Admitted(_) => CausalInspectionSupportPosture::Admitted,
            CausalInspectionProofFlow::Advisory(_) => CausalInspectionSupportPosture::Advisory,
            CausalInspectionProofFlow::Denied(_) => CausalInspectionSupportPosture::Denied,
        }
    }

    pub fn required_evidence(&self) -> &[CausalEvidenceReference] {
        self.reference_set.references()
    }

    pub fn admission(&self) -> &CausalInspectionProofFlow {
        &self.admission
    }

    pub fn decision_trace(&self) -> &CausalDecisionTraceIndex {
        self.admission.decision_trace()
    }

    pub fn estimated_cost(&self) -> CausalInspectionEstimatedCost {
        CausalInspectionEstimatedCost {
            anchor_derivation_count: 1,
            evidence_reference_resolution_count: 1,
            admission_count: 1,
            bridge_envelope_assembly_count: if self.support_posture()
                == CausalInspectionSupportPosture::Denied
            {
                0
            } else {
                1
            },
            evidence_reference_count: self.reference_set.references().len(),
        }
    }

    pub fn explain(&self) -> CausalInspectionPlanExplanation {
        CausalInspectionPlanExplanation {
            posture: self.support_posture(),
            reason: match &self.admission {
                CausalInspectionProofFlow::Admitted(_) => {
                    "query admission accepted the causal inspection request"
                }
                CausalInspectionProofFlow::Advisory(inspection) => {
                    inspection.decision().advisory_kind().map_or(
                        "query admission narrowed the causal inspection request",
                        |kind| kind.as_str(),
                    )
                }
                CausalInspectionProofFlow::Denied(inspection) => {
                    inspection.decision().violation_kind().map_or(
                        "query admission denied the causal inspection request",
                        |kind| kind.as_str(),
                    )
                }
            },
        }
    }

    pub fn anchor_digest(&self) -> &str {
        self.reference_set.anchor().anchor_digest().as_str()
    }

    pub fn reference_set_digest(&self) -> &str {
        self.reference_set.reference_set_digest().as_str()
    }

    pub fn request_digest(&self) -> &str {
        self.request.request_digest()
    }

    pub fn admission_digest(&self) -> &str {
        match &self.admission {
            CausalInspectionProofFlow::Admitted(inspection) => {
                inspection.admitted_inspection_digest()
            }
            CausalInspectionProofFlow::Advisory(inspection) => {
                inspection.advisory_inspection_digest()
            }
            CausalInspectionProofFlow::Denied(inspection) => inspection.denied_inspection_digest(),
        }
    }

    pub fn redaction_policy(&self) -> CausalInspectionRedactionPolicy {
        self.redaction_policy
    }

    pub fn materialization_policy(&self) -> CausalInspectionMaterializationPolicy {
        self.materialization_policy
    }

    pub fn requested_richness(&self) -> CausalInspectionRichness {
        self.request.requested_richness()
    }

    pub fn explanation_family(&self) -> CausalInspectionExplanationFamily {
        self.request.explanation_family()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalInspectionEstimatedCost {
    anchor_derivation_count: usize,
    evidence_reference_resolution_count: usize,
    admission_count: usize,
    bridge_envelope_assembly_count: usize,
    evidence_reference_count: usize,
}

impl CausalInspectionEstimatedCost {
    pub fn anchor_derivation_count(&self) -> usize {
        self.anchor_derivation_count
    }

    pub fn evidence_reference_resolution_count(&self) -> usize {
        self.evidence_reference_resolution_count
    }

    pub fn admission_count(&self) -> usize {
        self.admission_count
    }

    pub fn bridge_envelope_assembly_count(&self) -> usize {
        self.bridge_envelope_assembly_count
    }

    pub fn evidence_reference_count(&self) -> usize {
        self.evidence_reference_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalInspectionPlanExplanation {
    posture: CausalInspectionSupportPosture,
    reason: &'static str,
}

impl CausalInspectionPlanExplanation {
    pub fn posture(&self) -> CausalInspectionSupportPosture {
        self.posture
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

fn reason_for_outcome(outcome: CausalObservationOutcome) -> CausalInspectionReason {
    match outcome {
        CausalObservationOutcome::Changed => CausalInspectionReason::ChangedResult,
        CausalObservationOutcome::Suppressed => CausalInspectionReason::SuppressedResult,
        CausalObservationOutcome::Denied => CausalInspectionReason::DeniedResult,
        CausalObservationOutcome::BranchPreview => CausalInspectionReason::BranchPreviewResult,
        CausalObservationOutcome::Replayed => CausalInspectionReason::HistoricalReplayResult,
    }
}
