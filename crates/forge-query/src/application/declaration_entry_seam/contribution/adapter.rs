use crate::domain_capabilities::ForgeQueryDomainCapabilityTargetBinding;

use super::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionEvidence,
    ForgeQueryDeclarationEntryContributionEvidenceRecord,
    ForgeQueryDeclarationEntryContributionTargetFamily,
};

macro_rules! impl_from_admitted {
    ($wrapper:path, $target:ty, $variant:ident, $family:expr) => {
        impl From<$wrapper> for ForgeQueryDeclarationEntryContributionEvidence {
            fn from(value: $wrapper) -> Self {
                let payload = value.payload().payload();
                Self::$variant(ForgeQueryDeclarationEntryContributionEvidenceRecord {
                    target_family: target_family_for::<$target>(),
                    target_digest: value.payload().target().target_digest().to_string(),
                    target_binding_digest: value.payload().target().binding_digest().to_string(),
                    evidence_digest: value.admitted_digest(),
                    posture_label: payload.posture_label().to_string(),
                    semantic_code: payload.semantic_code().to_string(),
                    detail: payload.detail().to_string(),
                    decision_stage: decision_stage_for(&$family, payload),
                })
            }
        }
    };
}

trait EntryContributionPayloadView {
    fn posture_label(&self) -> &'static str;
}

impl EntryContributionPayloadView
    for crate::domain_capabilities::ForgeQueryAdmissionContributionPayload
{
    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }
}
impl EntryContributionPayloadView
    for crate::domain_capabilities::ForgeQuerySupportContributionPayload
{
    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }
}
impl EntryContributionPayloadView
    for crate::domain_capabilities::ForgeQueryExplanationContributionPayload
{
    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }
}
impl EntryContributionPayloadView
    for crate::domain_capabilities::ForgeQueryWorkflowContributionPayload
{
    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }
}
impl EntryContributionPayloadView
    for crate::domain_capabilities::ForgeQueryContinuityContributionPayload
{
    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }
}
impl EntryContributionPayloadView
    for crate::domain_capabilities::ForgeQueryAftermathContributionPayload
{
    fn posture_label(&self) -> &'static str {
        self.posture().as_str()
    }
}

fn target_family_for<T: ForgeQueryDomainCapabilityTargetBinding>(
) -> ForgeQueryDeclarationEntryContributionTargetFamily {
    let kind = std::any::type_name::<T>();
    if kind.ends_with("ForgeQueryDeclarationBoundContributionTarget") {
        ForgeQueryDeclarationEntryContributionTargetFamily::DeclarationBound
    } else if kind.ends_with("ForgeQueryAdmittedPlanBoundContributionTarget") {
        ForgeQueryDeclarationEntryContributionTargetFamily::AdmittedPlanBound
    } else {
        ForgeQueryDeclarationEntryContributionTargetFamily::LowerRuntimeBound
    }
}

fn decision_stage_for<P: EntryContributionPayloadView>(
    _category: &ForgeQueryDeclarationEntryContributionCategoryFamily,
    _payload: &P,
) -> Option<String> {
    None
}

impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedAdmissionContribution<
        crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    Admission,
    ForgeQueryDeclarationEntryContributionCategoryFamily::Admission
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedAdmissionContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    Admission,
    ForgeQueryDeclarationEntryContributionCategoryFamily::Admission
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedSupportContribution<
        crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    SupportTraceability,
    ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedSupportContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    SupportTraceability,
    ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedSupportContribution<
        crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    SupportTraceability,
    ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedExplanationContribution<
        crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    ExplanationInspection,
    ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedExplanationContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    ExplanationInspection,
    ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedExplanationContribution<
        crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ExplanationInspection,
    ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedWorkflowContribution<
        crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
    WorkflowPreview,
    ForgeQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedWorkflowContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    WorkflowPreview,
    ForgeQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedContinuityContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    ContinuityLineage,
    ForgeQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedAftermathContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    ConsequenceAftermath,
    ForgeQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
);
impl_from_admitted!(
    crate::domain_capabilities::ForgeQueryAdmittedAftermathContribution<
        crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ConsequenceAftermath,
    ForgeQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
);
