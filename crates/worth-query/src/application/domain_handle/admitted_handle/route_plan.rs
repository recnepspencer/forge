use crate::application::{
    worth_query_checked_declaration_route_plan, worth_query_declaration_foundational_evidence,
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDeclarationRouteIntent, WorthQueryDeclarationRoutePlan,
    WorthQueryDeclarationRoutePlanChecked, WorthQueryDeclarationRoutePlanDenialCause,
    WorthQueryDeclarationRoutePlanDenied, WorthQueryDeclarationRoutePlanInput,
    WorthQueryDeclarationRoutePlanTerminalError, WorthQueryDomainEntryMarker,
};
use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::WorthQueryAdmittedConfiguredDomainHandle;
use crate::application::WorthQueryDomainOperatingContext;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn plan_routes<I>(
        &self,
        subject: WorthQueryDeclarationRoutePlanInput<D, I>,
    ) -> Result<
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match self.plan_routes_checked(subject) {
            WorthQueryDeclarationRoutePlanChecked::Planned(plan) => Ok(plan),
            WorthQueryDeclarationRoutePlanChecked::Deferred(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Deferred(plan))
            }
            WorthQueryDeclarationRoutePlanChecked::Denied(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Denied(plan))
            }
            WorthQueryDeclarationRoutePlanChecked::Failed(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Failed(plan))
            }
        }
    }

    pub fn plan_routes_checked<I>(
        &self,
        subject: WorthQueryDeclarationRoutePlanInput<D, I>,
    ) -> WorthQueryDeclarationRoutePlanChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        worth_query_checked_declaration_route_plan(subject)
    }

    pub fn plan_routes_from_progressed<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match checked_route_plan_from_progressed_with_profile(
            self,
            progressed,
            None,
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        ) {
            WorthQueryDeclarationRoutePlanChecked::Planned(plan) => Ok(plan),
            WorthQueryDeclarationRoutePlanChecked::Deferred(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Deferred(plan))
            }
            WorthQueryDeclarationRoutePlanChecked::Denied(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Denied(plan))
            }
            WorthQueryDeclarationRoutePlanChecked::Failed(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Failed(plan))
            }
        }
    }

    pub fn plan_routes_from_progressed_with_intent<I>(
        &self,
        progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
        route_intent: WorthQueryDeclarationRouteIntent,
    ) -> Result<
        WorthQueryDeclarationRoutePlan<D, I>,
        WorthQueryDeclarationRoutePlanTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        match checked_route_plan_from_progressed_with_profile(
            self,
            progressed,
            Some(route_intent),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        ) {
            WorthQueryDeclarationRoutePlanChecked::Planned(plan) => Ok(plan),
            WorthQueryDeclarationRoutePlanChecked::Deferred(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Deferred(plan))
            }
            WorthQueryDeclarationRoutePlanChecked::Denied(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Denied(plan))
            }
            WorthQueryDeclarationRoutePlanChecked::Failed(plan) => {
                Err(WorthQueryDeclarationRoutePlanTerminalError::Failed(plan))
            }
        }
    }

    pub fn declare_review_progress_describe_and_plan<I>(
        &self,
        input: I,
    ) -> Result<
        WorthQueryDeclarationRoutePlan<D, I>,
        crate::application::WorthQueryDeclarationEntryRoutePlanError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
    {
        let progressed = self
            .declare_review_and_progress(input)
            .map_err(crate::application::WorthQueryDeclarationEntryRoutePlanError::Entry)?;
        self.plan_routes_from_progressed(progressed)
            .map_err(crate::application::WorthQueryDeclarationEntryRoutePlanError::RoutePlan)
    }
}

pub(crate) fn checked_route_plan_from_progressed_with_profile<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<WorthQueryDeclarationRouteIntent>,
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> WorthQueryDeclarationRoutePlanChecked<D, I> {
    if progressed.canonical_declaration().handle_identity_digest()
        != handle.handle_identity_digest()
        || progressed.operating_context_identity_digest()
            != handle.operating_context_identity_digest()
    {
        let world_basis = progressed.retained_world_basis().clone();
        let evidence = worth_query_declaration_foundational_evidence(
            &world_basis,
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            profile,
        )
        .unwrap_or_else(|_| {
            panic!(
                "retained admitted progression should describe foundational evidence inside its own admitted world"
            )
        });
        return WorthQueryDeclarationRoutePlanChecked::Denied(
            WorthQueryDeclarationRoutePlanDenied::new(
                progressed,
                evidence,
                route_intent,
                I::Family::route_contract(),
                WorthQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld,
            ),
        );
    }

    let evidence = handle
        .describe_foundational_with_profile(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            profile,
        )
        .unwrap_or_else(|_| {
            panic!("same-handle admitted progression should always describe foundational evidence")
        });
    let input = match route_intent {
        Some(intent) => {
            WorthQueryDeclarationRoutePlanInput::with_intent(progressed, evidence, intent)
        }
        None => WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    };
    worth_query_checked_declaration_route_plan(input)
}
