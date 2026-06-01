use crate::application::{
    ForgeQueryDeclarationEntryContributionCategoryFamily,
    ForgeQueryDeclarationEntryContributionComposition, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

use super::super::artifact::{
    ForgeQueryContributionComposedContribution, ForgeQueryContributionComposedOrchestration,
};
use super::super::composition::{
    classify_intent_results, strongest_stop, ForgeQueryContributionComposedComposition,
    ForgeQueryContributionComposedStop,
};
use super::super::input::{
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestrationInput,
};
use super::super::intent_result::ForgeQueryContributionComposedIntentResult;

pub(crate) fn request_descriptor<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: &ForgeQueryContributionComposedOrchestrationInput<D, I>,
) -> String {
    format!(
        "{}:{}:{}:{:?}",
        I::Family::semantic_family_key(),
        input
            .declaration_input()
            .canonical_declaration_entries()
            .len(),
        input.contributions().len(),
        input.materialization_policy()
    )
}

pub(crate) fn request_digest<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    input: &ForgeQueryContributionComposedOrchestrationInput<D, I>,
) -> String {
    let mut parts = vec![
        format!("family:{}", I::Family::semantic_family_key()),
        format!(
            "declaration_entries:{:?}",
            input.declaration_input().canonical_declaration_entries()
        ),
        format!("contribution_count:{}", input.contributions().len()),
    ];
    parts.extend(
        input
            .contributions()
            .iter()
            .enumerate()
            .map(|(index, value)| format!("intent:{index}:{value:?}")),
    );
    parts.push(format!(
        "materialization:{:?}",
        input.materialization_policy()
    ));
    hash_parts(&parts)
}

pub(crate) fn build_composed_artifact<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    intent_results: Vec<ForgeQueryContributionComposedIntentResult>,
) -> Result<
    ForgeQueryContributionComposedOrchestration<D, I>,
    (ForgeQueryContributionComposedStop, String),
> {
    let classification = classify_intent_results(&intent_results);
    let composition = ForgeQueryContributionComposedComposition::from_intent_results(
        classification,
        &intent_results,
    );
    let contribution_composition = ForgeQueryDeclarationEntryContributionComposition::new(
        composition.admitted_evidence().to_vec(),
        composition.rejected_category_families().to_vec(),
    );
    let contributions = intent_results
        .iter()
        .filter_map(|value| value.contribution().cloned())
        .collect::<Vec<ForgeQueryContributionComposedContribution>>();
    if contributions.is_empty() && !intent_results.is_empty() {
        return Err((
            strongest_stop(&intent_results),
            composition.composition_digest().to_string(),
        ));
    }
    Ok(ForgeQueryContributionComposedOrchestration::new(
        envelope,
        contribution_composition,
        contributions,
        intent_results,
        composition,
    ))
}

pub(crate) fn stop_reason(
    stop: ForgeQueryContributionComposedStop,
    intent_results: &[ForgeQueryContributionComposedIntentResult],
) -> String {
    let rejected_families = intent_results
        .iter()
        .filter(|value| !value.is_admitted())
        .map(ForgeQueryContributionComposedIntentResult::category_family)
        .map(ForgeQueryDeclarationEntryContributionCategoryFamily::as_str)
        .collect::<Vec<_>>();
    let stop_label = match stop {
        ForgeQueryContributionComposedStop::Deferred => "deferred",
        ForgeQueryContributionComposedStop::DeclarationDenied => "declaration-denied",
        ForgeQueryContributionComposedStop::ContributionDenied => "contribution-denied",
        ForgeQueryContributionComposedStop::Stale => "stale",
        ForgeQueryContributionComposedStop::RebindRequired => "rebind-required",
        ForgeQueryContributionComposedStop::Unsupported => "unsupported",
        ForgeQueryContributionComposedStop::Failed => "failed",
    };
    format!(
        "contribution composition stopped as {stop_label}; rejected families: {}",
        if rejected_families.is_empty() {
            "none".to_string()
        } else {
            rejected_families.join(", ")
        }
    )
}

pub(crate) fn materialization_policy_label(
    policy: &ForgeQueryContributionComposedMaterializationPolicy,
) -> &'static str {
    match policy {
        ForgeQueryContributionComposedMaterializationPolicy::None => "none",
        ForgeQueryContributionComposedMaterializationPolicy::Summary(_) => "summary",
    }
}
