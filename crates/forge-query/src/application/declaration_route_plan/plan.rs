use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationAspectPublication,
    ForgeQueryDeclarationFoundationalEvidence, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;
use crate::target_binding::ForgeQueryDeclarationRoutePlanBindingTarget;

use super::{
    class::{ForgeQueryDeclarationRoutePlanClass, ForgeQueryLowerAuthorityRouteFamily},
    explain::ForgeQueryDeclarationRoutePlanExplanation,
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
    automation_requires_explicit_handoff: bool,
    route_aspect_contract: ForgeQueryDeclarationAspectContract,
    route_aspect_fit: ForgeQueryDeclarationAspectFit,
    route_aspect_publication: ForgeQueryDeclarationAspectPublication,
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
        automation_requires_explicit_handoff: bool,
        route_aspect_contract: ForgeQueryDeclarationAspectContract,
        route_aspect_fit: ForgeQueryDeclarationAspectFit,
        route_aspect_publication: ForgeQueryDeclarationAspectPublication,
        explanation: ForgeQueryDeclarationRoutePlanExplanation,
    ) -> Self {
        let route_plan_digest = derive_route_plan_digest(
            &progressed,
            &evidence,
            route_intent,
            route_set.routes(),
            &route_aspect_contract,
            route_aspect_fit,
            &route_aspect_publication,
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

    pub(crate) fn automation_requires_explicit_handoff(&self) -> bool {
        self.automation_requires_explicit_handoff
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.route_aspect_contract
    }

    pub fn aspect_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.route_aspect_fit
    }

    pub fn aspect_publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        &self.route_aspect_publication
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

    pub fn binding_target(&self) -> ForgeQueryDeclarationRoutePlanBindingTarget {
        ForgeQueryDeclarationRoutePlanBindingTarget::for_route_plan(self)
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
        bool,
        ForgeQueryDeclarationAspectContract,
        ForgeQueryDeclarationAspectFit,
        ForgeQueryDeclarationAspectPublication,
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
            self.automation_requires_explicit_handoff,
            self.route_aspect_contract,
            self.route_aspect_fit,
            self.route_aspect_publication,
            self.explanation,
            self.declaration_digest,
            self.route_plan_digest,
        )
    }
}

fn derive_route_plan_digest<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    progressed: &ForgeQueryAdmittedDeclarationProgression<D, I>,
    evidence: &ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    routes: &[ForgeQueryDeclarationRouteSegment],
    route_aspect_contract: &ForgeQueryDeclarationAspectContract,
    route_aspect_fit: ForgeQueryDeclarationAspectFit,
    route_aspect_publication: &ForgeQueryDeclarationAspectPublication,
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
    ];
    if let Some(intent) = route_intent {
        parts.push(format!("intent:{}", intent.as_str()));
    }
    for route in routes {
        parts.push(format!("route:{}", route.family().as_str()));
    }
    hash_parts(&parts)
}
