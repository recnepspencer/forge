use crate::domain_capabilities::identity::domain_capability_scope_encoder;
use crate::evidence_identity::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceTag};

use super::common::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilitySemanticPosture, SealedPayload,
};
use super::{
    ForgeQueryWorkflowInspectionSemantics, ForgeQueryWorkflowLoweringSemantics,
    ForgeQueryWorkflowRuntimeSemantics,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryWorkflowContributionPosture {
    PreviewOnly,
    PromotionEligible,
    ConfirmationRequired,
    DiscardRequired,
}

impl ForgeQueryWorkflowContributionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreviewOnly => "preview-only",
            Self::PromotionEligible => "promotion-eligible",
            Self::ConfirmationRequired => "confirmation-required",
            Self::DiscardRequired => "discard-required",
        }
    }

    pub const fn semantic_posture(self) -> ForgeQueryDomainCapabilitySemanticPosture {
        match self {
            Self::PreviewOnly => ForgeQueryDomainCapabilitySemanticPosture::WorkflowPreviewOnly,
            Self::PromotionEligible => {
                ForgeQueryDomainCapabilitySemanticPosture::WorkflowPromotionEligible
            }
            Self::ConfirmationRequired => {
                ForgeQueryDomainCapabilitySemanticPosture::WorkflowConfirmationRequired
            }
            Self::DiscardRequired => {
                ForgeQueryDomainCapabilitySemanticPosture::WorkflowDiscardRequired
            }
        }
    }
}

fn compose_workflow_payload_identity(
    posture: ForgeQueryWorkflowContributionPosture,
    semantic_code: &str,
    detail: &str,
    runtime_semantics: Option<&ForgeQueryWorkflowRuntimeSemantics>,
    lowering_semantics: Option<&ForgeQueryWorkflowLoweringSemantics>,
    inspection_semantics: Option<&ForgeQueryWorkflowInspectionSemantics>,
) -> ForgeQueryEvidenceIdentity {
    let mut identity = domain_capability_scope_encoder("forge_query_domain_capability_payload_v5")
        .field_shape(
            ForgeQueryEvidenceTag::new("category"),
            ForgeQueryDomainCapabilityCategory::WorkflowPreview.as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("semantic_code"), semantic_code)
        .field_shape(ForgeQueryEvidenceTag::new("detail"), detail);
    identity = match runtime_semantics {
        Some(runtime) => identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime"),
            &runtime.semantics_identity(),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("runtime"), "none"),
    };
    identity = match lowering_semantics {
        Some(lowering) => identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("lowering"),
            &lowering.semantics_identity(),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("lowering"), "none"),
    };
    identity = match inspection_semantics {
        Some(inspection) => identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("inspection"),
            &inspection.semantics_identity(),
        ),
        None => identity.field_shape(ForgeQueryEvidenceTag::new("inspection"), "none"),
    };
    identity.seal()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWorkflowContributionPayload {
    posture: ForgeQueryWorkflowContributionPosture,
    semantic_code: String,
    detail: String,
    runtime_semantics: Option<ForgeQueryWorkflowRuntimeSemantics>,
    lowering_semantics: Option<ForgeQueryWorkflowLoweringSemantics>,
    inspection_semantics: Option<ForgeQueryWorkflowInspectionSemantics>,
    payload_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryWorkflowContributionPayload {
    pub fn new(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_all_semantics(posture, semantic_code, detail, None, None, None)
    }

    pub fn with_runtime_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryWorkflowRuntimeSemantics>,
    ) -> Self {
        Self::with_all_semantics(
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            None,
            None,
        )
    }

    pub fn with_runtime_and_lowering_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryWorkflowRuntimeSemantics>,
        lowering_semantics: Option<ForgeQueryWorkflowLoweringSemantics>,
    ) -> Self {
        Self::with_all_semantics(
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            lowering_semantics,
            None,
        )
    }

    pub fn with_runtime_and_inspection_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryWorkflowRuntimeSemantics>,
        inspection_semantics: Option<ForgeQueryWorkflowInspectionSemantics>,
    ) -> Self {
        Self::with_all_semantics(
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            None,
            inspection_semantics,
        )
    }

    pub fn with_all_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: Option<ForgeQueryWorkflowRuntimeSemantics>,
        lowering_semantics: Option<ForgeQueryWorkflowLoweringSemantics>,
        inspection_semantics: Option<ForgeQueryWorkflowInspectionSemantics>,
    ) -> Self {
        let semantic_code = semantic_code.into();
        let detail = detail.into();
        let payload_identity = compose_workflow_payload_identity(
            posture,
            &semantic_code,
            &detail,
            runtime_semantics.as_ref(),
            lowering_semantics.as_ref(),
            inspection_semantics.as_ref(),
        );
        Self {
            posture,
            semantic_code,
            detail,
            runtime_semantics,
            lowering_semantics,
            inspection_semantics,
            payload_identity,
        }
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        ForgeQueryDomainCapabilityCategory::WorkflowPreview
    }

    pub fn posture(&self) -> ForgeQueryWorkflowContributionPosture {
        self.posture
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn runtime_semantics(&self) -> Option<&ForgeQueryWorkflowRuntimeSemantics> {
        self.runtime_semantics.as_ref()
    }

    pub fn lowering_semantics(&self) -> Option<&ForgeQueryWorkflowLoweringSemantics> {
        self.lowering_semantics.as_ref()
    }

    pub fn inspection_semantics(&self) -> Option<&ForgeQueryWorkflowInspectionSemantics> {
        self.inspection_semantics.as_ref()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_for_reporting(&self) -> &str {
        self.payload_identity.as_str()
    }

    pub fn payload_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.payload_identity
    }
}

impl SealedPayload for ForgeQueryWorkflowContributionPayload {}

impl ForgeQueryDomainCapabilityPayload for ForgeQueryWorkflowContributionPayload {
    fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        self.category()
    }

    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }

    fn semantic_posture(&self) -> ForgeQueryDomainCapabilitySemanticPosture {
        self.posture().semantic_posture()
    }

    fn semantic_code(&self) -> &str {
        self.semantic_code()
    }

    fn detail(&self) -> &str {
        self.detail()
    }

    fn payload_digest(&self) -> &str {
        self.payload_digest()
    }

    fn payload_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.payload_identity
    }
}
