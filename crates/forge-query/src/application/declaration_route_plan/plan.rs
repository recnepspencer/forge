use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationFoundationalEvidenceClass,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

use super::{
    checked::ForgeQueryDeclarationRoutePlanChecked,
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
    route_set::{ForgeQueryDeclarationRouteSegment, ForgeQueryDeclarationRouteSet},
};

pub struct ForgeQueryDeclarationRoutePlan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    route_set: ForgeQueryDeclarationRouteSet,
    class: ForgeQueryDeclarationRoutePlanClass,
    explanation: ForgeQueryDeclarationRoutePlanExplanation,
    declaration_digest: String,
    route_plan_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRoutePlan<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
        route_intent: Option<ForgeQueryDeclarationRouteIntent>,
        route_set: ForgeQueryDeclarationRouteSet,
        class: ForgeQueryDeclarationRoutePlanClass,
        explanation: ForgeQueryDeclarationRoutePlanExplanation,
    ) -> Self {
        let route_plan_digest =
            derive_route_plan_digest(&progressed, &evidence, route_intent, route_set.routes());
        let declaration_digest = format!(
            "{:?}",
            progressed.canonical_declaration().declaration_digest()
        );
        Self {
            progressed,
            evidence,
            route_intent,
            route_set,
            class,
            explanation,
            declaration_digest,
            route_plan_digest,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationRoutePlanClass {
        self.class
    }

    pub fn route_set(&self) -> &ForgeQueryDeclarationRouteSet {
        &self.route_set
    }

    pub fn primary_route(&self) -> Option<&ForgeQueryDeclarationRouteSegment> {
        self.route_set.primary_route()
    }

    pub fn route_count(&self) -> usize {
        self.route_set.route_count()
    }

    pub fn route_families(&self) -> &[ForgeQueryLowerAuthorityRouteFamily] {
        self.route_set.route_families()
    }

    pub fn route_intent(&self) -> Option<ForgeQueryDeclarationRouteIntent> {
        self.route_intent
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.progressed.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.progressed
            .canonical_declaration()
            .handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.progressed.operating_context_identity_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> &str {
        self.progressed.progression_digest()
    }

    pub fn route_plan_digest(&self) -> &str {
        &self.route_plan_digest
    }

    pub fn foundational_evidence(&self) -> &ForgeQueryDeclarationFoundationalEvidence<D, I> {
        &self.evidence
    }

    pub fn progressed_declaration(&self) -> &ForgeQueryAdmittedDeclarationProgression<D, I> {
        &self.progressed
    }

    pub fn explain(&self) -> &ForgeQueryDeclarationRoutePlanExplanation {
        &self.explanation
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationFoundationalEvidence<D, I>,
        Option<ForgeQueryDeclarationRouteIntent>,
        ForgeQueryDeclarationRouteSet,
        ForgeQueryDeclarationRoutePlanClass,
        ForgeQueryDeclarationRoutePlanExplanation,
        String,
        String,
    ) {
        (
            self.progressed,
            self.evidence,
            self.route_intent,
            self.route_set,
            self.class,
            self.explanation,
            self.declaration_digest,
            self.route_plan_digest,
        )
    }
}

pub(crate) fn forge_query_checked_declaration_route_plan<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationRoutePlanInput<D, I>,
) -> ForgeQueryDeclarationRoutePlanChecked<D, I> {
    let (progressed, evidence, route_intent) = input.into_parts();
    let route_contract = I::Family::route_contract();

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
        ],
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

fn derive_route_plan_digest<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    progressed: &ForgeQueryAdmittedDeclarationProgression<D, I>,
    evidence: &ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    routes: &[ForgeQueryDeclarationRouteSegment],
) -> String {
    let mut parts = vec![
        format!(
            "handle:{}",
            progressed.canonical_declaration().handle_identity_digest()
        ),
        format!(
            "operating_context:{}",
            progressed.operating_context_identity_digest()
        ),
        format!("family:{}", progressed.declaration_family_key()),
        format!(
            "declaration:{:?}",
            progressed.canonical_declaration().declaration_digest()
        ),
        format!("progression:{}", progressed.progression_digest()),
        format!("evidence:{:?}", evidence.attachment_bundle_digest()),
    ];
    if let Some(intent) = route_intent {
        parts.push(format!("intent:{}", intent.as_str()));
    }
    for route in routes {
        parts.push(format!("route:{}", route.family().as_str()));
    }
    hash_parts(&parts)
}
