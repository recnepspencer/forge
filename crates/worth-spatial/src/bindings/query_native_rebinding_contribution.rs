use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryContinuityContributionAuthoring,
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
    ForgeQueryDomainOperatingContext, ForgeQueryExplanationContributionAuthoring,
};

use crate::bindings::query_native_rebinding::PrimitiveRebindingQueryDomain;
use crate::bindings::query_native_rebinding_authoring::PrimitiveRebindingDeclarationEntry;
use crate::bindings::query_native_rebinding_projection::{
    primitive_rebinding_projection_facts, PrimitiveRebindingProjectionFactError,
    PrimitiveRebindingProjectionFactReceipt,
};
use crate::bindings::rebinding::{RebindingOutcomeClass, UnsupportedRebindingReason};

pub fn primitive_rebinding_contribution_workflow<C>(
    declaration: PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> ForgeQueryContributionComposedOrchestrationInput<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingDeclarationEntry,
>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    ForgeQueryContributionComposedOrchestrationInput::new(declaration.clone()).with_contributions(
        primitive_rebinding_semantic_contributions(&declaration, handle),
    )
}

pub(crate) fn primitive_rebinding_semantic_contributions<C>(
    declaration: &PrimitiveRebindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Vec<ForgeQueryContributionIntent>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    match primitive_rebinding_projection_facts(declaration, handle) {
        Ok(receipt) => vec![contribution_for_receipt(receipt)],
        Err(error) => vec![contribution_error(error)],
    }
}

fn contribution_error(
    error: PrimitiveRebindingProjectionFactError,
) -> ForgeQueryContributionIntent {
    let detail = match error {
        PrimitiveRebindingProjectionFactError::DeclarationDenied(error) => format!(
            "rebinding declaration denied before admitted family workflow: {error:?}"
        ),
        PrimitiveRebindingProjectionFactError::OutcomeNotBound {
            kind,
            reason,
            next_step,
        } => format!(
            "rebinding projection drifted before contribution composition: {kind:?} {reason} {next_step:?}"
        ),
    };
    ForgeQueryContributionIntent::explanation(
        ForgeQueryExplanationContributionAuthoring::requires_context(
            "worth.spatial.rebinding.explanation.declaration_denied",
            detail,
        ),
    )
}

fn contribution_for_receipt(
    receipt: PrimitiveRebindingProjectionFactReceipt,
) -> ForgeQueryContributionIntent {
    match receipt.outcome_class() {
        RebindingOutcomeClass::Preserved => continuity_contribution(
            &receipt,
            "worth.spatial.rebinding.continuity.preserved",
            "rebinding preserved authoritative continuity inside the admitted local replacement neighborhood",
        ),
        RebindingOutcomeClass::ExactReattachment => continuity_contribution(
            &receipt,
            "worth.spatial.rebinding.continuity.exact_reattachment",
            "rebinding preserved authoritative continuity through an exact admitted successor",
        ),
        RebindingOutcomeClass::ContinuityJustifiedReattachment => continuity_contribution(
            &receipt,
            "worth.spatial.rebinding.continuity.authoritative_successor",
            "rebinding preserved authoritative continuity through an admitted successor justified by local topology correspondence",
        ),
        RebindingOutcomeClass::CorrespondenceOnly => ForgeQueryContributionIntent::continuity(
            ForgeQueryContinuityContributionAuthoring::correspondence_only(
                "worth.spatial.rebinding.continuity.correspondence_only",
                "rebinding retained correspondence-only meaning without authoritative continuity",
            ),
        ),
        RebindingOutcomeClass::Ambiguous => explanation_contribution(
            "worth.spatial.rebinding.explanation.ambiguous",
            "rebinding remained ambiguous within the admitted local replacement neighborhood",
        ),
        RebindingOutcomeClass::Orphaned => explanation_contribution(
            "worth.spatial.rebinding.explanation.orphaned",
            "rebinding remained orphaned and requires explicit rebind context",
        ),
        RebindingOutcomeClass::Unsupported => explanation_contribution(
            "worth.spatial.rebinding.explanation.unsupported",
            unsupported_detail(receipt.unsupported_reason()),
        ),
    }
}

fn continuity_contribution(
    receipt: &PrimitiveRebindingProjectionFactReceipt,
    semantic_code: &'static str,
    detail: &'static str,
) -> ForgeQueryContributionIntent {
    let successor_identity = receipt
        .selected_candidate_identity()
        .unwrap_or_else(|| receipt.prior_binding_identity());
    ForgeQueryContributionIntent::continuity(
        ForgeQueryContinuityContributionAuthoring::preserved_rebind(
            receipt.prior_binding_identity(),
            successor_identity,
            semantic_code,
            detail,
        ),
    )
}

fn explanation_contribution(
    semantic_code: &'static str,
    detail: impl Into<String>,
) -> ForgeQueryContributionIntent {
    ForgeQueryContributionIntent::explanation(
        ForgeQueryExplanationContributionAuthoring::requires_context(semantic_code, detail),
    )
}

fn unsupported_detail(reason: Option<UnsupportedRebindingReason>) -> String {
    match reason {
        Some(reason) => format!(
            "rebinding family is unsupported for the admitted local replacement neighborhood: {reason:?}"
        ),
        None => "rebinding family is unsupported for the admitted local replacement neighborhood"
            .to_string(),
    }
}
