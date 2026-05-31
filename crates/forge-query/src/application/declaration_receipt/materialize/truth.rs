use crate::application::{
    route_scoped_declaration_aspect_contract, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationFoundationalEvidence,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRoutePlan, ForgeQueryDomainEntryMarker,
};

pub(super) fn route_scoped_aspect_contract(
    declaration_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectContract {
    route_scoped_declaration_aspect_contract(declaration_contract)
}

pub(super) fn evidence_aspect_coverage<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    evidence: &ForgeQueryDeclarationFoundationalEvidence<D, I>,
) -> ForgeQueryDeclarationAspectCoverage {
    evidence.aspect_coverage().clone()
}

pub(super) fn completed_boundary_text(
    kind: crate::application::ForgeQueryDeclarationReceiptKind,
    family: &str,
) -> String {
    format!(
        "{} declaration boundary receipt for {}",
        match kind {
            crate::application::ForgeQueryDeclarationReceiptKind::Relational => "relational",
            crate::application::ForgeQueryDeclarationReceiptKind::Bridge => "bridge",
            crate::application::ForgeQueryDeclarationReceiptKind::Mixed => "mixed",
            crate::application::ForgeQueryDeclarationReceiptKind::Deferred => "deferred",
            crate::application::ForgeQueryDeclarationReceiptKind::Denied => "denied",
            crate::application::ForgeQueryDeclarationReceiptKind::Failed => "failed",
        },
        family
    )
}

pub(super) fn retained_truths_from_plan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    plan: &ForgeQueryDeclarationRoutePlan<D, I>,
) -> Vec<String> {
    let mut truths = retained_truths_from_evidence(plan.foundational_evidence(), None, None);
    truths.push(format!("route-plan:{}", plan.route_plan_digest()));
    truths.push(format!("route-count:{}", plan.route_count()));
    truths.push(format!("route-aspect-fit:{:?}", plan.aspect_fit()));
    truths.push(format!("route-publication:{:?}", plan.aspect_publication()));
    for family in plan.route_families() {
        truths.push(format!("route-family:{}", family.as_str()));
    }
    truths
}

pub(super) fn retained_truths_from_evidence<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    evidence: &ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_contract: Option<ForgeQueryDeclarationRouteContract>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> Vec<String> {
    let mut truths = vec![
        format!("handle:{}", evidence.handle_identity_digest()),
        format!(
            "operating_context:{}",
            evidence.operating_context_identity_digest()
        ),
        format!("declaration:{}", evidence.declaration_digest()),
        format!("support:{}", evidence.support_digest()),
        format!("coverage-basis:{:?}", evidence.aspect_coverage_basis()),
        format!("evidence-publication:{:?}", evidence.aspect_publication()),
    ];
    if let Some(progression) = evidence.progression_digest() {
        truths.push(format!("progression:{progression}"));
    }
    if let Some(contract) = route_contract {
        truths.push(format!("route-contract:{}", contract.reason()));
        for family in contract.allowed_route_families() {
            truths.push(format!("route-family:{}", family.as_str()));
        }
    }
    if let Some(intent) = route_intent {
        truths.push(format!("route-intent:{}", intent.as_str()));
    }
    truths
}

pub(super) fn governing_reason_from_plan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    plan: &ForgeQueryDeclarationRoutePlan<D, I>,
) -> String {
    let mut parts = vec![plan.explain().route_contract_reason().to_string()];
    parts.extend(plan.explain().route_segment_reasons().iter().cloned());
    if let Some(intent_reason) = plan.explain().intent_reason() {
        parts.push(intent_reason.to_string());
    }
    parts.join("; ")
}

pub(super) fn route_reference_for_non_success(
    route_contract: Option<ForgeQueryDeclarationRouteContract>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
) -> Option<String> {
    let contract_token = route_contract.map(|contract| {
        contract
            .allowed_route_families()
            .iter()
            .map(|family| family.as_str())
            .collect::<Vec<_>>()
            .join("+")
    });
    match (contract_token, route_intent) {
        (Some(family_tokens), Some(intent)) => Some(format!(
            "contract:{family_tokens}|intent:{}",
            intent.as_str()
        )),
        (Some(family_tokens), None) => Some(format!("contract:{family_tokens}")),
        (None, Some(intent)) => Some(format!("intent:{}", intent.as_str())),
        (None, None) => None,
    }
}
