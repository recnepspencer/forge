use worth_proof::{recipe, Recipe, RecipeStageDxExt, RecipeStageKind, Unresolved};

use crate::application::{
    WorthQueryAdmittedWorldBasis, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityEvidence, WorthQueryDomainEntryMarker,
};

use super::payload::WorthQueryDeclarationProgressionPayload;

pub type WorthQueryDeclarationProgressionRawRecipe<D, I> =
    Recipe<Unresolved, WorthQueryDeclarationProgressionPayload<D, I>>;

pub struct WorthQueryDeclarationProgressionRecipe<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    raw: WorthQueryDeclarationProgressionRawRecipe<D, I>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationProgressionRecipe<D, I>
{
    pub(crate) fn new(raw: WorthQueryDeclarationProgressionRawRecipe<D, I>) -> Self {
        Self { raw }
    }

    pub fn stage(&self) -> RecipeStageKind {
        self.raw.stage()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.raw.payload().declaration_family_key()
    }

    pub(crate) fn into_raw(self) -> WorthQueryDeclarationProgressionRawRecipe<D, I> {
        self.raw
    }
}

pub(crate) fn worth_query_declaration_progression_recipe<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    legality_evidence: WorthQueryDeclarationLegalityEvidence<D, I>,
    world_basis: WorthQueryAdmittedWorldBasis,
) -> WorthQueryDeclarationProgressionRecipe<D, I> {
    let payload = WorthQueryDeclarationProgressionPayload::new(legality_evidence, world_basis);
    WorthQueryDeclarationProgressionRecipe::new(recipe(payload))
}
