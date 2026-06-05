use crate::application::{
    forge_query_checked_declaration_route_plan, forge_query_declaration_foundational_evidence,
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRouteIntent, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDeclarationRoutePlanDenied, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDeclarationRoutePlanTerminalError, ForgeQueryDomainEntryMarker,
};
use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

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
        match checked_route_plan_from_progressed_with_profile(
            self,
            progressed,
            None,
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        ) {
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
        match checked_route_plan_from_progressed_with_profile(
            self,
            progressed,
            Some(route_intent),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        ) {
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
}

pub(crate) fn checked_route_plan_from_progressed_with_profile<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    route_intent: Option<ForgeQueryDeclarationRouteIntent>,
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> ForgeQueryDeclarationRoutePlanChecked<D, I> {
    if progressed.canonical_declaration().handle_identity_digest()
        != handle.handle_identity_digest()
        || progressed.operating_context_identity_digest()
            != handle.operating_context_identity_digest()
    {
        let world_basis = crate::application::ForgeQueryAdmittedWorldBasis::new(
            handle.domain_key(),
            handle.display_name(),
            progressed.operating_context_identity_digest().to_string(),
            progressed
                .canonical_declaration()
                .handle_identity_digest()
                .to_string(),
            progressed.support_report().support_digest().to_string(),
        );
        let evidence = forge_query_declaration_foundational_evidence(
            &world_basis,
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            profile,
        )
        .unwrap_or_else(|_| {
            panic!("retained admitted progression should describe foundational evidence inside its own admitted world")
        });
        return ForgeQueryDeclarationRoutePlanChecked::Denied(
            ForgeQueryDeclarationRoutePlanDenied::new(
                progressed,
                evidence,
                route_intent,
                I::Family::route_contract(),
                ForgeQueryDeclarationRoutePlanDenialCause::WrongAdmittedWorld,
            ),
        );
    }

    let evidence = handle
        .describe_foundational_with_profile(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            profile,
        )
        .unwrap_or_else(|_| {
            panic!("same-handle admitted progression should always describe foundational evidence")
        });
    let input = match route_intent {
        Some(intent) => {
            ForgeQueryDeclarationRoutePlanInput::with_intent(progressed, evidence, intent)
        }
        None => ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    };
    forge_query_checked_declaration_route_plan(input)
}
