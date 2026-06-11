use super::identity::{
    compose_causal_inspection_request_failure_identity, compose_causal_inspection_request_identity,
    compose_causal_inspection_target_identity, CausalInspectionRequestIdentity,
    CausalInspectionTargetIdentity,
};
use super::inventory::CausalEvidenceFamily;
use super::observation_identity::{CausalObservationTargetHandle, CausalResultShapeContextHandle};
use super::reference::{CausalEvidenceReference, CausalEvidenceReferenceSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionTarget {
    observation_target: CausalObservationTargetHandle,
    result_shape_context: CausalResultShapeContextHandle,
    target_identity: CausalInspectionTargetIdentity,
}

impl CausalInspectionTarget {
    fn new(
        observation_target: CausalObservationTargetHandle,
        result_shape_context: CausalResultShapeContextHandle,
    ) -> Result<Self, CausalInspectionRequestError> {
        if observation_target.identity().as_str().is_empty() {
            return Err(CausalInspectionRequestError::new(
                CausalInspectionRequestErrorKind::EmptyObservationTarget,
                "causal inspection targets require an observation target digest",
                &[],
            ));
        }
        if result_shape_context.identity().as_str().is_empty() {
            return Err(CausalInspectionRequestError::new(
                CausalInspectionRequestErrorKind::EmptyResultShapeContext,
                "causal inspection targets require a result-shape context digest",
                &[format!("target:{}", observation_target.rendered())],
            ));
        }
        Ok(Self {
            target_identity: compose_causal_inspection_target_identity(
                &observation_target,
                &result_shape_context,
            ),
            observation_target,
            result_shape_context,
        })
    }

    pub fn observation_target_digest(&self) -> &str {
        self.observation_target.identity().as_str()
    }

    pub fn result_shape_context_digest(&self) -> &str {
        self.result_shape_context.identity().as_str()
    }

    pub fn target_digest(&self) -> &str {
        self.target_identity.as_str()
    }

    pub fn observation_target(&self) -> &CausalObservationTargetHandle {
        &self.observation_target
    }

    pub fn result_shape_context(&self) -> &CausalResultShapeContextHandle {
        &self.result_shape_context
    }

    pub fn target_identity(&self) -> &CausalInspectionTargetIdentity {
        &self.target_identity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionExplanationFamily {
    CrossRuntimeCausalExplanation,
    DurableCausalArchive,
    StoreBackedReplayReconstruction,
}

impl CausalInspectionExplanationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CrossRuntimeCausalExplanation => "cross_runtime_causal_explanation",
            Self::DurableCausalArchive => "durable_causal_archive",
            Self::StoreBackedReplayReconstruction => "store_backed_replay_reconstruction",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionRichness {
    ReferenceOnly,
    MaterializedDetail,
}

impl CausalInspectionRichness {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReferenceOnly => "reference_only",
            Self::MaterializedDetail => "materialized_detail",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionRequest {
    reference_set: CausalEvidenceReferenceSet,
    target: CausalInspectionTarget,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: Vec<CausalEvidenceFamily>,
    request_identity: CausalInspectionRequestIdentity,
}

impl CausalInspectionRequest {
    pub(super) fn new(
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        explanation_family: CausalInspectionExplanationFamily,
        requested_richness: CausalInspectionRichness,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
    ) -> Self {
        let request_identity = request_digest(
            &reference_set,
            &target,
            explanation_family,
            requested_richness,
            &requested_evidence_families,
        );
        Self {
            reference_set,
            target,
            explanation_family,
            requested_richness,
            requested_evidence_families,
            request_identity,
        }
    }

    pub fn reference_set(&self) -> &CausalEvidenceReferenceSet {
        &self.reference_set
    }

    pub fn target(&self) -> &CausalInspectionTarget {
        &self.target
    }

    pub fn explanation_family(&self) -> CausalInspectionExplanationFamily {
        self.explanation_family
    }

    pub fn requested_richness(&self) -> CausalInspectionRichness {
        self.requested_richness
    }

    pub fn requested_evidence_families(&self) -> &[CausalEvidenceFamily] {
        &self.requested_evidence_families
    }

    pub fn request_digest(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn request_identity(&self) -> &CausalInspectionRequestIdentity {
        &self.request_identity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionRequestErrorKind {
    EmptyObservationTarget,
    EmptyResultShapeContext,
    TargetObservationMismatch,
    ResultShapeContextMismatch,
    RequestedEvidenceFamilyMissing,
}

impl CausalInspectionRequestErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyObservationTarget => "empty_observation_target",
            Self::EmptyResultShapeContext => "empty_result_shape_context",
            Self::TargetObservationMismatch => "target_observation_mismatch",
            Self::ResultShapeContextMismatch => "result_shape_context_mismatch",
            Self::RequestedEvidenceFamilyMissing => "requested_evidence_family_missing",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionRequestError {
    kind: CausalInspectionRequestErrorKind,
    message: &'static str,
    failure_digest: String,
}

impl CausalInspectionRequestError {
    fn new(
        kind: CausalInspectionRequestErrorKind,
        message: &'static str,
        evidence: &[String],
    ) -> Self {
        let mut parts = vec![kind.as_str().to_string(), message.to_string()];
        parts.extend(evidence.iter().cloned());
        Self {
            kind,
            message,
            failure_digest: compose_causal_inspection_request_failure_identity(
                kind.as_str(),
                message,
                &parts[2..],
            )
            .as_str()
            .to_string(),
        }
    }

    pub fn kind(&self) -> CausalInspectionRequestErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        self.message
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

pub fn causal_inspection_target(
    observation_target: CausalObservationTargetHandle,
    result_shape_context: CausalResultShapeContextHandle,
) -> Result<CausalInspectionTarget, CausalInspectionRequestError> {
    CausalInspectionTarget::new(observation_target, result_shape_context)
}

pub fn request_causal_inspection(
    reference_set: CausalEvidenceReferenceSet,
    target: CausalInspectionTarget,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: &[CausalEvidenceFamily],
) -> Result<CausalInspectionRequest, CausalInspectionRequestError> {
    let receipt = reference_set.anchor().observation_receipt();
    if target.observation_target_digest() != receipt.observation_target().identity().as_str() {
        return Err(CausalInspectionRequestError::new(
            CausalInspectionRequestErrorKind::TargetObservationMismatch,
            "causal inspection targets must match the anchored Query observation target",
            &[
                format!("target:{}", target.observation_target_digest()),
                format!(
                    "anchor-target:{}",
                    receipt.observation_target().identity().as_str()
                ),
            ],
        ));
    }
    if target.result_shape_context_digest() != receipt.result_shape_context().identity().as_str() {
        return Err(CausalInspectionRequestError::new(
            CausalInspectionRequestErrorKind::ResultShapeContextMismatch,
            "causal inspection targets must match the anchored result-shape context",
            &[
                format!("result-shape:{}", target.result_shape_context_digest()),
                format!(
                    "anchor-result-shape:{}",
                    receipt.result_shape_context().identity().as_str()
                ),
            ],
        ));
    }

    let families = requested_families_or_resolved(&reference_set, requested_evidence_families)?;
    Ok(CausalInspectionRequest::new(
        reference_set,
        target,
        explanation_family,
        requested_richness,
        families,
    ))
}

fn requested_families_or_resolved(
    reference_set: &CausalEvidenceReferenceSet,
    requested_families: &[CausalEvidenceFamily],
) -> Result<Vec<CausalEvidenceFamily>, CausalInspectionRequestError> {
    if requested_families.is_empty() {
        return Ok(reference_set
            .references()
            .iter()
            .map(CausalEvidenceReference::family)
            .collect());
    }
    let missing_family = requested_families.iter().find(|family| {
        !reference_set
            .references()
            .iter()
            .any(|reference| reference.family() == **family)
    });
    if let Some(family) = missing_family {
        return Err(CausalInspectionRequestError::new(
            CausalInspectionRequestErrorKind::RequestedEvidenceFamilyMissing,
            "causal inspection requests may only ask for resolved evidence families",
            &[
                format!("family:{}", family.as_str()),
                format!(
                    "reference-set:{}",
                    reference_set.reference_set_digest().as_str()
                ),
            ],
        ));
    }
    Ok(requested_families.to_vec())
}

fn request_digest(
    reference_set: &CausalEvidenceReferenceSet,
    target: &CausalInspectionTarget,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: &[CausalEvidenceFamily],
) -> CausalInspectionRequestIdentity {
    compose_causal_inspection_request_identity(
        reference_set.anchor().anchor_digest().as_str(),
        reference_set.reference_set_digest().as_str(),
        target.target_identity().as_str(),
        explanation_family,
        requested_richness,
        requested_evidence_families,
    )
}
