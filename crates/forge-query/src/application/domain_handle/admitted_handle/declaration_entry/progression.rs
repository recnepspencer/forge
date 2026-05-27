use crate::application::{
    forge_query_checked_declaration_progression, forge_query_declaration_progression_recipe,
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityEvidence,
    ForgeQueryDeclarationProgressionChecked, ForgeQueryDeclarationProgressionRecipe,
    ForgeQueryDeclarationProgressionTerminalError, ForgeQueryDomainEntryMarker,
};

use super::super::ForgeQueryAdmittedConfiguredDomainHandle;
use crate::application::ForgeQueryDomainOperatingContext;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn declaration_progression_recipe<I>(
        &self,
        legal: ForgeQueryDeclarationLegalityEvidence<D, I>,
    ) -> ForgeQueryDeclarationProgressionRecipe<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_declaration_progression_recipe(
            legal,
            self.operating_context_identity_digest().to_string(),
        )
    }

    pub fn progress_declaration<I>(
        &self,
        legal: ForgeQueryDeclarationLegalityEvidence<D, I>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationProgressionTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match self.progress_declaration_checked(legal) {
            ForgeQueryDeclarationProgressionChecked::Admitted(admitted) => Ok(admitted),
            ForgeQueryDeclarationProgressionChecked::Deferred(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Deferred(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Denied(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Denied(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Stale(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Stale(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
                Err(ForgeQueryDeclarationProgressionTerminalError::RebindRequired(progress))
            }
            ForgeQueryDeclarationProgressionChecked::Failed(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Failed(progress),
            ),
        }
    }

    pub fn progress_declaration_recipe<I>(
        &self,
        recipe: ForgeQueryDeclarationProgressionRecipe<D, I>,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationProgressionTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        match forge_query_checked_declaration_progression(recipe) {
            ForgeQueryDeclarationProgressionChecked::Admitted(admitted) => Ok(admitted),
            ForgeQueryDeclarationProgressionChecked::Deferred(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Deferred(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Denied(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Denied(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::Stale(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Stale(progress),
            ),
            ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
                Err(ForgeQueryDeclarationProgressionTerminalError::RebindRequired(progress))
            }
            ForgeQueryDeclarationProgressionChecked::Failed(progress) => Err(
                ForgeQueryDeclarationProgressionTerminalError::Failed(progress),
            ),
        }
    }

    pub fn progress_declaration_checked<I>(
        &self,
        legal: ForgeQueryDeclarationLegalityEvidence<D, I>,
    ) -> ForgeQueryDeclarationProgressionChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_progression(self.declaration_progression_recipe(legal))
    }

    pub fn progress_declaration_recipe_checked<I>(
        &self,
        recipe: ForgeQueryDeclarationProgressionRecipe<D, I>,
    ) -> ForgeQueryDeclarationProgressionChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        forge_query_checked_declaration_progression(recipe)
    }

    pub fn declare_review_and_progress<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationEntryProgressionError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        let legal = self
            .declare_and_review(input)
            .map_err(ForgeQueryDeclarationEntryProgressionError::Entry)?;
        self.progress_declaration(legal)
            .map_err(ForgeQueryDeclarationEntryProgressionError::Progression)
    }
}

pub enum ForgeQueryDeclarationEntryProgressionError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Entry(ForgeQueryDeclarationAdmissionOrLegalityError<D, I>),
    Progression(ForgeQueryDeclarationProgressionTerminalError<D, I>),
}
