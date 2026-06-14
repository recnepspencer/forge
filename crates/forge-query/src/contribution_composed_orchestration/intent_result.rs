use crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily;

use super::artifact::ForgeQueryContributionComposedContribution;
use super::aspect::ForgeQueryContributionComposedIntentAspectRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedIntentRequestDescriptor {
    order_index: usize,
    category_family: ForgeQueryDeclarationEntryContributionCategoryFamily,
    request_identity: crate::ForgeQueryEvidenceIdentity,
    binding_identity: crate::ForgeQueryEvidenceIdentity,
    semantic_code: String,
    detail: String,
    aspect_record: ForgeQueryContributionComposedIntentAspectRecord,
}

impl ForgeQueryContributionComposedIntentRequestDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        order_index: usize,
        category_family: ForgeQueryDeclarationEntryContributionCategoryFamily,
        request_identity: crate::ForgeQueryEvidenceIdentity,
        binding_identity: crate::ForgeQueryEvidenceIdentity,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        aspect_record: ForgeQueryContributionComposedIntentAspectRecord,
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

    pub fn category_family(&self) -> ForgeQueryDeclarationEntryContributionCategoryFamily {
        self.category_family
    }

    pub fn request_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_digest(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn binding_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
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

    pub fn aspect_record(&self) -> &ForgeQueryContributionComposedIntentAspectRecord {
        &self.aspect_record
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionComposedIntentStageKind {
    NotAttempted,
    Succeeded,
    Denied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedIntentStageResult {
    kind: ForgeQueryContributionComposedIntentStageKind,
    detail: String,
    stage_identity: Option<crate::ForgeQueryEvidenceIdentity>,
}

impl ForgeQueryContributionComposedIntentStageResult {
    pub fn not_attempted() -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::NotAttempted,
            "stage was not attempted",
            None,
        )
    }

    pub fn succeeded(
        detail: impl Into<String>,
        stage_identity: Option<crate::ForgeQueryEvidenceIdentity>,
    ) -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::Succeeded,
            detail,
            stage_identity,
        )
    }

    pub fn denied(detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::Denied,
            detail,
            None,
        )
    }

    pub fn stale(detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::Stale,
            detail,
            None,
        )
    }

    pub fn rebind_required(detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::RebindRequired,
            detail,
            None,
        )
    }

    pub fn unsupported(detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::Unsupported,
            detail,
            None,
        )
    }

    pub fn failed(detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryContributionComposedIntentStageKind::Failed,
            detail,
            None,
        )
    }

    pub fn kind(&self) -> ForgeQueryContributionComposedIntentStageKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn stage_identity(&self) -> Option<&crate::ForgeQueryEvidenceIdentity> {
        self.stage_identity.as_ref()
    }

    pub fn stage_for_reporting(&self) -> Option<&str> {
        self.stage_identity.as_ref().map(|identity| identity.as_str())
    }

    pub fn digest(&self) -> Option<&str> {
        self.stage_for_reporting()
    }

    fn new(
        kind: ForgeQueryContributionComposedIntentStageKind,
        detail: impl Into<String>,
        stage_identity: Option<crate::ForgeQueryEvidenceIdentity>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            stage_identity,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionComposedIntentClassification {
    Admitted,
    Denied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
    MaterializationFailedAfterAdmission,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedIntentResult {
    request: ForgeQueryContributionComposedIntentRequestDescriptor,
    evaluation: ForgeQueryContributionComposedIntentStageResult,
    admission: ForgeQueryContributionComposedIntentStageResult,
    materialization: ForgeQueryContributionComposedIntentStageResult,
    classification: ForgeQueryContributionComposedIntentClassification,
    contribution: Option<ForgeQueryContributionComposedContribution>,
}

impl ForgeQueryContributionComposedIntentResult {
    pub fn new(
        request: ForgeQueryContributionComposedIntentRequestDescriptor,
        evaluation: ForgeQueryContributionComposedIntentStageResult,
        admission: ForgeQueryContributionComposedIntentStageResult,
        materialization: ForgeQueryContributionComposedIntentStageResult,
        classification: ForgeQueryContributionComposedIntentClassification,
        contribution: Option<ForgeQueryContributionComposedContribution>,
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

    pub fn request(&self) -> &ForgeQueryContributionComposedIntentRequestDescriptor {
        &self.request
    }

    pub fn order_index(&self) -> usize {
        self.request.order_index()
    }

    pub fn category_family(&self) -> ForgeQueryDeclarationEntryContributionCategoryFamily {
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

    pub fn request_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
        self.request.request_identity()
    }

    pub fn binding_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
        self.request.binding_identity()
    }

    pub fn aspect_record(&self) -> &ForgeQueryContributionComposedIntentAspectRecord {
        self.request.aspect_record()
    }

    pub fn evaluation(&self) -> &ForgeQueryContributionComposedIntentStageResult {
        &self.evaluation
    }

    pub fn admission(&self) -> &ForgeQueryContributionComposedIntentStageResult {
        &self.admission
    }

    pub fn materialization(&self) -> &ForgeQueryContributionComposedIntentStageResult {
        &self.materialization
    }

    pub fn classification(&self) -> ForgeQueryContributionComposedIntentClassification {
        self.classification
    }

    pub fn contribution(&self) -> Option<&ForgeQueryContributionComposedContribution> {
        self.contribution.as_ref()
    }

    pub fn is_admitted(&self) -> bool {
        self.contribution.is_some()
    }
}
