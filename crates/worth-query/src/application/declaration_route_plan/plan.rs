use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectFit, WorthQueryDeclarationAspectPublication,
    WorthQueryDeclarationFoundationalEvidence, WorthQueryDeclarationFutureProjection,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};
use crate::identity::hash_parts;
use crate::target_binding::WorthQueryDeclarationRoutePlanBindingTarget;

use super::{
    class::{WorthQueryDeclarationRoutePlanClass, WorthQueryLowerAuthorityRouteFamily},
    explain::WorthQueryDeclarationRoutePlanExplanation,
    intent::WorthQueryDeclarationRouteIntent,
    route_set::{WorthQueryDeclarationRouteSegment, WorthQueryDeclarationRouteSet},
};

pub struct WorthQueryDeclarationRoutePlan<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<WorthQueryDeclarationRouteIntent>,
    route_set: WorthQueryDeclarationRouteSet,
    class: WorthQueryDeclarationRoutePlanClass,
    automation_requires_explicit_handoff: bool,
    route_aspect_contract: WorthQueryDeclarationAspectContract,
    route_aspect_fit: WorthQueryDeclarationAspectFit,
    route_aspect_publication: WorthQueryDeclarationAspectPublication,
    future_projection: WorthQueryDeclarationFutureProjection,
    explanation: WorthQueryDeclarationRoutePlanExplanation,
    declaration_digest: String,
    route_plan_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationRoutePlan<D, I>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
        route_intent: Option<WorthQueryDeclarationRouteIntent>,
        route_set: WorthQueryDeclarationRouteSet,
        class: WorthQueryDeclarationRoutePlanClass,
        automation_requires_explicit_handoff: bool,
        route_aspect_contract: WorthQueryDeclarationAspectContract,
        route_aspect_fit: WorthQueryDeclarationAspectFit,
        route_aspect_publication: WorthQueryDeclarationAspectPublication,
        future_projection: WorthQueryDeclarationFutureProjection,
        explanation: WorthQueryDeclarationRoutePlanExplanation,
    ) -> Self {
        let route_plan_digest = derive_route_plan_digest(
            &progressed,
            &evidence,
            route_intent,
            route_set.routes(),
            &route_aspect_contract,
            route_aspect_fit,
            &route_aspect_publication,
            &future_projection,
        );
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
            automation_requires_explicit_handoff,
            route_aspect_contract,
            route_aspect_fit,
            route_aspect_publication,
            future_projection,
            explanation,
            declaration_digest,
            route_plan_digest,
        }
    }

    pub fn class(&self) -> WorthQueryDeclarationRoutePlanClass {
        self.class
    }

    pub fn route_set(&self) -> &WorthQueryDeclarationRouteSet {
        &self.route_set
    }

    pub fn primary_route(&self) -> Option<&WorthQueryDeclarationRouteSegment> {
        self.route_set.primary_route()
    }

    pub fn route_count(&self) -> usize {
        self.route_set.route_count()
    }

    pub fn route_families(&self) -> &[WorthQueryLowerAuthorityRouteFamily] {
        self.route_set.route_families()
    }

    pub fn route_intent(&self) -> Option<WorthQueryDeclarationRouteIntent> {
        self.route_intent
    }

    pub(crate) fn automation_requires_explicit_handoff(&self) -> bool {
        self.automation_requires_explicit_handoff
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.route_aspect_contract
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.route_aspect_fit
    }

    pub fn aspect_publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.route_aspect_publication
    }

    pub fn future_projection(&self) -> &WorthQueryDeclarationFutureProjection {
        &self.future_projection
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

    pub fn binding_target(&self) -> WorthQueryDeclarationRoutePlanBindingTarget {
        WorthQueryDeclarationRoutePlanBindingTarget::for_route_plan(self)
    }

    pub fn foundational_evidence(&self) -> &WorthQueryDeclarationFoundationalEvidence<D, I> {
        &self.evidence
    }

    pub fn progressed_declaration(&self) -> &WorthQueryAdmittedDeclarationProgression<D, I> {
        &self.progressed
    }

    pub fn explain(&self) -> &WorthQueryDeclarationRoutePlanExplanation {
        &self.explanation
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryAdmittedDeclarationProgression<D, I>,
        WorthQueryDeclarationFoundationalEvidence<D, I>,
        Option<WorthQueryDeclarationRouteIntent>,
        WorthQueryDeclarationRouteSet,
        WorthQueryDeclarationRoutePlanClass,
        bool,
        WorthQueryDeclarationAspectContract,
        WorthQueryDeclarationAspectFit,
        WorthQueryDeclarationAspectPublication,
        WorthQueryDeclarationFutureProjection,
        WorthQueryDeclarationRoutePlanExplanation,
        String,
        String,
    ) {
        (
            self.progressed,
            self.evidence,
            self.route_intent,
            self.route_set,
            self.class,
            self.automation_requires_explicit_handoff,
            self.route_aspect_contract,
            self.route_aspect_fit,
            self.route_aspect_publication,
            self.future_projection,
            self.explanation,
            self.declaration_digest,
            self.route_plan_digest,
        )
    }
}

fn derive_route_plan_digest<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    progressed: &WorthQueryAdmittedDeclarationProgression<D, I>,
    evidence: &WorthQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<WorthQueryDeclarationRouteIntent>,
    routes: &[WorthQueryDeclarationRouteSegment],
    route_aspect_contract: &WorthQueryDeclarationAspectContract,
    route_aspect_fit: WorthQueryDeclarationAspectFit,
    route_aspect_publication: &WorthQueryDeclarationAspectPublication,
    future_projection: &WorthQueryDeclarationFutureProjection,
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
        format!("support:{}", evidence.support_digest()),
        format!("progression:{}", progressed.progression_digest()),
        format!("route_aspect_contract:{route_aspect_contract:?}"),
        format!("route_aspect_fit:{route_aspect_fit:?}"),
        format!("route_aspect_publication:{route_aspect_publication:?}"),
        format!(
            "future_projection:{}",
            future_projection.projection_digest()
        ),
    ];
    if let Some(intent) = route_intent {
        parts.push(format!("intent:{}", intent.as_str()));
    }
    for route in routes {
        parts.push(format!("route:{}", route.family().as_str()));
    }
    hash_parts(&parts)
}
