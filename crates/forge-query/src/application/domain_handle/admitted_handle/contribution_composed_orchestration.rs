use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::contribution_composed_orchestration::{
    orchestrate_declaration_with_contributions_on_handle,
    ordinary_outcome_from_contribution_composed_checked,
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationTranscript,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn orchestrate_declaration_with_contributions<I>(
        &self,
        input: ForgeQueryContributionComposedOrchestrationInput<D, I>,
    ) -> Result<
        ForgeQueryContributionComposedOrchestration<D, I>,
        ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match orchestrate_declaration_with_contributions_on_handle(self, input).into_checked() {
            ForgeQueryContributionComposedOrchestrationOutcome::Bound(value) => Ok(value),
            other => Err(other),
        }
    }

    pub fn orchestrate_declaration_with_contributions_outcome<I>(
        &self,
        input: ForgeQueryContributionComposedOrchestrationInput<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryContributionComposedOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_contribution_composed_checked(
            orchestrate_declaration_with_contributions_on_handle(self, input).into_checked(),
        )
    }

    pub fn orchestrate_declaration_with_contributions_checked<I>(
        &self,
        input: ForgeQueryContributionComposedOrchestrationInput<D, I>,
    ) -> ForgeQueryContributionComposedOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        orchestrate_declaration_with_contributions_on_handle(self, input).into_checked()
    }

    pub fn orchestrate_declaration_with_contributions_proof<I>(
        &self,
        input: ForgeQueryContributionComposedOrchestrationInput<D, I>,
    ) -> ForgeQueryContributionComposedOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        orchestrate_declaration_with_contributions_on_handle(self, input)
    }
}
