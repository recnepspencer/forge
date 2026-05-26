use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationFoundationalEvidence,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::intent::ForgeQueryDeclarationRouteIntent;

pub struct ForgeQueryDeclarationRoutePlanInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRoutePlanInput<D, I>
{
    pub fn admitted(
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
    ) -> Self {
        Self {
            progressed,
            evidence,
            route_intent: None,
        }
    }

    pub fn with_intent(
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        evidence: ForgeQueryDeclarationFoundationalEvidence<D, I>,
        route_intent: ForgeQueryDeclarationRouteIntent,
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
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationFoundationalEvidence<D, I>,
        Option<ForgeQueryDeclarationRouteIntent>,
    ) {
        (self.progressed, self.evidence, self.route_intent)
    }
}
