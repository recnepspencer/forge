use crate::application::WorthQueryDeclarationEntryContributionCategoryFamily;

use super::artifact::WorthQueryContributionComposedContribution;
use super::aspect::WorthQueryContributionComposedIntentAspectRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedIntentRequestDescriptor {
    order_index: usize,
    category_family: WorthQueryDeclarationEntryContributionCategoryFamily,
    request_identity: crate::WorthQueryEvidenceIdentity,
    binding_identity: crate::WorthQueryEvidenceIdentity,
    semantic_code: String,
    detail: String,
    aspect_record: WorthQueryContributionComposedIntentAspectRecord,
}

impl WorthQueryContributionComposedIntentRequestDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_index: usize,
        category_family: WorthQueryDeclarationEntryContributionCategoryFamily,
        request_identity: crate::WorthQueryEvidenceIdentity,
        binding_identity: crate::WorthQueryEvidenceIdentity,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        aspect_record: WorthQueryContributionComposedIntentAspectRecord,
    ) -> Self {
        Self {
            order_index,
            category_family,
            request_identity,
            binding_identity,
            semantic_code: semantic_code.into(),
            detail: detail.into(),
            aspect_record,
        }
    }

    pub fn order_index(&self) -> usize {
        self.order_index
    }

    pub fn category_family(&self) -> WorthQueryDeclarationEntryContributionCategoryFamily {
        self.category_family
    }

    pub fn request_identity(&self) -> &crate::WorthQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_digest(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn binding_identity(&self) -> &crate::WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn target_binding_digest(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn target_digest(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn aspect_record(&self) -> &WorthQueryContributionComposedIntentAspectRecord {
        &self.aspect_record
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryContributionComposedIntentStageKind {
    NotAttempted,
    Succeeded,
    Denied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedIntentStageResult {
    kind: WorthQueryContributionComposedIntentStageKind,
    detail: String,
    stage_identity: Option<crate::WorthQueryEvidenceIdentity>,
}

impl WorthQueryContributionComposedIntentStageResult {
    pub fn not_attempted() -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::NotAttempted,
            "stage was not attempted",
            None,
        )
    }

    pub fn succeeded(
        detail: impl Into<String>,
        stage_identity: Option<crate::WorthQueryEvidenceIdentity>,
    ) -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::Succeeded,
            detail,
            stage_identity,
        )
    }

    pub fn denied(detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::Denied,
            detail,
            None,
        )
    }

    pub fn stale(detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::Stale,
            detail,
            None,
        )
    }

    pub fn rebind_required(detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::RebindRequired,
            detail,
            None,
        )
    }

    pub fn unsupported(detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::Unsupported,
            detail,
            None,
        )
    }

    pub fn failed(detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryContributionComposedIntentStageKind::Failed,
            detail,
            None,
        )
    }

    pub fn kind(&self) -> WorthQueryContributionComposedIntentStageKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn stage_identity(&self) -> Option<&crate::WorthQueryEvidenceIdentity> {
        self.stage_identity.as_ref()
    }

    pub fn stage_for_reporting(&self) -> Option<&str> {
        self.stage_identity
            .as_ref()
            .map(|identity| identity.as_str())
    }

    pub fn digest(&self) -> Option<&str> {
        self.stage_for_reporting()
    }

    fn new(
        kind: WorthQueryContributionComposedIntentStageKind,
        detail: impl Into<String>,
        stage_identity: Option<crate::WorthQueryEvidenceIdentity>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            stage_identity,
        }
    }
}

pub(crate) fn primary_intent_descriptor(
    intent_results: &[WorthQueryContributionComposedIntentResult],
) -> Option<&WorthQueryContributionComposedIntentRequestDescriptor> {
    intent_results
        .iter()
        .find(|value| !value.is_admitted())
        .or_else(|| intent_results.first())
        .map(WorthQueryContributionComposedIntentResult::request)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryContributionComposedIntentClassification {
    Admitted,
    Denied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
    MaterializationFailedAfterAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedIntentResult {
    request: WorthQueryContributionComposedIntentRequestDescriptor,
    evaluation: WorthQueryContributionComposedIntentStageResult,
    admission: WorthQueryContributionComposedIntentStageResult,
    materialization: WorthQueryContributionComposedIntentStageResult,
    classification: WorthQueryContributionComposedIntentClassification,
    contribution: Option<WorthQueryContributionComposedContribution>,
}

impl WorthQueryContributionComposedIntentResult {
    pub fn new(
        request: WorthQueryContributionComposedIntentRequestDescriptor,
        evaluation: WorthQueryContributionComposedIntentStageResult,
        admission: WorthQueryContributionComposedIntentStageResult,
        materialization: WorthQueryContributionComposedIntentStageResult,
        classification: WorthQueryContributionComposedIntentClassification,
        contribution: Option<WorthQueryContributionComposedContribution>,
    ) -> Self {
        Self {
            request,
            evaluation,
            admission,
            materialization,
            classification,
            contribution,
        }
    }

    pub fn request(&self) -> &WorthQueryContributionComposedIntentRequestDescriptor {
        &self.request
    }

    pub fn order_index(&self) -> usize {
        self.request.order_index()
    }

    pub fn category_family(&self) -> WorthQueryDeclarationEntryContributionCategoryFamily {
        self.request.category_family()
    }

    pub fn request_digest(&self) -> &str {
        self.request.request_digest()
    }

    pub fn semantic_code(&self) -> &str {
        self.request.semantic_code()
    }

    pub fn detail(&self) -> &str {
        self.request.detail()
    }

    pub fn target_digest(&self) -> &str {
        self.request.target_digest()
    }

    pub fn target_binding_digest(&self) -> &str {
        self.request.target_binding_digest()
    }

    pub fn request_identity(&self) -> &crate::WorthQueryEvidenceIdentity {
        self.request.request_identity()
    }

    pub fn binding_identity(&self) -> &crate::WorthQueryEvidenceIdentity {
        self.request.binding_identity()
    }

    pub fn aspect_record(&self) -> &WorthQueryContributionComposedIntentAspectRecord {
        self.request.aspect_record()
    }

    pub fn evaluation(&self) -> &WorthQueryContributionComposedIntentStageResult {
        &self.evaluation
    }

    pub fn admission(&self) -> &WorthQueryContributionComposedIntentStageResult {
        &self.admission
    }

    pub fn materialization(&self) -> &WorthQueryContributionComposedIntentStageResult {
        &self.materialization
    }

    pub fn classification(&self) -> WorthQueryContributionComposedIntentClassification {
        self.classification
    }

    pub fn contribution(&self) -> Option<&WorthQueryContributionComposedContribution> {
        self.contribution.as_ref()
    }

    pub fn is_admitted(&self) -> bool {
        self.contribution.is_some()
    }
}
