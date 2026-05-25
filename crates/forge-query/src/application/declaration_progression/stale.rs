use forge_proof::{
    Lowered, ProofOutcomeKind, Recipe, RecipeStageDxExt, RecipeStageKind, StaleReadableBasis,
};

use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationLegalityEvidence, ForgeQueryDomainEntryMarker,
};

use super::payload::{derive_progression_digest, ForgeQueryDeclarationProgressionPayload};
use super::review::{
    ForgeQueryDeclarationProgressionBasis, ForgeQueryDeclarationProgressionContractClass,
    ForgeQueryDeclarationProgressionOutcomeView,
};

pub struct ForgeQueryDeclarationProgressionStale<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    recipe: Recipe<
        Lowered,
        ForgeQueryDeclarationProgressionPayload<D, I>,
        StaleReadableBasis<ForgeQueryDeclarationProgressionBasis>,
    >,
    progression_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationProgressionStale<D, I>
{
    pub(crate) fn new(
        recipe: Recipe<
            Lowered,
            ForgeQueryDeclarationProgressionPayload<D, I>,
            StaleReadableBasis<ForgeQueryDeclarationProgressionBasis>,
        >,
    ) -> Self {
        let progression_digest = derive_progression_digest(
            recipe.payload(),
            ForgeQueryDeclarationProgressionContractClass::Stale,
        );
        Self {
            recipe,
            progression_digest,
        }
    }

    pub fn legality_evidence(&self) -> &ForgeQueryDeclarationLegalityEvidence<D, I> {
        self.recipe.payload().legality_evidence()
    }

    pub fn support_report(
        &self,
    ) -> &crate::application::ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
        self.legality_evidence().support_report()
    }

    pub fn legality_contract(&self) -> ForgeQueryDeclarationLegalityContract {
        self.legality_evidence().legality_contract()
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.legality_evidence().declaration_family_key()
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn outcome(&self) -> ForgeQueryDeclarationProgressionOutcomeView {
        ForgeQueryDeclarationProgressionOutcomeView::new(ProofOutcomeKind::Stale)
    }

    pub fn stage(&self) -> RecipeStageKind {
        self.recipe.stage()
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        self.recipe.payload().operating_context_identity_digest()
    }
}
