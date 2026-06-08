use crate::application::{
    ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFoundationalEvidenceClass, ForgeQueryDeclarationFutureProjection,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::{
    aspects::{
        route_aspect_contract, route_aspect_fit, route_aspect_publication,
        route_aspect_publication_summary,
    },
    class::{
        ForgeQueryDeclarationRouteIntentRequirement, ForgeQueryDeclarationRouteMultiplicity,
        ForgeQueryDeclarationRoutePlanClass, ForgeQueryLowerAuthorityRouteFamily,
    },
    denial::{
        ForgeQueryDeclarationRoutePlanDeferred, ForgeQueryDeclarationRoutePlanDenialCause,
        ForgeQueryDeclarationRoutePlanDenied, ForgeQueryDeclarationRoutePlanFailed,
    },
    explain::ForgeQueryDeclarationRoutePlanExplanation,
    input::ForgeQueryDeclarationRoutePlanInput,
    intent::ForgeQueryDeclarationRouteIntent,
    plan::ForgeQueryDeclarationRoutePlan,
    route_set::{ForgeQueryDeclarationRouteSegment, ForgeQueryDeclarationRouteSet},
};

pub enum ForgeQueryDeclarationRoutePlanChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Planned(ForgeQueryDeclarationRoutePlan<D, I>),
    Deferred(ForgeQueryDeclarationRoutePlanDeferred<D, I>),
    Denied(ForgeQueryDeclarationRoutePlanDenied<D, I>),
    Failed(ForgeQueryDeclarationRoutePlanFailed<D, I>),
}

pub(crate) fn forge_query_checked_declaration_route_plan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationRoutePlanInput<D, I>,
) -> ForgeQueryDeclarationRoutePlanChecked<D, I> {
    let (progressed, evidence, route_intent) = input.into_parts();
    let route_contract = I::Family::route_contract();
    let future_projection =
        ForgeQueryDeclarationFutureProjection::from_declaration(progressed.canonical_declaration());
    let route_aspect_contract = route_aspect_contract(progressed.aspect_contract());
    let route_aspect_fit = route_aspect_fit(evidence.aspect_coverage(), &route_aspect_contract);
    let route_aspect_publication =
        route_aspect_publication(&route_aspect_contract, evidence.aspect_coverage());

    if evidence.class() != ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted {
        return ForgeQueryDeclarationRoutePlanChecked::Denied(
            ForgeQueryDeclarationRoutePlanDenied::new(
                progressed,
                evidence,
                route_intent,
                route_contract,
                ForgeQueryDeclarationRoutePlanDenialCause::EvidenceMismatch,
            ),
        );
    }

    match route_aspect_fit {
        ForgeQueryDeclarationAspectFit::Conflict => {
            return ForgeQueryDeclarationRoutePlanChecked::Denied(
                ForgeQueryDeclarationRoutePlanDenied::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    ForgeQueryDeclarationRoutePlanDenialCause::AspectConflict,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::MissingRequired => {
            return ForgeQueryDeclarationRoutePlanChecked::Denied(
                ForgeQueryDeclarationRoutePlanDenied::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    ForgeQueryDeclarationRoutePlanDenialCause::MissingRequiredAspect,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::Exact
        | ForgeQueryDeclarationAspectFit::CompatibleSuperset
        | ForgeQueryDeclarationAspectFit::Partial => {}
    }

    if progressed.canonical_declaration().handle_identity_digest()
        != evidence.handle_identity_digest()
    {
        return ForgeQueryDeclarationRoutePlanChecked::Denied(
            ForgeQueryDeclarationRoutePlanDenied::new(
                progressed,
                evidence,
                route_intent,
                route_contract,
                ForgeQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld,
            ),
        );
    }

    if progressed.operating_context_identity_digest()
        != evidence.operating_context_identity_digest()
        || progressed.progression_digest() != evidence.progression_digest().unwrap_or_default()
        || format!(
            "{:?}",
            progressed.canonical_declaration().declaration_digest()
        ) != evidence.declaration_digest()
    {
        return ForgeQueryDeclarationRoutePlanChecked::Denied(
            ForgeQueryDeclarationRoutePlanDenied::new(
                progressed,
                evidence,
                route_intent,
                route_contract,
                ForgeQueryDeclarationRoutePlanDenialCause::EvidenceMismatch,
            ),
        );
    }

    match route_contract.intent_requirement() {
        ForgeQueryDeclarationRouteIntentRequirement::Required if route_intent.is_none() => {
            return ForgeQueryDeclarationRoutePlanChecked::Denied(
                ForgeQueryDeclarationRoutePlanDenied::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    ForgeQueryDeclarationRoutePlanDenialCause::IntentRequired,
                ),
            );
        }
        ForgeQueryDeclarationRouteIntentRequirement::Forbidden => {
            if route_intent.is_some_and(|intent| intent != ForgeQueryDeclarationRouteIntent::Auto) {
                return ForgeQueryDeclarationRoutePlanChecked::Denied(
                    ForgeQueryDeclarationRoutePlanDenied::new(
                        progressed,
                        evidence,
                        route_intent,
                        route_contract,
                        ForgeQueryDeclarationRoutePlanDenialCause::IntentForbidden,
                    ),
                );
            }
        }
        _ => {}
    }

    if route_intent == Some(ForgeQueryDeclarationRouteIntent::DeferredRouting) {
        return if route_contract.can_defer() {
            ForgeQueryDeclarationRoutePlanChecked::Deferred(
                ForgeQueryDeclarationRoutePlanDeferred::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    "the declaration route remains explicitly deferred by caller intent",
                ),
            )
        } else {
            ForgeQueryDeclarationRoutePlanChecked::Denied(
                ForgeQueryDeclarationRoutePlanDenied::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    ForgeQueryDeclarationRoutePlanDenialCause::IntentConflictsWithRouteContract,
                ),
            )
        };
    }

    let mut routes = Vec::new();
    for family in route_contract.allowed_route_families() {
        if !intent_allows_family(route_intent, *family) {
            continue;
        }
        if *family == ForgeQueryLowerAuthorityRouteFamily::Mixed {
            return ForgeQueryDeclarationRoutePlanChecked::Failed(
                ForgeQueryDeclarationRoutePlanFailed::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    "mixed is a route-plan classification, not a concrete lower-authority route segment",
                ),
            );
        }
        routes.push(ForgeQueryDeclarationRouteSegment::new(
            *family,
            format!(
                "{} admitted through {}",
                family.as_str(),
                route_contract.reason()
            ),
        ));
    }

    if routes.is_empty() {
        return if route_contract.can_defer() {
            ForgeQueryDeclarationRoutePlanChecked::Deferred(
                ForgeQueryDeclarationRoutePlanDeferred::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    "no concrete lower-authority route is active yet, so routing remains deferred",
                ),
            )
        } else {
            ForgeQueryDeclarationRoutePlanChecked::Denied(
                ForgeQueryDeclarationRoutePlanDenied::new(
                    progressed,
                    evidence,
                    route_intent,
                    route_contract,
                    ForgeQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes,
                ),
            )
        };
    }

    if route_contract.multiplicity() == ForgeQueryDeclarationRouteMultiplicity::Singular
        && routes.len() > 1
    {
        return ForgeQueryDeclarationRoutePlanChecked::Denied(
            ForgeQueryDeclarationRoutePlanDenied::new(
                progressed,
                evidence,
                route_intent,
                route_contract,
                ForgeQueryDeclarationRoutePlanDenialCause::ForbiddenRouteCombination,
            ),
        );
    }

    let class = classify_routes(&routes);
    let explanation = ForgeQueryDeclarationRoutePlanExplanation::new(
        route_contract.reason(),
        vec![
            format!("family:{}", progressed.declaration_family_key()),
            format!(
                "operating_context:{}",
                progressed.operating_context_identity_digest()
            ),
            format!("progression:{}", progressed.progression_digest()),
            format!("route-aspect-fit:{route_aspect_fit:?}"),
            format!(
                "route-coverage-basis:{:?}",
                evidence.aspect_coverage_basis()
            ),
            format!(
                "route-publication:{}",
                route_aspect_publication_summary(&route_aspect_publication)
            ),
            format!(
                "future-projection-digest:{}",
                future_projection.projection_digest()
            ),
        ]
        .into_iter()
        .chain(future_projection.retained_facts())
        .collect(),
        routes
            .iter()
            .map(|route| route.reason().to_string())
            .collect(),
        route_intent.map(|intent| format!("intent:{} narrowed the route set", intent.as_str())),
    );
    ForgeQueryDeclarationRoutePlanChecked::Planned(ForgeQueryDeclarationRoutePlan::new(
        progressed,
        evidence,
        route_intent,
        ForgeQueryDeclarationRouteSet::new(routes),
        class,
        route_contract.automation_requires_explicit_handoff(),
        route_aspect_contract,
        route_aspect_fit,
        route_aspect_publication,
        future_projection,
        explanation,
    ))
}

