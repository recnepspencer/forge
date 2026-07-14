use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{
    WorthQueryDeclarationEntryContributionCategoryFamily,
    WorthQueryDeclarationEntryContributionComposition,
    WorthQueryDeclarationEntryContributionCompositionError,
    WorthQueryDeclarationEntryContributionCompositionFailureClass,
    WorthQueryDeclarationEntryContributionEvidenceSet,
    WorthQueryDeclarationEntryContributionTargetFamily,
    WorthQueryDeclarationEntryRetainedSubjectStrength,
};

pub(crate) struct WorthQueryDeclarationEntryContributionReconciliationContext<'a> {
    pub(crate) declaration_family_key: &'static str,
    pub(crate) declaration_digest: Option<&'a str>,
    pub(crate) subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength,
    pub(crate) admitted_plan_digest: Option<&'a str>,
    pub(crate) lower_runtime_boundary_digest: Option<&'a str>,
}

pub(crate) fn reconcile_contribution_evidence<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    context: WorthQueryDeclarationEntryContributionReconciliationContext<'_>,
    evidence_set: Option<&WorthQueryDeclarationEntryContributionEvidenceSet>,
) -> Result<
    Option<WorthQueryDeclarationEntryContributionComposition>,
    WorthQueryDeclarationEntryContributionCompositionError<D, I>,
> {
    let Some(evidence_set) = evidence_set else {
        return Ok(None);
    };
    if evidence_set.is_empty() {
        return Ok(None);
    }

    let mut rejected = Vec::new();
    let mut failure_class = None;
    let mut reason = None;
    for evidence in evidence_set.evidence() {
        if let Err(error) = reconcile_evidence(evidence, &context) {
            rejected.push(evidence.category_family());
            failure_class.get_or_insert(error.0);
            reason.get_or_insert(error.1);
        }
    }

    if let (Some(failure_class), Some(reason)) = (failure_class, reason) {
        return Err(WorthQueryDeclarationEntryContributionCompositionError::new(
            context.declaration_family_key,
            failure_class,
            rejected,
            reason,
        ));
    }

    Ok(Some(
        WorthQueryDeclarationEntryContributionComposition::new(
            evidence_set.evidence().to_vec(),
            Vec::new(),
        ),
    ))
}

fn reconcile_evidence(
    evidence: &super::WorthQueryDeclarationEntryContributionEvidence,
    context: &WorthQueryDeclarationEntryContributionReconciliationContext<'_>,
) -> Result<
    (),
    (
        WorthQueryDeclarationEntryContributionCompositionFailureClass,
        &'static str,
    ),
> {
    if context.declaration_digest.is_some_and(|digest| {
        evidence.target_family()
            == WorthQueryDeclarationEntryContributionTargetFamily::DeclarationBound
            && evidence.target_digest() != digest
    }) {
        return Err((
            WorthQueryDeclarationEntryContributionCompositionFailureClass::TargetDigestMismatch,
            "contribution composition requires declaration-entry contribution targets bound to the same retained declaration digest",
        ));
    }
    match evidence.target_family() {
        WorthQueryDeclarationEntryContributionTargetFamily::DeclarationBound => {
            if !category_allowed_for_target_and_context(
                evidence.category_family(),
                evidence.target_family(),
                context,
            ) {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::CategoryNotComposableForRetainedSeam,
                    "the supplied contribution category requires stronger retained downstream proof than this declaration-entry seam subject carries",
                ));
            }
        }
        WorthQueryDeclarationEntryContributionTargetFamily::AdmittedPlanBound => {
            if context.declaration_digest.is_none() || context.admitted_plan_digest.is_none() {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::TargetFamilyTooStrong,
                    "admitted-plan-bound contribution composition requires a retained declaration-entry subject plus matching retained admitted-plan proof",
                ));
            }
            if context
                .admitted_plan_digest
                .is_some_and(|digest| evidence.target_digest() != digest)
            {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::TargetDigestMismatch,
                    "contribution composition requires admitted-plan-bound evidence bound to the same retained admitted-plan digest",
                ));
            }
            if !category_allowed_for_target_and_context(
                evidence.category_family(),
                evidence.target_family(),
                context,
            ) {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::CategoryNotComposableForRetainedSeam,
                    "the supplied contribution category is not composable from retained declaration-entry plus admitted-plan proof",
                ));
            }
        }
        WorthQueryDeclarationEntryContributionTargetFamily::LowerRuntimeBound => {
            if context.declaration_digest.is_none()
                || context.lower_runtime_boundary_digest.is_none()
            {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::TargetFamilyTooStrong,
                    "lower-runtime-bound contribution composition requires a retained declaration-entry subject plus matching retained lower-runtime boundary proof",
                ));
            }
            if context
                .lower_runtime_boundary_digest
                .is_some_and(|digest| evidence.target_digest() != digest)
            {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::TargetDigestMismatch,
                    "contribution composition requires lower-runtime-bound evidence bound to the same retained lower-runtime boundary digest",
                ));
            }
            if !category_allowed_for_target_and_context(
                evidence.category_family(),
                evidence.target_family(),
                context,
            ) {
                return Err((
                    WorthQueryDeclarationEntryContributionCompositionFailureClass::CategoryNotComposableForRetainedSeam,
                    "the supplied contribution category is not composable from retained declaration-entry plus lower-runtime boundary proof",
                ));
            }
        }
    }
    Ok(())
}

fn category_allowed_for_target_and_context(
    category: WorthQueryDeclarationEntryContributionCategoryFamily,
    target_family: WorthQueryDeclarationEntryContributionTargetFamily,
    context: &WorthQueryDeclarationEntryContributionReconciliationContext<'_>,
) -> bool {
    match target_family {
        WorthQueryDeclarationEntryContributionTargetFamily::DeclarationBound => match category {
            WorthQueryDeclarationEntryContributionCategoryFamily::Admission
            | WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
            | WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability
            | WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection => true,
            WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview => {
                context.admitted_plan_digest.is_some()
                    && context.subject_strength
                        >= WorthQueryDeclarationEntryRetainedSubjectStrength::Envelope
            }
            WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage
            | WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath => false,
        },
        WorthQueryDeclarationEntryContributionTargetFamily::AdmittedPlanBound => {
            matches!(
                category,
                WorthQueryDeclarationEntryContributionCategoryFamily::Admission
                    | WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
                    | WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability
                    | WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
                    | WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview
                    | WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage
                    | WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
            )
        }
        WorthQueryDeclarationEntryContributionTargetFamily::LowerRuntimeBound => matches!(
            category,
            WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
                | WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability
                | WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
                | WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
        ),
    }
}
