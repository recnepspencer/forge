use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationFoundationalEvidence,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::intent::WorthQueryDeclarationRouteIntent;

pub struct WorthQueryDeclarationRoutePlanInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<WorthQueryDeclarationRouteIntent>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationRoutePlanInput<D, I>
{
    pub fn admitted(
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
    ) -> Self {
        Self {
            progressed,
            evidence,
            route_intent: None,
        }
    }

    pub fn with_intent(
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        evidence: WorthQueryDeclarationFoundationalEvidence<D, I>,
        route_intent: WorthQueryDeclarationRouteIntent,
    ) -> Self {
        Self {
            progressed,
            evidence,
            route_intent: Some(route_intent),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryAdmittedDeclarationProgression<D, I>,
        WorthQueryDeclarationFoundationalEvidence<D, I>,
        Option<WorthQueryDeclarationRouteIntent>,
    ) {
        (self.progressed, self.evidence, self.route_intent)
    }
}