fn classify_routes(
    routes: &[ForgeQueryDeclarationRouteSegment],
) -> ForgeQueryDeclarationRoutePlanClass {
    if routes.len() > 1 {
        return ForgeQueryDeclarationRoutePlanClass::Mixed;
    }
    match routes[0].family() {
        ForgeQueryLowerAuthorityRouteFamily::Relational => {
            ForgeQueryDeclarationRoutePlanClass::RelationalOnly
        }
        ForgeQueryLowerAuthorityRouteFamily::Bridge => {
            ForgeQueryDeclarationRoutePlanClass::BridgeOnly
        }
        ForgeQueryLowerAuthorityRouteFamily::Signal => {
            ForgeQueryDeclarationRoutePlanClass::SignalOnly
        }
        ForgeQueryLowerAuthorityRouteFamily::Mixed
        | ForgeQueryLowerAuthorityRouteFamily::Deferred
        | ForgeQueryLowerAuthorityRouteFamily::Forbidden => {
            ForgeQueryDeclarationRoutePlanClass::Mixed
        }
    }
}

fn intent_allows_family(
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    family: ForgeQueryLowerAuthorityRouteFamily,
) -> bool {
    match route_intent.unwrap_or(ForgeQueryDeclarationRouteIntent::Auto) {
        ForgeQueryDeclarationRouteIntent::Auto => true,
        ForgeQueryDeclarationRouteIntent::RelationalOnly => {
            family == ForgeQueryLowerAuthorityRouteFamily::Relational
        }
        ForgeQueryDeclarationRouteIntent::BridgeOnly => {
            family == ForgeQueryLowerAuthorityRouteFamily::Bridge
        }
        ForgeQueryDeclarationRouteIntent::SignalOnly => {
            family == ForgeQueryLowerAuthorityRouteFamily::Signal
        }
        ForgeQueryDeclarationRouteIntent::RelationalAndBridge => {
            matches!(
                family,
                ForgeQueryLowerAuthorityRouteFamily::Relational
                    | ForgeQueryLowerAuthorityRouteFamily::Bridge
            )
        }
        ForgeQueryDeclarationRouteIntent::DeferredRouting => false,
    }
}
