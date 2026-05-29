use crate::application::{
    forge_query_checked_declaration_route_plan, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDeclarationRoutePlanTerminalError, ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn plan_routes<I>(
        &self,
        subject: ForgeQueryDeclarationRoutePlanInput<D, I>,
    ) -> Result<
        ForgeQueryDeclarationRoutePlan<D, I>,
        ForgeQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.plan_routes_checked(subject) {
            ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => Ok(plan),
            ForgeQueryDeclarationRoutePlanChecked::Deferred(plan) => {
                Err(ForgeQueryDeclarationRoutePlanTerminalError::Deferred(plan))
            }
            ForgeQueryDeclarationRoutePlanChecked::Denied(plan) => {
                Err(ForgeQueryDeclarationRoutePlanTerminalError::Denied(plan))
            }
            ForgeQueryDeclarationRoutePlanChecked::Failed(plan) => {
                Err(ForgeQueryDeclarationRoutePlanTerminalError::Failed(plan))
            }
        }
    }

    pub fn plan_routes_checked<I>(
        &self,
        subject: ForgeQueryDeclarationRoutePlanInput<D, I>,
    ) -> ForgeQueryDeclarationRoutePlanChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_route_plan(subject)
    }

    pub fn plan_routes_from_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        ForgeQueryDeclarationRoutePlan<D, I>,
        ForgeQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let evidence = self.foundational_evidence_for_progressed(progressed.clone());
        self.plan_routes(ForgeQueryDeclarationRoutePlanInput::admitted(
            progressed, evidence,
        ))
    }

    pub fn plan_routes_from_progressed_with_intent<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
        route_intent: ForgeQueryDeclarationRouteIntent,
    ) -> Result<
        ForgeQueryDeclarationRoutePlan<D, I>,
        ForgeQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let evidence = self.foundational_evidence_for_progressed(progressed.clone());
        self.plan_routes(ForgeQueryDeclarationRoutePlanInput::with_intent(
            progressed,
            evidence,
            route_intent,
        ))
    }

    pub fn declare_review_progress_describe_and_plan<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationRoutePlan<D, I>,
        crate::application::ForgeQueryDeclarationEntryRoutePlanError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let progressed = self
            .declare_review_and_progress(input)
            .map_err(crate::application::ForgeQueryDeclarationEntryRoutePlanError::Entry)?;
        self.plan_routes_from_progressed(progressed)
            .map_err(crate::application::ForgeQueryDeclarationEntryRoutePlanError::RoutePlan)
    }

    fn foundational_evidence_for_progressed<I>(
        &self,
        progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> crate::application::ForgeQueryDeclarationFoundationalEvidence<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed),
        ) {
            Ok(evidence) => evidence,
            Err(_) => panic!(
                "same-handle admitted progression should always describe foundational evidence"
            ),
        }
    }
}
