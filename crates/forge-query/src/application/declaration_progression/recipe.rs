use forge_proof::{recipe, Recipe, RecipeStageDxExt, RecipeStageKind, Unresolved};

use crate::application::{
    ForgeQueryAdmittedWorldBasis, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDomainEntryMarker,
};

use super::payload::ForgeQueryDeclarationProgressionPayload;

pub type ForgeQueryDeclarationProgressionRawRecipe<D, I> =
    Recipe<Unresolved, ForgeQueryDeclarationProgressionPayload<D, I>>;

pub struct ForgeQueryDeclarationProgressionRecipe<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    raw: ForgeQueryDeclarationProgressionRawRecipe<D, I>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationProgressionRecipe<D, I>
{
    pub(crate) fn new(raw: ForgeQueryDeclarationProgressionRawRecipe<D, I>) -> Self {
        Self { raw }
    }

    pub fn stage(&self) -> RecipeStageKind {
        self.raw.stage()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.raw.payload().declaration_family_key()
    }

    pub(crate) fn into_raw(self) -> ForgeQueryDeclarationProgressionRawRecipe<D, I> {
        self.raw
    }
}

pub(crate) fn forge_query_declaration_progression_recipe<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    legality_evidence: ForgeQueryDeclarationLegalityEvidence<D, I>,
    world_basis: ForgeQueryAdmittedWorldBasis,
) -> ForgeQueryDeclarationProgressionRecipe<D, I> {
    let payload = ForgeQueryDeclarationProgressionPayload::new(legality_evidence, world_basis);
    ForgeQueryDeclarationProgressionRecipe::new(recipe(payload))
}
